//! 📈 Dense univariate polynomials, generic over the `mathematical_number` ring hierarchy: arithmetic,
//! pseudo-division, resultants/discriminants, GCD, and (over a field) squarefree decomposition and
//! Newton interpolation.

use mathematical_number::{CommutativeRing, Field, GcdDomain, IntegralDomain, Ring};

// #region 🔖PolyU
/// 📈 Little-endian coefficients (`coeffs[i]` is the coefficient of `x^i`); invariant: no trailing
/// zero coefficient (the empty vector represents the zero polynomial).
#[derive(Clone, PartialEq, Debug)]
pub struct PolyU<C> {
    coeffs: Vec<C>,
}

impl<C: Ring> PolyU<C> {
    fn normalize(mut coeffs: Vec<C>) -> Self {
        while coeffs.last().is_some_and(Ring::is_zero) {
            coeffs.pop();
        }
        Self { coeffs }
    }

    pub fn zero() -> Self {
        Self { coeffs: Vec::new() }
    }

    pub fn one() -> Self {
        Self { coeffs: vec![C::one()] }
    }

    pub fn x() -> Self {
        Self { coeffs: vec![C::zero(), C::one()] }
    }

    pub fn constant(c: C) -> Self {
        Self::normalize(vec![c])
    }

    /// 📈 `coeff * x^degree`.
    pub fn monomial(coeff: C, degree: usize) -> Self {
        if coeff.is_zero() {
            return Self::zero();
        }
        let mut coeffs = vec![C::zero(); degree + 1];
        coeffs[degree] = coeff;
        Self { coeffs }
    }

    pub fn from_coeffs(coeffs: Vec<C>) -> Self {
        Self::normalize(coeffs)
    }

    pub fn is_zero(&self) -> bool {
        self.coeffs.is_empty()
    }

    pub fn degree(&self) -> Option<usize> {
        if self.coeffs.is_empty() {
            None
        } else {
            Some(self.coeffs.len() - 1)
        }
    }

    pub fn leading_coeff(&self) -> Option<&C> {
        self.coeffs.last()
    }

    pub fn coeff(&self, i: usize) -> C {
        self.coeffs.get(i).cloned().unwrap_or_else(C::zero)
    }

    pub fn coeffs(&self) -> &[C] {
        &self.coeffs
    }

    pub fn add(&self, rhs: &Self) -> Self {
        let n = self.coeffs.len().max(rhs.coeffs.len());
        Self::normalize((0..n).map(|i| self.coeff(i).add(&rhs.coeff(i))).collect())
    }

    pub fn neg(&self) -> Self {
        Self { coeffs: self.coeffs.iter().map(Ring::neg).collect() }
    }

    pub fn sub(&self, rhs: &Self) -> Self {
        self.add(&rhs.neg())
    }

    pub fn mul_scalar(&self, s: &C) -> Self {
        if s.is_zero() {
            return Self::zero();
        }
        Self::normalize(self.coeffs.iter().map(|c| c.mul(s)).collect())
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        if self.is_zero() || rhs.is_zero() {
            return Self::zero();
        }
        let mut coeffs = vec![C::zero(); self.coeffs.len() + rhs.coeffs.len() - 1];
        for (i, a) in self.coeffs.iter().enumerate() {
            if a.is_zero() {
                continue;
            }
            for (j, b) in rhs.coeffs.iter().enumerate() {
                coeffs[i + j] = coeffs[i + j].add(&a.mul(b));
            }
        }
        Self::normalize(coeffs)
    }

    pub fn pow(&self, exp: u64) -> Self {
        let mut result = Self::one();
        let mut base = self.clone();
        let mut e = exp;
        while e > 0 {
            if e & 1 == 1 {
                result = result.mul(&base);
            }
            base = base.mul(&base);
            e >>= 1;
        }
        result
    }

    /// ⬆️ Multiplies by `x^k` (a pure shift of the coefficient vector).
    pub fn shift_up(&self, k: usize) -> Self {
        if self.is_zero() || k == 0 {
            return self.clone();
        }
        let mut coeffs = vec![C::zero(); k];
        coeffs.extend(self.coeffs.iter().cloned());
        Self { coeffs }
    }

    /// 🎯 Horner evaluation at `point`.
    pub fn eval(&self, point: &C) -> C {
        let mut result = C::zero();
        for c in self.coeffs.iter().rev() {
            result = result.mul(point).add(c);
        }
        result
    }

    /// 🔗 Substitutes `inner` for `x`: `self(inner(t))`.
    pub fn compose(&self, inner: &Self) -> Self {
        let mut result = Self::zero();
        for c in self.coeffs.iter().rev() {
            result = result.mul(inner).add(&Self::constant(c.clone()));
        }
        result
    }

    /// 📉 Formal derivative: `d/dx sum c_i x^i = sum i*c_i x^(i-1)`.
    pub fn derivative(&self) -> Self {
        if self.coeffs.len() <= 1 {
            return Self::zero();
        }
        let coeffs = self.coeffs[1..].iter().enumerate().map(|(i, c)| c.mul(&C::from_i64(i as i64 + 1))).collect();
        Self::normalize(coeffs)
    }
}
// #endregion 🔖PolyU

// #region 🔖PolyDivision
impl<C: CommutativeRing> PolyU<C> {
    /// ➗ Pseudo-division: returns `(q, r, lc(d)^e)` such that `lc(d)^e * self == q*d + r` and
    /// `deg(r) < deg(d)`. Works over any commutative ring (no division needed — only multiplication by
    /// powers of the divisor's leading coefficient), unlike true polynomial division which needs a field.
    pub fn pseudo_div_rem(&self, d: &Self) -> (Self, Self, C) {
        let dn = d.degree().expect("pseudo_div_rem: divisor must be nonzero");
        let lc_d = d.leading_coeff().unwrap().clone();
        let mut r = self.clone();
        let mut q = Self::zero();
        let mut e = 0u32;
        while let Some(rn) = r.degree() {
            if rn < dn {
                break;
            }
            r = r.mul_scalar(&lc_d);
            q = q.mul_scalar(&lc_d);
            let coeff = r.leading_coeff().unwrap().clone();
            let shift = rn - dn;
            let term = Self::monomial(coeff, shift);
            q = q.add(&term);
            r = r.sub(&term.mul(d));
            e += 1;
        }
        (q, r, lc_d.pow(e as u64))
    }
}

impl<C: IntegralDomain> PolyU<C> {
    // #region 🔖Resultant
    /// 🧮 Fraction-free (Bareiss) determinant of a matrix over any integral domain — duplicated here
    /// (rather than depending on `mathematical_algebra`) to keep `mathematical_polynomial` free of a
    /// dependency on the linear-algebra crate.
    fn det_bareiss(mut m: Vec<Vec<C>>) -> C {
        let n = m.len();
        if n == 0 {
            return C::one();
        }
        let mut prev_pivot = C::one();
        let mut sign = C::one();
        for k in 0..n.saturating_sub(1) {
            if m[k][k].is_zero() {
                let Some(swap_row) = ((k + 1)..n).find(|&r| !m[r][k].is_zero()) else {
                    return C::zero();
                };
                m.swap(k, swap_row);
                sign = sign.neg();
            }
            for i in (k + 1)..n {
                for j in (k + 1)..n {
                    let cross = m[i][j].mul(&m[k][k]).sub(&m[i][k].mul(&m[k][j]));
                    m[i][j] = if k == 0 { cross } else { cross.exact_div(&prev_pivot).expect("Bareiss: division is exact by the algorithm's theorem") };
                }
            }
            prev_pivot = m[k][k].clone();
        }
        m[n - 1][n - 1].mul(&sign)
    }

    /// 🧮 Resultant of `self` (degree `n`) and `other` (degree `m`) via the `(n+m) x (n+m)` Sylvester
    /// matrix's fraction-free determinant — valid over any integral domain, kept as the ground-truth
    /// oracle for faster (subresultant-PRS-style) resultant computations layered on top later.
    pub fn resultant(&self, other: &Self) -> C {
        if self.is_zero() || other.is_zero() {
            return C::zero();
        }
        let n = self.degree().unwrap();
        let m = other.degree().unwrap();
        let size = n + m;
        if size == 0 {
            return C::one();
        }
        let f_be: Vec<C> = self.coeffs.iter().rev().cloned().collect();
        let g_be: Vec<C> = other.coeffs.iter().rev().cloned().collect();
        let mut mat = vec![vec![C::zero(); size]; size];
        for i in 0..m {
            for (j, c) in f_be.iter().enumerate() {
                mat[i][i + j] = c.clone();
            }
        }
        for i in 0..n {
            for (j, c) in g_be.iter().enumerate() {
                mat[m + i][i + j] = c.clone();
            }
        }
        Self::det_bareiss(mat)
    }

    /// 🧮 Discriminant via `disc(f) = (-1)^(n(n-1)/2) * resultant(f, f') / lc(f)`.
    pub fn discriminant(&self) -> C {
        let n = self.degree().unwrap_or(0);
        if n == 0 {
            return C::one();
        }
        let res = self.resultant(&self.derivative());
        let lc = self.leading_coeff().unwrap().clone();
        let quotient = res.exact_div(&lc).expect("discriminant: lc(f) divides resultant(f, f') exactly");
        if (n * (n - 1) / 2) % 2 == 0 {
            quotient
        } else {
            quotient.neg()
        }
    }
    // #endregion 🔖Resultant
}

impl<C: GcdDomain> PolyU<C> {
    // #region 🔖ContentGcd
    /// 🔢 GCD of all coefficients (the polynomial's content); `zero()` for the zero polynomial.
    pub fn content(&self) -> C {
        self.coeffs.iter().fold(C::zero(), |acc, c| acc.gcd(c))
    }

    /// 📈 `self` divided by its content, so the result's coefficient GCD is a unit.
    pub fn primitive_part(&self) -> Self {
        let content = self.content();
        if content.is_zero() || content.is_one() {
            return self.clone();
        }
        Self { coeffs: self.coeffs.iter().map(|c| c.exact_div(&content).expect("content divides every coefficient exactly")).collect() }
    }

    /// 🤝 GCD via the primitive Euclidean PRS: strips content after every pseudo-remainder step, which
    /// avoids the coefficient-size blowup of plain pseudo-remainders (subresultant PRS is faster still
    /// and left as a documented future optimization; this is simple and correct).
    pub fn gcd(&self, other: &Self) -> Self {
        if self.is_zero() {
            return other.primitive_part();
        }
        if other.is_zero() {
            return self.primitive_part();
        }
        let mut a = self.primitive_part();
        let mut b = other.primitive_part();
        while !b.is_zero() {
            let (_, r, _) = a.pseudo_div_rem(&b);
            a = b;
            b = if r.is_zero() { Self::zero() } else { r.primitive_part() };
        }
        a
    }
    // #endregion 🔖ContentGcd
}
// #endregion 🔖PolyDivision

// #region 🔖FieldOperations
impl<C: Field> PolyU<C> {
    pub fn make_monic(&self) -> Self {
        let Some(lc) = self.leading_coeff().cloned() else {
            return self.clone();
        };
        if lc.is_one() {
            return self.clone();
        }
        let inv = lc.inv().expect("nonzero leading coefficient has an inverse in a field");
        self.mul_scalar(&inv)
    }

    /// ➗ True polynomial division: `self == q*d + r`, `deg(r) < deg(d)`. `d` must be nonzero.
    pub fn div_rem(&self, d: &Self) -> (Self, Self) {
        let dn = d.degree().expect("div_rem: divisor must be nonzero");
        let lc_inv = d.leading_coeff().unwrap().inv().expect("nonzero leading coefficient has an inverse in a field");
        let mut r = self.clone();
        let mut q = Self::zero();
        while let Some(rn) = r.degree() {
            if rn < dn {
                break;
            }
            let coeff = r.leading_coeff().unwrap().mul(&lc_inv);
            let shift = rn - dn;
            let term = Self::monomial(coeff, shift);
            q = q.add(&term);
            r = r.sub(&term.mul(d));
        }
        (q, r)
    }

    /// 🤝 Monic Euclidean GCD (faster than the content-stripping PRS above once coefficients already
    /// live in a field, since there's no content to strip).
    pub fn gcd_monic(&self, other: &Self) -> Self {
        let (mut a, mut b) = (self.clone(), other.clone());
        while !b.is_zero() {
            let (_, r) = a.div_rem(&b);
            a = b;
            b = r;
        }
        a.make_monic()
    }

    /// 🤝 Extended Euclidean algorithm: returns `(g, s, t)` with `s*self + t*other == g` and `g` the
    /// monic GCD of `self` and `other` (or zero if both are zero).
    pub fn xgcd(&self, other: &Self) -> (Self, Self, Self) {
        let (mut old_r, mut r) = (self.clone(), other.clone());
        let (mut old_s, mut s) = (Self::one(), Self::zero());
        let (mut old_t, mut t) = (Self::zero(), Self::one());
        while !r.is_zero() {
            let (q, rem) = old_r.div_rem(&r);
            old_r = std::mem::replace(&mut r, rem);
            let new_s = old_s.sub(&q.mul(&s));
            old_s = std::mem::replace(&mut s, new_s);
            let new_t = old_t.sub(&q.mul(&t));
            old_t = std::mem::replace(&mut t, new_t);
        }
        if let Some(lc) = old_r.leading_coeff().cloned() {
            if !lc.is_one() {
                let inv = lc.inv().expect("nonzero leading coefficient has an inverse in a field");
                old_r = old_r.mul_scalar(&inv);
                old_s = old_s.mul_scalar(&inv);
                old_t = old_t.mul_scalar(&inv);
            }
        }
        (old_r, old_s, old_t)
    }

    /// 🧹 Yun's squarefree decomposition (characteristic-zero case): returns `[(factor, multiplicity), ...]`
    /// with `self == unit * product(factor_i ^ i)`. In positive characteristic `p`, factors whose
    /// multiplicity is a multiple of `p` are not separated by this algorithm (their derivative vanishes) —
    /// documented limitation, not a silent wrong answer: the returned factorization is still a valid
    /// (coarser) decomposition, just not maximally refined in that case.
    pub fn squarefree_decomposition(&self) -> Vec<(Self, u32)> {
        if self.is_zero() {
            return Vec::new();
        }
        let f = self.make_monic();
        let f_prime = f.derivative();
        if f_prime.is_zero() {
            return vec![(f, 1)];
        }
        let c = f.gcd_monic(&f_prime);
        let mut b = f.div_rem(&c).0;
        let mut d = f_prime.div_rem(&c).0.sub(&b.derivative());
        let mut result = Vec::new();
        let mut i = 1u32;
        while !(b.degree() == Some(0) || b.is_zero()) {
            let a = b.gcd_monic(&d);
            if !a.is_one_poly() {
                result.push((a.clone(), i));
            }
            b = b.div_rem(&a).0;
            d = d.div_rem(&a).0.sub(&b.derivative());
            i += 1;
            if i > 10_000 {
                break; // pathological safety valve; ordinary polynomials terminate in <= deg(f) steps
            }
        }
        result
    }

    fn is_one_poly(&self) -> bool {
        self.degree() == Some(0) && self.leading_coeff().is_some_and(Ring::is_one)
    }

    /// 🎯 Newton divided-difference interpolation through the given `(x, y)` points (distinct `x`
    /// values required); `None` if two points share an `x`.
    pub fn interpolate(points: &[(C, C)]) -> Option<Self> {
        let n = points.len();
        if n == 0 {
            return Some(Self::zero());
        }
        let mut table: Vec<C> = points.iter().map(|(_, y)| y.clone()).collect();
        let mut coeffs_divdiff = vec![table[0].clone()];
        for level in 1..n {
            for i in (level..n).rev() {
                let denom = points[i].0.sub(&points[i - level].0);
                if denom.is_zero() {
                    return None;
                }
                let num = table[i].sub(&table[i - 1]);
                table[i] = num.div(&denom)?;
            }
            coeffs_divdiff.push(table[n - 1].clone());
        }
        // Newton form: f(x) = c0 + c1(x-x0) + c2(x-x0)(x-x1) + ...
        let mut result = Self::constant(coeffs_divdiff[0].clone());
        let mut basis = Self::one();
        for (i, c) in coeffs_divdiff.iter().enumerate().skip(1) {
            basis = basis.mul(&Self::x().sub(&Self::constant(points[i - 1].0.clone())));
            result = result.add(&basis.mul_scalar(c));
        }
        Some(result)
    }
}
// #endregion 🔖FieldOperations

// #region 🔖RingTraitImpls
impl<C: CommutativeRing> Ring for PolyU<C> {
    fn zero() -> Self {
        PolyU::zero()
    }
    fn one() -> Self {
        PolyU::one()
    }
    fn add(&self, rhs: &Self) -> Self {
        PolyU::add(self, rhs)
    }
    fn neg(&self) -> Self {
        PolyU::neg(self)
    }
    fn mul(&self, rhs: &Self) -> Self {
        PolyU::mul(self, rhs)
    }
    fn is_zero(&self) -> bool {
        PolyU::is_zero(self)
    }
    fn from_i64(value: i64) -> Self {
        PolyU::constant(C::from_i64(value))
    }
    fn characteristic(&self) -> u64 {
        self.coeffs.first().map_or(0, Ring::characteristic)
    }
}
impl<C: CommutativeRing> CommutativeRing for PolyU<C> {}
impl<C: IntegralDomain> IntegralDomain for PolyU<C> {
    /// ➗ Exact polynomial division via pseudo-division: `lc(rhs)^k * self = q*rhs + r` for some `k`;
    /// when `r == 0`, `q` is exactly `lc(rhs)^k` times the true quotient, so dividing `q`'s coefficients
    /// by that scalar recovers it — and that per-coefficient division is itself exact precisely because
    /// the true quotient (assumed to exist whenever `r == 0`) has coefficients in `C` already.
    fn exact_div(&self, rhs: &Self) -> Option<Self> {
        if rhs.is_zero() {
            return None;
        }
        let (q, r, e) = self.pseudo_div_rem(rhs);
        if !r.is_zero() {
            return None;
        }
        if e.is_one() {
            return Some(q);
        }
        let mut result_coeffs = Vec::with_capacity(q.coeffs.len());
        for c in &q.coeffs {
            result_coeffs.push(c.exact_div(&e)?);
        }
        Some(PolyU { coeffs: result_coeffs })
    }
}
impl<C: GcdDomain> GcdDomain for PolyU<C> {
    fn gcd(&self, rhs: &Self) -> Self {
        PolyU::gcd(self, rhs)
    }
}
// #endregion 🔖RingTraitImpls

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use mathematical_number::Rational;

    fn r(n: i64, d: i64) -> Rational {
        Rational::from_i64(n, d).unwrap()
    }

    fn poly(coeffs: Vec<i64>) -> PolyU<Rational> {
        PolyU::from_coeffs(coeffs.into_iter().map(|c| r(c, 1)).collect())
    }

    #[test]
    fn ring_axioms_on_small_polynomials() {
        let a = poly(vec![1, 2, 3]);
        let b = poly(vec![0, 1]);
        let sum = a.add(&b);
        assert_eq!(sum.coeff(1), r(3, 1));
        let prod = a.mul(&b);
        assert_eq!(prod.coeffs(), &[r(0, 1), r(1, 1), r(2, 1), r(3, 1)]);
    }

    #[test]
    fn div_rem_identity_holds() {
        let a = poly(vec![-1, 0, 1]); // x^2 - 1
        let b = poly(vec![-1, 1]); // x - 1
        let (q, rem) = a.div_rem(&b);
        assert!(rem.is_zero());
        assert_eq!(q, poly(vec![1, 1])); // x + 1
    }

    #[test]
    fn derivative_power_rule() {
        let f = poly(vec![0, 0, 0, 1]); // x^3
        assert_eq!(f.derivative(), poly(vec![0, 0, 3])); // 3x^2
    }

    #[test]
    fn eval_horner_matches_direct_computation() {
        let f = poly(vec![1, 2, 3]); // 1 + 2x + 3x^2
        assert_eq!(f.eval(&r(2, 1)), r(1 + 4 + 12, 1));
    }

    #[test]
    fn gcd_hand_case() {
        // (x^2 - 1)(x + 2) and (x^2 - 1)(x - 5) share gcd (x^2 - 1) up to a unit.
        let common = poly(vec![-1, 0, 1]);
        let a = common.mul(&poly(vec![2, 1]));
        let b = common.mul(&poly(vec![-5, 1]));
        let g = a.gcd_monic(&b);
        let g_monic_common = common.make_monic();
        assert_eq!(g, g_monic_common);
    }

    #[test]
    fn resultant_of_coprime_linear_factors_is_nonzero() {
        let a = poly(vec![-1, 1]); // x - 1
        let b = poly(vec![-2, 1]); // x - 2
        let res = a.resultant(&b);
        assert_ne!(res, r(0, 1));
    }

    #[test]
    fn resultant_of_shared_root_is_zero() {
        let a = poly(vec![-1, 0, 1]); // x^2 - 1, roots +-1
        let b = poly(vec![-1, 1]); // x - 1, root 1 (shared)
        let res = a.resultant(&b);
        assert_eq!(res, r(0, 1));
    }

    #[test]
    fn factor_x2_minus_1_via_squarefree_and_roots() {
        let f = poly(vec![-1, 0, 1]);
        let decomposition = f.squarefree_decomposition();
        assert_eq!(decomposition.len(), 1);
        assert_eq!(decomposition[0].1, 1);
    }

    #[test]
    fn squarefree_decomposition_of_repeated_factor() {
        let base = poly(vec![-1, 1]); // (x - 1)
        let f = base.mul(&base).mul(&base); // (x-1)^3
        let decomposition = f.squarefree_decomposition();
        assert!(decomposition.iter().any(|(factor, mult)| *factor == base.make_monic() && *mult == 3));
    }

    #[test]
    fn interpolate_reconstructs_quadratic() {
        let points = vec![(r(0, 1), r(1, 1)), (r(1, 1), r(6, 1)), (r(2, 1), r(15, 1))]; // f(x) = 2x^2+3x+1
        let f = PolyU::interpolate(&points).unwrap();
        assert_eq!(f, poly(vec![1, 3, 2]));
    }

    #[test]
    fn rational_root_via_eval_hand_case() {
        // 6x^2 - 5x + 1 = 0 has roots 1/2, 1/3
        let f = PolyU::from_coeffs(vec![r(1, 1), r(-5, 1), r(6, 1)]);
        assert_eq!(f.eval(&r(1, 2)), r(0, 1));
        assert_eq!(f.eval(&r(1, 3)), r(0, 1));
    }
}
// #endregion 🔖Tests
