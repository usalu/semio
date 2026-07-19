//! ➕➖ Signed arbitrary-precision integer: a [`crate::natural::Natural`] magnitude plus a sign, with
//! the three division conventions (truncated, floor, Euclidean) that downstream modular/rational code needs.

use crate::natural::Natural;
use crate::traits::{CommutativeRing, EuclideanDomain, GcdDomain, IntegralDomain, Ring};

// #region 🔖Sign
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Sign {
    Negative,
    Zero,
    Positive,
}
// #endregion 🔖Sign

// #region 🔖Integer
/// ➕➖ Sign-magnitude signed integer; invariant: `sign == Zero` if and only if `magnitude == 0`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Integer {
    sign: Sign,
    magnitude: Natural,
}

impl Integer {
    fn from_parts(sign: Sign, magnitude: Natural) -> Self {
        if magnitude.is_zero() {
            Self { sign: Sign::Zero, magnitude }
        } else {
            Self { sign, magnitude }
        }
    }

    pub fn zero() -> Self {
        Self { sign: Sign::Zero, magnitude: Natural::zero() }
    }

    pub fn one() -> Self {
        Self { sign: Sign::Positive, magnitude: Natural::one() }
    }

    pub fn from_i64(value: i64) -> Self {
        if value == 0 {
            return Self::zero();
        }
        let sign = if value < 0 { Sign::Negative } else { Sign::Positive };
        Self::from_parts(sign, Natural::from_u64(value.unsigned_abs()))
    }

    pub fn from_i128(value: i128) -> Self {
        if value == 0 {
            return Self::zero();
        }
        let sign = if value < 0 { Sign::Negative } else { Sign::Positive };
        Self::from_parts(sign, Natural::from_u128(value.unsigned_abs()))
    }

    pub fn from_natural(magnitude: Natural) -> Self {
        Self::from_parts(Sign::Positive, magnitude)
    }

    pub fn to_i64(&self) -> Option<i64> {
        let mag = self.magnitude.to_u64()?;
        match self.sign {
            Sign::Zero => Some(0),
            Sign::Positive => i64::try_from(mag).ok(),
            Sign::Negative => {
                if mag <= i64::MAX as u64 + 1 {
                    Some((mag as i128 * -1) as i64)
                } else {
                    None
                }
            }
        }
    }

    pub fn sign(&self) -> Sign {
        self.sign
    }

    pub fn signum(&self) -> i8 {
        match self.sign {
            Sign::Negative => -1,
            Sign::Zero => 0,
            Sign::Positive => 1,
        }
    }

    pub fn is_zero(&self) -> bool {
        matches!(self.sign, Sign::Zero)
    }

    pub fn is_positive(&self) -> bool {
        matches!(self.sign, Sign::Positive)
    }

    pub fn is_negative(&self) -> bool {
        matches!(self.sign, Sign::Negative)
    }

    pub fn magnitude(&self) -> &Natural {
        &self.magnitude
    }

    pub fn abs(&self) -> Natural {
        self.magnitude.clone()
    }

    pub fn abs_integer(&self) -> Self {
        Self::from_parts(Sign::Positive, self.magnitude.clone())
    }

    pub fn neg(&self) -> Self {
        match self.sign {
            Sign::Zero => Self::zero(),
            Sign::Negative => Self::from_parts(Sign::Positive, self.magnitude.clone()),
            Sign::Positive => Self::from_parts(Sign::Negative, self.magnitude.clone()),
        }
    }

    // #region 🔖AddSubMul
    pub fn add(&self, rhs: &Self) -> Self {
        match (self.sign, rhs.sign) {
            (Sign::Zero, _) => rhs.clone(),
            (_, Sign::Zero) => self.clone(),
            (a, b) if a == b => Self::from_parts(a, self.magnitude.add(&rhs.magnitude)),
            _ => {
                if self.magnitude >= rhs.magnitude {
                    Self::from_parts(self.sign, self.magnitude.checked_sub(&rhs.magnitude).unwrap())
                } else {
                    Self::from_parts(rhs.sign, rhs.magnitude.checked_sub(&self.magnitude).unwrap())
                }
            }
        }
    }

    pub fn sub(&self, rhs: &Self) -> Self {
        self.add(&rhs.neg())
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        if self.is_zero() || rhs.is_zero() {
            return Self::zero();
        }
        let sign = if self.sign == rhs.sign { Sign::Positive } else { Sign::Negative };
        Self::from_parts(sign, self.magnitude.mul(&rhs.magnitude))
    }
    // #endregion 🔖AddSubMul

    // #region 🔖DivRem
    /// ➗ Truncated toward zero (Rust/C convention): `q * rhs + r == self`, `sign(r) == sign(self)` or `r == 0`.
    pub fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        assert!(!rhs.is_zero(), "div_rem: division by zero");
        let (qm, rm) = self.magnitude.div_rem(&rhs.magnitude);
        let q_sign = if self.sign == rhs.sign { Sign::Positive } else { Sign::Negative };
        (Self::from_parts(q_sign, qm), Self::from_parts(self.sign, rm))
    }

    /// ➗ Floored: `q = floor(self / rhs)`, remainder has the same sign as `rhs` (or is zero).
    pub fn div_rem_floor(&self, rhs: &Self) -> (Self, Self) {
        let (q, r) = self.div_rem(rhs);
        if r.is_zero() || r.sign == rhs.sign {
            (q, r)
        } else {
            (q.sub(&Self::one()), r.add(rhs))
        }
    }

    /// ➗ Euclidean: remainder is always non-negative (`0 <= r < |rhs|`).
    pub fn div_rem_euclid(&self, rhs: &Self) -> (Self, Self) {
        let (q, r) = self.div_rem(rhs);
        if r.is_negative() {
            if rhs.is_positive() {
                (q.sub(&Self::one()), r.add(rhs))
            } else {
                (q.add(&Self::one()), r.sub(rhs))
            }
        } else {
            (q, r)
        }
    }
    // #endregion 🔖DivRem

    pub fn pow(&self, exp: u64) -> Self {
        let mag = self.magnitude.pow(exp);
        let sign = if mag.is_zero() {
            Sign::Zero
        } else if self.is_negative() && exp % 2 == 1 {
            Sign::Negative
        } else {
            Sign::Positive
        };
        Self::from_parts(sign, mag)
    }

    /// √ Integer square root of a non-negative `Integer`; `None` for negative inputs.
    pub fn checked_isqrt(&self) -> Option<Self> {
        if self.is_negative() {
            return None;
        }
        Some(Self::from_parts(Sign::Positive, self.magnitude.isqrt()))
    }

    /// √ⁿ Integer n-th root; even `n` rejects negative inputs, odd `n` accepts them.
    pub fn nth_root(&self, n: u32) -> Option<Self> {
        if self.is_negative() && n % 2 == 0 {
            return None;
        }
        let mag_root = self.magnitude.nth_root(n);
        Some(Self::from_parts(self.sign, mag_root))
    }

    pub fn gcd(&self, rhs: &Self) -> Natural {
        self.magnitude.gcd(&rhs.magnitude)
    }

    /// 🤝 Extended Euclidean algorithm: returns `(g, x, y)` with `g = x*self + y*rhs` and `g = gcd(self, rhs)`.
    pub fn extended_gcd(&self, rhs: &Self) -> (Self, Self, Self) {
        let (mut old_r, mut r) = (self.clone(), rhs.clone());
        let (mut old_s, mut s) = (Self::one(), Self::zero());
        let (mut old_t, mut t) = (Self::zero(), Self::one());
        while !r.is_zero() {
            let (q, _) = old_r.div_rem(&r);
            let new_r = old_r.sub(&q.mul(&r));
            old_r = std::mem::replace(&mut r, new_r);
            let new_s = old_s.sub(&q.mul(&s));
            old_s = std::mem::replace(&mut s, new_s);
            let new_t = old_t.sub(&q.mul(&t));
            old_t = std::mem::replace(&mut t, new_t);
        }
        if old_r.is_negative() {
            (old_r.neg(), old_s.neg(), old_t.neg())
        } else {
            (old_r, old_s, old_t)
        }
    }

    pub fn lcm(&self, rhs: &Self) -> Natural {
        if self.is_zero() || rhs.is_zero() {
            return Natural::zero();
        }
        let g = self.gcd(rhs);
        let (prod, _) = self.magnitude.mul(&rhs.magnitude).div_rem(&g);
        prod
    }

    pub fn to_decimal(&self) -> String {
        if self.is_negative() {
            format!("-{}", self.magnitude.to_decimal())
        } else {
            self.magnitude.to_decimal()
        }
    }
}
// #endregion 🔖Integer

// #region 🔖IntegerTraitImpls
impl std::fmt::Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_decimal())
    }
}

impl std::str::FromStr for Integer {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(rest) = s.strip_prefix('-') {
            let mag = Natural::from_str_radix(rest, 10).ok_or(())?;
            Ok(Self::from_parts(Sign::Negative, mag))
        } else {
            let mag = Natural::from_str_radix(s, 10).ok_or(())?;
            Ok(Self::from_parts(Sign::Positive, mag))
        }
    }
}

impl Ord for Integer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        let rank = |s: Sign| match s {
            Sign::Negative => 0,
            Sign::Zero => 1,
            Sign::Positive => 2,
        };
        match rank(self.sign).cmp(&rank(other.sign)) {
            Ordering::Equal => match self.sign {
                Sign::Negative => other.magnitude.cmp(&self.magnitude),
                _ => self.magnitude.cmp(&other.magnitude),
            },
            other_order => other_order,
        }
    }
}

impl PartialOrd for Integer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

macro_rules! impl_from_int {
    ($($t:ty),*) => {
        $(impl From<$t> for Integer {
            fn from(value: $t) -> Self {
                Integer::from_i64(value as i64)
            }
        })*
    };
}
impl_from_int!(i8, i16, i32, i64, u8, u16, u32);

impl From<u64> for Integer {
    fn from(value: u64) -> Self {
        Integer::from_parts(if value == 0 { Sign::Zero } else { Sign::Positive }, Natural::from_u64(value))
    }
}

impl From<Natural> for Integer {
    fn from(value: Natural) -> Self {
        Integer::from_natural(value)
    }
}

impl Ring for Integer {
    fn zero() -> Self {
        Integer::zero()
    }
    fn one() -> Self {
        Integer::one()
    }
    fn add(&self, rhs: &Self) -> Self {
        Integer::add(self, rhs)
    }
    fn neg(&self) -> Self {
        Integer::neg(self)
    }
    fn mul(&self, rhs: &Self) -> Self {
        Integer::mul(self, rhs)
    }
    fn sub(&self, rhs: &Self) -> Self {
        Integer::sub(self, rhs)
    }
    fn is_zero(&self) -> bool {
        Integer::is_zero(self)
    }
    fn from_i64(value: i64) -> Self {
        Integer::from_i64(value)
    }
    fn pow(&self, exp: u64) -> Self {
        Integer::pow(self, exp)
    }
    fn characteristic(&self) -> u64 {
        0
    }
}
impl CommutativeRing for Integer {}
impl IntegralDomain for Integer {
    fn exact_div(&self, rhs: &Self) -> Option<Self> {
        if rhs.is_zero() {
            return None;
        }
        let (q, r) = self.div_rem(rhs);
        if r.is_zero() {
            Some(q)
        } else {
            None
        }
    }
}
impl GcdDomain for Integer {
    fn gcd(&self, rhs: &Self) -> Self {
        Self::from_natural(Integer::gcd(self, rhs))
    }
    fn lcm(&self, rhs: &Self) -> Self {
        Self::from_natural(Integer::lcm(self, rhs))
    }
}
impl EuclideanDomain for Integer {
    fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        Integer::div_rem(self, rhs)
    }
}
// #endregion 🔖IntegerTraitImpls

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn i(s: &str) -> Integer {
        Integer::from_str(s).unwrap()
    }

    #[test]
    fn string_roundtrip_positive_and_negative() {
        assert_eq!(i("12345").to_decimal(), "12345");
        assert_eq!(i("-12345").to_decimal(), "-12345");
        assert_eq!(i("0").to_decimal(), "0");
        assert_eq!(i("-0").to_decimal(), "0");
    }

    #[test]
    fn add_sub_sign_handling() {
        assert_eq!(i("5").add(&i("-3")), i("2"));
        assert_eq!(i("-5").add(&i("3")), i("-2"));
        assert_eq!(i("-5").add(&i("-3")), i("-8"));
        assert_eq!(i("5").sub(&i("8")), i("-3"));
    }

    #[test]
    fn mul_sign_handling() {
        assert_eq!(i("-3").mul(&i("4")), i("-12"));
        assert_eq!(i("-3").mul(&i("-4")), i("12"));
        assert_eq!(i("0").mul(&i("-4")), i("0"));
    }

    #[test]
    fn div_rem_truncated_matches_rust_semantics() {
        assert_eq!(i("7").div_rem(&i("2")), (i("3"), i("1")));
        assert_eq!(i("-7").div_rem(&i("2")), (i("-3"), i("-1")));
        assert_eq!(i("7").div_rem(&i("-2")), (i("-3"), i("1")));
        assert_eq!(i("-7").div_rem(&i("-2")), (i("3"), i("-1")));
    }

    #[test]
    fn div_rem_floor_matches_math_floor_division() {
        assert_eq!(i("-7").div_rem_floor(&i("2")), (i("-4"), i("1")));
        assert_eq!(i("7").div_rem_floor(&i("-2")), (i("-4"), i("-1")));
    }

    #[test]
    fn div_rem_euclid_remainder_always_nonnegative() {
        for (a, b) in [("-7", "2"), ("7", "-2"), ("-7", "-2"), ("7", "2")] {
            let (q, r) = i(a).div_rem_euclid(&i(b));
            assert!(!r.is_negative());
            assert_eq!(q.mul(&i(b)).add(&r), i(a));
        }
    }

    #[test]
    fn extended_gcd_satisfies_bezout_identity() {
        for (a, b) in [(240, 46), (-240, 46), (240, -46), (17, 5), (0, 5)] {
            let ia = Integer::from_i64(a);
            let ib = Integer::from_i64(b);
            let (g, x, y) = ia.extended_gcd(&ib);
            assert_eq!(x.mul(&ia).add(&y.mul(&ib)), g);
            assert!(!g.is_negative());
        }
    }

    #[test]
    fn ordering_across_signs() {
        assert!(i("-5") < i("-3"));
        assert!(i("-1") < i("0"));
        assert!(i("0") < i("1"));
        assert!(i("-100") < i("1"));
    }

    #[test]
    fn checked_isqrt_rejects_negative() {
        assert!(i("-4").checked_isqrt().is_none());
        assert_eq!(i("4").checked_isqrt().unwrap(), i("2"));
    }

    #[test]
    fn nth_root_odd_accepts_negative() {
        assert_eq!(i("-27").nth_root(3).unwrap(), i("-3"));
        assert!(i("-4").nth_root(2).is_none());
    }

    #[test]
    fn ring_trait_impl_matches_inherent_methods() {
        let a = i("7");
        let b = i("3");
        assert_eq!(Ring::add(&a, &b), a.add(&b));
        assert_eq!(Ring::mul(&a, &b), a.mul(&b));
        assert_eq!(IntegralDomain::exact_div(&i("12"), &i("4")), Some(i("3")));
        assert_eq!(IntegralDomain::exact_div(&i("12"), &i("5")), None);
    }
}
// #endregion 🔖Tests
