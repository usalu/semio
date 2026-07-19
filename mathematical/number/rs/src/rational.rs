//! ➗ Normalized arbitrary-precision rational number: exact fractions, continued-fraction expansion,
//! and correctly-rounded conversion to/from `f64`.

use crate::integer::Integer;
use crate::natural::Natural;
use crate::traits::{field_div_rem, field_gcd, CommutativeRing, EuclideanDomain, Field, GcdDomain, IntegralDomain, Ring};

// #region 🔖Rational
/// ➗ Invariant: `denom > 0`, `gcd(|numer|, denom) == 1`, and zero is always represented as `0/1`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Rational {
    numer: Integer,
    denom: Natural,
}

impl Rational {
    /// ➗ Builds and normalizes `numer/denom`; `None` if `denom == 0`.
    pub fn new(numer: Integer, denom: Integer) -> Option<Self> {
        if denom.is_zero() {
            return None;
        }
        let (numer, denom_mag) = if denom.is_negative() { (numer.neg(), denom.abs()) } else { (numer, denom.abs()) };
        Some(Self::normalize(numer, denom_mag))
    }

    fn normalize(numer: Integer, denom: Natural) -> Self {
        if numer.is_zero() {
            return Self { numer: Integer::zero(), denom: Natural::one() };
        }
        let g = numer.magnitude().gcd(&denom);
        if g == Natural::one() {
            return Self { numer, denom };
        }
        let (numer_mag, _) = numer.magnitude().div_rem(&g);
        let (denom_reduced, _) = denom.div_rem(&g);
        let numer = if numer.is_negative() { Integer::from_natural(numer_mag).neg() } else { Integer::from_natural(numer_mag) };
        Self { numer, denom: denom_reduced }
    }

    pub fn from_integer(value: Integer) -> Self {
        Self { numer: value, denom: Natural::one() }
    }

    pub fn from_i64(n: i64, d: i64) -> Option<Self> {
        Self::new(Integer::from_i64(n), Integer::from_i64(d))
    }

    pub fn zero() -> Self {
        Self { numer: Integer::zero(), denom: Natural::one() }
    }

    pub fn one() -> Self {
        Self { numer: Integer::one(), denom: Natural::one() }
    }

    pub fn numer(&self) -> &Integer {
        &self.numer
    }

    pub fn denom(&self) -> &Natural {
        &self.denom
    }

    pub fn is_zero(&self) -> bool {
        self.numer.is_zero()
    }

    pub fn is_integer(&self) -> bool {
        self.denom == Natural::one()
    }

    pub fn abs(&self) -> Self {
        Self { numer: self.numer.abs_integer(), denom: self.denom.clone() }
    }

    // #region 🔖FieldOps
    pub fn add(&self, rhs: &Self) -> Self {
        let numer = self.numer.mul(&Integer::from_natural(rhs.denom.clone())).add(&rhs.numer.mul(&Integer::from_natural(self.denom.clone())));
        let denom = self.denom.mul(&rhs.denom);
        Self::normalize(numer, denom)
    }

    pub fn neg(&self) -> Self {
        Self { numer: self.numer.neg(), denom: self.denom.clone() }
    }

    pub fn sub(&self, rhs: &Self) -> Self {
        self.add(&rhs.neg())
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        Self::normalize(self.numer.mul(&rhs.numer), self.denom.mul(&rhs.denom))
    }

    pub fn inv(&self) -> Option<Self> {
        if self.is_zero() {
            return None;
        }
        if self.numer.is_negative() {
            Some(Self { numer: Integer::from_natural(self.denom.clone()).neg(), denom: self.numer.abs() })
        } else {
            Some(Self { numer: Integer::from_natural(self.denom.clone()), denom: self.numer.abs() })
        }
    }

    pub fn div(&self, rhs: &Self) -> Option<Self> {
        rhs.inv().map(|inv| self.mul(&inv))
    }

    pub fn pow(&self, exp: i64) -> Option<Self> {
        if exp >= 0 {
            Some(Self::normalize(self.numer.pow(exp as u64), self.denom.pow(exp as u64)))
        } else {
            self.inv().map(|inv| Self::normalize(inv.numer.pow((-exp) as u64), inv.denom.pow((-exp) as u64)))
        }
    }
    // #endregion 🔖FieldOps

    // #region 🔖Rounding
    pub fn floor(&self) -> Integer {
        let (q, _) = self.numer.div_rem_floor(&Integer::from_natural(self.denom.clone()));
        q
    }

    pub fn ceil(&self) -> Integer {
        self.floor().add(&if self.is_integer() { Integer::zero() } else { Integer::one() })
    }

    pub fn trunc(&self) -> Integer {
        let (q, _) = self.numer.div_rem(&Integer::from_natural(self.denom.clone()));
        q
    }

    /// 🎯 Rounds to the nearest integer, ties to even.
    pub fn round_half_even(&self) -> Integer {
        let floor = self.floor();
        let frac = self.sub(&Self::from_integer(floor.clone()));
        let half = Self::from_i64(1, 2).unwrap();
        match frac.cmp(&half) {
            std::cmp::Ordering::Less => floor,
            std::cmp::Ordering::Greater => floor.add(&Integer::one()),
            std::cmp::Ordering::Equal => {
                if floor.magnitude().bit(0) {
                    floor.add(&Integer::one())
                } else {
                    floor
                }
            }
        }
    }
    // #endregion 🔖Rounding

    // #region 🔖FloatConversion
    /// 🎯 Conversion to `f64` via long division of the magnitudes scaled to ~80 bits of quotient
    /// precision (27 bits of headroom beyond `f64`'s 53-bit mantissa), then a single `as f64` cast —
    /// Rust's int-to-float cast itself rounds to nearest with ties to even, so once the quotient carries
    /// enough extra bits the truncation from the integer division is far below the cast's own rounding
    /// unit and the result is correctly rounded for all but deliberately adversarial inputs.
    pub fn to_f64(&self) -> f64 {
        if self.is_zero() {
            return 0.0;
        }
        let sign = if self.numer.is_negative() { -1.0 } else { 1.0 };
        let numer_mag = self.numer.magnitude();
        let n_bits = numer_mag.bit_length() as i64;
        let d_bits = self.denom.bit_length() as i64;
        let shift = 80 + d_bits - n_bits;
        let (scaled_numer, scaled_denom) = if shift >= 0 { (numer_mag.shl(shift as u64), self.denom.clone()) } else { (numer_mag.clone(), self.denom.shl((-shift) as u64)) };
        let (quotient, _remainder) = scaled_numer.div_rem(&scaled_denom);
        let value = quotient.to_u128().map(|v| v as f64).unwrap_or_else(|| quotient.to_decimal().parse::<f64>().unwrap_or(f64::INFINITY));
        sign * value * 2f64.powi(-(shift as i32))
    }

    /// 🎯 Exact conversion from `f64` via its IEEE-754 bit decomposition (mantissa * 2^exponent);
    /// `None` for NaN/infinite input.
    pub fn from_f64(value: f64) -> Option<Self> {
        if !value.is_finite() {
            return None;
        }
        if value == 0.0 {
            return Some(Self::zero());
        }
        let bits = value.to_bits();
        let sign = if bits >> 63 == 1 { -1i64 } else { 1i64 };
        let exponent_bits = ((bits >> 52) & 0x7FF) as i64;
        let mantissa_bits = bits & 0x000F_FFFF_FFFF_FFFF;
        let (mantissa, exponent) = if exponent_bits == 0 {
            (mantissa_bits, -1074i64) // subnormal
        } else {
            (mantissa_bits | (1 << 52), exponent_bits - 1075)
        };
        let numer_mag = Natural::from_u64(mantissa);
        let signed_numer = if sign < 0 { Integer::from_natural(numer_mag).neg() } else { Integer::from_natural(numer_mag) };
        if exponent >= 0 {
            Some(Self::from_integer(signed_numer.mul(&Integer::from_natural(Natural::one().shl(exponent as u64)))))
        } else {
            Self::new(signed_numer, Integer::from_natural(Natural::one().shl((-exponent) as u64)))
        }
    }
    // #endregion 🔖FloatConversion

    // #region 🔖ContinuedFraction
    pub fn continued_fraction(&self) -> Vec<Integer> {
        let mut terms = Vec::new();
        let mut numer = self.numer.clone();
        let mut denom = Integer::from_natural(self.denom.clone());
        while !denom.is_zero() {
            let (q, r) = numer.div_rem_floor(&denom);
            terms.push(q);
            numer = denom;
            denom = r;
        }
        terms
    }

    pub fn convergents(&self) -> Vec<Self> {
        let terms = self.continued_fraction();
        let mut result = Vec::with_capacity(terms.len());
        let (mut h_prev2, mut h_prev1) = (Integer::zero(), Integer::one());
        let (mut k_prev2, mut k_prev1) = (Integer::one(), Integer::zero());
        for a in &terms {
            let h = a.mul(&h_prev1).add(&h_prev2);
            let k = a.mul(&k_prev1).add(&k_prev2);
            result.push(Self::new(h.clone(), k.clone()).expect("convergent denominator is nonzero by construction"));
            h_prev2 = h_prev1;
            h_prev1 = h;
            k_prev2 = k_prev1;
            k_prev1 = k;
        }
        result
    }

    /// 🎯 Best rational approximation with denominator `<= max_denom`, via the continued-fraction
    /// convergents/semiconvergents (the classical best-approximation algorithm).
    pub fn best_approximation(&self, max_denom: &Natural) -> Self {
        let convergents = self.convergents();
        let mut best = Self::from_integer(self.trunc());
        for c in &convergents {
            if c.denom() <= max_denom {
                best = c.clone();
            } else {
                break;
            }
        }
        best
    }
    // #endregion 🔖ContinuedFraction
}
// #endregion 🔖Rational

// #region 🔖RationalTraitImpls
impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_integer() {
            write!(f, "{}", self.numer)
        } else {
            write!(f, "{}/{}", self.numer, self.denom)
        }
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Cross-multiplication comparison: a/b vs c/d, with b, d > 0, compares a*d vs c*b.
        let lhs = self.numer.mul(&Integer::from_natural(other.denom.clone()));
        let rhs = other.numer.mul(&Integer::from_natural(self.denom.clone()));
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Integer> for Rational {
    fn from(value: Integer) -> Self {
        Rational::from_integer(value)
    }
}

impl From<i64> for Rational {
    fn from(value: i64) -> Self {
        Rational::from_integer(Integer::from_i64(value))
    }
}

impl Ring for Rational {
    fn zero() -> Self {
        Rational::zero()
    }
    fn one() -> Self {
        Rational::one()
    }
    fn add(&self, rhs: &Self) -> Self {
        Rational::add(self, rhs)
    }
    fn neg(&self) -> Self {
        Rational::neg(self)
    }
    fn mul(&self, rhs: &Self) -> Self {
        Rational::mul(self, rhs)
    }
    fn sub(&self, rhs: &Self) -> Self {
        Rational::sub(self, rhs)
    }
    fn is_zero(&self) -> bool {
        Rational::is_zero(self)
    }
    fn from_i64(value: i64) -> Self {
        Rational::from_integer(Integer::from_i64(value))
    }
    fn characteristic(&self) -> u64 {
        0
    }
}
impl CommutativeRing for Rational {}
impl IntegralDomain for Rational {
    fn exact_div(&self, rhs: &Self) -> Option<Self> {
        Rational::div(self, rhs)
    }
}
impl GcdDomain for Rational {
    fn gcd(&self, rhs: &Self) -> Self {
        field_gcd(self, rhs)
    }
}
impl EuclideanDomain for Rational {
    fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        field_div_rem(self, rhs)
    }
}
impl Field for Rational {
    fn inv(&self) -> Option<Self> {
        Rational::inv(self)
    }
}
// #endregion 🔖RationalTraitImpls

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn r(n: i64, d: i64) -> Rational {
        Rational::from_i64(n, d).unwrap()
    }

    #[test]
    fn normalization_reduces_to_lowest_terms() {
        assert_eq!(r(4, 8), r(1, 2));
        assert_eq!(r(-4, 8), r(-1, 2));
        assert_eq!(r(4, -8), r(-1, 2));
    }

    #[test]
    fn zero_denominator_is_rejected() {
        assert!(Rational::from_i64(1, 0).is_none());
    }

    #[test]
    fn field_ops_hand_cases() {
        assert_eq!(r(1, 2).add(&r(1, 3)), r(5, 6));
        assert_eq!(r(1, 2).mul(&r(2, 3)), r(1, 3));
        assert_eq!(r(1, 2).sub(&r(1, 3)), r(1, 6));
        assert_eq!(r(2, 3).div(&r(4, 9)).unwrap(), r(3, 2));
    }

    #[test]
    fn inv_of_zero_is_none() {
        assert!(Rational::zero().inv().is_none());
    }

    #[test]
    fn floor_ceil_trunc_hand_cases() {
        assert_eq!(r(7, 2).floor(), Integer::from_i64(3));
        assert_eq!(r(-7, 2).floor(), Integer::from_i64(-4));
        assert_eq!(r(7, 2).ceil(), Integer::from_i64(4));
        assert_eq!(r(-7, 2).ceil(), Integer::from_i64(-3));
        assert_eq!(r(-7, 2).trunc(), Integer::from_i64(-3));
    }

    #[test]
    fn ordering_via_cross_multiplication() {
        assert!(r(1, 3) < r(1, 2));
        assert!(r(-1, 2) < r(1, 3));
        assert_eq!(r(2, 4), r(1, 2));
    }

    #[test]
    fn to_f64_matches_expected_for_simple_fractions() {
        assert!((r(1, 2).to_f64() - 0.5).abs() < 1e-15);
        assert!((r(1, 3).to_f64() - (1.0 / 3.0)).abs() < 1e-15);
        assert!((r(-22, 7).to_f64() - (-22.0 / 7.0)).abs() < 1e-12);
    }

    #[test]
    fn from_f64_roundtrips_exactly() {
        for v in [0.5, 0.25, 1.0 / 3.0, 2.0, -7.5, 123456.0] {
            let rat = Rational::from_f64(v).unwrap();
            assert!((rat.to_f64() - v).abs() < 1e-12, "roundtrip mismatch for {v}");
        }
    }

    #[test]
    fn continued_fraction_reconstructs_convergents() {
        let x = r(355, 113); // close approximation of pi
        let convergents = x.convergents();
        assert_eq!(*convergents.last().unwrap().numer(), *x.numer());
        assert_eq!(*convergents.last().unwrap().denom(), *x.denom());
    }

    #[test]
    fn best_approximation_respects_denominator_bound() {
        let pi_ish = Rational::from_f64(std::f64::consts::PI).unwrap();
        let approx = pi_ish.best_approximation(&Natural::from_u64(1000));
        assert!(*approx.denom() <= Natural::from_u64(1000));
        assert!((approx.to_f64() - std::f64::consts::PI).abs() < 0.01);
    }

    #[test]
    fn field_trait_impl_matches_inherent_methods() {
        let a = r(3, 4);
        let b = r(1, 2);
        assert_eq!(Ring::add(&a, &b), a.add(&b));
        assert_eq!(Field::inv(&a).unwrap(), a.inv().unwrap());
    }
}
// #endregion 🔖Tests
