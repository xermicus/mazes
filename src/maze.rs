use rand::prelude::*;
use std::fmt;

#[derive(Clone)]
pub struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let mut cells = Vec::with_capacity(width * height);
        for _ in 0..height {
            for _ in 0..width {
                cells.push(Cell::default())
            }
        }
        Grid {
            width,
            height,
            cells,
        }
    }

    fn cell_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(self.cell_index(x, y))
    }

    pub fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        let index = &self.cell_index(x, y);
        self.cells.get_mut(*index)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let buf_height = (self.height * 2) + 1;
        let buf_width = (self.width * 4) + 2;
        let mut buf = String::with_capacity(buf_height * buf_width);
        let cell_top_closed = "---+";
        let cell_body_closed = "   |";
        let cell_top_open = "   +";
        let cell_body_open = "    ";

        for y in 0..self.height {
            buf.push_str("+");
            for x in 0..self.width {
                if self.cell(x, self.height - 1 - y).unwrap().top {
                    buf.push_str(cell_top_open)
                } else {
                    buf.push_str(cell_top_closed)
                }
            }
            buf.push_str("\n|");
            for x in 0..self.width {
                if self.cell(x, self.height - 1 - y).unwrap().right {
                    buf.push_str(cell_body_open);
                } else {
                    buf.push_str(cell_body_closed)
                }
            }
            buf.push_str("\n");
            if y == self.height - 1 {
                buf.push_str("+");
                for _ in 0..self.width {
                    buf.push_str(cell_top_closed)
                }
                buf.push_str("\n");
            }
        }

        write!(f, "{}", &buf)
    }
}

#[derive(Clone, Default)]
pub struct Cell {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}

pub trait GeneratorAlgorithm {
    fn create(width: usize, height: usize) -> Self;
}

pub struct BinaryTree {
    pub grid: Grid,
}

impl GeneratorAlgorithm for BinaryTree {
    fn create(width: usize, height: usize) -> Self {
        let mut grid = Grid::new(width, height);
        let mut rng = thread_rng();
        for y in 0..grid.height {
            for x in 0..grid.width {
                let head: bool = rng.gen();
                if y < grid.height - 1 && (head || x == grid.width - 1) {
                    grid.cell_mut(x, y).unwrap().top = true;
                    grid.cell_mut(x, y + 1).unwrap().bottom = true;
                }
                if x < grid.width - 1 && (!head || y == grid.height - 1) {
                    grid.cell_mut(x, y).unwrap().right = true;
                    grid.cell_mut(x + 1, y).unwrap().left = true;
                }
            }
        }
        BinaryTree { grid }
    }
}

impl fmt::Display for BinaryTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BinaryTree {}x{} Maze:\n{}",
            self.grid.width, self.grid.height, self.grid
        )
    }
}

pub struct SideWinder {
    pub grid: Grid,
}

impl GeneratorAlgorithm for SideWinder {
    fn create(width: usize, height: usize) -> Self {
        let mut grid = Grid::new(width, height);
        let mut rng = thread_rng();
        let mut run = Vec::new();
        for y in 0..grid.height {
            for x in 0..grid.width {
                let head: bool = rng.gen();
                run.push((x, y));
                if y < grid.height - 1 && (head || x == grid.width - 1) {
                    let cand = run.iter().choose(&mut rng).unwrap();
                    grid.cell_mut(cand.0, cand.1).unwrap().top = true;
                    grid.cell_mut(cand.0, cand.1 + 1).unwrap().bottom = true;
                    run = Vec::new();
                }
                if x < grid.width - 1 && (!head || y == grid.height - 1) {
                    grid.cell_mut(x, y).unwrap().right = true;
                    grid.cell_mut(x + 1, y).unwrap().left = true;
                }
            }
        }
        SideWinder { grid }
    }
}

impl fmt::Display for SideWinder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SideWinder {}x{} Maze:\n{}",
            self.grid.width, self.grid.height, self.grid
        )
    }
}
