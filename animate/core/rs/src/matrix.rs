//! 🔢 Matrix types as gridded Sobject groups.

use crate::color::Color;
use crate::geometry::rectangle;
use crate::sobject::{arrange, Group, Sobject};
use crate::text::{MathText, Text};
use mathematical_geometry::{Point, Vec2};

/// 📊 Matrix of string entries with optional brackets.
pub struct Matrix {
    pub group: Group,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    pub fn from_rows(rows: Vec<Vec<String>>, cell_size: (f64, f64), color: Color) -> Self {
        let nrows = rows.len();
        let ncols = rows.first().map(|r| r.len()).unwrap_or(0);
        let mut children: Vec<Box<dyn Sobject>> = Vec::new();
        for row in rows {
            for cell in row {
                let mut t = Text::new(cell, color);
                children.push(Box::new(t.inner));
            }
        }
        let mut group = Group::new(children);
        arrange(&mut group, Vec2::new(1.0, 0.0), cell_size.0 * 0.2);
        Self {
            group,
            rows: nrows,
            cols: ncols,
        }
    }

    pub fn math(entries: &[&str], cell_size: (f64, f64), color: Color) -> Self {
        let mut children: Vec<Box<dyn Sobject>> = entries
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
        Self {
            group,
            rows,
            cols,
        }
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
        let rows: Vec<Vec<String>> = self
            .values
            .iter()
            .map(|row| row.iter().map(|v| format!("{v:.2}")).collect())
            .collect();
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
}
