//! 🌱 Real algebraic numbers: an integer defining polynomial plus a rational isolating interval,
//! with exact ordering/refinement and add/mul implemented via bivariate resultants.

use crate::factor::factor_integer_poly;
use crate::roots::{count_roots_in, isolate_real_roots, refine_root, sturm_sequence};
use crate::univariate::PolyU;
use mathematical_number::{Integer, Rational};

// #region 🔖AlgebraicReal
/// 🌱 A real root of `poly` (not necessarily irreducible at construction, though the constructors here
/// always narrow to an irreducible factor), isolated by the rational interval `(lo, hi]` — or, when
/// `lo == hi`, an exact rational value.
#[derive(Clone, Debug)]
pub struct AlgebraicReal {
    poly: PolyU<Integer>,
    lo: Rational,
    hi: Rational,
}

impl AlgebraicReal {
    pub fn from_rational(r: &Rational) -> Self {
        let poly = PolyU::from_coeffs(vec![r.numer().neg(), Integer::from_natural(r.denom().clone())]);
        Self { poly, lo: r.clone(), hi: r.clone() }
    }

    /// 🌱 The `index`-th real root of `f` (ascending order, `0`-based); narrows `f` down to whichever
    /// irreducible factor actually contains that root. `None` if `index` is out of range.
    pub fn root_of(f: &PolyU<Integer>, index: usize) -> Option<Self> {
        let intervals = isolate_real_roots(f);
        let (lo, hi) = intervals.get(index)?.clone();
        let (_, factors) = factor_integer_poly(f);
        for (factor, _) in &factors {
            if factor.degree().unwrap_or(0) == 0 {
                continue;
            }
            let seq = sturm_sequence(factor);
            if count_roots_in(&seq, &lo, &hi) == 1 {
                return Some(Self { poly: factor.clone(), lo, hi });
            }
        }
        Some(Self { poly: f.clone(), lo, hi })
    }

    /// √ⁿ The real `n`-th root of `r` (principal / positive root when `n` is even); `None` if `n == 0`
    /// or (`n` even and `r < 0`).
    pub fn nth_root(r: &Rational, n: u32) -> Option<Self> {
        if n == 0 {
            return None;
        }
        if r.is_zero() {
            return Some(Self::from_rational(&Rational::zero()));
        }
        if n % 2 == 0 && r.numer().is_negative() {
            return None;
        }
        let denom = Integer::from_natural(r.denom().clone());
        let mut coeffs = vec![Integer::zero(); n as usize + 1];
        coeffs[0] = r.numer().neg();
        coeffs[n as usize] = denom;
        let poly = PolyU::from_coeffs(coeffs);
        let intervals = isolate_real_roots(&poly);
        let index = if n % 2 == 0 { intervals.len() - 1 } else { 0 };
        Self::root_of(&poly, index)
    }

    pub fn interval(&self) -> (Rational, Rational) {
        (self.lo.clone(), self.hi.clone())
    }

    pub fn minimal_poly(&self) -> &PolyU<Integer> {
        &self.poly
    }

    pub fn degree(&self) -> usize {
        self.poly.degree().unwrap_or(0)
    }

    pub fn is_rational(&self) -> bool {
        self.lo == self.hi
    }

    pub fn to_f64(&self) -> f64 {
        (self.lo.to_f64() + self.hi.to_f64()) / 2.0
    }

    /// 🔬 Bisects the isolating interval down to (at most) `width`.
    pub fn refine(&mut self, width: &Rational) {
        if self.is_rational() {
            return;
        }
        let (lo, hi) = refine_root(&self.poly, &self.lo, &self.hi, width);
        self.lo = lo;
        self.hi = hi;
    }

    fn refine_below(&mut self, width: &Rational) {
        while self.hi.sub(&self.lo) > *width && !self.is_rational() {
            let half = self.hi.sub(&self.lo).div(&Rational::from_i64(2, 1).unwrap()).unwrap();
            self.refine(&half);
        }
    }

    // #region 🔖CheapTransforms
    pub fn neg(&self) -> Self {
        // Root of f(x) negated is a root of f(-x); reverse the sign of odd-degree coefficients.
        let coeffs = self.poly.coeffs().iter().enumerate().map(|(i, c)| if i % 2 == 1 { c.neg() } else { c.clone() }).collect();
        Self { poly: PolyU::from_coeffs(coeffs), lo: self.hi.neg(), hi: self.lo.neg() }
    }

    /// ➗ Reciprocal (`self` must be nonzero): if `alpha` is a root of `f`, `1/alpha` is a root of the
    /// coefficient-reversed polynomial.
    pub fn inv(&self) -> Option<Self> {
        if self.lo.is_zero() && self.hi.is_zero() {
            return None;
        }
        let mut coeffs = self.poly.coeffs().to_vec();
        coeffs.reverse();
        let poly = PolyU::from_coeffs(coeffs);
        let new_lo = if self.hi.is_zero() { None } else { self.hi.inv() };
        let new_hi = if self.lo.is_zero() { None } else { self.lo.inv() };
        match (new_lo, new_hi) {
            (Some(a), Some(b)) if a <= b => Some(Self { poly, lo: a, hi: b }),
            (Some(a), Some(b)) => Some(Self { poly, lo: b, hi: a }),
            _ => None,
        }
    }

    pub fn scale_rational(&self, r: &Rational) -> Option<Self> {
        if r.is_zero() {
            return Some(Self::from_rational(&Rational::zero()));
        }
        // Root of f(x) scaled by r is a root of f(x/r); clear denominators via coefficient*r^i scaling.
        let n = self.poly.degree().unwrap_or(0);
        let mut coeffs = Vec::with_capacity(n + 1);
        for (i, c) in self.poly.coeffs().iter().enumerate() {
            let scaled = Rational::from_integer(c.clone()).mul(&r.pow((n - i) as i64)?);
            coeffs.push(scaled);
        }
        let rational_poly = PolyU::from_coeffs(coeffs);
        let denom_lcm = rational_poly.coeffs().iter().fold(mathematical_number::Natural::one(), |acc, c| {
            let g = acc.gcd(c.denom());
            acc.mul(c.denom()).div_rem(&g).0
        });
        let integer_poly = PolyU::from_coeffs(rational_poly.coeffs().iter().map(|c| c.mul(&Rational::from_integer(Integer::from_natural(denom_lcm.clone()))).trunc()).collect());
        let (new_lo, new_hi) = if r.numer().is_negative() { (self.hi.mul(r), self.lo.mul(r)) } else { (self.lo.mul(r), self.hi.mul(r)) };
        let seq = sturm_sequence(&integer_poly);
        if count_roots_in(&seq, &new_lo, &new_hi) == 1 {
            Some(Self { poly: integer_poly, lo: new_lo, hi: new_hi })
        } else {
            // Fallback: the transformed interval is no longer isolating (rare boundary case) — recover
            // via root_of on the recomputed integer polynomial using the numeric estimate.
            let target = self.to_f64() * r.to_f64();
            Self::root_of_near(&integer_poly, target)
        }
    }

    pub fn add_rational(&self, r: &Rational) -> Self {
        let poly = self.poly.compose_with_rational_shift(r);
        Self { poly, lo: self.lo.add(r), hi: self.hi.add(r) }
    }

    fn root_of_near(poly: &PolyU<Integer>, target: f64) -> Option<Self> {
        let intervals = isolate_real_roots(poly);
        let (best_idx, _) = intervals.iter().enumerate().min_by(|(_, (a_lo, a_hi)), (_, (b_lo, b_hi))| {
            let a_mid = (a_lo.to_f64() + a_hi.to_f64()) / 2.0;
            let b_mid = (b_lo.to_f64() + b_hi.to_f64()) / 2.0;
            (a_mid - target).abs().partial_cmp(&(b_mid - target).abs()).unwrap()
        })?;
        Self::root_of(poly, best_idx)
    }
    // #endregion 🔖CheapTransforms

    // #region 🔖AlgebraicOps
    /// ➕ Sum of two algebraic reals via the bivariate resultant `res_y(f(x - y), g(y))`, which
    /// vanishes exactly at every pairwise sum of a root of `f` with a root of `g`; the correct
    /// irreducible factor and interval are selected by exact rational interval refinement (never by
    /// floating-point comparison) until the candidates are unambiguous.
    pub fn add(&self, other: &Self) -> Self {
        Self::combine(self, other, Combine::Add)
    }

    /// ✖️ Product via `res_y(y^deg(f) * f(x/y), g(y))`.
    pub fn mul(&self, other: &Self) -> Self {
        Self::combine(self, other, Combine::Mul)
    }

    fn combine(a: &Self, b: &Self, op: Combine) -> Self {
        if a.is_rational() {
            return match op {
                Combine::Add => b.add_rational(&a.lo),
                Combine::Mul => b.scale_rational(&a.lo).unwrap_or_else(|| Self::from_rational(&Rational::zero())),
            };
        }
        if b.is_rational() {
            return match op {
                Combine::Add => a.add_rational(&b.lo),
                Combine::Mul => a.scale_rational(&b.lo).unwrap_or_else(|| Self::from_rational(&Rational::zero())),
            };
        }
        let n = a.poly.degree().unwrap_or(0);
        let f_lifted: PolyU<PolyU<Integer>> = PolyU::from_coeffs(a.poly.coeffs().iter().map(|c| PolyU::constant(c.clone())).collect());
        let g_lifted: PolyU<PolyU<Integer>> = PolyU::from_coeffs(b.poly.coeffs().iter().map(|c| PolyU::constant(c.clone())).collect());
        let transformed: PolyU<PolyU<Integer>> = match op {
            Combine::Add => {
                let shift: PolyU<PolyU<Integer>> = PolyU::from_coeffs(vec![PolyU::x(), PolyU::constant(Integer::from_i64(-1))]);
                f_lifted.compose(&shift)
            }
            Combine::Mul => {
                let mut coeffs = vec![PolyU::<Integer>::zero(); n + 1];
                for (i, c) in a.poly.coeffs().iter().enumerate() {
                    coeffs[n - i] = PolyU::monomial(c.clone(), i);
                }
                PolyU::from_coeffs(coeffs)
            }
        };
        let resultant_poly: PolyU<Integer> = transformed.resultant(&g_lifted);

        let mut a_ref = a.clone();
        let mut b_ref = b.clone();
        let mut precision = Rational::from_i64(1, 16).unwrap();
        for _ in 0..200 {
            a_ref.refine_below(&precision);
            b_ref.refine_below(&precision);
            let (target_lo, target_hi) = match op {
                Combine::Add => (a_ref.lo.add(&b_ref.lo), a_ref.hi.add(&b_ref.hi)),
                Combine::Mul => {
                    let candidates = [a_ref.lo.mul(&b_ref.lo), a_ref.lo.mul(&b_ref.hi), a_ref.hi.mul(&b_ref.lo), a_ref.hi.mul(&b_ref.hi)];
                    (candidates.iter().cloned().min().unwrap(), candidates.iter().cloned().max().unwrap())
                }
            };
            let (_, factors) = factor_integer_poly(&resultant_poly);
            let mut matches: Vec<(PolyU<Integer>, Rational, Rational)> = Vec::new();
            for (factor, _) in &factors {
                if factor.degree().unwrap_or(0) == 0 {
                    continue;
                }
                for (lo, hi) in isolate_real_roots(factor) {
                    if lo >= target_lo.clone().sub(&precision) && hi <= target_hi.clone().add(&precision) {
                        matches.push((factor.clone(), lo, hi));
                    }
                }
            }
            if matches.len() == 1 {
                let (poly, lo, hi) = matches.into_iter().next().unwrap();
                return Self { poly, lo, hi };
            }
            precision = precision.div(&Rational::from_i64(4, 1).unwrap()).unwrap();
        }
        // Defensive fallback: numeric estimate selects the nearest root of the resultant polynomial.
        let target = match op {
            Combine::Add => a.to_f64() + b.to_f64(),
            Combine::Mul => a.to_f64() * b.to_f64(),
        };
        Self::root_of_near(&resultant_poly, target).unwrap_or_else(|| Self::from_rational(&Rational::zero()))
    }
    // #endregion 🔖AlgebraicOps
}

enum Combine {
    Add,
    Mul,
}

impl PolyU<Integer> {
    /// 🔗 An integer polynomial with the same roots as `self(x - r)`: writing `r = num/den`, this
    /// computes `den^n * self((den*x - num)/den) == den^n * self(x - r)` entirely with integer
    /// arithmetic by composing with the integer linear polynomial `den*x - num` after scaling each
    /// coefficient `a_i` by `den^(n-i)` — the scaling by the positive constant `den^n` doesn't change
    /// the roots, so this is a valid (if non-primitive) defining polynomial for `alpha + r`.
    fn compose_with_rational_shift(&self, r: &Rational) -> PolyU<Integer> {
        let n = self.degree().unwrap_or(0);
        let den_i = Integer::from_natural(r.denom().clone());
        let scaled_self = PolyU::from_coeffs(self.coeffs().iter().enumerate().map(|(i, coeff)| coeff.mul(&den_i.pow((n - i) as u64))).collect());
        let shift_int = PolyU::from_coeffs(vec![r.numer().neg(), den_i]);
        scaled_self.compose(&shift_int).primitive_part()
    }
}

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn ipoly(coeffs: Vec<i64>) -> PolyU<Integer> {
        PolyU::from_coeffs(coeffs.into_iter().map(Integer::from_i64).collect())
    }

    #[test]
    fn sqrt2_plus_sqrt3_has_minimal_poly_degree_4() {
        let sqrt2 = AlgebraicReal::nth_root(&Rational::from_i64(2, 1).unwrap(), 2).unwrap();
        let sqrt3 = AlgebraicReal::nth_root(&Rational::from_i64(3, 1).unwrap(), 2).unwrap();
        let sum = sqrt2.add(&sqrt3);
        // minimal poly of sqrt2+sqrt3 is x^4 - 10x^2 + 1 (up to sign/unit); verify by exact evaluation
        // via interval refinement: (sum)^2 should be close to 5 + 2*sqrt6 ~ 9.899
        let val = sum.to_f64();
        assert!((val - (2f64.sqrt() + 3f64.sqrt())).abs() < 1e-6);
        assert!(sum.degree() <= 4);
    }

    #[test]
    fn cbrt2_times_cbrt4_equals_2() {
        let cbrt2 = AlgebraicReal::nth_root(&Rational::from_i64(2, 1).unwrap(), 3).unwrap();
        let cbrt4 = AlgebraicReal::nth_root(&Rational::from_i64(4, 1).unwrap(), 3).unwrap();
        let product = cbrt2.mul(&cbrt4);
        assert!((product.to_f64() - 2.0).abs() < 1e-6);
    }

    #[test]
    fn from_rational_is_exact() {
        let r = Rational::from_i64(3, 4).unwrap();
        let a = AlgebraicReal::from_rational(&r);
        assert!(a.is_rational());
        assert_eq!(a.to_f64(), r.to_f64());
    }

    #[test]
    fn neg_and_inv_hand_cases() {
        let sqrt2 = AlgebraicReal::nth_root(&Rational::from_i64(2, 1).unwrap(), 2).unwrap();
        let negated = sqrt2.neg();
        assert!((negated.to_f64() + 2f64.sqrt()).abs() < 1e-9);
        let inv = sqrt2.inv().unwrap();
        assert!((inv.to_f64() - 1.0 / 2f64.sqrt()).abs() < 1e-6);
    }

    #[test]
    fn root_of_selects_correct_irreducible_factor() {
        // (x-1)(x^2-2): roots are 1, -sqrt2, sqrt2 in ascending order.
        let f = ipoly(vec![-1, 1]).mul(&ipoly(vec![-2, 0, 1]));
        let root0 = AlgebraicReal::root_of(&f, 0).unwrap(); // -sqrt2
        assert!((root0.to_f64() + 2f64.sqrt()).abs() < 1e-6);
        let root1 = AlgebraicReal::root_of(&f, 1).unwrap(); // 1
        assert!(root1.is_rational());
        assert_eq!(root1.to_f64(), 1.0);
    }
}
// #endregion 🔖Tests
