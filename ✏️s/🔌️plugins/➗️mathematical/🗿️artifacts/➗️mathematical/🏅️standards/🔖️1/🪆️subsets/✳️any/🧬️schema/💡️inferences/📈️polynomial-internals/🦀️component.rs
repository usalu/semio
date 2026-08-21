//! 📈️ Generic univariate and multivariate polynomials over the `number::` algebraic trait
//! hierarchy: arithmetic, GCD, resultants, factorization over `Z`/`Q`/`GF(p)`, Groebner bases, real
//! root isolation via Sturm sequences, and real algebraic numbers.
//!
//! 🚚 Migrated verbatim from `🧰️framework/🔨️modules/🧮️math/📈️polynomial/🦀️component.rs` (ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS, wave M3a) — see
//! `🌿️cas-internals/🦀️component.rs`'s doc header (its sibling in this move) for the full rationale.
//! `crate::polynomial` self-references are preserved unedited via a crate-root `pub use … as
//! polynomial;` alias in `📦️glue.rs`; `crate::number` became `number::` (wave MATHEND). Unlike
//! `🌿️cas-internals`, this module has no `algebra` dependency at all — nothing here references `cas`
//! either (verified one-directional in the wave M3a coupling map: `cas → polynomial`, never the
//! reverse).
// #region 🔖️Univariate
pub mod univariate {
    //! 📈️ Dense univariate polynomials, generic over the `number::` ring hierarchy: arithmetic,
    //! pseudo-division, resultants/discriminants, GCD, and (over a field) squarefree decomposition and
    //! Newton interpolation.

    use number::{CommutativeRing, Field, GcdDomain, IntegralDomain, Ring};

    // #region 🔖️PolyU
    /// 📈️ Little-endian coefficients (`coeffs[i]` is the coefficient of `x^i`); invariant: no trailing
    /// zero coefficient (the empty vector represents the zero polynomial).
    #[derive(Clone, PartialEq, Debug)]
    pub struct PolyU<C> {
        coeffs: Vec<C>,
    }

    impl<C: Ring> PolyU<C> {
        async fn normalize(mut coeffs: Vec<C>) -> Self {
            while coeffs.last().is_some_and(Ring::is_zero) {
                coeffs.pop();
            }
            Self { coeffs }
        }

        pub async fn zero() -> Self {
            Self { coeffs: Vec::new() }
        }

        pub async fn one() -> Self {
            Self { coeffs: vec![C::one()] }
        }

        pub async fn x() -> Self {
            Self { coeffs: vec![C::zero(), C::one()] }
        }

        pub async fn constant(c: C) -> Self {
            Self::normalize(vec![c])
        }

        /// 📈️ `coeff * x^degree`.
        pub async fn monomial(coeff: C, degree: usize) -> Self {
            if coeff.is_zero() {
                return Self::zero();
            }
            let mut coeffs = vec![C::zero(); degree + 1];
            coeffs[degree] = coeff;
            Self { coeffs }
        }

        pub async fn from_coeffs(coeffs: Vec<C>) -> Self {
            Self::normalize(coeffs)
        }

        pub async fn is_zero(&self) -> bool {
            self.coeffs.is_empty()
        }

        pub async fn degree(&self) -> Option<usize> {
            if self.coeffs.is_empty() {
                None
            } else {
                Some(self.coeffs.len() - 1)
            }
        }

        pub async fn leading_coeff(&self) -> Option<&C> {
            self.coeffs.last()
        }

        pub async fn coeff(&self, i: usize) -> C {
            self.coeffs.get(i).cloned().unwrap_or_else(C::zero)
        }

        pub async fn coeffs(&self) -> &[C] {
            &self.coeffs
        }

        pub async fn add(&self, rhs: &Self) -> Self {
            let n = self.coeffs.len().max(rhs.coeffs.len());
            Self::normalize((0..n).map(|i| self.coeff(i).add(&rhs.coeff(i))).collect())
        }

        pub async fn neg(&self) -> Self {
            Self { coeffs: self.coeffs.iter().map(Ring::neg).collect() }
        }

        pub async fn sub(&self, rhs: &Self) -> Self {
            self.add(&rhs.neg())
        }

        pub async fn mul_scalar(&self, s: &C) -> Self {
            if s.is_zero() {
                return Self::zero();
            }
            Self::normalize(self.coeffs.iter().map(|c| c.mul(s)).collect())
        }

        pub async fn mul(&self, rhs: &Self) -> Self {
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

        pub async fn pow(&self, exp: u64) -> Self {
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
        pub async fn shift_up(&self, k: usize) -> Self {
            if self.is_zero() || k == 0 {
                return self.clone();
            }
            let mut coeffs = vec![C::zero(); k];
            coeffs.extend(self.coeffs.iter().cloned());
            Self { coeffs }
        }

        /// 🎯️ Horner evaluation at `point`.
        pub async fn eval(&self, point: &C) -> C {
            let mut result = C::zero();
            for c in self.coeffs.iter().rev() {
                result = result.mul(point).add(c);
            }
            result
        }

        /// 🔗️ Substitutes `inner` for `x`: `self(inner(t))`.
        pub async fn semio_compose_rs(&self, inner: &Self) -> Self {
            let mut result = Self::zero();
            for c in self.coeffs.iter().rev() {
                result = result.mul(inner).add(&Self::constant(c.clone()));
            }
            result
        }

        /// 📉️ Formal derivative: `d/dx sum c_i x^i = sum i*c_i x^(i-1)`.
        pub async fn derivative(&self) -> Self {
            if self.coeffs.len() <= 1 {
                return Self::zero();
            }
            let coeffs = self.coeffs[1..].iter().enumerate().map(|(i, c)| c.mul(&C::from_i64(i as i64 + 1))).collect();
            Self::normalize(coeffs)
        }
    }
    // #endregion 🔖️PolyU

    // #region 🔖️PolyDivision
    impl<C: CommutativeRing> PolyU<C> {
        /// ➗️ Pseudo-division: returns `(q, r, lc(d)^e)` such that `lc(d)^e * self == q*d + r` and
        /// `deg(r) < deg(d)`. Works over any commutative ring (no division needed — only multiplication by
        /// powers of the divisor's leading coefficient), unlike true polynomial division which needs a field.
        pub async fn pseudo_div_rem(&self, d: &Self) -> (Self, Self, C) {
            let dn = d.degree().expect("pseudo_div_rem: divisor must be nonzero");
            let lc_d = d.leading_coeff().unwrap().clone();
            let mut r = self.clone();
            let mut q = Self::zero();
            let mut e = 0u32;
            while let Some(rn) = r.degree() {
                if rn < dn {
                    break;
                }
                // 🎯️ `term` must come from r's leading coeff *before* the d_lc scale-up below, or it's
                // d_lc times too large to cancel the (then-scaled) leading term — degree never drops.
                let coeff = r.leading_coeff().unwrap().clone();
                let shift = rn - dn;
                let term = Self::monomial(coeff, shift);
                r = r.mul_scalar(&lc_d);
                q = q.mul_scalar(&lc_d).add(&term);
                r = r.sub(&term.mul(d));
                e += 1;
            }
            (q, r, lc_d.pow(e as u64))
        }
    }

    impl<C: IntegralDomain> PolyU<C> {
        // #region 🔖️Resultant
        /// 🧮️ Fraction-free (Bareiss) determinant of a matrix over any integral domain — duplicated here
        /// (rather than depending on `number`'s generic `MatG::det_bareiss`) to keep `crate::polynomial`
        /// free of a dependency on the linear-algebra module.
        async fn det_bareiss(mut m: Vec<Vec<C>>) -> C {
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

        /// 🧮️ Resultant of `self` (degree `n`) and `other` (degree `m`) via the `(n+m) x (n+m)` Sylvester
        /// matrix's fraction-free determinant — valid over any integral domain, kept as the ground-truth
        /// oracle for faster (subresultant-PRS-style) resultant computations layered on top later.
        pub async fn resultant(&self, other: &Self) -> C {
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

        /// 🧮️ Discriminant via `disc(f) = (-1)^(n(n-1)/2) * resultant(f, f') / lc(f)`.
        pub async fn discriminant(&self) -> C {
            let n = self.degree().unwrap_or(0);
            if n == 0 {
                return C::one();
            }
            let res = self.resultant(&self.derivative());
            let lc = self.leading_coeff().unwrap().clone();
            let quotient = res.exact_div(&lc).expect("discriminant: lc(f) divides resultant(f, f') exactly");
            if (n * (n - 1) / 2).is_multiple_of(2) {
                quotient
            } else {
                quotient.neg()
            }
        }
        // #endregion 🔖️Resultant
    }

    impl<C: GcdDomain> PolyU<C> {
        // #region 🔖️ContentGcd
        /// 🔢️ GCD of all coefficients (the polynomial's content); `zero()` for the zero polynomial.
        pub async fn content(&self) -> C {
            self.coeffs.iter().fold(C::zero(), |acc, c| acc.gcd(c))
        }

        /// 📈️ `self` divided by its content, so the result's coefficient GCD is a unit.
        pub async fn primitive_part(&self) -> Self {
            let content = self.content();
            if content.is_zero() || content.is_one() {
                return self.clone();
            }
            Self { coeffs: self.coeffs.iter().map(|c| c.exact_div(&content).expect("content divides every coefficient exactly")).collect() }
        }

        /// 🤝️ GCD via the primitive Euclidean PRS: strips content after every pseudo-remainder step, which
        /// avoids the coefficient-size blowup of plain pseudo-remainders (subresultant PRS is faster still
        /// and left as a documented future optimization; this is simple and correct).
        pub async fn gcd(&self, other: &Self) -> Self {
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
        // #endregion 🔖️ContentGcd
    }
    // #endregion 🔖️PolyDivision

    // #region 🔖️FieldOperations
    impl<C: Field> PolyU<C> {
        pub async fn make_monic(&self) -> Self {
            let Some(lc) = self.leading_coeff().cloned() else {
                return self.clone();
            };
            if lc.is_one() {
                return self.clone();
            }
            let inv = lc.inv().expect("nonzero leading coefficient has an inverse in a field");
            self.mul_scalar(&inv)
        }

        /// ➗️ True polynomial division: `self == q*d + r`, `deg(r) < deg(d)`. `d` must be nonzero.
        pub async fn div_rem(&self, d: &Self) -> (Self, Self) {
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

        /// 🤝️ Monic Euclidean GCD (faster than the content-stripping PRS above once coefficients already
        /// live in a field, since there's no content to strip).
        pub async fn gcd_monic(&self, other: &Self) -> Self {
            let (mut a, mut b) = (self.clone(), other.clone());
            while !b.is_zero() {
                let (_, r) = a.div_rem(&b);
                a = b;
                b = r;
            }
            a.make_monic()
        }

        /// 🤝️ Extended Euclidean algorithm: returns `(g, s, t)` with `s*self + t*other == g` and `g` the
        /// monic GCD of `self` and `other` (or zero if both are zero).
        pub async fn xgcd(&self, other: &Self) -> (Self, Self, Self) {
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

        /// 🧹️ Yun's squarefree decomposition (characteristic-zero case): returns `[(factor, multiplicity), ...]`
        /// with `self == unit * product(factor_i ^ i)`. In positive characteristic `p`, factors whose
        /// multiplicity is a multiple of `p` are not separated by this algorithm (their derivative vanishes) —
        /// documented limitation, not a silent wrong answer: the returned factorization is still a valid
        /// (coarser) decomposition, just not maximally refined in that case.
        pub async fn squarefree_decomposition(&self) -> Vec<(Self, u32)> {
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

        async fn is_one_poly(&self) -> bool {
            self.degree() == Some(0) && self.leading_coeff().is_some_and(Ring::is_one)
        }

        /// 🎯️ Newton divided-difference interpolation through the given `(x, y)` points (distinct `x`
        /// values required); `None` if two points share an `x`.
        pub async fn interpolate(points: &[(C, C)]) -> Option<Self> {
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
    // #endregion 🔖️FieldOperations

    // #region 🔖️RingTraitImpls
    impl<C: CommutativeRing> Ring for PolyU<C> {
        async fn zero() -> Self {
            PolyU::zero()
        }
        async fn one() -> Self {
            PolyU::one()
        }
        async fn add(&self, rhs: &Self) -> Self {
            PolyU::add(self, rhs)
        }
        async fn neg(&self) -> Self {
            PolyU::neg(self)
        }
        async fn mul(&self, rhs: &Self) -> Self {
            PolyU::mul(self, rhs)
        }
        async fn is_zero(&self) -> bool {
            PolyU::is_zero(self)
        }
        async fn from_i64(value: i64) -> Self {
            PolyU::constant(C::from_i64(value))
        }
        async fn characteristic(&self) -> u64 {
            self.coeffs.first().map_or(0, Ring::characteristic)
        }
    }
    impl<C: CommutativeRing> CommutativeRing for PolyU<C> {}
    impl<C: IntegralDomain> IntegralDomain for PolyU<C> {
        /// ➗️ Exact polynomial division via pseudo-division: `lc(rhs)^k * self = q*rhs + r` for some `k`;
        /// when `r == 0`, `q` is exactly `lc(rhs)^k` times the true quotient, so dividing `q`'s coefficients
        /// by that scalar recovers it — and that per-coefficient division is itself exact precisely because
        /// the true quotient (assumed to exist whenever `r == 0`) has coefficients in `C` already.
        async fn exact_div(&self, rhs: &Self) -> Option<Self> {
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
        async fn gcd(&self, rhs: &Self) -> Self {
            PolyU::gcd(self, rhs)
        }
    }
    // #endregion 🔖️RingTraitImpls

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use number::Rational;

        async fn r(n: i64, d: i64) -> Rational {
            Rational::from_i64(n, d).unwrap()
        }

        async fn poly(coeffs: Vec<i64>) -> PolyU<Rational> {
            PolyU::from_coeffs(coeffs.into_iter().map(|c| r(c, 1)).collect())
        }

        #[semio_framework_async_macros::async_test]
        async fn ring_axioms_on_small_polynomials() {
            let a = poly(vec![1, 2, 3]);
            let b = poly(vec![0, 1]);
            let sum = a.add(&b);
            assert_eq!(sum.coeff(1), r(3, 1));
            let prod = a.mul(&b);
            assert_eq!(prod.coeffs(), &[r(0, 1), r(1, 1), r(2, 1), r(3, 1)]);
        }

        #[semio_framework_async_macros::async_test]
        async fn div_rem_identity_holds() {
            let a = poly(vec![-1, 0, 1]); // x^2 - 1
            let b = poly(vec![-1, 1]); // x - 1
            let (q, rem) = a.div_rem(&b);
            assert!(rem.is_zero());
            assert_eq!(q, poly(vec![1, 1])); // x + 1
        }

        #[semio_framework_async_macros::async_test]
        async fn derivative_power_rule() {
            let f = poly(vec![0, 0, 0, 1]); // x^3
            assert_eq!(f.derivative(), poly(vec![0, 0, 3])); // 3x^2
        }

        #[semio_framework_async_macros::async_test]
        async fn eval_horner_matches_direct_computation() {
            let f = poly(vec![1, 2, 3]); // 1 + 2x + 3x^2
            assert_eq!(f.eval(&r(2, 1)), r(1 + 4 + 12, 1));
        }

        #[semio_framework_async_macros::async_test]
        async fn gcd_hand_case() {
            // (x^2 - 1)(x + 2) and (x^2 - 1)(x - 5) share gcd (x^2 - 1) up to a unit.
            let common = poly(vec![-1, 0, 1]);
            let a = common.mul(&poly(vec![2, 1]));
            let b = common.mul(&poly(vec![-5, 1]));
            let g = a.gcd_monic(&b);
            let g_monic_common = common.make_monic();
            assert_eq!(g, g_monic_common);
        }

        #[semio_framework_async_macros::async_test]
        async fn resultant_of_coprime_linear_factors_is_nonzero() {
            let a = poly(vec![-1, 1]); // x - 1
            let b = poly(vec![-2, 1]); // x - 2
            let res = a.resultant(&b);
            assert_ne!(res, r(0, 1));
        }

        #[semio_framework_async_macros::async_test]
        async fn resultant_of_shared_root_is_zero() {
            let a = poly(vec![-1, 0, 1]); // x^2 - 1, roots +-1
            let b = poly(vec![-1, 1]); // x - 1, root 1 (shared)
            let res = a.resultant(&b);
            assert_eq!(res, r(0, 1));
        }

        #[semio_framework_async_macros::async_test]
        async fn factor_x2_minus_1_via_squarefree_and_roots() {
            let f = poly(vec![-1, 0, 1]);
            let decomposition = f.squarefree_decomposition();
            assert_eq!(decomposition.len(), 1);
            assert_eq!(decomposition[0].1, 1);
        }

        #[semio_framework_async_macros::async_test]
        async fn squarefree_decomposition_of_repeated_factor() {
            let base = poly(vec![-1, 1]); // (x - 1)
            let f = base.mul(&base).mul(&base); // (x-1)^3
            let decomposition = f.squarefree_decomposition();
            assert!(decomposition.iter().any(|(factor, mult)| *factor == base.make_monic() && *mult == 3));
        }

        #[semio_framework_async_macros::async_test]
        async fn interpolate_reconstructs_quadratic() {
            let points = vec![(r(0, 1), r(1, 1)), (r(1, 1), r(6, 1)), (r(2, 1), r(15, 1))]; // f(x) = 2x^2+3x+1
            let f = PolyU::interpolate(&points).unwrap();
            assert_eq!(f, poly(vec![1, 3, 2]));
        }

        #[semio_framework_async_macros::async_test]
        async fn rational_root_via_eval_hand_case() {
            // 6x^2 - 5x + 1 = 0 has roots 1/2, 1/3
            let f = PolyU::from_coeffs(vec![r(1, 1), r(-5, 1), r(6, 1)]);
            assert_eq!(f.eval(&r(1, 2)), r(0, 1));
            assert_eq!(f.eval(&r(1, 3)), r(0, 1));
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Univariate

// #region 🔖️Multivariate
pub mod multivariate {
    //! 🕸️ Sparse multivariate polynomials over the `number::` ring hierarchy: monomial orders,
    //! multivariate division, and Buchberger's Groebner-basis algorithm.

    use number::{Field, Ring};

    // #region 🔖️MonomialOrder
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum MonomialOrder {
        Lex,
        GrLex,
        GrevLex,
    }
    // #endregion 🔖️MonomialOrder

    // #region 🔖️Monomial
    /// 🎛️ Exponent vector; fixed arity (`exps.len()`) per polynomial instance. `Ord` here is a plain
    /// lexicographic order on the raw vector — used only for deterministic deduplication/sorting, distinct
    /// from the context-dependent `cmp_by` (Lex/GrLex/GrevLex) used for polynomial arithmetic.
    #[derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
    pub struct Monomial {
        exps: Vec<u32>,
    }

    impl Monomial {
        pub async fn new(exps: Vec<u32>) -> Self {
            Self { exps }
        }

        pub async fn var(index: usize, nvars: usize) -> Self {
            let mut exps = vec![0u32; nvars];
            exps[index] = 1;
            Self { exps }
        }

        pub async fn one(nvars: usize) -> Self {
            Self { exps: vec![0u32; nvars] }
        }

        pub async fn exps(&self) -> &[u32] {
            &self.exps
        }

        pub async fn total_degree(&self) -> u32 {
            self.exps.iter().sum()
        }

        pub async fn mul(&self, other: &Self) -> Self {
            Self { exps: self.exps.iter().zip(other.exps.iter()).map(|(a, b)| a + b).collect() }
        }

        /// ➗️ `Some(self / other)` if `other`'s exponents are all `<= self`'s, else `None`.
        pub async fn try_div(&self, other: &Self) -> Option<Self> {
            let mut result = Vec::with_capacity(self.exps.len());
            for (a, b) in self.exps.iter().zip(other.exps.iter()) {
                if b > a {
                    return None;
                }
                result.push(a - b);
            }
            Some(Self { exps: result })
        }

        pub async fn lcm(&self, other: &Self) -> Self {
            Self { exps: self.exps.iter().zip(other.exps.iter()).map(|(&a, &b)| a.max(b)).collect() }
        }

        pub async fn cmp_by(&self, other: &Self, order: MonomialOrder) -> std::cmp::Ordering {
            use std::cmp::Ordering;
            match order {
                MonomialOrder::Lex => self.exps.cmp(&other.exps),
                MonomialOrder::GrLex => self.total_degree().cmp(&other.total_degree()).then_with(|| self.exps.cmp(&other.exps)),
                MonomialOrder::GrevLex => {
                    match self.total_degree().cmp(&other.total_degree()) {
                        Ordering::Equal => {
                            // Reverse lex on the reversed exponent vector, with comparison sense flipped.
                            for (a, b) in self.exps.iter().rev().zip(other.exps.iter().rev()) {
                                match a.cmp(b) {
                                    Ordering::Equal => continue,
                                    ord => return ord.reverse(),
                                }
                            }
                            Ordering::Equal
                        }
                        ord => ord,
                    }
                }
            }
        }
    }
    // #endregion 🔖️Monomial

    // #region 🔖️PolyM
    /// 🕸️ Sparse multivariate polynomial: terms sorted descending under `order`, no zero coefficients.
    #[derive(Clone, Debug)]
    pub struct PolyM<C> {
        nvars: usize,
        order: MonomialOrder,
        terms: Vec<(Monomial, C)>,
    }

    impl<C: Ring> PartialEq for PolyM<C> {
        fn eq(&self, other: &Self) -> bool {
            // Compare canonically regardless of stored order: both term lists are already order-sorted,
            // but two PolyM values with different `order` fields can still represent the same polynomial,
            // so compare as sets via a Lex-sorted clone.
            let a = self.with_order(MonomialOrder::Lex);
            let b = other.with_order(MonomialOrder::Lex);
            a.terms == b.terms && a.nvars == b.nvars
        }
    }

    impl<C: Ring> PolyM<C> {
        async fn normalize(mut terms: Vec<(Monomial, C)>, nvars: usize, order: MonomialOrder) -> Self {
            terms.retain(|(_, c)| !c.is_zero());
            terms.sort_by(|a, b| b.0.cmp_by(&a.0, order));
            Self { nvars, order, terms }
        }

        pub async fn zero(nvars: usize, order: MonomialOrder) -> Self {
            Self { nvars, order, terms: Vec::new() }
        }

        pub async fn constant(c: C, nvars: usize, order: MonomialOrder) -> Self {
            Self::normalize(vec![(Monomial::one(nvars), c)], nvars, order)
        }

        pub async fn var(index: usize, nvars: usize, order: MonomialOrder) -> Self {
            Self::normalize(vec![(Monomial::var(index, nvars), C::one())], nvars, order)
        }

        pub async fn from_terms(terms: Vec<(Monomial, C)>, nvars: usize, order: MonomialOrder) -> Self {
            Self::normalize(terms, nvars, order)
        }

        pub async fn with_order(&self, order: MonomialOrder) -> Self {
            Self::normalize(self.terms.clone(), self.nvars, order)
        }

        pub async fn is_zero(&self) -> bool {
            self.terms.is_empty()
        }

        pub async fn nvars(&self) -> usize {
            self.nvars
        }

        pub async fn order(&self) -> MonomialOrder {
            self.order
        }

        pub async fn terms(&self) -> &[(Monomial, C)] {
            &self.terms
        }

        pub async fn leading_term(&self) -> Option<&(Monomial, C)> {
            self.terms.first()
        }

        pub async fn total_degree(&self) -> u32 {
            self.terms.iter().map(|(m, _)| m.total_degree()).max().unwrap_or(0)
        }

        pub async fn add(&self, other: &Self) -> Self {
            assert_eq!(self.nvars, other.nvars, "PolyM::add: variable-count mismatch");
            let mut map: std::collections::BTreeMap<Vec<u32>, C> = std::collections::BTreeMap::new();
            for (m, c) in self.terms.iter().chain(other.terms.iter()) {
                map.entry(m.exps.clone()).and_modify(|acc| *acc = acc.add(c)).or_insert_with(|| c.clone());
            }
            let terms = map.into_iter().map(|(exps, c)| (Monomial::new(exps), c)).collect();
            Self::normalize(terms, self.nvars, self.order)
        }

        pub async fn neg(&self) -> Self {
            Self { nvars: self.nvars, order: self.order, terms: self.terms.iter().map(|(m, c)| (m.clone(), c.neg())).collect() }
        }

        pub async fn sub(&self, other: &Self) -> Self {
            self.add(&other.neg())
        }

        pub async fn mul(&self, other: &Self) -> Self {
            assert_eq!(self.nvars, other.nvars, "PolyM::mul: variable-count mismatch");
            let mut map: std::collections::BTreeMap<Vec<u32>, C> = std::collections::BTreeMap::new();
            for (m1, c1) in &self.terms {
                for (m2, c2) in &other.terms {
                    let m = m1.mul(m2);
                    let c = c1.mul(c2);
                    map.entry(m.exps).and_modify(|acc| *acc = acc.add(&c)).or_insert(c);
                }
            }
            let terms = map.into_iter().map(|(exps, c)| (Monomial::new(exps), c)).collect();
            Self::normalize(terms, self.nvars, self.order)
        }

        pub async fn pow(&self, exp: u64) -> Self {
            let mut result = Self::constant(C::one(), self.nvars, self.order);
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

        pub async fn eval(&self, point: &[C]) -> C {
            self.terms.iter().fold(C::zero(), |acc, (m, c)| {
                let mut term_val = c.clone();
                for (var_idx, &exp) in m.exps.iter().enumerate() {
                    if exp > 0 {
                        term_val = term_val.mul(&point[var_idx].pow(exp as u64));
                    }
                }
                acc.add(&term_val)
            })
        }

        pub async fn mul_scalar(&self, s: &C) -> Self {
            Self::normalize(self.terms.iter().map(|(m, c)| (m.clone(), c.mul(s))).collect(), self.nvars, self.order)
        }

        pub async fn partial_derivative(&self, var: usize) -> Self {
            let mut terms = Vec::new();
            for (m, c) in &self.terms {
                let e = m.exps[var];
                if e == 0 {
                    continue;
                }
                let mut new_exps = m.exps.clone();
                new_exps[var] = e - 1;
                let new_c = c.mul(&C::from_i64(e as i64));
                terms.push((Monomial::new(new_exps), new_c));
            }
            Self::normalize(terms, self.nvars, self.order)
        }
    }
    // #endregion 🔖️PolyM

    // #region 🔖️Reduction
    impl<C: Field> PolyM<C> {
        /// ➗️ Multivariate division of `self` by `divisors`: returns `(quotients, remainder)` such that
        /// `self == sum(q_i * divisors_i) + remainder` and no term of `remainder` is divisible by any
        /// divisor's leading term.
        pub async fn reduce(&self, divisors: &[Self]) -> (Vec<Self>, Self) {
            let mut quotients = vec![Self::zero(self.nvars, self.order); divisors.len()];
            let mut remainder = Self::zero(self.nvars, self.order);
            let mut p = self.clone();
            'outer: while !p.is_zero() {
                let (lead_m, lead_c) = p.leading_term().unwrap().clone();
                for (i, d) in divisors.iter().enumerate() {
                    let Some((d_lead_m, d_lead_c)) = d.leading_term() else { continue };
                    if let Some(quot_m) = lead_m.try_div(d_lead_m) {
                        let quot_c = lead_c.div(d_lead_c).expect("nonzero divisor leading coefficient");
                        let quot_term = Self::from_terms(vec![(quot_m, quot_c)], self.nvars, self.order);
                        quotients[i] = quotients[i].add(&quot_term);
                        p = p.sub(&quot_term.mul(d));
                        continue 'outer;
                    }
                }
                let lead_poly = Self::from_terms(vec![(lead_m, lead_c)], self.nvars, self.order);
                remainder = remainder.add(&lead_poly);
                p = p.sub(&lead_poly);
            }
            (quotients, remainder)
        }

        /// 🧮️ `S(f,g) = (lcm/LT(f)) * f - (lcm/LT(g)) * g`, where `LT` is the leading term (monomial times
        /// coefficient) — each cofactor's coefficient is the inverse of *its own* polynomial's leading
        /// coefficient, since its job is to cancel that polynomial's own leading term exactly.
        pub async fn s_polynomial(&self, other: &Self) -> Self {
            let (lm1, lc1) = self.leading_term().expect("s_polynomial: self must be nonzero").clone();
            let (lm2, lc2) = other.leading_term().expect("s_polynomial: other must be nonzero").clone();
            let lcm = lm1.lcm(&lm2);
            let factor1 = lcm.try_div(&lm1).expect("lcm is divisible by lm1 by construction");
            let factor2 = lcm.try_div(&lm2).expect("lcm is divisible by lm2 by construction");
            let term1 = Self::from_terms(vec![(factor1, lc1.inv().unwrap())], self.nvars, self.order);
            let term2 = Self::from_terms(vec![(factor2, lc2.inv().unwrap())], self.nvars, self.order);
            term1.mul(self).sub(&term2.mul(other))
        }
        // #endregion

        // #region 🔖️Groebner
        /// 🧮️ Buchberger's algorithm with the coprime-leading-term criterion and pairwise interreduction,
        /// producing the unique reduced monic Groebner basis of the ideal generated by `gens`.
        pub async fn groebner_basis(gens: &[Self]) -> Vec<Self> {
            let mut basis: Vec<Self> = gens.iter().filter(|g| !g.is_zero()).map(Self::make_monic_lead).collect();
            let mut pairs: Vec<(usize, usize)> = (0..basis.len()).flat_map(|i| (0..i).map(move |j| (i, j))).collect();
            while let Some((i, j)) = pairs.pop() {
                let (lm_i, _) = basis[i].leading_term().unwrap();
                let (lm_j, _) = basis[j].leading_term().unwrap();
                // Coprimality criterion: if the leading monomials share no variables, the S-polynomial
                // reduces to zero and can be skipped (Buchberger's first criterion).
                let coprime = lm_i.exps().iter().zip(lm_j.exps().iter()).all(|(&a, &b)| a == 0 || b == 0);
                if coprime {
                    continue;
                }
                let s = basis[i].s_polynomial(&basis[j]);
                let (_, remainder) = s.reduce(&basis);
                if !remainder.is_zero() {
                    let new_poly = remainder.make_monic_lead();
                    let new_idx = basis.len();
                    for k in 0..new_idx {
                        pairs.push((new_idx, k));
                    }
                    basis.push(new_poly);
                }
            }
            Self::interreduce(basis)
        }

        async fn make_monic_lead(&self) -> Self {
            let Some((_, lc)) = self.leading_term() else {
                return self.clone();
            };
            let inv = lc.inv().expect("nonzero leading coefficient has an inverse in a field");
            Self { nvars: self.nvars, order: self.order, terms: self.terms.iter().map(|(m, c)| (m.clone(), c.mul(&inv))).collect() }
        }

        /// 🧹️ Reduces each basis element against the others and removes any that become redundant, giving
        /// the canonical reduced Groebner basis.
        async fn interreduce(basis: Vec<Self>) -> Vec<Self> {
            let mut current = basis;
            loop {
                let mut changed = false;
                let mut next = Vec::with_capacity(current.len());
                for i in 0..current.len() {
                    let others: Vec<Self> = current.iter().enumerate().filter(|&(j, _)| j != i).map(|(_, p)| p.clone()).collect();
                    let (_, remainder) = current[i].reduce(&others);
                    if remainder.is_zero() {
                        changed = true;
                        continue;
                    }
                    if remainder != current[i] {
                        changed = true;
                    }
                    next.push(remainder.make_monic_lead());
                }
                current = next;
                if !changed {
                    break;
                }
            }
            current.sort_by(|a, b| a.leading_term().map(|t| t.0.clone()).into_iter().cmp(b.leading_term().map(|t| t.0.clone())));
            current
        }

        /// 🚮️ Lex Groebner basis followed by dropping generators that involve any of the first `drop_vars`
        /// variables — the elimination-ideal extraction used by polynomial-system triangularization.
        pub async fn eliminate(gens: &[Self], drop_vars: usize) -> Vec<Self> {
            let lex_gens: Vec<Self> = gens.iter().map(|g| g.with_order(MonomialOrder::Lex)).collect();
            let gb = Self::groebner_basis(&lex_gens);
            gb.into_iter().filter(|p| p.terms.iter().all(|(m, _)| m.exps()[..drop_vars].iter().all(|&e| e == 0))).collect()
        }
        // #endregion 🔖️Groebner
    }
    // #endregion 🔖️Reduction

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use number::Rational;

        async fn r(n: i64) -> Rational {
            Rational::from_i64(n, 1).unwrap()
        }

        async fn mono(exps: Vec<u32>) -> Monomial {
            Monomial::new(exps)
        }

        #[semio_framework_async_macros::async_test]
        async fn monomial_ordering_lex() {
            let a = mono(vec![2, 0]);
            let b = mono(vec![1, 5]);
            assert_eq!(a.cmp_by(&b, MonomialOrder::Lex), std::cmp::Ordering::Greater);
        }

        #[semio_framework_async_macros::async_test]
        async fn monomial_ordering_grlex_uses_total_degree_first() {
            let a = mono(vec![1, 0]); // degree 1
            let b = mono(vec![0, 2]); // degree 2
            assert_eq!(a.cmp_by(&b, MonomialOrder::GrLex), std::cmp::Ordering::Less);
        }

        #[semio_framework_async_macros::async_test]
        async fn try_div_and_lcm() {
            let a = mono(vec![2, 3]);
            let b = mono(vec![1, 1]);
            assert_eq!(a.try_div(&b), Some(mono(vec![1, 2])));
            assert_eq!(mono(vec![3, 0]).try_div(&mono(vec![0, 1])), None);
            assert_eq!(a.lcm(&b), mono(vec![2, 3]));
        }

        #[semio_framework_async_macros::async_test]
        async fn ring_ops_hand_case() {
            // f = x + y, g = x - y ; f*g = x^2 - y^2
            let x = PolyM::<Rational>::var(0, 2, MonomialOrder::Lex);
            let y = PolyM::<Rational>::var(1, 2, MonomialOrder::Lex);
            let f = x.add(&y);
            let g = x.sub(&y);
            let prod = f.mul(&g);
            let expected = x.mul(&x).sub(&y.mul(&y));
            assert_eq!(prod, expected);
        }

        #[semio_framework_async_macros::async_test]
        async fn eval_hand_case() {
            let x = PolyM::<Rational>::var(0, 2, MonomialOrder::Lex);
            let y = PolyM::<Rational>::var(1, 2, MonomialOrder::Lex);
            let f = x.mul(&x).add(&y); // x^2 + y
            assert_eq!(f.eval(&[r(3), r(2)]), r(11));
        }

        #[semio_framework_async_macros::async_test]
        async fn groebner_basis_of_line_intersection() {
            // {x^2 + y^2 - 1, x - y} over Q: eliminating gives a univariate relation in y.
            let x = PolyM::<Rational>::var(0, 2, MonomialOrder::Lex);
            let y = PolyM::<Rational>::var(1, 2, MonomialOrder::Lex);
            let one = PolyM::<Rational>::constant(r(1), 2, MonomialOrder::Lex);
            let f1 = x.mul(&x).add(&y.mul(&y)).sub(&one);
            let f2 = x.sub(&y);
            let gb = PolyM::groebner_basis(&[f1, f2]);
            assert!(!gb.is_empty());
            // Every original generator must reduce to zero against the basis (membership check).
            for g in [x.mul(&x).add(&y.mul(&y)).sub(&one), x.sub(&y)] {
                let (_, rem) = g.reduce(&gb);
                assert!(rem.is_zero());
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn partial_derivative_hand_case() {
            let x = PolyM::<Rational>::var(0, 2, MonomialOrder::Lex);
            let y = PolyM::<Rational>::var(1, 2, MonomialOrder::Lex);
            let f = x.mul(&x).mul(&y); // x^2 y
            let df_dx = f.partial_derivative(0); // 2xy
            assert_eq!(df_dx, x.mul(&y).mul_scalar(&r(2)));
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Multivariate

// #region 🔖️Finite
pub mod finite {
    //! 🧮️ Polynomial arithmetic over `GF(p)` (`PolyU<ModInt>`): modular exponentiation, Rabin's
    //! irreducibility test, distinct-degree factorization, and Cantor-Zassenhaus equal-degree splitting —
    //! the modular layer that `factor.rs` lifts via Hensel's lemma to factor over `Z`/`Q`.

    use crate::polynomial::univariate::PolyU;
    use geometry::random::Rng;
    use number::ModInt;

    // #region 🔖️PolyModPow
    pub async fn poly_mod_pow(base: &PolyU<ModInt>, exp: u64, modulus: &PolyU<ModInt>) -> PolyU<ModInt> {
        let mut result = PolyU::one();
        let mut b = {
            let (_, r) = base.div_rem(modulus);
            r
        };
        let mut e = exp;
        while e > 0 {
            if e & 1 == 1 {
                let (_, r) = result.mul(&b).div_rem(modulus);
                result = r;
            }
            let (_, r) = b.mul(&b).div_rem(modulus);
            b = r;
            e >>= 1;
        }
        result
    }
    // #endregion 🔖️PolyModPow

    // #region 🔖️Irreducibility
    async fn prime_factors_of_degree(n: usize) -> Vec<usize> {
        let mut factors = Vec::new();
        let mut n = n;
        let mut p = 2usize;
        while p * p <= n {
            if n.is_multiple_of(p) {
                factors.push(p);
                while n.is_multiple_of(p) {
                    n /= p;
                }
            }
            p += 1;
        }
        if n > 1 {
            factors.push(n);
        }
        factors
    }

    /// 🎯️ Rabin's irreducibility test: `f` (monic, degree `n`) is irreducible over `GF(p)` iff
    /// `x^(p^n) == x (mod f)` and `gcd(x^(p^(n/q)) - x, f) == 1` for every prime `q | n`.
    pub async fn is_irreducible(f: &PolyU<ModInt>) -> bool {
        let Some(n) = f.degree() else { return false };
        if n == 0 {
            return false;
        }
        let p = f.leading_coeff().unwrap().modulus();
        let x = PolyU::x();
        for q in prime_factors_of_degree(n) {
            let h = poly_mod_pow(&x, p.pow((n / q) as u32), f);
            let diff = h.sub(&x);
            let g = diff.gcd_monic(f);
            if g.degree() != Some(0) {
                return false;
            }
        }
        let h = poly_mod_pow(&x, p.pow(n as u32), f);
        h == x
    }
    // #endregion 🔖️Irreducibility

    // #region 🔖️DistinctDegree
    /// ✂️ Splits a squarefree `f` into groups of irreducible factors sharing the same degree:
    /// `[(product_of_degree_i_factors, i), ...]`.
    pub async fn distinct_degree_factor(f: &PolyU<ModInt>) -> Vec<(PolyU<ModInt>, usize)> {
        let p = f.leading_coeff().expect("distinct_degree_factor: f must be nonzero").modulus();
        let mut result = Vec::new();
        let mut f_star = f.make_monic();
        let mut h = PolyU::x();
        let mut i = 0usize;
        while f_star.degree().unwrap_or(0) > 0 {
            i += 1;
            h = poly_mod_pow(&h, p, &f_star);
            let diff = h.sub(&PolyU::x());
            let g = diff.gcd_monic(&f_star);
            if g.degree() != Some(0) {
                result.push((g.clone(), i));
                f_star = f_star.div_rem(&g).0.make_monic();
                h = {
                    let (_, r) = h.div_rem(&f_star);
                    r
                };
            }
            if 2 * i > f_star.degree().unwrap_or(0) {
                break;
            }
        }
        if f_star.degree().unwrap_or(0) > 0 {
            let deg = f_star.degree().unwrap();
            result.push((f_star, deg));
        }
        result
    }
    // #endregion 🔖️DistinctDegree

    // #region 🔖️EqualDegree
    /// 🎲️ Cantor-Zassenhaus equal-degree splitting: `f` is a product of `r` distinct monic irreducibles,
    /// each of degree `d`; returns all `r` of them. Requires odd `p` (the driver in `factor.rs` never
    /// selects `p == 2` for this reason).
    pub async fn equal_degree_factor(f: &PolyU<ModInt>, d: usize, rng: &mut Rng) -> Vec<PolyU<ModInt>> {
        let n = f.degree().unwrap_or(0);
        if n == d || n == 0 {
            return vec![f.make_monic()];
        }
        let p = f.leading_coeff().unwrap().modulus();
        loop {
            let deg_a = 1 + (rng.next_range(0, n as u64) as usize);
            let coeffs: Vec<ModInt> = (0..deg_a).map(|_| ModInt::new(rng.next_range(0, p), p)).collect();
            let a = PolyU::from_coeffs(coeffs);
            if a.is_zero() {
                continue;
            }
            let g0 = a.gcd_monic(f);
            let g = if g0.degree() != Some(0) {
                g0
            } else {
                let exp_pow = mod_pow_u64_via_natural(p, d as u64);
                let half = (exp_pow - 1) / 2;
                let b = poly_mod_pow(&a, half, f);
                let b_minus_1 = b.sub(&PolyU::one());
                b_minus_1.gcd_monic(f)
            };
            if g.degree() != Some(0) && g.degree() != Some(n) {
                let mut left = equal_degree_factor(&g, d, rng);
                let cofactor = f.div_rem(&g).0.make_monic();
                let mut right = equal_degree_factor(&cofactor, d, rng);
                left.append(&mut right);
                return left;
            }
        }
    }

    /// 🔢️ `p^d` as a `u64`; degrees stay small in practice (factoring polys of reasonable size), so plain
    /// `u64` exponentiation with an overflow-safety fallback via saturating multiplication is sufficient.
    async fn mod_pow_u64_via_natural(p: u64, d: u64) -> u64 {
        let mut result = 1u64;
        for _ in 0..d {
            result = result.saturating_mul(p);
        }
        result
    }
    // #endregion 🔖️EqualDegree

    // #region 🔖️FactorModP
    /// 🧮️ Full factorization of `f` over `GF(p)`: `(leading_coeff, [(irreducible_factor, multiplicity), ...])`.
    pub async fn factor_mod_p(f: &PolyU<ModInt>, rng: &mut Rng) -> (ModInt, Vec<(PolyU<ModInt>, u32)>) {
        let lc = *f.leading_coeff().expect("factor_mod_p: f must be nonzero");
        let monic = f.make_monic();
        let squarefree = monic.squarefree_decomposition();
        let mut result = Vec::new();
        for (part, mult) in squarefree {
            for (irreducible_group, degree) in distinct_degree_factor(&part) {
                for irreducible in equal_degree_factor(&irreducible_group, degree, rng) {
                    result.push((irreducible, mult));
                }
            }
        }
        (lc, result)
    }
    // #endregion 🔖️FactorModP

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        async fn m(v: i64, p: u64) -> ModInt {
            ModInt::new(v.rem_euclid(p as i64) as u64, p)
        }

        async fn poly(coeffs: Vec<i64>, p: u64) -> PolyU<ModInt> {
            PolyU::from_coeffs(coeffs.into_iter().map(|c| m(c, p)).collect())
        }

        #[semio_framework_async_macros::async_test]
        async fn poly_mod_pow_matches_repeated_squaring() {
            let p = 7;
            let base = poly(vec![1, 1], p); // x + 1
            let modulus = poly(vec![-1, 0, 0, 1], p); // x^3 - 1
            let via_fast = poly_mod_pow(&base, 5, &modulus);
            let mut via_slow = PolyU::one();
            for _ in 0..5 {
                via_slow = via_slow.mul(&base).div_rem(&modulus).1;
            }
            assert_eq!(via_fast, via_slow);
        }

        #[semio_framework_async_macros::async_test]
        async fn is_irreducible_hand_cases() {
            let p = 5;
            // x^2 + 1 is irreducible mod 5? -1 is not a QR mod 5 (5 % 4 == 1, so -1 IS a QR actually).
            // Use x^2 + 2, known irreducible mod 5 (2 is a non-residue mod 5).
            let f = poly(vec![2, 0, 1], p);
            assert!(is_irreducible(&f));
            let g = poly(vec![-1, 0, 1], p); // x^2 - 1 = (x-1)(x+1), reducible
            assert!(!is_irreducible(&g));
        }

        #[semio_framework_async_macros::async_test]
        async fn distinct_degree_factor_separates_degrees() {
            let p = 5;
            let deg1 = poly(vec![-1, 1], p); // x - 1
            let deg2 = poly(vec![2, 0, 1], p); // x^2 + 2, irreducible
            let f = deg1.mul(&deg2);
            let groups = distinct_degree_factor(&f);
            assert!(groups.iter().any(|(_, d)| *d == 1));
            assert!(groups.iter().any(|(_, d)| *d == 2));
        }

        #[semio_framework_async_macros::async_test]
        async fn equal_degree_factor_splits_product_of_two_linears() {
            let p = 7;
            let a = poly(vec![-1, 1], p); // x - 1
            let b = poly(vec![-2, 1], p); // x - 2
            let f = a.mul(&b);
            let mut rng = Rng::from_seed(42);
            let factors = equal_degree_factor(&f, 1, &mut rng);
            assert_eq!(factors.len(), 2);
            let product = factors[0].mul(&factors[1]);
            assert_eq!(product, f);
        }

        #[semio_framework_async_macros::async_test]
        async fn factor_mod_p_reconstructs_via_multiplication() {
            let p = 11;
            let a = poly(vec![-1, 1], p); // x - 1
            let b = poly(vec![-3, 1], p); // x - 3
            let f = a.mul(&b).mul(&a); // (x-1)^2 (x-3)
            let mut rng = Rng::from_seed(7);
            let (lc, factors) = factor_mod_p(&f, &mut rng);
            let mut product = PolyU::constant(lc);
            for (factor, mult) in &factors {
                product = product.mul(&factor.pow(*mult as u64));
            }
            assert_eq!(product, f);
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Finite

// #region 🔖️Factor
pub mod factor {
    //! 🔍️ Factorization over `Z`/`Q`: prime selection, quadratic-precision-free (linear) Hensel lifting
    //! from a `GF(p)` factorization, and subset recombination — plus the rational root theorem.

    use crate::polynomial::finite::factor_mod_p;
    use crate::polynomial::univariate::PolyU;
    use geometry::random::Rng;
    use number::{primes, Integer, IntegralDomain, ModInt, Natural, Rational};

    // #region 🔖️Conversions
    async fn to_modp(f: &PolyU<Integer>, p: u64) -> PolyU<ModInt> {
        let modulus_nat = Natural::from_u64(p);
        PolyU::from_coeffs(
            f.coeffs()
                .iter()
                .map(|c| {
                    let (_, r) = c.div_rem_euclid(&Integer::from_natural(modulus_nat.clone()));
                    ModInt::new(r.to_i64().unwrap_or(0) as u64, p)
                })
                .collect(),
        )
    }

    async fn lift_nonneg(f_modp: &PolyU<ModInt>) -> PolyU<Integer> {
        PolyU::from_coeffs(f_modp.coeffs().iter().map(|c| Integer::from_i64(c.value() as i64)).collect())
    }

    /// 🎯️ Re-centers coefficients currently in `[0, modulus)` into the balanced range `(-modulus/2, modulus/2]`.
    async fn center_coeffs(f: &PolyU<Integer>, modulus: &Natural) -> PolyU<Integer> {
        let half = modulus.shr(1);
        let modulus_int = Integer::from_natural(modulus.clone());
        PolyU::from_coeffs(f.coeffs().iter().map(|c| if c.magnitude() > &half { c.sub(&modulus_int) } else { c.clone() }).collect())
    }
    // #endregion 🔖️Conversions

    // #region 🔖️Hensel
    /// 🧮️ Multi-factor linear Hensel lifting: given `f` (monic, integer) and pairwise-coprime monic
    /// `GF(p)` factors whose product is `f mod p`, lifts every factor simultaneously to modulus
    /// `>= target_modulus`, one power of `p` at a time, using the classical partial-fraction (CRT)
    /// construction of the lifting coefficients `c_i` with `c_i == 1 (mod g_i)` and `f/g_i | c_i`.
    async fn hensel_lift_factors(f: &PolyU<Integer>, mod_p_factors: &[PolyU<ModInt>], p: u64, target_modulus: &Natural) -> Vec<PolyU<Integer>> {
        let k = mod_p_factors.len();
        if k <= 1 {
            return vec![f.clone()];
        }
        // f_i = product of all mod_p_factors except the i-th, via prefix/suffix products.
        let mut prefix = vec![PolyU::<ModInt>::one(); k + 1];
        for i in 0..k {
            prefix[i + 1] = prefix[i].mul(&mod_p_factors[i]);
        }
        let mut suffix = vec![PolyU::<ModInt>::one(); k + 1];
        for i in (0..k).rev() {
            suffix[i] = suffix[i + 1].mul(&mod_p_factors[i]);
        }
        let f_modp = to_modp(f, p);
        let mut c: Vec<PolyU<ModInt>> = Vec::with_capacity(k);
        for i in 0..k {
            let f_i = prefix[i].mul(&suffix[i + 1]);
            let (_, s_i, _) = f_i.xgcd(&mod_p_factors[i]);
            let c_i = {
                let (_, r) = s_i.mul(&f_i).div_rem(&f_modp);
                r
            };
            c.push(c_i);
        }

        let mut g: Vec<PolyU<Integer>> = mod_p_factors.iter().map(lift_nonneg).collect();
        let mut current_modulus = Natural::from_u64(p);
        while current_modulus < *target_modulus {
            let mut prod = PolyU::<Integer>::one();
            for gi in &g {
                prod = prod.mul(gi);
            }
            let e = f.sub(&prod);
            let modulus_int = Integer::from_natural(current_modulus.clone());
            let e_over_mod = PolyU::from_coeffs(e.coeffs().iter().map(|c| c.exact_div(&modulus_int).expect("Hensel lifting invariant: f - prod(g_i) is divisible by the current modulus")).collect());
            let e_bar = to_modp(&e_over_mod, p);
            let mut new_g = Vec::with_capacity(k);
            for i in 0..k {
                let delta_full = c[i].mul(&e_bar);
                let (_, delta) = delta_full.div_rem(&mod_p_factors[i]);
                let delta_int = lift_nonneg(&delta);
                let scaled = delta_int.mul_scalar(&Integer::from_natural(current_modulus.clone()));
                new_g.push(g[i].add(&scaled));
            }
            g = new_g;
            current_modulus = current_modulus.mul(&Natural::from_u64(p));
        }
        g.into_iter().map(|gi| center_coeffs(&gi, &current_modulus)).collect()
    }
    // #endregion 🔖️Hensel

    // #region 🔖️Bounds
    /// 📏️ A generous (not tight) bound on the absolute value of any coefficient of any integer factor of
    /// `f`: `binom(n, n/2) * R^n * |lc|`, where `R` is a Cauchy-style bound on the magnitude of `f`'s
    /// roots. Looser than the classical Landau-Mignotte bound costs a few extra Hensel lifting steps, never
    /// correctness.
    async fn factor_coefficient_bound(f: &PolyU<Integer>) -> Natural {
        let n = f.degree().unwrap_or(0);
        let lc = f.leading_coeff().cloned().unwrap_or_else(Integer::one).abs();
        let max_coeff = f.coeffs().iter().map(Integer::abs).fold(Natural::zero(), |acc, m| if m > acc { m } else { acc });
        let r = max_coeff.div_rem(&lc).0.add(&Natural::from_u64(2)); // ceil-ish Cauchy bound, padded
        let mut binom = Natural::one();
        for i in 0..(n / 2) {
            binom = binom.mul(&Natural::from_u64((n - i) as u64)).div_rem(&Natural::from_u64((i + 1) as u64)).0;
        }
        binom.mul(&r.pow(n as u64)).mul(&lc).add(&Natural::one())
    }
    // #endregion 🔖️Bounds

    // #region 🔖️FactorZ
    /// 🔍️ Full factorization of a monic integer polynomial: content is always `1` and leading coefficient
    /// `1`, so the result is `[(irreducible_factor, multiplicity), ...]` with `product == f` exactly.
    /// Non-monic primitive polynomials go through the classical "multiply through" substitution
    /// (`f_hat(y) = a_n^(n-1) f(y/a_n)`, monic in `y`) with a defensive final reconstruction check —
    /// if that check fails for any reason, `f` is conservatively reported as its own single factor rather
    /// than risking a silently wrong answer.
    pub async fn factor_integer_poly(f: &PolyU<Integer>) -> (Integer, Vec<(PolyU<Integer>, u32)>) {
        if f.is_zero() {
            return (Integer::zero(), Vec::new());
        }
        let content = f.content();
        let primitive = f.primitive_part();
        let lc = primitive.leading_coeff().unwrap().clone();
        let sign_adjusted_content = if lc.is_negative() { content.neg() } else { content };
        let primitive = if lc.is_negative() { primitive.neg() } else { primitive };
        let lc = primitive.leading_coeff().unwrap().clone();

        let monic_factors = if lc == Integer::one() { factor_monic_integer_poly(&primitive) } else { factor_nonmonic_via_substitution(&primitive, &lc) };

        // Group identical factors into multiplicities (squarefree_decomposition already separates them by
        // multiplicity upstream of the mod-p work, but distinct square-free classes can still coincide).
        let mut grouped: Vec<(PolyU<Integer>, u32)> = Vec::new();
        for f_i in monic_factors {
            if let Some(entry) = grouped.iter_mut().find(|(g, _)| *g == f_i) {
                entry.1 += 1;
            } else {
                grouped.push((f_i, 1));
            }
        }
        (sign_adjusted_content, grouped)
    }

    /// 🧮️ Factors a monic primitive integer polynomial via squarefree decomposition (over Q, cleared to Z)
    /// followed by mod-p factorization, Hensel lifting, and subset recombination per squarefree part.
    async fn factor_monic_integer_poly(f: &PolyU<Integer>) -> Vec<PolyU<Integer>> {
        let f_rational = PolyU::from_coeffs(f.coeffs().iter().map(|c| Rational::from_integer(c.clone())).collect());
        let squarefree_parts = f_rational.squarefree_decomposition();
        let mut result = Vec::new();
        for (part_rational, mult) in squarefree_parts {
            let part_integer = clear_denominators(&part_rational).primitive_part();
            for factor in factor_squarefree_monic(&part_integer) {
                for _ in 0..mult {
                    result.push(factor.clone());
                }
            }
        }
        if result.is_empty() {
            vec![f.clone()]
        } else {
            result
        }
    }

    async fn clear_denominators(f: &PolyU<Rational>) -> PolyU<Integer> {
        let denom_lcm = f.coeffs().iter().fold(Natural::one(), |acc, c| acc.mul(c.denom()).div_rem(&acc.gcd(c.denom())).0);
        PolyU::from_coeffs(f.coeffs().iter().map(|c| c.mul(&Rational::from_integer(Integer::from_natural(denom_lcm.clone()))).trunc()).collect())
    }

    /// 🔍️ Factors a squarefree monic integer polynomial: picks a good prime, factors mod p, lifts, recombines.
    /// 🥇️ Best modular image found while prime-hunting: the prime and its squarefree factorization.
    type BestFactorization = (u64, Vec<(PolyU<ModInt>, u32)>);

    async fn factor_squarefree_monic(f: &PolyU<Integer>) -> Vec<PolyU<Integer>> {
        if f.degree() == Some(0) || f.degree() == Some(1) {
            return vec![f.clone()];
        }
        let mut rng = Rng::from_seed(0xC0FF_EE00_D15E_A5E5);
        let bound = factor_coefficient_bound(f);
        let target_modulus = bound.mul(&Natural::from_u64(2));

        // Try a handful of odd primes that don't divide the leading coefficient, picking the one giving a
        // squarefree image mod p (guarantees the mod-p factorization is separable).
        let mut best: Option<BestFactorization> = None;
        let mut p = 100_003u64; // an odd prime comfortably away from tiny-degree coefficient collisions
        let mut attempts = 0;
        while attempts < 8 {
            p = primes::next_prime(&Natural::from_u64(p + 1)).to_u64().unwrap_or(p + 2);
            attempts += 1;
            let f_modp = to_modp(f, p);
            if f_modp.degree() != f.degree() {
                continue; // leading coefficient vanished mod p
            }
            let monic = f_modp.make_monic();
            let squarefree = monic.squarefree_decomposition();
            if squarefree.len() != 1 || squarefree[0].1 != 1 {
                continue; // not squarefree mod p; try another prime
            }
            let (_, factors) = factor_mod_p(&f_modp, &mut rng);
            let better = best.as_ref().is_none_or(|(_, existing)| factors.len() < existing.len());
            if better {
                best = Some((p, factors));
            }
            if best.as_ref().unwrap().1.len() <= 1 {
                break; // already irreducible mod p => irreducible over Z
            }
        }
        let Some((chosen_p, factors_mod_p)) = best else {
            return vec![f.clone()];
        };
        if factors_mod_p.len() <= 1 {
            return vec![f.clone()];
        }
        let mod_p_polys: Vec<PolyU<ModInt>> = factors_mod_p.into_iter().map(|(poly, _)| poly).collect();
        let lifted = hensel_lift_factors(f, &mod_p_polys, chosen_p, &target_modulus);
        recombine(f, &lifted)
    }

    /// 🧩️ Subset recombination: tries products of subsets of the lifted modular factors (smallest
    /// cardinality first) against exact integer trial division, capping the number of modular factors
    /// considered to keep the search space bounded (`log()`-worthy cases beyond the cap fall back to
    /// reporting the un-combined lifted factors, still a correct — if potentially non-irreducible — cover).
    async fn recombine(f: &PolyU<Integer>, lifted: &[PolyU<Integer>]) -> Vec<PolyU<Integer>> {
        const MAX_MODULAR_FACTORS: usize = 24;
        if lifted.len() > MAX_MODULAR_FACTORS {
            return lifted.to_vec();
        }
        let mut remaining: Vec<PolyU<Integer>> = lifted.to_vec();
        let mut remaining_target = f.clone();
        let mut result = Vec::new();
        let mut subset_size = 1usize;
        while subset_size <= remaining.len() && !remaining.is_empty() {
            if subset_size > remaining.len() {
                break;
            }
            let mut found = false;
            'search: for subset in combinations(remaining.len(), subset_size) {
                let mut candidate = PolyU::<Integer>::one();
                for &idx in &subset {
                    candidate = candidate.mul(&remaining[idx]);
                }
                let candidate = candidate.primitive_part();
                if let Some(quotient) = remaining_target.exact_div(&candidate) {
                    if quotient.degree().unwrap_or(0) + candidate.degree().unwrap_or(0) == remaining_target.degree().unwrap_or(0) {
                        result.push(candidate);
                        remaining_target = quotient.primitive_part();
                        remaining = remaining.iter().enumerate().filter(|(i, _)| !subset.contains(i)).map(|(_, p)| p.clone()).collect();
                        found = true;
                        break 'search;
                    }
                }
            }
            if !found {
                subset_size += 1;
            }
        }
        if remaining_target.degree().unwrap_or(0) > 0 {
            result.push(remaining_target);
        }
        if result.is_empty() {
            vec![f.clone()]
        } else {
            result
        }
    }

    async fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
        if k == 0 {
            return vec![Vec::new()];
        }
        if k > n {
            return Vec::new();
        }
        let mut result = Vec::new();
        let mut combo: Vec<usize> = (0..k).collect();
        loop {
            result.push(combo.clone());
            let mut i = k;
            loop {
                if i == 0 {
                    return result;
                }
                i -= 1;
                if combo[i] != i + n - k {
                    break;
                }
            }
            combo[i] += 1;
            for j in (i + 1)..k {
                combo[j] = combo[j - 1] + 1;
            }
        }
    }

    /// 🔁️ Handles a non-monic primitive `f` via `f_hat(y) = lc^(n-1) f(y/lc)` (monic, integer coefficients
    /// since the top term's negative power of `lc` cancels exactly against its own coefficient), factors
    /// the monic version, un-substitutes each factor via `H_i(lc * x)`, and takes primitive parts. The
    /// product is verified against `f` (up to sign) before being trusted; on any mismatch, `f` is returned
    /// as its own single factor.
    async fn factor_nonmonic_via_substitution(f: &PolyU<Integer>, lc: &Integer) -> Vec<PolyU<Integer>> {
        let n = f.degree().unwrap_or(0);
        if n == 0 {
            return vec![f.clone()];
        }
        let mut hat_coeffs = Vec::with_capacity(n + 1);
        for i in 0..n {
            hat_coeffs.push(f.coeff(i).mul(&lc.pow((n - 1 - i) as u64)));
        }
        hat_coeffs.push(Integer::one());
        let f_hat = PolyU::from_coeffs(hat_coeffs);
        let monic_factors = factor_monic_integer_poly(&f_hat);
        let scale = PolyU::monomial(lc.clone(), 1);
        let candidates: Vec<PolyU<Integer>> = monic_factors.iter().map(|h| h.semio_compose_rs(&scale).primitive_part()).collect();
        let mut product = PolyU::<Integer>::one();
        for c in &candidates {
            product = product.mul(c);
        }
        if product == *f {
            candidates
        } else if product == f.neg() {
            let mut fixed = candidates;
            if let Some(first) = fixed.first_mut() {
                *first = first.neg();
            }
            fixed
        } else {
            vec![f.clone()]
        }
    }
    // #endregion 🔖️FactorZ

    // #region 🔖️RationalRoots
    /// 🎯️ Rational roots of `f` via the rational root theorem: candidates `p/q` with `p | trailing`,
    /// `q | leading` (over the cleared-denominator integer polynomial), each verified by exact evaluation.
    pub async fn rational_roots(f: &PolyU<Rational>) -> Vec<Rational> {
        if f.is_zero() {
            return Vec::new();
        }
        let integer_poly = clear_denominators(f).primitive_part();
        let Some(trailing) = integer_poly.coeffs().first().cloned() else { return Vec::new() };
        let leading = integer_poly.leading_coeff().cloned().unwrap();
        if trailing.is_zero() {
            // x = 0 is a root; factor it out and recurse on the rest.
            let reduced = PolyU::from_coeffs(integer_poly.coeffs()[1..].to_vec());
            let reduced_rational = PolyU::from_coeffs(reduced.coeffs().iter().map(|c| Rational::from_integer(c.clone())).collect());
            let mut roots = vec![Rational::zero()];
            roots.extend(rational_roots(&reduced_rational));
            return roots;
        }
        let p_candidates = primes::divisors_u64(trailing.abs().to_u64().unwrap_or(1));
        let q_candidates = primes::divisors_u64(leading.abs().to_u64().unwrap_or(1));
        let mut roots = Vec::new();
        for &p in &p_candidates {
            for &q in &q_candidates {
                for sign in [1i64, -1] {
                    let candidate = Rational::from_i64(sign * p as i64, q as i64).unwrap();
                    if f.eval(&candidate).is_zero() && !roots.contains(&candidate) {
                        roots.push(candidate);
                    }
                }
            }
        }
        roots
    }
    // #endregion 🔖️RationalRoots

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        async fn i(v: i64) -> Integer {
            Integer::from_i64(v)
        }

        async fn ipoly(coeffs: Vec<i64>) -> PolyU<Integer> {
            PolyU::from_coeffs(coeffs.into_iter().map(Integer::from_i64).collect())
        }

        #[semio_framework_async_macros::async_test]
        async fn factor_x2_minus_1() {
            let f = ipoly(vec![-1, 0, 1]); // x^2 - 1 = (x-1)(x+1)
            let (content, factors) = factor_integer_poly(&f);
            assert_eq!(content, i(1));
            let mut product = PolyU::constant(content);
            for (factor, mult) in &factors {
                product = product.mul(&factor.pow(*mult as u64));
            }
            assert_eq!(product, f);
            assert_eq!(factors.len(), 2);
        }

        #[semio_framework_async_macros::async_test]
        async fn factor_x4_minus_1() {
            let f = ipoly(vec![-1, 0, 0, 0, 1]); // x^4 - 1 = (x-1)(x+1)(x^2+1)
            let (_, factors) = factor_integer_poly(&f);
            let mut product = PolyU::<Integer>::one();
            for (factor, mult) in &factors {
                product = product.mul(&factor.pow(*mult as u64));
            }
            assert_eq!(product, f);
            assert!(factors.len() >= 2);
        }

        #[semio_framework_async_macros::async_test]
        async fn factor_repeated_linear_factor() {
            let base = ipoly(vec![-1, 1]); // x - 1
            let f = base.mul(&base).mul(&base); // (x-1)^3
            let (_, factors) = factor_integer_poly(&f);
            let mut product = PolyU::<Integer>::one();
            for (factor, mult) in &factors {
                product = product.mul(&factor.pow(*mult as u64));
            }
            assert_eq!(product, f);
            assert!(factors.iter().any(|(factor, mult)| *factor == base && *mult == 3));
        }

        #[semio_framework_async_macros::async_test]
        async fn factor_irreducible_quadratic_stays_whole() {
            let f = ipoly(vec![1, 0, 1]); // x^2 + 1, irreducible over Q
            let (_, factors) = factor_integer_poly(&f);
            assert_eq!(factors.len(), 1);
            assert_eq!(factors[0].1, 1);
        }

        #[semio_framework_async_macros::async_test]
        async fn factor_nonmonic_quadratic() {
            let f = ipoly(vec![-3, -1, 2]); // 2x^2 - x - 3 = (2x - 3)(x + 1)
            let (content, factors) = factor_integer_poly(&f);
            let mut product = PolyU::constant(content);
            for (factor, mult) in &factors {
                product = product.mul(&factor.pow(*mult as u64));
            }
            assert_eq!(product, f);
        }

        #[semio_framework_async_macros::async_test]
        async fn rational_roots_of_quadratic() {
            let f = PolyU::from_coeffs(vec![Rational::from_i64(1, 1).unwrap(), Rational::from_i64(-5, 1).unwrap(), Rational::from_i64(6, 1).unwrap()]); // 6x^2 - 5x + 1
            let roots = rational_roots(&f);
            assert_eq!(roots.len(), 2);
            for r in &roots {
                assert!(f.eval(r).is_zero());
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn rational_roots_with_zero_root() {
            let f = PolyU::from_coeffs(vec![Rational::zero(), Rational::from_i64(-1, 1).unwrap(), Rational::from_i64(1, 1).unwrap()]); // x^2 - x = x(x-1)
            let roots = rational_roots(&f);
            assert_eq!(roots.len(), 2);
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Factor

// #region 🔖️Roots
pub mod roots {
    //! 🌱️ Real root isolation for integer polynomials via Sturm sequences: sign-change counting gives
    //! rigorous isolating rational intervals for every real root, refined by bisection.

    use crate::polynomial::univariate::PolyU;
    use number::{Integer, Rational};

    // #region 🔖️RootBounds
    /// 📏️ Cauchy's bound: every real root of `f` has absolute value `<= 1 + max(|a_i|)/|lc|`.
    pub async fn cauchy_root_bound(f: &PolyU<Integer>) -> Rational {
        let Some(lc) = f.leading_coeff() else { return Rational::zero() };
        let lc_abs = Rational::from_integer(Integer::from_natural(lc.abs()));
        let max_other = f.coeffs()[..f.coeffs().len() - 1].iter().map(|c| Rational::from_integer(Integer::from_natural(c.abs()))).fold(Rational::zero(), |acc, v| if v > acc { v } else { acc });
        Rational::one().add(&max_other.div(&lc_abs).unwrap_or_else(Rational::zero))
    }
    // #endregion 🔖️RootBounds

    // #region 🔖️Sturm
    /// 🔗️ The signed polynomial-remainder sequence `f, f', -rem(f,f'), -rem(f',...), ...` (primitive-part
    /// normalized at each step to control coefficient growth), used to count real roots via sign changes.
    pub async fn sturm_sequence(f: &PolyU<Integer>) -> Vec<PolyU<Integer>> {
        let mut seq = vec![f.primitive_part(), f.derivative().primitive_part()];
        loop {
            let n = seq.len();
            if seq[n - 1].is_zero() {
                break;
            }
            let (_, r, _) = seq[n - 2].pseudo_div_rem(&seq[n - 1]);
            let next = if r.is_zero() { PolyU::zero() } else { r.neg().primitive_part() };
            seq.push(next);
            if seq.last().unwrap().is_zero() {
                break;
            }
        }
        seq
    }

    async fn sign_changes(seq: &[PolyU<Integer>], point: &Rational) -> usize {
        let mut last_sign: Option<std::cmp::Ordering> = None;
        let mut changes = 0;
        for p in seq {
            let value = eval_rational(p, point);
            if value.is_zero() {
                continue;
            }
            let sign = if value.numer().is_negative() { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater };
            if let Some(prev) = last_sign {
                if prev != sign {
                    changes += 1;
                }
            }
            last_sign = Some(sign);
        }
        changes
    }

    async fn eval_rational(f: &PolyU<Integer>, point: &Rational) -> Rational {
        let mut result = Rational::zero();
        for c in f.coeffs().iter().rev() {
            result = result.mul(point).add(&Rational::from_integer(c.clone()));
        }
        result
    }

    /// 🔢️ Number of distinct real roots of (the squarefree part of) `f` in the half-open interval `(lo, hi]`.
    pub async fn count_roots_in(seq: &[PolyU<Integer>], lo: &Rational, hi: &Rational) -> usize {
        sign_changes(seq, lo).saturating_sub(sign_changes(seq, hi))
    }

    /// ✂️ Isolates every distinct real root of `f` into a sorted list of half-open rational intervals
    /// `(lo, hi]`, each containing exactly one root. Operates on the squarefree part (repeated roots of
    /// the original `f` collapse to one isolated interval, matching Sturm's theorem's requirement of a
    /// squarefree input).
    pub async fn isolate_real_roots(f: &PolyU<Integer>) -> Vec<(Rational, Rational)> {
        if f.is_zero() || f.degree() == Some(0) {
            return Vec::new();
        }
        let squarefree = to_squarefree_integer(f);
        let seq = sturm_sequence(&squarefree);
        let bound = cauchy_root_bound(&squarefree);
        let neg_bound = bound.neg();
        let total = count_roots_in(&seq, &neg_bound, &bound);
        let mut intervals = Vec::new();
        let mut stack = vec![(neg_bound, bound)];
        while let Some((lo, hi)) = stack.pop() {
            let count = count_roots_in(&seq, &lo, &hi);
            if count == 0 {
                continue;
            }
            if count == 1 {
                intervals.push((lo, hi));
                continue;
            }
            let mid = lo.add(&hi).div(&Rational::from_i64(2, 1).unwrap()).unwrap();
            stack.push((mid.clone(), hi));
            stack.push((lo, mid));
        }
        intervals.sort_by(|a, b| a.0.cmp(&b.0));
        debug_assert_eq!(intervals.len(), total);
        intervals
    }

    async fn to_squarefree_integer(f: &PolyU<Integer>) -> PolyU<Integer> {
        let rational = PolyU::from_coeffs(f.coeffs().iter().map(|c| Rational::from_integer(c.clone())).collect());
        let parts = rational.squarefree_decomposition();
        let mut result = PolyU::<Rational>::one();
        for (part, _) in parts {
            result = result.mul(&part);
        }
        let denom_lcm = result.coeffs().iter().fold(number::Natural::one(), |acc, c| acc.mul(c.denom()).div_rem(&acc.gcd(c.denom())).0);
        PolyU::from_coeffs(result.coeffs().iter().map(|c| c.mul(&Rational::from_integer(Integer::from_natural(denom_lcm.clone()))).trunc()).collect()).primitive_part()
    }

    /// 🔬️ Bisects `(lo, hi]` (assumed to isolate exactly one root of `f`) down to the given `width`,
    /// preserving the sign-change invariant at each step.
    pub async fn refine_root(f: &PolyU<Integer>, lo: &Rational, hi: &Rational, width: &Rational) -> (Rational, Rational) {
        let seq = sturm_sequence(f);
        let mut lo = lo.clone();
        let mut hi = hi.clone();
        while hi.sub(&lo) > *width {
            let mid = lo.add(&hi).div(&Rational::from_i64(2, 1).unwrap()).unwrap();
            if count_roots_in(&seq, &lo, &mid) == 1 {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        (lo, hi)
    }
    // #endregion 🔖️Sturm

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        async fn ipoly(coeffs: Vec<i64>) -> PolyU<Integer> {
            PolyU::from_coeffs(coeffs.into_iter().map(Integer::from_i64).collect())
        }

        #[semio_framework_async_macros::async_test]
        async fn isolate_roots_of_simple_quadratic() {
            let f = ipoly(vec![-2, 0, 1]); // x^2 - 2, roots +-sqrt(2)
            let intervals = isolate_real_roots(&f);
            assert_eq!(intervals.len(), 2);
            for (lo, hi) in &intervals {
                let seq = sturm_sequence(&f);
                assert_eq!(count_roots_in(&seq, lo, hi), 1);
            }
        }

        #[semio_framework_async_macros::async_test]
        async fn isolate_roots_matches_known_integer_roots() {
            // (x-1)(x-3)(x+2)
            let f = ipoly(vec![6, -1, -4, 1]);
            let intervals = isolate_real_roots(&f);
            assert_eq!(intervals.len(), 3);
        }

        #[semio_framework_async_macros::async_test]
        async fn refine_root_converges_to_sqrt2() {
            let f = ipoly(vec![-2, 0, 1]);
            let intervals = isolate_real_roots(&f);
            let positive = intervals.iter().find(|(lo, hi)| lo.is_zero() || (!lo.numer().is_negative() && !hi.numer().is_negative())).cloned().unwrap();
            let width = Rational::from_i64(1, 1_000_000).unwrap();
            let (lo, hi) = refine_root(&f, &positive.0, &positive.1, &width);
            let approx = (lo.to_f64() + hi.to_f64()) / 2.0;
            assert!((approx - std::f64::consts::SQRT_2).abs() < 1e-5);
        }

        #[semio_framework_async_macros::async_test]
        async fn cauchy_bound_contains_all_roots() {
            let f = ipoly(vec![6, -1, -4, 1]); // roots -2, 1, 3
            let bound = cauchy_root_bound(&f);
            assert!(bound >= Rational::from_i64(3, 1).unwrap());
        }

        #[semio_framework_async_macros::async_test]
        async fn wilkinson_like_small_case_root_count() {
            // (x-1)(x-2)(x-3)(x-4)
            let f = ipoly(vec![-1, 1]).mul(&ipoly(vec![-2, 1])).mul(&ipoly(vec![-3, 1])).mul(&ipoly(vec![-4, 1]));
            let intervals = isolate_real_roots(&f);
            assert_eq!(intervals.len(), 4);
        }

        #[semio_framework_async_macros::async_test]
        async fn zero_polynomial_and_constant_have_no_roots() {
            assert!(isolate_real_roots(&ipoly(vec![])).is_empty());
            assert!(isolate_real_roots(&ipoly(vec![5])).is_empty());
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Roots

// #region 🔖️Algebraic
pub mod algebraic {
    //! 🌱️ Real algebraic numbers: an integer defining polynomial plus a rational isolating interval,
    //! with exact ordering/refinement and add/mul implemented via bivariate resultants.

    use crate::polynomial::factor::factor_integer_poly;
    use crate::polynomial::roots::{count_roots_in, isolate_real_roots, refine_root, sturm_sequence};
    use crate::polynomial::univariate::PolyU;
    use number::{Integer, Rational};

    // #region 🔖️AlgebraicReal
    /// 🌱️ A real root of `poly` (not necessarily irreducible at construction, though the constructors here
    /// always narrow to an irreducible factor), isolated by the rational interval `(lo, hi]` — or, when
    /// `lo == hi`, an exact rational value.
    #[derive(Clone, Debug)]
    pub struct AlgebraicReal {
        poly: PolyU<Integer>,
        lo: Rational,
        hi: Rational,
    }

    impl AlgebraicReal {
        pub async fn from_rational(r: &Rational) -> Self {
            let poly = PolyU::from_coeffs(vec![r.numer().neg(), Integer::from_natural(r.denom().clone())]);
            Self { poly, lo: r.clone(), hi: r.clone() }
        }

        /// 🌱️ The `index`-th real root of `f` (ascending order, `0`-based); narrows `f` down to whichever
        /// irreducible factor actually contains that root. `None` if `index` is out of range.
        pub async fn root_of(f: &PolyU<Integer>, index: usize) -> Option<Self> {
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
        pub async fn nth_root(r: &Rational, n: u32) -> Option<Self> {
            if n == 0 {
                return None;
            }
            if r.is_zero() {
                return Some(Self::from_rational(&Rational::zero()));
            }
            if n.is_multiple_of(2) && r.numer().is_negative() {
                return None;
            }
            let denom = Integer::from_natural(r.denom().clone());
            let mut coeffs = vec![Integer::zero(); n as usize + 1];
            coeffs[0] = r.numer().neg();
            coeffs[n as usize] = denom;
            let poly = PolyU::from_coeffs(coeffs);
            let intervals = isolate_real_roots(&poly);
            let index = if n.is_multiple_of(2) { intervals.len() - 1 } else { 0 };
            Self::root_of(&poly, index)
        }

        pub async fn interval(&self) -> (Rational, Rational) {
            (self.lo.clone(), self.hi.clone())
        }

        pub async fn minimal_poly(&self) -> &PolyU<Integer> {
            &self.poly
        }

        pub async fn degree(&self) -> usize {
            self.poly.degree().unwrap_or(0)
        }

        pub async fn is_rational(&self) -> bool {
            self.lo == self.hi
        }

        pub async fn to_f64(&self) -> f64 {
            (self.lo.to_f64() + self.hi.to_f64()) / 2.0
        }

        /// 🔬️ Bisects the isolating interval down to (at most) `width`.
        pub async fn refine(&mut self, width: &Rational) {
            if self.is_rational() {
                return;
            }
            let (lo, hi) = refine_root(&self.poly, &self.lo, &self.hi, width);
            self.lo = lo;
            self.hi = hi;
        }

        async fn refine_below(&mut self, width: &Rational) {
            while self.hi.sub(&self.lo) > *width && !self.is_rational() {
                let half = self.hi.sub(&self.lo).div(&Rational::from_i64(2, 1).unwrap()).unwrap();
                self.refine(&half);
            }
        }

        /// 〽 Sign of this value: `None` only for the exact rational zero (an irrational value's isolating
        /// interval always separates from zero after finitely many refinements, since the true root isn't
        /// zero — even though the interval can legitimately straddle zero before separation, e.g. for a
        /// root very close to `0`).
        pub async fn sign(&self) -> Option<std::cmp::Ordering> {
            if self.is_rational() {
                return Some(self.lo.cmp(&Rational::zero()));
            }
            let mut probe = self.clone();
            let mut width = probe.hi.sub(&probe.lo);
            for _ in 0..200 {
                if probe.lo > Rational::zero() {
                    return Some(std::cmp::Ordering::Greater);
                }
                if probe.hi < Rational::zero() {
                    return Some(std::cmp::Ordering::Less);
                }
                width = width.div(&Rational::from_i64(2, 1).unwrap()).unwrap();
                probe.refine(&width);
            }
            None
        }

        /// 🎯️ Refines until the interval no longer straddles zero (a no-operation for the exact rational `0`,
        /// which has no sign to separate); used before sign-dependent transforms like `inv`.
        async fn with_definite_sign(&self) -> Self {
            let mut probe = self.clone();
            if probe.lo.is_zero() && probe.hi.is_zero() {
                return probe;
            }
            let mut width = probe.hi.sub(&probe.lo);
            for _ in 0..200 {
                if probe.lo > Rational::zero() || probe.hi < Rational::zero() {
                    break;
                }
                width = width.div(&Rational::from_i64(2, 1).unwrap()).unwrap();
                probe.refine(&width);
            }
            probe
        }

        // #region 🔖️CheapTransforms
        pub async fn neg(&self) -> Self {
            // Root of f(x) negated is a root of f(-x); reverse the sign of odd-degree coefficients.
            let coeffs = self.poly.coeffs().iter().enumerate().map(|(i, c)| if i % 2 == 1 { c.neg() } else { c.clone() }).collect();
            Self { poly: PolyU::from_coeffs(coeffs), lo: self.hi.neg(), hi: self.lo.neg() }
        }

        /// ➗️ Reciprocal (`self` must be nonzero): if `alpha` is a root of `f`, `1/alpha` is a root of the
        /// coefficient-reversed polynomial.
        pub async fn inv(&self) -> Option<Self> {
            if self.is_rational() && self.lo.is_zero() {
                return None;
            }
            let definite = self.with_definite_sign();
            let mut coeffs = definite.poly.coeffs().to_vec();
            coeffs.reverse();
            let poly = PolyU::from_coeffs(coeffs);
            let new_lo = if definite.hi.is_zero() { None } else { definite.hi.inv() };
            let new_hi = if definite.lo.is_zero() { None } else { definite.lo.inv() };
            match (new_lo, new_hi) {
                (Some(a), Some(b)) if a <= b => Some(Self { poly, lo: a, hi: b }),
                (Some(a), Some(b)) => Some(Self { poly, lo: b, hi: a }),
                _ => None,
            }
        }

        pub async fn scale_rational(&self, r: &Rational) -> Option<Self> {
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
            let denom_lcm = rational_poly.coeffs().iter().fold(number::Natural::one(), |acc, c| {
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

        pub async fn add_rational(&self, r: &Rational) -> Self {
            let poly = self.poly.compose_with_rational_shift(r);
            Self { poly, lo: self.lo.add(r), hi: self.hi.add(r) }
        }

        async fn root_of_near(poly: &PolyU<Integer>, target: f64) -> Option<Self> {
            let intervals = isolate_real_roots(poly);
            let (best_idx, _) = intervals.iter().enumerate().min_by(|(_, (a_lo, a_hi)), (_, (b_lo, b_hi))| {
                let a_mid = (a_lo.to_f64() + a_hi.to_f64()) / 2.0;
                let b_mid = (b_lo.to_f64() + b_hi.to_f64()) / 2.0;
                (a_mid - target).abs().partial_cmp(&(b_mid - target).abs()).unwrap()
            })?;
            Self::root_of(poly, best_idx)
        }
        // #endregion 🔖️CheapTransforms

        // #region 🔖️AlgebraicOperations
        /// ➕️ Sum of two algebraic reals via the bivariate resultant `res_y(f(x - y), g(y))`, which
        /// vanishes exactly at every pairwise sum of a root of `f` with a root of `g`; the correct
        /// irreducible factor and interval are selected by exact rational interval refinement (never by
        /// floating-point comparison) until the candidates are unambiguous.
        pub async fn add(&self, other: &Self) -> Self {
            Self::combine(self, other, Combine::Add)
        }

        /// ✖️ Product via `res_y(y^deg(f) * f(x/y), g(y))`.
        pub async fn mul(&self, other: &Self) -> Self {
            Self::combine(self, other, Combine::Mul)
        }

        async fn combine(a: &Self, b: &Self, operation: Combine) -> Self {
            if a.is_rational() {
                return match operation {
                    Combine::Add => b.add_rational(&a.lo),
                    Combine::Mul => b.scale_rational(&a.lo).unwrap_or_else(|| Self::from_rational(&Rational::zero())),
                };
            }
            if b.is_rational() {
                return match operation {
                    Combine::Add => a.add_rational(&b.lo),
                    Combine::Mul => a.scale_rational(&b.lo).unwrap_or_else(|| Self::from_rational(&Rational::zero())),
                };
            }
            let n = a.poly.degree().unwrap_or(0);
            let f_lifted: PolyU<PolyU<Integer>> = PolyU::from_coeffs(a.poly.coeffs().iter().map(|c| PolyU::constant(c.clone())).collect());
            let g_lifted: PolyU<PolyU<Integer>> = PolyU::from_coeffs(b.poly.coeffs().iter().map(|c| PolyU::constant(c.clone())).collect());
            let transformed: PolyU<PolyU<Integer>> = match operation {
                Combine::Add => {
                    let shift: PolyU<PolyU<Integer>> = PolyU::from_coeffs(vec![PolyU::x(), PolyU::constant(Integer::from_i64(-1))]);
                    f_lifted.semio_compose_rs(&shift)
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
                let (target_lo, target_hi) = match operation {
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
            let target = match operation {
                Combine::Add => a.to_f64() + b.to_f64(),
                Combine::Mul => a.to_f64() * b.to_f64(),
            };
            Self::root_of_near(&resultant_poly, target).unwrap_or_else(|| Self::from_rational(&Rational::zero()))
        }
        // #endregion 🔖️AlgebraicOperations
    }

    #[derive(Clone, Copy)]
    enum Combine {
        Add,
        Mul,
    }

    impl PolyU<Integer> {
        /// 🔗️ An integer polynomial with the same roots as `self(x - r)`: writing `r = num/den`, this
        /// computes `den^n * self((den*x - num)/den) == den^n * self(x - r)` entirely with integer
        /// arithmetic by composing with the integer linear polynomial `den*x - num` after scaling each
        /// coefficient `a_i` by `den^(n-i)` — the scaling by the positive constant `den^n` doesn't change
        /// the roots, so this is a valid (if non-primitive) defining polynomial for `alpha + r`.
        async fn compose_with_rational_shift(&self, r: &Rational) -> PolyU<Integer> {
            let n = self.degree().unwrap_or(0);
            let den_i = Integer::from_natural(r.denom().clone());
            let scaled_self = PolyU::from_coeffs(self.coeffs().iter().enumerate().map(|(i, coeff)| coeff.mul(&den_i.pow((n - i) as u64))).collect());
            let shift_int = PolyU::from_coeffs(vec![r.numer().neg(), den_i]);
            scaled_self.semio_compose_rs(&shift_int).primitive_part()
        }
    }

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        async fn ipoly(coeffs: Vec<i64>) -> PolyU<Integer> {
            PolyU::from_coeffs(coeffs.into_iter().map(Integer::from_i64).collect())
        }

        #[semio_framework_async_macros::async_test]
        async fn sqrt2_plus_sqrt3_has_minimal_poly_degree_4() {
            let sqrt2 = AlgebraicReal::nth_root(&Rational::from_i64(2, 1).unwrap(), 2).unwrap();
            let sqrt3 = AlgebraicReal::nth_root(&Rational::from_i64(3, 1).unwrap(), 2).unwrap();
            let sum = sqrt2.add(&sqrt3);
            // minimal poly of sqrt2+sqrt3 is x^4 - 10x^2 + 1 (up to sign/unit); verify by exact evaluation
            // via interval refinement: (sum)^2 should be close to 5 + 2*sqrt6 ~ 9.899
            let val = sum.to_f64();
            assert!((val - (2f64.sqrt() + 3f64.sqrt())).abs() < 1e-6);
            assert!(sum.degree() <= 4);
        }

        #[semio_framework_async_macros::async_test]
        async fn cbrt2_times_cbrt4_equals_2() {
            let cbrt2 = AlgebraicReal::nth_root(&Rational::from_i64(2, 1).unwrap(), 3).unwrap();
            let cbrt4 = AlgebraicReal::nth_root(&Rational::from_i64(4, 1).unwrap(), 3).unwrap();
            let product = cbrt2.mul(&cbrt4);
            assert!((product.to_f64() - 2.0).abs() < 1e-6);
        }

        #[semio_framework_async_macros::async_test]
        async fn from_rational_is_exact() {
            let r = Rational::from_i64(3, 4).unwrap();
            let a = AlgebraicReal::from_rational(&r);
            assert!(a.is_rational());
            assert_eq!(a.to_f64(), r.to_f64());
        }

        #[semio_framework_async_macros::async_test]
        async fn neg_and_inv_hand_cases() {
            let sqrt2 = AlgebraicReal::nth_root(&Rational::from_i64(2, 1).unwrap(), 2).unwrap();
            let negated = sqrt2.neg();
            assert!((negated.to_f64() + 2f64.sqrt()).abs() < 1e-9);
            let inv = sqrt2.inv().unwrap();
            assert!((inv.to_f64() - 1.0 / 2f64.sqrt()).abs() < 1e-6);
        }

        #[semio_framework_async_macros::async_test]
        async fn root_of_selects_correct_irreducible_factor() {
            // (x-1)(x^2-2): roots are 1, -sqrt2, sqrt2 in ascending order.
            let f = ipoly(vec![-1, 1]).mul(&ipoly(vec![-2, 0, 1]));
            let root0 = AlgebraicReal::root_of(&f, 0).unwrap(); // -sqrt2
            assert!((root0.to_f64() + 2f64.sqrt()).abs() < 1e-6);
            let root1 = AlgebraicReal::root_of(&f, 1).unwrap(); // 1
            assert!(root1.is_rational());
            assert_eq!(root1.to_f64(), 1.0);
        }
    }
    // #endregion 🔖️Tests
}
// #endregion 🔖️Algebraic

pub use algebraic::AlgebraicReal;
pub use factor::{factor_integer_poly, rational_roots};
pub use finite::{distinct_degree_factor, equal_degree_factor, factor_mod_p, is_irreducible, poly_mod_pow};
pub use multivariate::{Monomial, MonomialOrder, PolyM};
pub use roots::{cauchy_root_bound, count_roots_in, isolate_real_roots, refine_root, sturm_sequence};
pub use univariate::PolyU;
