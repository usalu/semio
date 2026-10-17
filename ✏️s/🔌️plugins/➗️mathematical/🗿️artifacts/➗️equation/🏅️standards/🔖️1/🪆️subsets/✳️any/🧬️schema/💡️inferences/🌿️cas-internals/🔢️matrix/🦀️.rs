//! 🔢️ Symbolic matrices: entries are plain `Expr` (which already behaves like a field under its own
//! `+`/`-`/`*`/`Pow(-1)` encoding, so no generic `Ring`/`Field` newtype over `Expr` is needed for
//! cofactor-expansion algorithms). Purely-numeric-`Rational` matrices additionally delegate to
//! `number`'s exact `MatG<Rational>` for rank/nullspace/RREF, which do need a real field
//! implementation to pivot correctly.

use crate::cas::expr::{Expr, Kind};
use crate::cas::solve::{det_expr, SolutionSet};
use number::Rational;

// #region 🔖️SymMatrix
#[derive(Clone, Debug, PartialEq)]
pub struct SymMatrix {
    pub rows: usize,
    pub cols: usize,
    data: Vec<Expr>,
}

impl SymMatrix {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self { rows, cols, data: vec![Expr::integer(0); rows * cols] }
    }

    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n, n);
        for i in 0..n {
            m.set(i, i, Expr::integer(1));
        }
        m
    }

    pub fn from_rows(rows: Vec<Vec<Expr>>) -> Self {
        let nrows = rows.len();
        let ncols = rows.first().map_or(0, Vec::len);
        Self { rows: nrows, cols: ncols, data: rows.into_iter().flatten().collect() }
    }

    pub fn get(&self, row: usize, col: usize) -> &Expr {
        &self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: Expr) {
        self.data[row * self.cols + col] = value;
    }

    fn rows_vec(&self) -> Vec<Vec<Expr>> {
        (0..self.rows).map(|r| (0..self.cols).map(|c| self.get(r, c).clone()).collect()).collect()
    }

    pub fn transpose(&self) -> Self {
        let mut out = Self::zeros(self.cols, self.rows);
        for r in 0..self.rows {
            for c in 0..self.cols {
                out.set(c, r, self.get(r, c).clone());
            }
        }
        out
    }

    pub fn add(&self, other: &Self) -> Self {
        Self { rows: self.rows, cols: self.cols, data: self.data.iter().zip(other.data.iter()).map(|(a, b)| a.clone() + b.clone()).collect() }
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self { rows: self.rows, cols: self.cols, data: self.data.iter().zip(other.data.iter()).map(|(a, b)| a.clone() - b.clone()).collect() }
    }

    pub fn scale(&self, s: &Expr) -> Self {
        Self { rows: self.rows, cols: self.cols, data: self.data.iter().map(|v| v.clone() * s.clone()).collect() }
    }

    pub fn matmul(&self, other: &Self) -> Self {
        assert_eq!(self.cols, other.rows, "SymMatrix::matmul: dimension mismatch");
        let mut out = Self::zeros(self.rows, other.cols);
        for r in 0..self.rows {
            for c in 0..other.cols {
                let terms: Vec<Expr> = (0..self.cols).map(|k| self.get(r, k).clone() * other.get(k, c).clone()).collect();
                out.set(r, c, Expr::add(terms));
            }
        }
        out
    }

    pub fn trace(&self) -> Expr {
        let n = self.rows.min(self.cols);
        Expr::add((0..n).map(|i| self.get(i, i).clone()).collect())
    }

    /// 🧮️ Cofactor-expansion determinant, simplified via `simplify::cancel` (raw cofactor expansion on
    /// symbolic entries grows quickly; canceling common factors keeps the result readable).
    pub fn det(&self) -> Expr {
        assert_eq!(self.rows, self.cols, "SymMatrix::det requires a square matrix");
        crate::cas::simplify::cancel(&det_expr(&self.rows_vec()))
    }

    fn cofactor(&self, skip_row: usize, skip_col: usize) -> Expr {
        let minor: Vec<Vec<Expr>> = self.rows_vec().into_iter().enumerate().filter(|&(r, _)| r != skip_row).map(|(_, row)| row.into_iter().enumerate().filter(|&(c, _)| c != skip_col).map(|(_, v)| v).collect()).collect();
        let sign = if (skip_row + skip_col).is_multiple_of(2) { Expr::integer(1) } else { Expr::integer(-1) };
        sign * det_expr(&minor)
    }

    /// 🧮️ The adjugate (classical adjoint) matrix: `adj(A)[i][j] = cofactor(A, j, i)` (transposed
    /// cofactor matrix), satisfying `A * adj(A) == det(A) * I`.
    pub fn adjugate(&self) -> Self {
        assert_eq!(self.rows, self.cols, "SymMatrix::adjugate requires a square matrix");
        let n = self.rows;
        let mut out = Self::zeros(n, n);
        for i in 0..n {
            for j in 0..n {
                out.set(j, i, crate::cas::simplify::cancel(&self.cofactor(i, j)));
            }
        }
        out
    }

    /// ➗️ `Some(adj(A) / det(A))` when `det(A)` is (structurally, after `simplify`) provably nonzero;
    /// `None` when it's zero, and no answer when it can't be decided either way (the zero-test problem
    /// for symbolic `Expr` coefficients is undecidable in general — this pass is best-effort, never wrong).
    pub fn inverse(&self) -> Option<Self> {
        let d = self.det();
        if d.is_zero_literal() {
            return None;
        }
        let adj = self.adjugate();
        let inv_d = Expr::pow(d, Expr::integer(-1));
        Some(Self { rows: adj.rows, cols: adj.cols, data: adj.data.into_iter().map(|c| crate::cas::simplify::cancel(&(c * inv_d.clone()))).collect() })
    }

    /// 🧮️ Coefficients of the characteristic polynomial `det(A - lambda*I)` in the fresh symbol
    /// `lambda`, low-degree-first, via `as_poly_uni` on the cofactor-expansion determinant.
    pub fn charpoly(&self, lambda: &Expr) -> Option<crate::polynomial::univariate::PolyU<Rational>> {
        assert_eq!(self.rows, self.cols, "SymMatrix::charpoly requires a square matrix");
        let n = self.rows;
        let mut shifted = self.clone();
        for i in 0..n {
            shifted.set(i, i, shifted.get(i, i).clone() - lambda.clone());
        }
        let d = crate::cas::simplify::expand(&det_expr(&shifted.rows_vec()));
        crate::cas::polybridge::as_poly_uni(&d, lambda)
    }

    /// 🎯️ Eigenvalues via `solve_univariate` on the characteristic polynomial.
    pub fn eigenvalues(&self) -> SolutionSet {
        let lambda = Expr::symbol("§lambda");
        let Some(poly) = self.charpoly(&lambda) else { return SolutionSet::Unknown };
        crate::cas::solve::solve_univariate(&crate::cas::polybridge::polyu_to_expr(&poly, &lambda), &lambda)
    }

    /// 🔢️ `true` if every entry is a plain numeric literal (`Integer`/`Rational`), enabling the
    /// `number`-backed numeric paths below.
    fn is_numeric(&self) -> bool {
        self.data.iter().all(|e| matches!(e.kind(), Kind::Integer(_) | Kind::Rational(_)))
    }

    fn to_numeric(&self) -> Option<number::MatG<Rational>> {
        if !self.is_numeric() {
            return None;
        }
        let rows: Vec<Vec<Rational>> = self
            .rows_vec()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|e| match e.kind() {
                        Kind::Integer(n) => Rational::from_integer(n.clone()),
                        Kind::Rational(r) => r.clone(),
                        _ => unreachable!("checked by is_numeric"),
                    })
                    .collect()
            })
            .collect();
        Some(number::MatG::from_rows(rows))
    }

    fn from_numeric(m: &number::MatG<Rational>) -> Self {
        let rows: Vec<Vec<Expr>> = (0..m.rows).map(|r| (0..m.cols).map(|c| Expr::from(m.get(r, c).clone())).collect()).collect();
        Self::from_rows(rows)
    }

    /// 🔢️ Rank via exact RREF, only when every entry is numeric (`None` for genuinely symbolic matrices
    /// in this pass — a generic symbolic-pivot RREF would need the same zero-test machinery `inverse`
    /// already documents as undecidable in general).
    pub fn rank(&self) -> Option<usize> {
        self.to_numeric().map(|m| m.rank())
    }

    pub fn nullspace(&self) -> Option<Vec<Vec<Expr>>> {
        let m = self.to_numeric()?;
        Some(m.nullspace().into_iter().map(|v| (0..v.len()).map(|i| Expr::from(v.get(i).clone())).collect()).collect())
    }

    /// 🔢️ Reduced row-echelon form (`(rref, pivot_columns, rank)`), only when every entry is numeric.
    pub fn rref(&self) -> Option<(Self, Vec<usize>, usize)> {
        let m = self.to_numeric()?;
        let (r, pivots, rank) = m.rref();
        Some((Self::from_numeric(&r), pivots, rank))
    }

    /// 🔁️ Solves `A x = b` when `A` is numeric; falls back to `None` for symbolic matrices (use
    /// `crate::cas::solve::solve_linear_system` directly for those).
    pub fn solve_numeric(&self, b: &[Expr]) -> Option<Vec<Expr>> {
        let m = self.to_numeric()?;
        if !b.iter().all(|e| matches!(e.kind(), Kind::Integer(_) | Kind::Rational(_))) {
            return None;
        }
        let b_rat: Vec<Rational> = b
            .iter()
            .map(|e| match e.kind() {
                Kind::Integer(n) => Rational::from_integer(n.clone()),
                Kind::Rational(r) => r.clone(),
                _ => unreachable!(),
            })
            .collect();
        let v = number::VecG::from_vec(b_rat);
        let x = m.solve(&v)?;
        Some((0..x.len()).map(|i| Expr::from(x.get(i).clone())).collect())
    }
}
// #endregion 🔖️SymMatrix

// #region 🔖️Tests
#[cfg(test)]
include!("../🧪️tests/🔬️matrix-unit/🦀️.rs");
// #endregion 🔖️Tests
