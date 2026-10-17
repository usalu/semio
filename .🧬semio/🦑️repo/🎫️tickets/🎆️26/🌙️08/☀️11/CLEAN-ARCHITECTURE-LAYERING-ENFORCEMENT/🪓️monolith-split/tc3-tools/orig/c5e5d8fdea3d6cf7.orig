//! 🧮️ Generic exact linear algebra: dense vectors/matrices over any [`crate::traits::Ring`], with
//! elimination/decomposition algorithms gated on the appropriate trait (`Field` for RREF/rank/det/
//! inverse/solve/nullspace/charpoly, `IntegralDomain`/`GcdDomain` for fraction-free Bareiss
//! determinant/rank, `EuclideanDomain` for Smith normal form). Relocated verbatim from
//! `📸️remodel`'s `➕️algebra-internals` (its own `VecG`/`MatG`/`ExactElimination`/`Charpoly`/`Smith`
//! regions) in ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave
//! FIXALG: wave M3a gave the `➗️mathematical` plugin's `cas-internals` a second consumer of these
//! generic types (`MatG::solve` for exact partial-fraction decomposition) that wave M3d's earlier
//! sole-consumer census of `📸️remodel` did not — and could not — see, since M3d's move predated
//! M3a's. A generic matrix/vector over a field is a numeric primitive like `Rational` itself, not
//! domain content, so it belongs here rather than duplicated per plugin. `📸️remodel`'s own
//! `lll_reduce` (LLL lattice reduction over `Rational` rows) stays behind in that plugin's
//! `➕️algebra-internals` as a free function over these types — it has no second consumer and is not
//! an inherent impl, so nothing forces it to move.

use crate::traits::{EuclideanDomain, Field, GcdDomain, IntegralDomain, Ring};

// #region 🔖️VecG
/// 📐️ Generic dense vector over any [`Ring`] — the exact-arithmetic counterpart to a fixed-size
/// render vector, used when entries are `Rational`, `Integer`, `ModInt`, or (from a CAS plugin's own
/// `crate::cas`) symbolic expressions.
#[derive(Clone, Debug, PartialEq)]
pub struct VecG<T> {
    pub data: Vec<T>,
}

impl<T: Ring> VecG<T> {
    pub fn zeros(n: usize) -> Self {
        Self { data: (0..n).map(|_| T::zero()).collect() }
    }

    pub fn from_vec(data: Vec<T>) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get(&self, i: usize) -> &T {
        &self.data[i]
    }

    pub fn set(&mut self, i: usize, value: T) {
        self.data[i] = value;
    }

    pub fn add(&self, other: &Self) -> Self {
        Self { data: self.data.iter().zip(other.data.iter()).map(|(a, b)| a.add(b)).collect() }
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self { data: self.data.iter().zip(other.data.iter()).map(|(a, b)| a.sub(b)).collect() }
    }

    pub fn scale(&self, s: &T) -> Self {
        Self { data: self.data.iter().map(|v| v.mul(s)).collect() }
    }

    pub fn dot(&self, other: &Self) -> T {
        self.data.iter().zip(other.data.iter()).fold(T::zero(), |acc, (a, b)| acc.add(&a.mul(b)))
    }
}
// #endregion 🔖️VecG

// #region 🔖️MatG
/// 🧮️ Generic dense row-major matrix over any [`Ring`] — the exact-arithmetic counterpart to a
/// fixed-size render matrix.
#[derive(Clone, Debug, PartialEq)]
pub struct MatG<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<T: Ring> MatG<T> {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self { rows, cols, data: (0..rows * cols).map(|_| T::zero()).collect() }
    }

    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n, n);
        for i in 0..n {
            m.set(i, i, T::one());
        }
        m
    }

    pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
        let nrows = rows.len();
        let ncols = rows.first().map_or(0, |r| r.len());
        let data = rows.into_iter().flatten().collect();
        Self { rows: nrows, cols: ncols, data }
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row * self.cols + col] = value;
    }

    pub fn transpose(&self) -> Self {
        let mut out = Self::zeros(self.cols, self.rows);
        for row in 0..self.rows {
            for col in 0..self.cols {
                out.set(col, row, self.get(row, col).clone());
            }
        }
        out
    }

    pub fn matmul(&self, other: &Self) -> Self {
        assert_eq!(self.cols, other.rows, "MatG::matmul: dimension mismatch");
        let mut out = Self::zeros(self.rows, other.cols);
        for row in 0..self.rows {
            for col in 0..other.cols {
                let mut sum = T::zero();
                for k in 0..self.cols {
                    sum = sum.add(&self.get(row, k).mul(other.get(k, col)));
                }
                out.set(row, col, sum);
            }
        }
        out
    }

    pub fn mul_vec(&self, x: &VecG<T>) -> VecG<T> {
        assert_eq!(self.cols, x.len(), "MatG::mul_vec: dimension mismatch");
        let mut out = VecG::zeros(self.rows);
        for row in 0..self.rows {
            let mut sum = T::zero();
            for col in 0..self.cols {
                sum = sum.add(&self.get(row, col).mul(x.get(col)));
            }
            out.set(row, sum);
        }
        out
    }

    pub fn add(&self, other: &Self) -> Self {
        Self { rows: self.rows, cols: self.cols, data: self.data.iter().zip(other.data.iter()).map(|(a, b)| a.add(b)).collect() }
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self { rows: self.rows, cols: self.cols, data: self.data.iter().zip(other.data.iter()).map(|(a, b)| a.sub(b)).collect() }
    }

    pub fn scale(&self, s: &T) -> Self {
        Self { rows: self.rows, cols: self.cols, data: self.data.iter().map(|v| v.mul(s)).collect() }
    }
}
// #endregion 🔖️MatG

// #region 🔖️ExactElimination
impl<T: Field> MatG<T> {
    /// 🧮️ Reduced row-echelon form via Gauss-Jordan elimination with first-nonzero pivoting (exact
    /// rings have no notion of numerical stability, so there's no reason to pivot on magnitude).
    /// Returns `(rref, pivot_columns, rank)`.
    pub fn rref(&self) -> (Self, Vec<usize>, usize) {
        let mut m = self.clone();
        let mut pivots = Vec::new();
        let mut pivot_row = 0;
        for col in 0..m.cols {
            if pivot_row >= m.rows {
                break;
            }
            let Some(found) = (pivot_row..m.rows).find(|&r| !m.get(r, col).is_zero()) else {
                continue;
            };
            if found != pivot_row {
                for c in 0..m.cols {
                    let tmp = m.get(found, c).clone();
                    let cur = m.get(pivot_row, c).clone();
                    m.set(found, c, cur);
                    m.set(pivot_row, c, tmp);
                }
            }
            let pivot_val = m.get(pivot_row, col).clone();
            let inv = pivot_val.inv().expect("nonzero pivot has an inverse in a field");
            for c in 0..m.cols {
                let scaled = m.get(pivot_row, c).mul(&inv);
                m.set(pivot_row, c, scaled);
            }
            for r in 0..m.rows {
                if r == pivot_row {
                    continue;
                }
                let factor = m.get(r, col).clone();
                if factor.is_zero() {
                    continue;
                }
                for c in 0..m.cols {
                    let sub = m.get(r, c).sub(&factor.mul(m.get(pivot_row, c)));
                    m.set(r, c, sub);
                }
            }
            pivots.push(col);
            pivot_row += 1;
        }
        let rank = pivots.len();
        (m, pivots, rank)
    }

    pub fn rank(&self) -> usize {
        self.rref().2
    }

    /// 🧮️ Determinant via Gauss-Jordan with first-nonzero pivoting, tracking the pivot product and
    /// row-swap sign (valid over any field; degenerates to `zero()` for a singular matrix).
    pub fn det(&self) -> T {
        assert_eq!(self.rows, self.cols, "MatG::det: requires a square matrix");
        let n = self.rows;
        let mut m = self.clone();
        let mut sign = T::one();
        let mut det = T::one();
        for col in 0..n {
            let Some(found) = (col..n).find(|&r| !m.get(r, col).is_zero()) else {
                return T::zero();
            };
            if found != col {
                for c in 0..n {
                    let tmp = m.get(found, c).clone();
                    let cur = m.get(col, c).clone();
                    m.set(found, c, cur);
                    m.set(col, c, tmp);
                }
                sign = sign.neg();
            }
            let pivot_val = m.get(col, col).clone();
            det = det.mul(&pivot_val);
            let inv = pivot_val.inv().expect("nonzero pivot has an inverse in a field");
            for r in (col + 1)..n {
                let factor = m.get(r, col).mul(&inv);
                if factor.is_zero() {
                    continue;
                }
                for c in col..n {
                    let sub = m.get(r, c).sub(&factor.mul(m.get(col, c)));
                    m.set(r, c, sub);
                }
            }
        }
        det.mul(&sign)
    }

    pub fn inverse(&self) -> Option<Self> {
        assert_eq!(self.rows, self.cols, "MatG::inverse: requires a square matrix");
        let n = self.rows;
        // 🐛️ Rank must come from `self` alone: the augmented `[A | I]` matrix's own rref rank is
        // NOT a valid full-rank check, because `rref`'s first-nonzero pivoting can plant pivot
        // columns inside the appended identity block once `A`'s columns run out of pivots — that
        // silently inflates the reported rank to `n` even when `A` is singular.
        if self.rank() < n {
            return None;
        }
        let mut augmented = Self::zeros(n, 2 * n);
        for row in 0..n {
            for col in 0..n {
                augmented.set(row, col, self.get(row, col).clone());
            }
            augmented.set(row, n + row, T::one());
        }
        let (rref, _, _) = augmented.rref();
        let mut inv = Self::zeros(n, n);
        for row in 0..n {
            for col in 0..n {
                inv.set(row, col, rref.get(row, n + col).clone());
            }
        }
        Some(inv)
    }

    pub fn solve(&self, b: &VecG<T>) -> Option<VecG<T>> {
        assert_eq!(self.rows, self.cols, "MatG::solve: requires a square matrix");
        let n = self.rows;
        // 🐛️ See `inverse`: rank must be checked on `self` alone, not on the `[A | b]` augmented
        // rref, since a stray pivot found inside the `b` column can otherwise mask a singular `A`.
        if self.rank() < n {
            return None;
        }
        let mut augmented = Self::zeros(n, n + 1);
        for row in 0..n {
            for col in 0..n {
                augmented.set(row, col, self.get(row, col).clone());
            }
            augmented.set(row, n, b.get(row).clone());
        }
        let (rref, _, _) = augmented.rref();
        Some(VecG::from_vec((0..n).map(|row| rref.get(row, n).clone()).collect()))
    }

    /// 🕳️ Basis of the null space, read off the RREF's free columns.
    pub fn nullspace(&self) -> Vec<VecG<T>> {
        let (rref, pivots, _) = self.rref();
        let pivot_set: std::collections::BTreeSet<usize> = pivots.iter().copied().collect();
        let free_cols: Vec<usize> = (0..self.cols).filter(|c| !pivot_set.contains(c)).collect();
        free_cols
            .into_iter()
            .map(|free_col| {
                let mut v = VecG::zeros(self.cols);
                v.set(free_col, T::one());
                for (pivot_row, &pivot_col) in pivots.iter().enumerate() {
                    let coeff = rref.get(pivot_row, free_col).clone();
                    v.set(pivot_col, coeff.neg());
                }
                v
            })
            .collect()
    }
}

impl<T: IntegralDomain> MatG<T> {
    /// 🧮️ Bareiss fraction-free elimination: every interior division is provably exact in any integral
    /// domain, so this computes an exact determinant over `Integer`/`ModInt`/polynomial coefficient
    /// rings without ever leaving the ring (no need for `Field`).
    pub fn det_bareiss(&self) -> T {
        assert_eq!(self.rows, self.cols, "MatG::det_bareiss: requires a square matrix");
        let n = self.rows;
        if n == 0 {
            return T::one();
        }
        let mut m = self.clone();
        let mut prev_pivot = T::one();
        let mut sign = T::one();
        for k in 0..n - 1 {
            if m.get(k, k).is_zero() {
                let Some(swap_row) = ((k + 1)..n).find(|&r| !m.get(r, k).is_zero()) else {
                    return T::zero();
                };
                for c in 0..n {
                    let tmp = m.get(swap_row, c).clone();
                    let cur = m.get(k, c).clone();
                    m.set(swap_row, c, cur);
                    m.set(k, c, tmp);
                }
                sign = sign.neg();
            }
            for i in (k + 1)..n {
                for j in (k + 1)..n {
                    let cross = m.get(i, j).mul(m.get(k, k)).sub(&m.get(i, k).mul(m.get(k, j)));
                    let divided = if k == 0 { cross } else { cross.exact_div(&prev_pivot).expect("Bareiss: division is exact by the algorithm's theorem") };
                    m.set(i, j, divided);
                }
            }
            prev_pivot = m.get(k, k).clone();
        }
        m.get(n - 1, n - 1).mul(&sign)
    }

    pub fn rank_bareiss(&self) -> usize
    where
        T: GcdDomain,
    {
        // Fraction-free rank via row-reduction to echelon form, counting nonzero pivot rows.
        let mut m = self.clone();
        let mut prev_pivot = T::one();
        let mut pivot_row = 0;
        for col in 0..m.cols {
            if pivot_row >= m.rows {
                break;
            }
            let Some(found) = (pivot_row..m.rows).find(|&r| !m.get(r, col).is_zero()) else {
                continue;
            };
            if found != pivot_row {
                for c in 0..m.cols {
                    let tmp = m.get(found, c).clone();
                    let cur = m.get(pivot_row, c).clone();
                    m.set(found, c, cur);
                    m.set(pivot_row, c, tmp);
                }
            }
            for r in (pivot_row + 1)..m.rows {
                for c in (col + 1)..m.cols {
                    let cross = m.get(r, c).mul(m.get(pivot_row, col)).sub(&m.get(r, col).mul(m.get(pivot_row, c)));
                    let divided = if pivot_row == 0 { cross } else { cross.exact_div(&prev_pivot).unwrap_or(cross) };
                    m.set(r, c, divided);
                }
                m.set(r, col, T::zero());
            }
            prev_pivot = m.get(pivot_row, col).clone();
            pivot_row += 1;
        }
        pivot_row
    }
}
// #endregion 🔖️ExactElimination

// #region 🔖️Charpoly
impl<T: Field> MatG<T> {
    /// 🧮️ Characteristic polynomial coefficients (low-to-high, monic) via the Faddeev-LeVerrier
    /// algorithm: `M_0 = 0`, `c_0 = 1`; `M_k = A*M_{k-1} + c_{k-1}*I`, `c_k = -trace(A*M_k)/k`. Requires
    /// `T: Field` (division by `1..=n`), which holds whenever the characteristic doesn't divide any of
    /// `1..=n` — true for `Rational` and for `ModInt` under a prime modulus larger than the matrix size.
    /// Returns `Vec<T>`, not a `PolyU`, because this crate must not depend on any plugin's polynomial
    /// type — the plugin wraps the result where a polynomial type is wanted.
    pub fn charpoly(&self) -> Vec<T> {
        assert_eq!(self.rows, self.cols, "MatG::charpoly: requires a square matrix");
        let n = self.rows;
        if n == 0 {
            return vec![T::one()];
        }
        // `T::from_i64(k)` alone can be an "unbound" value for runtime-modulus types like `ModInt`
        // (no modulus known yet) — anchor it to the matrix's own modulus via a bound zero derived from
        // an actual entry before inverting, which is a no-operation for ordinary fields like `Rational`.
        let bound_zero = self.get(0, 0).sub(self.get(0, 0));
        let mut m = MatG::<T>::zeros(n, n);
        let mut c = bound_zero.add(&T::one());
        let mut coeffs_high_to_low = vec![c.clone()];
        for k in 1..=n {
            let mut m_k = self.matmul(&m);
            for i in 0..n {
                let updated = m_k.get(i, i).add(&c);
                m_k.set(i, i, updated);
            }
            let am = self.matmul(&m_k);
            let mut trace = T::zero();
            for i in 0..n {
                trace = trace.add(am.get(i, i));
            }
            let k_bound = bound_zero.add(&T::from_i64(k as i64));
            let k_inv = k_bound.inv().expect("Faddeev-LeVerrier: 1..=n must be invertible in T (fails only when the characteristic divides some k <= n)");
            c = trace.mul(&k_inv).neg();
            coeffs_high_to_low.push(c.clone());
            m = m_k;
        }
        coeffs_high_to_low.reverse();
        coeffs_high_to_low
    }
}
// #endregion 🔖️Charpoly

// #region 🔖️Smith
impl<T: EuclideanDomain> MatG<T> {
    /// 🧮️ Smith normal form via repeated pivot-reduction: returns `(s, u, v)` with `u * self * v == s`
    /// and `s` diagonal with each entry dividing the next (the divisibility chain). Used for integer
    /// linear-system/Diophantine solving in a CAS plugin's `crate::cas`.
    pub fn smith_normal_form(&self) -> (Self, Self, Self) {
        let mut s = self.clone();
        let mut u = Self::identity(self.rows);
        let mut v = Self::identity(self.cols);
        let mut t = 0;
        while t < s.rows.min(s.cols) {
            // Find a nonzero pivot in the remaining submatrix.
            let Some((pr, pc)) = (t..s.rows).flat_map(|r| (t..s.cols).map(move |c| (r, c))).find(|&(r, c)| !s.get(r, c).is_zero()) else {
                break;
            };
            s.swap_rows(t, pr);
            u.swap_rows(t, pr);
            s.swap_cols(t, pc);
            v.swap_cols(t, pc);
            loop {
                let mut done = true;
                for r in (t + 1)..s.rows {
                    if s.get(r, t).is_zero() {
                        continue;
                    }
                    let (q, rem) = s.get(r, t).div_rem(s.get(t, t));
                    s.row_op_subtract_multiple(r, t, &q);
                    u.row_op_subtract_multiple(r, t, &q);
                    if !rem.is_zero() {
                        s.swap_rows(r, t);
                        u.swap_rows(r, t);
                        done = false;
                    }
                }
                for c in (t + 1)..s.cols {
                    if s.get(t, c).is_zero() {
                        continue;
                    }
                    let (q, rem) = s.get(t, c).div_rem(s.get(t, t));
                    s.col_op_subtract_multiple(c, t, &q);
                    v.col_op_subtract_multiple(c, t, &q);
                    if !rem.is_zero() {
                        s.swap_cols(c, t);
                        v.swap_cols(c, t);
                        done = false;
                    }
                }
                if done {
                    break;
                }
            }
            // Ensure the pivot divides every remaining entry; if not, add its row into an offending row
            // and restart the reduction at this pivot (classical Smith-form cleanup step).
            let mut clean = true;
            'scan: for r in (t + 1)..s.rows {
                for c in (t + 1)..s.cols {
                    if s.get(r, c).exact_div(s.get(t, t)).is_none() {
                        s.row_op_add(t, r);
                        u.row_op_add(t, r);
                        clean = false;
                        break 'scan;
                    }
                }
            }
            if !clean {
                continue;
            }
            t += 1;
        }
        (s, u, v)
    }

    fn swap_rows(&mut self, r1: usize, r2: usize) {
        for c in 0..self.cols {
            let tmp = self.get(r1, c).clone();
            let cur = self.get(r2, c).clone();
            self.set(r1, c, cur);
            self.set(r2, c, tmp);
        }
    }

    fn swap_cols(&mut self, c1: usize, c2: usize) {
        for r in 0..self.rows {
            let tmp = self.get(r, c1).clone();
            let cur = self.get(r, c2).clone();
            self.set(r, c1, cur);
            self.set(r, c2, tmp);
        }
    }

    /// ➖️ `row[target] -= factor * row[source]`.
    fn row_op_subtract_multiple(&mut self, target: usize, source: usize, factor: &T) {
        for c in 0..self.cols {
            let sub = self.get(target, c).sub(&factor.mul(self.get(source, c)));
            self.set(target, c, sub);
        }
    }

    /// ➖️ `col[target] -= factor * col[source]`.
    fn col_op_subtract_multiple(&mut self, target: usize, source: usize, factor: &T) {
        for r in 0..self.rows {
            let sub = self.get(r, target).sub(&factor.mul(self.get(r, source)));
            self.set(r, target, sub);
        }
    }

    /// ➕️ `row[target] += row[source]`.
    fn row_op_add(&mut self, source: usize, target: usize) {
        for c in 0..self.cols {
            let sum = self.get(target, c).add(self.get(source, c));
            self.set(target, c, sum);
        }
    }
}
// #endregion 🔖️Smith

// #region 🔖️Tests
#[cfg(test)]
include!("../🧪️tests/🔬️algebra-unit/🦀️.rs");
// #endregion 🔖️Tests
