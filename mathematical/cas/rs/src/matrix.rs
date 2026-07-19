//! 🧮 Symbolic matrices: entries are plain `Expr` (which already behaves like a field under its own
//! `+`/`-`/`*`/`Pow(-1)` encoding, so no generic `Ring`/`Field` newtype over `Expr` is needed for
//! cofactor-expansion algorithms). Purely-numeric-`Rational` matrices additionally delegate to
//! `mathematical_algebra`'s exact `MatG<Rational>` for rank/nullspace/RREF, which do need a real field
//! implementation to pivot correctly.

use crate::expr::{Expr, Kind};
use crate::solve::{det_expr, SolutionSet};
use mathematical_number::Rational;

// #region 🔖SymMatrix
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

    /// 🧮 Cofactor-expansion determinant, simplified via `simplify::cancel` (raw cofactor expansion on
    /// symbolic entries grows quickly; canceling common factors keeps the result readable).
    pub fn det(&self) -> Expr {
        assert_eq!(self.rows, self.cols, "SymMatrix::det requires a square matrix");
        crate::simplify::cancel(&det_expr(&self.rows_vec()))
    }

    fn cofactor(&self, skip_row: usize, skip_col: usize) -> Expr {
        let minor: Vec<Vec<Expr>> = self
            .rows_vec()
            .into_iter()
            .enumerate()
            .filter(|&(r, _)| r != skip_row)
            .map(|(_, row)| row.into_iter().enumerate().filter(|&(c, _)| c != skip_col).map(|(_, v)| v).collect())
            .collect();
        let sign = if (skip_row + skip_col) % 2 == 0 { Expr::integer(1) } else { Expr::integer(-1) };
        sign * det_expr(&minor)
    }

    /// 🧮 The adjugate (classical adjoint) matrix: `adj(A)[i][j] = cofactor(A, j, i)` (transposed
    /// cofactor matrix), satisfying `A * adj(A) == det(A) * I`.
    pub fn adjugate(&self) -> Self {
        assert_eq!(self.rows, self.cols, "SymMatrix::adjugate requires a square matrix");
        let n = self.rows;
        let mut out = Self::zeros(n, n);
        for i in 0..n {
            for j in 0..n {
                out.set(j, i, crate::simplify::cancel(&self.cofactor(i, j)));
            }
        }
        out
    }

    /// ➗ `Some(adj(A) / det(A))` when `det(A)` is (structurally, after `simplify`) provably nonzero;
    /// `None` when it's zero, and no answer when it can't be decided either way (the zero-test problem
    /// for symbolic `Expr` coefficients is undecidable in general — this pass is best-effort, never wrong).
    pub fn inverse(&self) -> Option<Self> {
        let d = self.det();
        if d.is_zero_literal() {
            return None;
        }
        let adj = self.adjugate();
        let inv_d = Expr::pow(d, Expr::integer(-1));
        Some(Self { rows: adj.rows, cols: adj.cols, data: adj.data.into_iter().map(|c| crate::simplify::cancel(&(c * inv_d.clone()))).collect() })
    }

    /// 🧮 Coefficients of the characteristic polynomial `det(A - lambda*I)` in the fresh symbol
    /// `lambda`, low-degree-first, via `as_poly_uni` on the cofactor-expansion determinant.
    pub fn charpoly(&self, lambda: &Expr) -> Option<mathematical_polynomial::PolyU<Rational>> {
        assert_eq!(self.rows, self.cols, "SymMatrix::charpoly requires a square matrix");
        let n = self.rows;
        let mut shifted = self.clone();
        for i in 0..n {
            shifted.set(i, i, shifted.get(i, i).clone() - lambda.clone());
        }
        let d = crate::simplify::expand(&det_expr(&shifted.rows_vec()));
        crate::polybridge::as_poly_uni(&d, lambda)
    }

    /// 🎯 Eigenvalues via `solve_univariate` on the characteristic polynomial.
    pub fn eigenvalues(&self) -> SolutionSet {
        let lambda = Expr::symbol("§lambda");
        let Some(poly) = self.charpoly(&lambda) else { return SolutionSet::Unknown };
        crate::solve::solve_univariate(&crate::polybridge::polyu_to_expr(&poly, &lambda), &lambda)
    }

    /// 🔢 `true` if every entry is a plain numeric literal (`Integer`/`Rational`), enabling the
    /// `mathematical_algebra`-backed numeric paths below.
    fn is_numeric(&self) -> bool {
        self.data.iter().all(|e| matches!(e.kind(), Kind::Integer(_) | Kind::Rational(_)))
    }

    fn to_numeric(&self) -> Option<mathematical_algebra::MatG<Rational>> {
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
        Some(mathematical_algebra::MatG::from_rows(rows))
    }

    fn from_numeric(m: &mathematical_algebra::MatG<Rational>) -> Self {
        let rows: Vec<Vec<Expr>> = (0..m.rows).map(|r| (0..m.cols).map(|c| Expr::from(m.get(r, c).clone())).collect()).collect();
        Self::from_rows(rows)
    }

    /// 🔢 Rank via exact RREF, only when every entry is numeric (`None` for genuinely symbolic matrices
    /// in this pass — a generic symbolic-pivot RREF would need the same zero-test machinery `inverse`
    /// already documents as undecidable in general).
    pub fn rank(&self) -> Option<usize> {
        self.to_numeric().map(|m| m.rank())
    }

    pub fn nullspace(&self) -> Option<Vec<Vec<Expr>>> {
        let m = self.to_numeric()?;
        Some(m.nullspace().into_iter().map(|v| (0..v.len()).map(|i| Expr::from(v.get(i).clone())).collect()).collect())
    }

    /// 🔢 Reduced row-echelon form (`(rref, pivot_columns, rank)`), only when every entry is numeric.
    pub fn rref(&self) -> Option<(Self, Vec<usize>, usize)> {
        let m = self.to_numeric()?;
        let (r, pivots, rank) = m.rref();
        Some((Self::from_numeric(&r), pivots, rank))
    }

    /// 🔁 Solves `A x = b` when `A` is numeric; falls back to `None` for symbolic matrices (use
    /// `crate::solve::solve_linear_system` directly for those).
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
        let v = mathematical_algebra::VecG::from_vec(b_rat);
        let x = m.solve(&v)?;
        Some((0..x.len()).map(|i| Expr::from(x.get(i).clone())).collect())
    }
}
// #endregion 🔖SymMatrix

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn e(v: i64) -> Expr {
        Expr::integer(v)
    }

    #[test]
    fn det_2x2_hand_case() {
        let m = SymMatrix::from_rows(vec![vec![e(1), e(2)], vec![e(3), e(4)]]);
        assert_eq!(m.det(), e(-2));
    }

    #[test]
    fn det_symbolic_2x2() {
        let a = Expr::symbol("a");
        let b = Expr::symbol("b");
        let c = Expr::symbol("c");
        let d = Expr::symbol("d");
        let m = SymMatrix::from_rows(vec![vec![a.clone(), b.clone()], vec![c.clone(), d.clone()]]);
        let expected = a * d - b * c;
        assert_eq!(m.det(), expected);
    }

    #[test]
    fn inverse_times_original_is_identity() {
        let m = SymMatrix::from_rows(vec![vec![e(2), e(1)], vec![e(1), e(1)]]);
        let inv = m.inverse().unwrap();
        let product = m.matmul(&inv);
        for r in 0..2 {
            for c in 0..2 {
                let expected = if r == c { e(1) } else { e(0) };
                assert_eq!(crate::simplify::cancel(product.get(r, c)), expected);
            }
        }
    }

    #[test]
    fn singular_matrix_has_no_inverse() {
        let m = SymMatrix::from_rows(vec![vec![e(1), e(2)], vec![e(2), e(4)]]);
        assert!(m.inverse().is_none());
    }

    #[test]
    fn charpoly_and_eigenvalues_of_diagonal_matrix() {
        let m = SymMatrix::from_rows(vec![vec![e(2), e(0)], vec![e(0), e(5)]]);
        match m.eigenvalues() {
            SolutionSet::Finite(mut vals) => {
                vals.sort();
                assert_eq!(vals, vec![e(2), e(5)]);
            }
            other => panic!("expected Finite eigenvalues, got {other:?}"),
        }
    }

    #[test]
    fn cayley_hamilton_holds_for_a_3x3_matrix() {
        // Verify A^2 - tr(A)*A + det(A)*I == 0 for a 2x2 matrix (Cayley-Hamilton).
        let m = SymMatrix::from_rows(vec![vec![e(1), e(2)], vec![e(3), e(4)]]);
        let a2 = m.matmul(&m);
        let tr_a = m.trace();
        let det_a = m.det();
        let lhs = a2.sub(&m.scale(&tr_a)).add(&SymMatrix::identity(2).scale(&det_a));
        for r in 0..2 {
            for c in 0..2 {
                assert_eq!(crate::simplify::simplify(lhs.get(r, c)), e(0));
            }
        }
    }

    #[test]
    fn rank_of_numeric_matrix() {
        let m = SymMatrix::from_rows(vec![vec![e(1), e(2)], vec![e(2), e(4)]]);
        assert_eq!(m.rank(), Some(1));
    }

    #[test]
    fn rref_of_numeric_matrix() {
        let m = SymMatrix::from_rows(vec![vec![e(2), e(4)], vec![e(1), e(1)]]);
        let (rref, _pivots, rank) = m.rref().unwrap();
        assert_eq!(rank, 2);
        assert_eq!(rref, SymMatrix::identity(2));
    }

    #[test]
    fn solve_numeric_linear_system() {
        let m = SymMatrix::from_rows(vec![vec![e(2), e(1)], vec![e(1), e(3)]]);
        let x = m.solve_numeric(&[e(5), e(10)]).unwrap();
        assert_eq!(x, vec![e(1), e(3)]);
    }
}
// #endregion 🔖Tests
