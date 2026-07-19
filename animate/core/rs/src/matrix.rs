//! 🔢 Matrix types as gridded Sobject groups.

use crate::color::Color;
use crate::geometry::rectangle;
use crate::sobject::{arrange, Group, Sobject};
use crate::text::{MathText, Text};
use mathematical_geometry::{Point, Vec2};

fn arrange_grid(group: &mut Group, rows: usize, cols: usize, cell_size: (f64, f64)) {
    if group.children.is_empty() || rows == 0 || cols == 0 {
        return;
    }
    let origin = group.children[0].center();
    for (idx, child) in group.children.iter_mut().enumerate() {
        let row = idx / cols;
        let col = idx % cols;
        let x = origin.x() + col as f64 * cell_size.0;
        let y = origin.y() - row as f64 * cell_size.1;
        child.move_to(Point::new(x, y));
    }
}

/// 📊 Matrix of string entries with optional brackets.
pub struct Matrix {
    pub group: Group,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    pub fn from_rows(rows: Vec<Vec<String>>, cell_size: (f64, f64), color: Color) -> Self {
        let nrows = rows.len();
        let ncols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
        let mut children: Vec<Box<dyn Sobject>> = Vec::new();
        for row in rows {
            for cell in row {
                let t = Text::new(cell, color);
                children.push(Box::new(t.inner));
            }
        }
        let mut group = Group::new(children);
        arrange_grid(&mut group, nrows, ncols, cell_size);
        Self { group, rows: nrows, cols: ncols }
    }

    pub fn math(entries: &[&str], cell_size: (f64, f64), color: Color) -> Self {
        let children: Vec<Box<dyn Sobject>> = entries
            .iter()
            .map(|e| {
                let m = MathText::new(*e, color);
                Box::new(m.inner) as Box<dyn Sobject>
            })
            .collect();
        let cols = (entries.len() as f64).sqrt().ceil() as usize;
        let rows = entries.len().div_ceil(cols);
        let mut group = Group::new(children);
        arrange(&mut group, Vec2::new(1.0, 0.0), cell_size.0 * 0.15);
        Self { group, rows, cols }
    }

    pub fn with_brackets(mut self, color: Color, padding: f64) -> Self {
        let b = self.group.bounds();
        let w = b.width() + padding * 2.0;
        let h = b.height() + padding * 2.0;
        let c = b.center();
        let frame = rectangle(w, h, c, Color::TRANSPARENT, Some(color), 3.0);
        self.group.add_child(Box::new(frame));
        self
    }
}

/// 📋 Table with header row and body rows in a 2D grid.
pub struct Table {
    pub group: Group,
    pub rows: usize,
    pub cols: usize,
}

impl Table {
    pub fn new(headers: Vec<String>, rows: &[Vec<String>], cell_size: (f64, f64), color: Color) -> Self {
        let ncols = headers.len().max(rows.iter().map(|r| r.len()).max().unwrap_or(0));
        let nrows = rows.len() + 1;
        let mut children: Vec<Box<dyn Sobject>> = Vec::new();
        for header in headers {
            children.push(Box::new(Text::new(header, color).inner));
        }
        for row in rows {
            for cell in row {
                children.push(Box::new(Text::new(cell.clone(), color).inner));
            }
            let pad = ncols.saturating_sub(row.len());
            for _ in 0..pad {
                children.push(Box::new(Text::new("", color).inner));
            }
        }
        let mut group = Group::new(children);
        arrange_grid(&mut group, nrows, ncols, cell_size);
        Self { group, rows: nrows, cols: ncols }
    }

    pub fn with_frame(mut self, color: Color, padding: f64) -> Self {
        let b = self.group.bounds();
        let frame = rectangle(b.width() + padding * 2.0, b.height() + padding * 2.0, b.center(), Color::TRANSPARENT, Some(color), 2.0);
        self.group.add_child(Box::new(frame));
        self
    }
}

/// 🧮 Decimal matrix for numeric interpolation animations.
#[derive(Clone, Debug)]
pub struct DecimalMatrix {
    pub values: Vec<Vec<f64>>,
}

impl DecimalMatrix {
    pub fn new(values: Vec<Vec<f64>>) -> Self {
        Self { values }
    }

    pub fn lerp(&self, other: &Self, t: f64) -> Self {
        let rows = self.values.len().min(other.values.len());
        let mut out = Vec::with_capacity(rows);
        for r in 0..rows {
            let cols = self.values[r].len().min(other.values[r].len());
            let mut row = Vec::with_capacity(cols);
            for c in 0..cols {
                let a = self.values[r][c];
                let b = other.values[r][c];
                row.push(a + (b - a) * t);
            }
            out.push(row);
        }
        Self { values: out }
    }

    pub fn to_matrix_sobject(&self, cell_size: (f64, f64), color: Color) -> Matrix {
        let rows: Vec<Vec<String>> = self.values.iter().map(|row| row.iter().map(|v| format!("{v:.2}")).collect()).collect();
        Matrix::from_rows(rows, cell_size, color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_matrix_lerps() {
        let a = DecimalMatrix::new(vec![vec![0.0, 1.0]]);
        let b = DecimalMatrix::new(vec![vec![2.0, 3.0]]);
        let m = a.lerp(&b, 0.5);
        assert!((m.values[0][0] - 1.0).abs() < 1e-9);
    }

    #[test]
    fn matrix_grid_layout() {
        let m = Matrix::from_rows(vec![vec!["a".into(), "b".into()], vec!["c".into(), "d".into()]], (1.0, 1.0), Color::WHITE);
        assert_eq!(m.rows, 2);
        assert_eq!(m.cols, 2);
        assert_eq!(m.group.children.len(), 4);
    }

    #[test]
    fn table_has_header_and_rows() {
        let t = Table::new(vec!["x".into()], &[vec!["1".into()]], (1.0, 1.0), Color::WHITE);
        assert_eq!(t.rows, 2);
        assert_eq!(t.cols, 1);
    }
}
