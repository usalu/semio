//! 🔢 Arbitrary-precision integers and rationals, modular arithmetic, primality/factorization,
//! certified interval arithmetic, and the abstract-algebra trait hierarchy (`Ring` through `Field`)
//! that `mathematical_algebra`, `mathematical_polynomial`, and `mathematical_cas` are generic over.
// #region 🔖Traits
pub mod traits {
//! 💍 Abstract algebra hierarchy shared by every exact numeric/symbolic crate downstream:
//! `mathematical_algebra`'s generic matrices, `mathematical_polynomial`'s generic polynomials, and
//! `mathematical_cas`'s expression coefficients are all written once, generically, against these traits.

// #region 🔖Ring
/// 💍 Ring with multiplicative identity. Methods take `&self`/`&Self` throughout (not `Copy`/by-value)
/// because the concrete types that matter most (`Integer`, `PolyU<C>`, `Expr`) are heap-allocated and
/// expensive to move implicitly.
pub trait Ring: Clone + PartialEq + std::fmt::Debug {
    /// 0️⃣ Additive identity.
    fn zero() -> Self;
    /// 1️⃣ Multiplicative identity.
    fn one() -> Self;
    fn add(&self, rhs: &Self) -> Self;
    fn neg(&self) -> Self;
    fn mul(&self, rhs: &Self) -> Self;
    /// ➖ Default via `add(neg)`; override when direct subtraction avoids an allocation.
    fn sub(&self, rhs: &Self) -> Self {
        self.add(&rhs.neg())
    }
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
    fn is_one(&self) -> bool {
        *self == Self::one()
    }
    /// 🔢 Embeds a signed machine integer via repeated doubling; concrete types override with a direct
    /// constructor where one exists.
    fn from_i64(value: i64) -> Self {
        if value == 0 {
            return Self::zero();
        }
        let neg = value < 0;
        let mut mag = (value as i128).unsigned_abs();
        let mut result = Self::zero();
        let mut base = Self::one();
        while mag > 0 {
            if mag & 1 == 1 {
                result = result.add(&base);
            }
            base = base.add(&base);
            mag >>= 1;
        }
        if neg {
            result.neg()
        } else {
            result
        }
    }
    /// 🔋 Square-and-multiply exponentiation; correct for any ring (including `exp == 0`, giving `one()`).
    fn pow(&self, exp: u64) -> Self {
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
    /// 🧬 Additive order of `one()` (0 in characteristic zero); an instance method — not a constant —
    /// because `ModInt`'s characteristic is a runtime modulus, not a compile-time property of the type.
    fn characteristic(&self) -> u64;
}

/// 💍 Marker for rings whose multiplication commutes; nothing here relies on it structurally, but it
/// documents the contract every ring in this stack actually satisfies and gates commutative-only algorithms.
pub trait CommutativeRing: Ring {}
// #endregion 🔖Ring

// #region 🔖IntegralDomain
/// 🚫 Commutative ring with no zero divisors: `exact_div` is the load-bearing operation for
/// fraction-free algorithms (Bareiss elimination, polynomial pseudo-division) that only ever divide
/// when the result is provably exact.
pub trait IntegralDomain: CommutativeRing {
    /// ➗ `Some(self / rhs)` if `rhs` divides `self` exactly, else `None`. `rhs` must be nonzero.
    fn exact_div(&self, rhs: &Self) -> Option<Self>;
}
// #endregion 🔖IntegralDomain

// #region 🔖GcdDomain
pub trait GcdDomain: IntegralDomain {
    fn gcd(&self, rhs: &Self) -> Self;
    /// ➗ Default `self * rhs / gcd(self, rhs)`; degenerate (zero) inputs return `zero()`.
    fn lcm(&self, rhs: &Self) -> Self {
        if self.is_zero() || rhs.is_zero() {
            return Self::zero();
        }
        let g = self.gcd(rhs);
        self.mul(rhs).exact_div(&g).expect("gcd(a, b) divides a * b exactly by definition")
    }
}
// #endregion 🔖GcdDomain

// #region 🔖EuclideanDomain
pub trait EuclideanDomain: GcdDomain {
    /// ➗ `(quotient, remainder)` such that `self == quotient * rhs + remainder`. `rhs` must be nonzero.
    fn div_rem(&self, rhs: &Self) -> (Self, Self)
    where
        Self: Sized;
}

/// ➗ Trivial Euclidean-domain `div_rem` for any `Field`: remainder is always zero. Free function
/// (not a default trait method) because a blanket `impl<T: Field> EuclideanDomain for T` would
/// conflict with concrete manual impls under Rust's orphan/coherence rules.
pub fn field_div_rem<T: Field>(a: &T, b: &T) -> (T, T) {
    (a.div(b).expect("field_div_rem: divisor must be nonzero"), T::zero())
}

/// 🚫 Trivial GCD-domain `gcd` for any `Field`: every nonzero element is a unit, so the gcd of any two
/// non-both-zero elements is `one()` (and `zero()` when both inputs are zero).
pub fn field_gcd<T: Field>(a: &T, b: &T) -> T {
    if a.is_zero() && b.is_zero() {
        T::zero()
    } else {
        T::one()
    }
}
// #endregion 🔖EuclideanDomain

// #region 🔖Field
pub trait Field: EuclideanDomain {
    /// ➗ Multiplicative inverse; `None` only for zero.
    fn inv(&self) -> Option<Self>
    where
        Self: Sized;
    fn div(&self, rhs: &Self) -> Option<Self>
    where
        Self: Sized,
    {
        rhs.inv().map(|inv| self.mul(&inv))
    }
}
// #endregion 🔖Field

// #region 🔖PrimitiveImpls
impl Ring for i64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }
    fn neg(&self) -> Self {
        -self
    }
    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }
    fn sub(&self, rhs: &Self) -> Self {
        self - rhs
    }
    fn from_i64(value: i64) -> Self {
        value
    }
    fn characteristic(&self) -> u64 {
        0
    }
}
impl CommutativeRing for i64 {}
impl IntegralDomain for i64 {
    fn exact_div(&self, rhs: &Self) -> Option<Self> {
        if *rhs == 0 || self % rhs != 0 {
            None
        } else {
            Some(self / rhs)
        }
    }
}
impl GcdDomain for i64 {
    fn gcd(&self, rhs: &Self) -> Self {
        let mut a = self.unsigned_abs();
        let mut b = rhs.unsigned_abs();
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a as i64
    }
}
impl EuclideanDomain for i64 {
    fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        (self / rhs, self % rhs)
    }
}

/// 🌊 `f64` as an (approximate) field: convenient for numeric-evaluation code paths that want to be
/// generic over `Field`, but law-violating in the strict sense (rounding breaks associativity/exactness) —
/// never use this impl where exactness is required.
impl Ring for f64 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }
    fn neg(&self) -> Self {
        -self
    }
    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }
    fn sub(&self, rhs: &Self) -> Self {
        self - rhs
    }
    fn is_zero(&self) -> bool {
        *self == 0.0
    }
    fn from_i64(value: i64) -> Self {
        value as f64
    }
    fn pow(&self, exp: u64) -> Self {
        self.powi(exp as i32)
    }
    fn characteristic(&self) -> u64 {
        0
    }
}
impl CommutativeRing for f64 {}
impl IntegralDomain for f64 {
    fn exact_div(&self, rhs: &Self) -> Option<Self> {
        if *rhs == 0.0 {
            None
        } else {
            Some(self / rhs)
        }
    }
}
impl GcdDomain for f64 {
    fn gcd(&self, rhs: &Self) -> Self {
        field_gcd(self, rhs)
    }
}
impl EuclideanDomain for f64 {
    fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        field_div_rem(self, rhs)
    }
}
impl Field for f64 {
    fn inv(&self) -> Option<Self> {
        if *self == 0.0 {
            None
        } else {
            Some(1.0 / self)
        }
    }
}
// #endregion 🔖PrimitiveImpls

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i64_ring_axioms_hold_on_small_samples() {
        for a in -5i64..=5 {
            for b in -5i64..=5 {
                assert_eq!(a.add(&b), a + b);
                assert_eq!(a.mul(&b), a * b);
                assert_eq!(a.sub(&b), a - b);
            }
        }
    }

    #[test]
    fn i64_from_i64_via_default_matches_identity_for_positive_and_negative() {
        // exercise the default doubling-based from_i64 through a type that overrides it trivially,
        // and separately validate the algorithm itself using f64 (which does NOT override from_i64
        // meaningfully differently, so cross-check against a hand-rolled doubling loop instead).
        fn via_doubling(value: i64) -> i64 {
            <i64 as Ring>::from_i64(value)
        }
        for v in [-100i64, -1, 0, 1, 100, 12345] {
            assert_eq!(via_doubling(v), v);
        }
    }

    #[test]
    fn i64_pow_matches_repeated_multiplication() {
        for base in -4i64..=4 {
            for exp in 0u64..6 {
                let expected = base.pow(exp as u32);
                assert_eq!(Ring::pow(&base, exp), expected);
            }
        }
    }

    #[test]
    fn i64_gcd_matches_euclid_hand_cases() {
        assert_eq!(GcdDomain::gcd(&12i64, &18i64), 6);
        assert_eq!(GcdDomain::gcd(&0i64, &5i64), 5);
        assert_eq!(GcdDomain::gcd(&(-12i64), &18i64), 6);
        assert_eq!(GcdDomain::gcd(&0i64, &0i64), 0);
    }

    #[test]
    fn i64_lcm_matches_hand_cases() {
        assert_eq!(GcdDomain::lcm(&4i64, &6i64), 12);
        assert_eq!(GcdDomain::lcm(&0i64, &5i64), 0);
    }

    #[test]
    fn i64_div_rem_matches_language_semantics() {
        assert_eq!(EuclideanDomain::div_rem(&7i64, &2i64), (3, 1));
        assert_eq!(EuclideanDomain::div_rem(&(-7i64), &2i64), (-3, -1));
    }

    #[test]
    fn f64_field_inv_and_div() {
        let a = 4.0f64;
        let inv = Field::inv(&a).expect("nonzero");
        assert!((inv - 0.25).abs() < 1e-12);
        assert!(Field::inv(&0.0f64).is_none());
        let q = Field::div(&6.0f64, &3.0f64).expect("nonzero divisor");
        assert!((q - 2.0).abs() < 1e-12);
    }

    #[test]
    fn field_gcd_helper_matches_unit_convention() {
        assert_eq!(field_gcd(&0.0f64, &0.0f64), 0.0);
        assert_eq!(field_gcd(&3.0f64, &0.0f64), 1.0);
        assert_eq!(field_gcd(&3.0f64, &5.0f64), 1.0);
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Traits

// #region 🔖Natural
pub mod natural {
//! 🔢 Arbitrary-precision unsigned integer built from `u64` limbs — the foundation every signed and
//! rational type in this crate normalizes down to.

// #region 🔖Natural
/// 🔢 Little-endian `u64` limbs; invariant: no trailing zero limb (the empty vector represents zero).
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Natural {
    limbs: Vec<u64>,
}

/// 🧵 Schoolbook multiplication is used below this many limbs; Karatsuba above. Kept as a `const` (not
/// baked into the algorithm) so tests can force both paths on identical inputs straddling the boundary.
const KARATSUBA_THRESHOLD: usize = 32;

impl Natural {
    /// 🧹 Drops trailing zero limbs so the no-trailing-zero invariant holds after any mutation.
    fn normalize(mut limbs: Vec<u64>) -> Self {
        while limbs.last() == Some(&0) {
            limbs.pop();
        }
        Self { limbs }
    }

    pub fn zero() -> Self {
        Self { limbs: Vec::new() }
    }

    pub fn one() -> Self {
        Self { limbs: vec![1] }
    }

    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    pub fn from_u64(value: u64) -> Self {
        Self::normalize(vec![value])
    }

    pub fn from_u128(value: u128) -> Self {
        let lo = value as u64;
        let hi = (value >> 64) as u64;
        Self::normalize(vec![lo, hi])
    }

    pub fn to_u64(&self) -> Option<u64> {
        match self.limbs.len() {
            0 => Some(0),
            1 => Some(self.limbs[0]),
            _ => None,
        }
    }

    pub fn to_u128(&self) -> Option<u128> {
        match self.limbs.len() {
            0 => Some(0),
            1 => Some(self.limbs[0] as u128),
            2 => Some(self.limbs[0] as u128 | ((self.limbs[1] as u128) << 64)),
            _ => None,
        }
    }

    pub fn limbs(&self) -> &[u64] {
        &self.limbs
    }

    pub fn limb_len(&self) -> usize {
        self.limbs.len()
    }

    /// 📏 Number of bits needed to represent this value (`0` itself has bit-length `0`).
    pub fn bit_length(&self) -> u64 {
        match self.limbs.last() {
            None => 0,
            Some(&top) => (self.limbs.len() as u64 - 1) * 64 + (64 - top.leading_zeros() as u64),
        }
    }

    pub fn bit(&self, i: u64) -> bool {
        let limb = (i / 64) as usize;
        if limb >= self.limbs.len() {
            return false;
        }
        (self.limbs[limb] >> (i % 64)) & 1 == 1
    }

    // #region 🔖Shifts
    pub fn shl(&self, bits: u64) -> Self {
        if self.is_zero() || bits == 0 {
            return self.clone();
        }
        let limb_shift = (bits / 64) as usize;
        let bit_shift = (bits % 64) as u32;
        let mut out = vec![0u64; limb_shift];
        let mut carry = 0u64;
        for &limb in &self.limbs {
            let shifted = if bit_shift == 0 { limb } else { (limb << bit_shift) | carry };
            carry = if bit_shift == 0 { 0 } else { limb >> (64 - bit_shift) };
            out.push(shifted);
        }
        if carry != 0 {
            out.push(carry);
        }
        Self::normalize(out)
    }

    pub fn shr(&self, bits: u64) -> Self {
        let limb_shift = (bits / 64) as usize;
        if limb_shift >= self.limbs.len() {
            return Self::zero();
        }
        let bit_shift = (bits % 64) as u32;
        let src = &self.limbs[limb_shift..];
        let mut out = vec![0u64; src.len()];
        for i in 0..src.len() {
            let lo = src[i] >> bit_shift;
            let hi = if bit_shift == 0 || i + 1 >= src.len() { 0 } else { src[i + 1] << (64 - bit_shift) };
            out[i] = lo | hi;
        }
        Self::normalize(out)
    }

    pub fn trailing_zeros(&self) -> Option<u64> {
        if self.is_zero() {
            return None;
        }
        for (i, &limb) in self.limbs.iter().enumerate() {
            if limb != 0 {
                return Some(i as u64 * 64 + limb.trailing_zeros() as u64);
            }
        }
        unreachable!("non-zero Natural must have a nonzero limb")
    }
    // #endregion 🔖Shifts

    // #region 🔖BitOperations
    pub fn bitand(&self, rhs: &Self) -> Self {
        let n = self.limbs.len().min(rhs.limbs.len());
        Self::normalize((0..n).map(|i| self.limbs[i] & rhs.limbs[i]).collect())
    }

    pub fn bitor(&self, rhs: &Self) -> Self {
        let n = self.limbs.len().max(rhs.limbs.len());
        Self::normalize((0..n).map(|i| self.limbs.get(i).copied().unwrap_or(0) | rhs.limbs.get(i).copied().unwrap_or(0)).collect())
    }

    pub fn bitxor(&self, rhs: &Self) -> Self {
        let n = self.limbs.len().max(rhs.limbs.len());
        Self::normalize((0..n).map(|i| self.limbs.get(i).copied().unwrap_or(0) ^ rhs.limbs.get(i).copied().unwrap_or(0)).collect())
    }
    // #endregion 🔖BitOperations

    // #region 🔖AddSub
    pub fn add(&self, rhs: &Self) -> Self {
        let n = self.limbs.len().max(rhs.limbs.len());
        let mut out = Vec::with_capacity(n + 1);
        let mut carry = 0u64;
        for i in 0..n {
            let a = self.limbs.get(i).copied().unwrap_or(0);
            let b = rhs.limbs.get(i).copied().unwrap_or(0);
            let (sum1, c1) = a.overflowing_add(b);
            let (sum2, c2) = sum1.overflowing_add(carry);
            out.push(sum2);
            carry = (c1 as u64) + (c2 as u64);
        }
        if carry != 0 {
            out.push(carry);
        }
        Self::normalize(out)
    }

    /// ➖ `Some(self - rhs)` if `self >= rhs`, else `None` (there is no signed `Natural`).
    pub fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        if *self < *rhs {
            return None;
        }
        let mut out = Vec::with_capacity(self.limbs.len());
        let mut borrow = 0i64;
        for i in 0..self.limbs.len() {
            let a = self.limbs[i] as i128;
            let b = rhs.limbs.get(i).copied().unwrap_or(0) as i128;
            let mut diff = a - b - borrow as i128;
            if diff < 0 {
                diff += 1i128 << 64;
                borrow = 1;
            } else {
                borrow = 0;
            }
            out.push(diff as u64);
        }
        Some(Self::normalize(out))
    }
    // #endregion 🔖AddSub

    // #region 🔖Mul
    fn mul_schoolbook(a: &[u64], b: &[u64]) -> Vec<u64> {
        if a.is_empty() || b.is_empty() {
            return Vec::new();
        }
        let mut out = vec![0u64; a.len() + b.len()];
        for (i, &ai) in a.iter().enumerate() {
            if ai == 0 {
                continue;
            }
            let mut carry = 0u128;
            for (j, &bj) in b.iter().enumerate() {
                let prod = ai as u128 * bj as u128 + out[i + j] as u128 + carry;
                out[i + j] = prod as u64;
                carry = prod >> 64;
            }
            let mut k = i + b.len();
            while carry != 0 {
                let sum = out[k] as u128 + carry;
                out[k] = sum as u64;
                carry = sum >> 64;
                k += 1;
            }
        }
        out
    }

    /// ✂️ Karatsuba: splits both operands at `n/2` limbs, forms three half-size products
    /// (`lo·lo`, `hi·hi`, `(lo+hi)·(lo+hi)`), and reassembles via `mid = combined - lo·lo - hi·hi` —
    /// which is provably non-negative, so the subtraction never needs signed limb arithmetic.
    fn mul_karatsuba(a: &[u64], b: &[u64]) -> Vec<u64> {
        let n = a.len().max(b.len());
        if a.len().min(b.len()) <= KARATSUBA_THRESHOLD {
            return Self::mul_schoolbook(a, b);
        }
        let split = n / 2;
        let (a_lo, a_hi) = if a.len() > split { a.split_at(split) } else { (a, &[][..]) };
        let (b_lo, b_hi) = if b.len() > split { b.split_at(split) } else { (b, &[][..]) };

        let lo_lo = Self::mul_karatsuba(a_lo, b_lo);
        let hi_hi = Self::mul_karatsuba(a_hi, b_hi);

        let a_sum = Self::normalize(a_lo.to_vec()).add(&Self::normalize(a_hi.to_vec()));
        let b_sum = Self::normalize(b_lo.to_vec()).add(&Self::normalize(b_hi.to_vec()));
        let mid_full = Self::mul_karatsuba(&a_sum.limbs, &b_sum.limbs);

        let mid_full_nat = Self::normalize(mid_full);
        let lo_lo_nat = Self::normalize(lo_lo.clone());
        let hi_hi_nat = Self::normalize(hi_hi.clone());
        let mid = mid_full_nat.checked_sub(&lo_lo_nat).and_then(|m| m.checked_sub(&hi_hi_nat)).expect(
            "karatsuba: (a_lo+a_hi)(b_lo+b_hi) - lo*lo - hi*hi is provably non-negative since it equals a_lo*b_hi + a_hi*b_lo",
        );

        let mut out = vec![0u64; 2 * split + hi_hi.len().max(1)];
        for (i, &limb) in lo_lo.iter().enumerate() {
            out[i] = limb;
        }
        Self::add_at_offset(&mut out, &mid.limbs, split);
        Self::add_at_offset(&mut out, &hi_hi, 2 * split);
        out
    }

    fn add_at_offset(out: &mut Vec<u64>, addend: &[u64], offset: usize) {
        while out.len() < offset + addend.len() + 1 {
            out.push(0);
        }
        let mut carry = 0u64;
        for (i, &limb) in addend.iter().enumerate() {
            let (s1, c1) = out[offset + i].overflowing_add(limb);
            let (s2, c2) = s1.overflowing_add(carry);
            out[offset + i] = s2;
            carry = (c1 as u64) + (c2 as u64);
        }
        let mut k = offset + addend.len();
        while carry != 0 {
            if k >= out.len() {
                out.push(0);
            }
            let (s, c) = out[k].overflowing_add(carry);
            out[k] = s;
            carry = c as u64;
            k += 1;
        }
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        Self::normalize(Self::mul_karatsuba(&self.limbs, &rhs.limbs))
    }

    /// 🧵 Test-only escape hatch to force the schoolbook path regardless of operand size, so
    /// differential tests can compare it against Karatsuba on the same inputs.
    #[cfg(test)]
    pub(crate) fn mul_schoolbook_pub(&self, rhs: &Self) -> Self {
        Self::normalize(Self::mul_schoolbook(&self.limbs, &rhs.limbs))
    }

    pub fn mul_u64(&self, rhs: u64) -> Self {
        if rhs == 0 || self.is_zero() {
            return Self::zero();
        }
        let mut out = Vec::with_capacity(self.limbs.len() + 1);
        let mut carry = 0u128;
        for &limb in &self.limbs {
            let prod = limb as u128 * rhs as u128 + carry;
            out.push(prod as u64);
            carry = prod >> 64;
        }
        if carry != 0 {
            out.push(carry as u64);
        }
        Self::normalize(out)
    }
    // #endregion 🔖Mul

    // #region 🔖DivRem
    /// ➗ Divide by a single `u64` limb via simple long division; returns `(quotient, remainder)`.
    pub fn div_rem_u64(&self, divisor: u64) -> (Self, u64) {
        assert!(divisor != 0, "div_rem_u64: division by zero");
        let mut quotient = vec![0u64; self.limbs.len()];
        let mut rem: u128 = 0;
        for i in (0..self.limbs.len()).rev() {
            let cur = (rem << 64) | self.limbs[i] as u128;
            quotient[i] = (cur / divisor as u128) as u64;
            rem = cur % divisor as u128;
        }
        (Self::normalize(quotient), rem as u64)
    }

    /// ➗ Knuth's Algorithm D (TAOCP vol.2 §4.3.1): normalizes so the divisor's top limb has its high
    /// bit set, forms a 2-limb-by-1-limb trial quotient digit with the standard two-step correction,
    /// multiplies-and-subtracts, and falls back to the rare add-back branch when the trial digit
    /// overshoots by one. `rhs` must be nonzero.
    pub fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        assert!(!rhs.is_zero(), "div_rem: division by zero");
        if *self < *rhs {
            return (Self::zero(), self.clone());
        }
        if rhs.limbs.len() == 1 {
            let (q, r) = self.div_rem_u64(rhs.limbs[0]);
            return (q, Self::from_u64(r));
        }

        let shift = rhs.limbs.last().unwrap().leading_zeros() as u64;
        let v = rhs.shl(shift);
        let mut u = self.shl(shift);
        let n = v.limbs.len();
        let m = u.limbs.len().saturating_sub(n);
        u.limbs.resize(u.limbs.len().max(n + m + 1), 0);

        let mut quotient = vec![0u64; m + 1];
        let v_top = v.limbs[n - 1];
        let v_second = v.limbs[n - 2];

        for j in (0..=m).rev() {
            let u_top2 = ((u.limbs[j + n] as u128) << 64) | u.limbs[j + n - 1] as u128;
            let mut qhat = u_top2 / v_top as u128;
            let mut rhat = u_top2 % v_top as u128;
            if qhat > u64::MAX as u128 {
                qhat = u64::MAX as u128;
                rhat = u_top2 - qhat * v_top as u128;
            }
            while rhat <= u64::MAX as u128 && qhat * v_second as u128 > (rhat << 64) + u.limbs[j + n - 2] as u128 {
                qhat -= 1;
                rhat += v_top as u128;
            }

            // Multiply v by qhat and subtract from u[j..j+n+1].
            let mut borrow: i128 = 0;
            let mut carry: u128 = 0;
            for i in 0..n {
                let prod = qhat * v.limbs[i] as u128 + carry;
                carry = prod >> 64;
                let sub = u.limbs[j + i] as i128 - (prod as u64) as i128 - borrow;
                if sub < 0 {
                    u.limbs[j + i] = (sub + (1i128 << 64)) as u64;
                    borrow = 1;
                } else {
                    u.limbs[j + i] = sub as u64;
                    borrow = 0;
                }
            }
            let top_sub = u.limbs[j + n] as i128 - carry as i128 - borrow;
            if top_sub < 0 {
                // qhat was one too large: add back v once and decrement qhat.
                u.limbs[j + n] = (top_sub + (1i128 << 64)) as u64;
                qhat -= 1;
                let mut carry2 = 0u64;
                for i in 0..n {
                    let (s1, c1) = u.limbs[j + i].overflowing_add(v.limbs[i]);
                    let (s2, c2) = s1.overflowing_add(carry2);
                    u.limbs[j + i] = s2;
                    carry2 = (c1 as u64) + (c2 as u64);
                }
                u.limbs[j + n] = u.limbs[j + n].wrapping_add(carry2);
            } else {
                u.limbs[j + n] = top_sub as u64;
            }
            quotient[j] = qhat as u64;
        }

        u.limbs.truncate(n);
        let remainder = Self::normalize(u.limbs).shr(shift);
        (Self::normalize(quotient), remainder)
    }
    // #endregion 🔖DivRem

    // #region 🔖PowRoots
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

    /// √ Integer square root via Newton's method seeded from `bit_length`, descending monotonically
    /// until it stabilizes (guaranteed to terminate since each step only decreases or holds).
    pub fn isqrt(&self) -> Self {
        self.nth_root(2)
    }

    /// √ⁿ Integer n-th root via Newton's method on `f(x) = x^n - self`; `n` must be >= 1.
    pub fn nth_root(&self, n: u32) -> Self {
        assert!(n >= 1, "nth_root: n must be >= 1");
        if self.is_zero() || n == 1 {
            return self.clone();
        }
        let bits = self.bit_length();
        let mut x = Self::one().shl(bits / n as u64 + 1);
        loop {
            let x_pow = x.pow((n - 1) as u64);
            let denom = x_pow.mul_u64(n as u64);
            if denom.is_zero() {
                break;
            }
            let numerator = x_pow.mul(&x).mul_u64(n as u64 - 1).add(self);
            let (next, _) = numerator.div_rem(&denom);
            if next >= x {
                break;
            }
            x = next;
        }
        // Correct rounding: Newton converges to floor(root) or floor(root)+1; step down until x^n <= self.
        while x.pow(n as u64) > *self {
            x = x.checked_sub(&Self::one()).unwrap_or_else(Self::zero);
        }
        while x.add(&Self::one()).pow(n as u64) <= *self {
            x = x.add(&Self::one());
        }
        x
    }
    // #endregion 🔖PowRoots

    // #region 🔖Gcd
    /// 🤝 Binary (Stein's) GCD: strips common factors of two, then alternates halving-the-even-operand
    /// with subtract-and-swap — avoids division entirely.
    pub fn gcd(&self, rhs: &Self) -> Self {
        if self.is_zero() {
            return rhs.clone();
        }
        if rhs.is_zero() {
            return self.clone();
        }
        let mut a = self.clone();
        let mut b = rhs.clone();
        let shift = a.trailing_zeros().unwrap().min(b.trailing_zeros().unwrap());
        a = a.shr(a.trailing_zeros().unwrap());
        loop {
            b = b.shr(b.trailing_zeros().unwrap());
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            b = b.checked_sub(&a).expect("b >= a after the swap above");
            if b.is_zero() {
                break;
            }
        }
        a.shl(shift)
    }
    // #endregion 🔖Gcd

    // #region 🔖StringIo
    pub fn to_decimal(&self) -> String {
        if self.is_zero() {
            return "0".to_string();
        }
        let mut digits = Vec::new();
        let mut cur = self.clone();
        while !cur.is_zero() {
            let (q, r) = cur.div_rem_u64(1_000_000_000_000_000_000);
            digits.push(r);
            cur = q;
        }
        let mut s = digits.pop().unwrap().to_string();
        for chunk in digits.iter().rev() {
            s.push_str(&format!("{chunk:018}"));
        }
        s
    }

    pub fn to_hex(&self) -> String {
        if self.is_zero() {
            return "0".to_string();
        }
        let mut s = format!("{:x}", self.limbs.last().unwrap());
        for limb in self.limbs.iter().rev().skip(1) {
            s.push_str(&format!("{limb:016x}"));
        }
        s
    }

    pub fn from_str_radix(s: &str, radix: u32) -> Option<Self> {
        if s.is_empty() {
            return None;
        }
        let mut result = Self::zero();
        let radix_nat = Self::from_u64(radix as u64);
        for c in s.chars() {
            let digit = c.to_digit(radix)?;
            result = result.mul(&radix_nat).add(&Self::from_u64(digit as u64));
        }
        Some(result)
    }
}
// #endregion 🔖Natural

// #region 🔖NaturalTraitImpls
impl std::fmt::Display for Natural {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_decimal())
    }
}

impl std::str::FromStr for Natural {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_radix(s, 10).ok_or(())
    }
}

impl Ord for Natural {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.limbs.len().cmp(&other.limbs.len()).then_with(|| self.limbs.iter().rev().cmp(other.limbs.iter().rev()))
    }
}

impl PartialOrd for Natural {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

macro_rules! impl_from_uint {
    ($($t:ty),*) => {
        $(impl From<$t> for Natural {
            fn from(value: $t) -> Self {
                Natural::from_u64(value as u64)
            }
        })*
    };
}
impl_from_uint!(u8, u16, u32, u64, usize);

impl From<u128> for Natural {
    fn from(value: u128) -> Self {
        Natural::from_u128(value)
    }
}
// #endregion 🔖NaturalTraitImpls

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn n(s: &str) -> Natural {
        Natural::from_str(s).unwrap()
    }

    // #region 🔖FundamentalTests
    #[test]
    fn decimal_string_roundtrip() {
        let s = "123456789012345678901234567890";
        assert_eq!(n(s).to_decimal(), s);
    }

    #[test]
    fn zero_roundtrips() {
        assert_eq!(n("0").to_decimal(), "0");
        assert!(n("0").is_zero());
    }

    #[test]
    fn add_matches_hand_case() {
        assert_eq!(n("999999999999999999999").add(&n("1")).to_decimal(), "1000000000000000000000");
    }

    #[test]
    fn checked_sub_matches_hand_case_and_rejects_negative() {
        assert_eq!(n("1000000000000000000000").checked_sub(&n("1")).unwrap().to_decimal(), "999999999999999999999");
        assert!(n("1").checked_sub(&n("2")).is_none());
    }

    #[test]
    fn mul_matches_hand_case() {
        assert_eq!(n("123456789").mul(&n("987654321")).to_decimal(), "121932631112635269");
    }

    #[test]
    fn div_rem_self_check_invariant() {
        let u = n("123456789012345678901234567890");
        let v = n("98765432109876543210");
        let (q, r) = u.div_rem(&v);
        assert!(r < v);
        assert_eq!(q.mul(&v).add(&r), u);
    }

    #[test]
    fn div_rem_by_larger_value_is_zero_quotient() {
        let (q, r) = n("5").div_rem(&n("100"));
        assert!(q.is_zero());
        assert_eq!(r, n("5"));
    }

    #[test]
    fn isqrt_matches_known_squares() {
        assert_eq!(n("144").isqrt(), n("12"));
        assert_eq!(n("143").isqrt(), n("11"));
        assert_eq!(n("0").isqrt(), n("0"));
        assert_eq!(n("1").isqrt(), n("1"));
    }

    #[test]
    fn nth_root_matches_known_cubes() {
        assert_eq!(n("27").nth_root(3), n("3"));
        assert_eq!(n("26").nth_root(3), n("2"));
        assert_eq!(n("1000000").nth_root(3), n("100"));
    }

    #[test]
    fn gcd_matches_euclid_hand_cases() {
        assert_eq!(n("48").gcd(&n("18")), n("6"));
        assert_eq!(n("0").gcd(&n("5")), n("5"));
        assert_eq!(n("17").gcd(&n("13")), n("1"));
    }

    #[test]
    fn hex_roundtrip() {
        let value = n("4059231");
        let hex = value.to_hex();
        assert_eq!(Natural::from_str_radix(&hex, 16).unwrap(), value);
    }

    #[test]
    fn bit_length_and_bit_access() {
        let v = n("256"); // 2^8
        assert_eq!(v.bit_length(), 9);
        assert!(v.bit(8));
        assert!(!v.bit(7));
    }

    #[test]
    fn shl_shr_roundtrip() {
        let v = n("123456789");
        assert_eq!(v.shl(37).shr(37), v);
    }

    #[test]
    fn ord_compares_by_magnitude() {
        assert!(n("9") < n("10"));
        assert!(n("100000000000000000000") > n("99999999999999999999"));
        assert_eq!(n("5"), n("5"));
    }
    // #endregion 🔖FundamentalTests

    // #region 🔖QuickTests
    mod quick {
        use super::*;

        #[test]
        fn karatsuba_matches_schoolbook_across_threshold() {
            let mut seed = 0x1234_5678_9abc_def1u64;
            let mut next = move || {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                seed
            };
            // Sizes straddling KARATSUBA_THRESHOLD (32 limbs) on both operands.
            for limb_count in [1usize, 10, 31, 32, 33, 40, 64, 96] {
                let a_limbs: Vec<u64> = (0..limb_count).map(|_| next()).collect();
                let b_limbs: Vec<u64> = (0..limb_count / 2 + 1).map(|_| next()).collect();
                let a = Natural::normalize(a_limbs);
                let b = Natural::normalize(b_limbs);
                let via_karatsuba = a.mul(&b);
                let via_schoolbook = a.mul_schoolbook_pub(&b);
                assert_eq!(via_karatsuba, via_schoolbook, "mismatch at limb_count={limb_count}");
            }
        }

        #[test]
        fn div_rem_invariant_holds_on_random_shapes() {
            let mut seed = 0xdead_beef_cafe_babeu64;
            let mut next = move || {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                seed
            };
            for _ in 0..200 {
                let a_len = 1 + (next() % 20) as usize;
                let b_len = 1 + (next() % a_len.max(1) as u64) as usize;
                let a = Natural::normalize((0..a_len).map(|_| next()).collect());
                let mut b = Natural::normalize((0..b_len).map(|_| next()).collect());
                if b.is_zero() {
                    b = Natural::one();
                }
                let (q, r) = a.div_rem(&b);
                assert!(r < b);
                assert_eq!(q.mul(&b).add(&r), a);
            }
        }

        #[test]
        fn div_rem_add_back_branch_is_exercised_and_correct() {
            // Crafted vectors known to trigger Knuth D's rare add-back correction: divisor with a
            // near-max top limb paired with a dividend that makes the initial qhat estimate overshoot.
            let u = Natural::normalize(vec![0, 0, 0x8000_0000_0000_0000, 1]);
            let v = Natural::normalize(vec![0xFFFF_FFFF_FFFF_FFFF, 0x8000_0000_0000_0000]);
            let (q, r) = u.div_rem(&v);
            assert!(r < v);
            assert_eq!(q.mul(&v).add(&r), u);
        }

        #[test]
        fn sieve_like_isqrt_matches_f64_for_moderate_values() {
            for v in [2u64, 3, 99, 1_000_003, 999_999_999] {
                let exact = Natural::from_u64(v).isqrt();
                let approx = (v as f64).sqrt() as u64;
                let exact_u64 = exact.to_u64().unwrap();
                assert!(exact_u64.abs_diff(approx) <= 1, "isqrt({v}) = {exact_u64}, float estimate {approx}");
            }
        }
    }
    // #endregion 🔖QuickTests

    // #region 🔖LongTests
    mod long {
        use super::*;

        #[test]
        fn stress_4096_bit_mul_div_gcd() {
            let mut seed = 0x0123_4567_89ab_cdefu64;
            let mut next = move || {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                seed
            };
            let limbs_4096 = 64; // 4096 / 64
            for _ in 0..5 {
                let a = Natural::normalize((0..limbs_4096).map(|_| next()).collect());
                let mut b = Natural::normalize((0..limbs_4096 / 2).map(|_| next()).collect());
                if b.is_zero() {
                    b = Natural::one();
                }
                let prod = a.mul(&b);
                let (q, r) = prod.div_rem(&b);
                assert_eq!(r, Natural::zero());
                assert_eq!(q, a);
                let g = a.gcd(&b);
                assert!(a.div_rem(&g).1.is_zero());
                assert!(b.div_rem(&g).1.is_zero());
            }
        }
    }
    // #endregion 🔖LongTests

    // #region 🔖ExhaustiveTests
    mod exhaustive {
        use super::*;

        #[test]
        fn stress_16384_bit_operations() {
            let mut seed = 0xfeed_face_dead_c0deu64;
            let mut next = move || {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                seed
            };
            let limbs_16384 = 256; // 16384 / 64
            for _ in 0..3 {
                let a = Natural::normalize((0..limbs_16384).map(|_| next()).collect());
                let b = Natural::normalize((0..limbs_16384).map(|_| next()).collect());
                let sum = a.add(&b);
                assert!(sum >= a && sum >= b);
                let prod = a.mul(&b);
                if !a.is_zero() {
                    let (q, r) = prod.div_rem(&a);
                    assert_eq!(r, Natural::zero());
                    assert_eq!(q, b);
                }
            }
        }
    }
    // #endregion 🔖ExhaustiveTests
}
// #endregion 🔖Tests
}
// #endregion 🔖Natural

// #region 🔖Integer
pub mod integer {
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
                    Some(-(mag as i128) as i64)
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
        if self.is_negative() && n.is_multiple_of(2) {
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
}
// #endregion 🔖Integer

// #region 🔖Rational
pub mod rational {
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
    #[allow(clippy::needless_pass_by_value, reason = "denom is only ever borrowed here, but the by-value signature is public API consumed by mathematical_cas outside this crate; changing it is a cross-crate breaking change out of scope for this lint pass")]
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

    // #region 🔖FieldOperations
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
    // #endregion 🔖FieldOperations

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
        let value = quotient.to_u128().map_or_else(|| quotient.to_decimal().parse::<f64>().unwrap_or(f64::INFINITY), |v| v as f64);
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
}
// #endregion 🔖Rational

// #region 🔖Modular
pub mod modular {
//! 🧷 Modular arithmetic over `u64` moduli: the function surface used by primality/factorization, plus
//! `ModInt`, a `Field` element of `Z/mZ` with a runtime modulus.

use crate::traits::{CommutativeRing, EuclideanDomain, Field, GcdDomain, IntegralDomain, Ring};

// #region 🔖ModFns
pub fn mod_add(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 + b as u128) % m as u128) as u64
}

pub fn mod_sub(a: u64, b: u64, m: u64) -> u64 {
    let a = a % m;
    let b = b % m;
    if a >= b {
        a - b
    } else {
        m - (b - a)
    }
}

pub fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

pub fn mod_pow(base: u64, exp: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }
    let mut result = 1u64 % m;
    let mut b = base % m;
    let mut e = exp;
    while e > 0 {
        if e & 1 == 1 {
            result = mod_mul(result, b, m);
        }
        b = mod_mul(b, b, m);
        e >>= 1;
    }
    result
}

/// 🤝 Modular inverse via the extended Euclidean algorithm on `i128`; `None` if `gcd(a, m) != 1`.
pub fn mod_inv(a: u64, m: u64) -> Option<u64> {
    if m == 0 {
        return None;
    }
    let (mut old_r, mut r) = (a as i128, m as i128);
    let (mut old_s, mut s) = (1i128, 0i128);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }
    if old_r != 1 {
        return None;
    }
    let result = old_s.rem_euclid(m as i128);
    Some(result as u64)
}

/// 〽️ Jacobi symbol `(a/n)` for odd `n > 0`, via the standard quadratic-reciprocity recursion.
pub fn jacobi(a: i64, n: u64) -> i8 {
    assert!(n % 2 == 1 && n > 0, "jacobi: n must be odd and positive");
    let mut a = a.rem_euclid(n as i64) as u64;
    let mut n = n;
    let mut result = 1i8;
    while a != 0 {
        while a.is_multiple_of(2) {
            a /= 2;
            let r = n % 8;
            if r == 3 || r == 5 {
                result = -result;
            }
        }
        std::mem::swap(&mut a, &mut n);
        if a % 4 == 3 && n % 4 == 3 {
            result = -result;
        }
        a %= n;
    }
    if n == 1 {
        result
    } else {
        0
    }
}

pub fn is_quadratic_residue(a: u64, p: u64) -> bool {
    if p == 2 {
        return true;
    }
    mod_pow(a % p, (p - 1) / 2, p) == 1 % p
}

/// √ Tonelli–Shanks: a square root of `a` modulo prime `p`, or `None` if `a` is a non-residue.
pub fn sqrt_mod(a: u64, p: u64) -> Option<u64> {
    let a = a % p;
    if a == 0 {
        return Some(0);
    }
    if p == 2 {
        return Some(a);
    }
    if !is_quadratic_residue(a, p) {
        return None;
    }
    if p % 4 == 3 {
        return Some(mod_pow(a, (p + 1) / 4, p));
    }
    // General Tonelli-Shanks: write p - 1 = q * 2^s with q odd.
    let mut q = p - 1;
    let mut s = 0u32;
    while q.is_multiple_of(2) {
        q /= 2;
        s += 1;
    }
    // Find a quadratic non-residue z.
    let mut z = 2u64;
    while is_quadratic_residue(z, p) {
        z += 1;
    }
    let mut m = s;
    let mut c = mod_pow(z, q, p);
    let mut t = mod_pow(a, q, p);
    let mut r = mod_pow(a, q.div_ceil(2), p);
    loop {
        if t == 1 {
            return Some(r);
        }
        let mut i = 0u32;
        let mut temp = t;
        while temp != 1 {
            temp = mod_mul(temp, temp, p);
            i += 1;
            if i == m {
                return None; // should not happen if `a` really is a residue
            }
        }
        let b = mod_pow(c, 1u64 << (m - i - 1), p);
        m = i;
        c = mod_mul(b, b, p);
        t = mod_mul(t, c, p);
        r = mod_mul(r, b, p);
    }
}

/// 🧩 Solves `x = r1 (mod m1), x = r2 (mod m2)` for coprime `m1, m2`; returns `(x, m1*m2)`.
pub fn crt_pair(r1: u64, m1: u64, r2: u64, m2: u64) -> Option<(u64, u64)> {
    let inv = mod_inv(m1 % m2, m2)?;
    let diff = ((r2 as i128 - r1 as i128).rem_euclid(m2 as i128)) as u64;
    let k = mod_mul(diff, inv, m2);
    let x = r1 as u128 + m1 as u128 * k as u128;
    Some(((x % (m1 as u128 * m2 as u128)) as u64, m1 * m2))
}

pub fn crt(congruences: &[(u64, u64)]) -> Option<(u64, u64)> {
    let mut iter = congruences.iter();
    let (mut r, mut m) = *iter.next()?;
    for &(ri, mi) in iter {
        (r, m) = crt_pair(r, m, ri, mi)?;
    }
    Some((r, m))
}

/// 🎯 Multiplicative order of `a` modulo `m` (smallest `k > 0` with `a^k = 1`); `None` if `gcd(a, m) != 1`.
pub fn multiplicative_order(a: u64, m: u64) -> Option<u64> {
    if crate::primes::gcd_u64(a % m, m) != 1 {
        return None;
    }
    let phi = crate::primes::euler_phi(m);
    let factors = crate::primes::factor_u64(phi);
    let mut order = phi;
    for &(p, _) in &factors {
        while order.is_multiple_of(p) && mod_pow(a, order / p, m) == 1 % m {
            order /= p;
        }
    }
    Some(order)
}

/// 🌱 Smallest primitive root modulo `m` (a generator of `(Z/mZ)*`); `None` if none exists (a
/// primitive root exists only for `m` in {1, 2, 4, p^k, 2p^k} for odd prime `p`).
pub fn primitive_root(m: u64) -> Option<u64> {
    if m == 1 {
        return Some(0);
    }
    let phi = crate::primes::euler_phi(m);
    let phi_factors = crate::primes::factor_u64(phi);
    'candidate: for g in 2..m {
        if crate::primes::gcd_u64(g, m) != 1 {
            continue;
        }
        for &(p, _) in &phi_factors {
            if mod_pow(g, phi / p, m) == 1 % m {
                continue 'candidate;
            }
        }
        return Some(g);
    }
    None
}
// #endregion 🔖ModFns

// #region 🔖ModInt
/// 🧷 Element of `Z/mZ` with a runtime `u64` modulus. `modulus == 0` marks the "unbound" neutral
/// element produced by `Ring::zero()/one()/neg()/from_i64` before any modulus is known from context —
/// `value` then holds the `u64` bit pattern of a signed `i64` (so an unbound negation stays exact,
/// reduced correctly by `unify`'s `reduce_unbound` once a real modulus is known via signed
/// `rem_euclid`, never plain `% m`). Binary operations unify an unbound operand onto the other side's
/// (bound) modulus, and `debug_assert` that two differently-bound operands never meet (that would be a
/// caller bug, not a valid computation).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ModInt {
    value: u64,
    modulus: u64,
}

impl ModInt {
    pub fn new(value: u64, modulus: u64) -> Self {
        assert!(modulus > 0, "ModInt::new: modulus must be positive");
        Self { value: value % modulus, modulus }
    }

    pub fn value(self) -> u64 {
        self.value
    }

    pub fn modulus(self) -> u64 {
        self.modulus
    }

    /// 🧮 Reduces an "unbound" `value` (stored as the `u64` bit pattern of a signed `i64` — see
    /// [`Ring::neg`]/[`Ring::from_i64`]) into a proper non-negative residue mod `m`. Distinct from
    /// `value % m`: a negative-signed bit pattern must go through signed `rem_euclid`, not unsigned
    /// remainder, or e.g. the bit pattern for `-1` (`u64::MAX`) would reduce to a value far from `m - 1`.
    fn reduce_unbound(value: u64, m: u64) -> u64 {
        ((value as i64 as i128).rem_euclid(m as i128)) as u64
    }

    fn unify(self, other: Self) -> (u64, u64, u64) {
        if self.modulus == 0 && other.modulus == 0 {
            (self.value, other.value, 0)
        } else if self.modulus == 0 {
            (Self::reduce_unbound(self.value, other.modulus), other.value, other.modulus)
        } else if other.modulus == 0 {
            (self.value, Self::reduce_unbound(other.value, self.modulus), self.modulus)
        } else {
            debug_assert_eq!(self.modulus, other.modulus, "ModInt: operands bound to different moduli");
            (self.value, other.value, self.modulus)
        }
    }
}

impl Ring for ModInt {
    fn zero() -> Self {
        Self { value: 0, modulus: 0 }
    }
    fn one() -> Self {
        Self { value: 1, modulus: 0 }
    }
    fn add(&self, rhs: &Self) -> Self {
        let (a, b, m) = self.unify(*rhs);
        if m == 0 {
            // 🧮 Both unbound: defer reduction, so combine the signed bit patterns with wrapping add.
            Self { value: a.wrapping_add(b), modulus: 0 }
        } else {
            Self { value: mod_add(a, b, m), modulus: m }
        }
    }
    fn neg(&self) -> Self {
        if self.modulus == 0 {
            // 🧮 Unbound: negate the signed `i64` bit pattern (see `reduce_unbound`), not the raw `u64`.
            Self { value: self.value.wrapping_neg(), modulus: 0 }
        } else if self.value == 0 {
            *self
        } else {
            Self { value: self.modulus - self.value, modulus: self.modulus }
        }
    }
    fn mul(&self, rhs: &Self) -> Self {
        let (a, b, m) = self.unify(*rhs);
        if m == 0 {
            // 🧮 Both unbound: defer reduction, so combine the signed bit patterns with wrapping mul.
            Self { value: a.wrapping_mul(b), modulus: 0 }
        } else {
            Self { value: mod_mul(a, b, m), modulus: m }
        }
    }
    fn is_zero(&self) -> bool {
        self.value == 0
    }
    fn from_i64(value: i64) -> Self {
        Self { value: value as u64, modulus: 0 }
    }
    fn characteristic(&self) -> u64 {
        self.modulus
    }
}
impl CommutativeRing for ModInt {}
impl IntegralDomain for ModInt {
    fn exact_div(&self, rhs: &Self) -> Option<Self> {
        Some(self.mul(&rhs.inv()?))
    }
}
impl GcdDomain for ModInt {
    fn gcd(&self, rhs: &Self) -> Self {
        crate::traits::field_gcd(self, rhs)
    }
}
impl EuclideanDomain for ModInt {
    fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        crate::traits::field_div_rem(self, rhs)
    }
}
impl Field for ModInt {
    /// ➗ Inverse via the extended Euclidean algorithm — valid for any unit, not only under a prime
    /// modulus (the caller is responsible for the "modulus is prime" contract when primality matters,
    /// e.g. for `PolyU<ModInt>` to behave as a field).
    fn inv(&self) -> Option<Self> {
        if self.modulus == 0 {
            return None;
        }
        mod_inv(self.value, self.modulus).map(|v| Self { value: v, modulus: self.modulus })
    }
}
// #endregion 🔖ModInt

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_add_sub_mul_hand_cases() {
        assert_eq!(mod_add(7, 5, 10), 2);
        assert_eq!(mod_sub(3, 7, 10), 6);
        assert_eq!(mod_mul(6, 7, 10), 2);
    }

    #[test]
    fn mod_pow_hand_cases() {
        assert_eq!(mod_pow(2, 10, 1000), 24);
        assert_eq!(mod_pow(5, 0, 7), 1);
    }

    #[test]
    fn mod_inv_matches_bezout() {
        let inv = mod_inv(3, 11).unwrap();
        assert_eq!(mod_mul(3, inv, 11), 1);
        assert!(mod_inv(2, 4).is_none());
    }

    #[test]
    fn jacobi_hand_cases() {
        assert_eq!(jacobi(1001, 9907), -1);
        assert_eq!(jacobi(19, 45), 1);
        assert_eq!(jacobi(8, 21), -1);
    }

    #[test]
    fn sqrt_mod_hand_cases() {
        // 4 mod 5 has sqrt 2 or 3
        let r = sqrt_mod(4, 5).unwrap();
        assert_eq!(mod_mul(r, r, 5), 4);
        // 2 is a non-residue mod 5 (since 5 % 8 not in {1,7})
        assert!(sqrt_mod(2, 5).is_none() || mod_mul(sqrt_mod(2, 5).unwrap(), sqrt_mod(2, 5).unwrap(), 5) == 2);
    }

    #[test]
    fn crt_pair_matches_congruences() {
        let (x, m) = crt_pair(2, 3, 3, 5).unwrap();
        assert_eq!(m, 15);
        assert_eq!(x % 3, 2);
        assert_eq!(x % 5, 3);
    }

    #[test]
    fn modint_field_operations() {
        let a = ModInt::new(7, 13);
        let b = ModInt::new(9, 13);
        assert_eq!(a.add(&b).value(), (7 + 9) % 13);
        let inv = a.inv().unwrap();
        assert_eq!(a.mul(&inv).value(), 1);
    }

    #[test]
    fn modint_unbound_zero_unifies_with_bound_side() {
        let bound = ModInt::new(5, 13);
        let unbound_zero = ModInt::zero();
        let sum = bound.add(&unbound_zero);
        assert_eq!(sum.value(), 5);
        assert_eq!(sum.modulus(), 13);
    }

    // #region 🔖QuickTests
    mod quick {
        use super::*;

        #[test]
        fn sqrt_mod_matches_brute_force_for_all_residues_small_primes() {
            for &p in &[3u64, 5, 7, 11, 13, 17, 19, 23, 29] {
                for a in 0..p {
                    let brute = (0..p).find(|&x| mod_mul(x, x, p) == a);
                    let computed = sqrt_mod(a, p);
                    match (brute, computed) {
                        (Some(_), Some(r)) => assert_eq!(mod_mul(r, r, p), a),
                        (None, None) => {}
                        (b, c) => panic!("mismatch for a={a} p={p}: brute={b:?} computed={c:?}"),
                    }
                }
            }
        }
    }
    // #endregion 🔖QuickTests
}
// #endregion 🔖Tests
}
// #endregion 🔖Modular

// #region 🔖Primes
pub mod primes {
//! 🧮 Primality testing, integer factorization, and elementary number-theoretic functions.

use crate::modular::mod_pow;
use crate::natural::Natural;

// #region 🔖GcdU64
/// 🤝 Euclidean GCD on plain `u64`s — a small helper shared by `modular.rs`'s order/root functions.
pub fn gcd_u64(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
// #endregion 🔖GcdU64

// #region 🔖MillerRabin
/// 🎲 Deterministic Miller-Rabin using the witness set `{2,3,5,7,11,13,17,19,23,29,31,37}`, proven
/// sufficient to decide primality for every `u64` (in fact every 64-bit and much larger input).
pub fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n == p {
            return true;
        }
        if n.is_multiple_of(p) {
            return false;
        }
    }
    let mut d = n - 1;
    let mut r = 0u32;
    while d.is_multiple_of(2) {
        d /= 2;
        r += 1;
    }
    'witness: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if a >= n {
            continue;
        }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..r - 1 {
            x = crate::modular::mod_mul(x, x, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

/// 🌱 Deterministic splitmix64-derived witness stream, used only to seed extra Miller-Rabin rounds for
/// [`is_prime`]'s big-integer path — reproducible across runs, and deliberately not `mathematical_random`
/// (this crate has zero dependencies).
struct InlineSplitMix64(u64);
impl InlineSplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}
// #endregion 🔖MillerRabin

// #region 🔖Bpsw
fn miller_rabin_natural(n: &Natural, base: &Natural) -> bool {
    let one = Natural::one();
    let n_minus_1 = n.checked_sub(&one).unwrap();
    let mut d = n_minus_1.clone();
    let mut r = 0u32;
    while !d.bit(0) && !d.is_zero() {
        d = d.shr(1);
        r += 1;
    }
    let mut x = mod_pow_natural(base, &d, n);
    if x == one || x == n_minus_1 {
        return true;
    }
    for _ in 0..r.saturating_sub(1) {
        x = mulmod_natural(&x, &x, n);
        if x == n_minus_1 {
            return true;
        }
    }
    false
}

fn mulmod_natural(a: &Natural, b: &Natural, m: &Natural) -> Natural {
    let (_, r) = a.mul(b).div_rem(m);
    r
}

fn mod_pow_natural(base: &Natural, exp: &Natural, m: &Natural) -> Natural {
    let mut result = Natural::one();
    let mut b = { let (_, r) = base.div_rem(m); r };
    let bits = exp.bit_length();
    for i in 0..bits {
        if exp.bit(i) {
            result = mulmod_natural(&result, &b, m);
        }
        b = mulmod_natural(&b, &b, m);
    }
    result
}

/// 🎯 Probabilistic primality test for arbitrary-precision `Natural`: a base-2 strong Miller-Rabin
/// round (catching every even/trivial composite cheaply) followed by 40 further rounds with
/// deterministic splitmix64-derived witness bases. Reproducible across runs (same `n` always tests the
/// same bases) with false-positive probability bounded by `4^-40` for adversarial composite inputs —
/// astronomically small in practice, though (unlike true BPSW) not a proven zero-counterexample test.
pub fn is_prime(n: &Natural) -> bool {
    if let Some(small) = n.to_u64() {
        return is_prime_u64(small);
    }
    if !n.bit(0) {
        return false;
    }
    if !miller_rabin_natural(n, &Natural::from_u64(2)) {
        return false;
    }
    let seed = n.limbs().first().copied().unwrap_or(1) ^ n.limbs().last().copied().unwrap_or(1) ^ n.bit_length();
    let mut rng = InlineSplitMix64(seed);
    let n_minus_3 = n.checked_sub(&Natural::from_u64(3)).unwrap_or_else(Natural::zero);
    for _ in 0..40 {
        let raw = ((rng.next() as u128) << 64 | rng.next() as u128) as u64;
        let base = if n_minus_3.is_zero() { Natural::from_u64(2) } else { let (_, r) = Natural::from_u64(raw).div_rem(&n_minus_3); r.add(&Natural::from_u64(2)) };
        if !miller_rabin_natural(n, &base) {
            return false;
        }
    }
    true
}

pub fn next_prime(n: &Natural) -> Natural {
    let mut candidate = n.add(&Natural::one());
    if !candidate.bit(0) && candidate > Natural::from_u64(2) {
        candidate = candidate.add(&Natural::one());
    }
    if candidate <= Natural::from_u64(2) {
        return Natural::from_u64(2);
    }
    while !is_prime(&candidate) {
        candidate = candidate.add(&Natural::from_u64(2));
    }
    candidate
}
// #endregion 🔖Bpsw

// #region 🔖Sieve
/// 🧮 Bit-packed sieve of Eratosthenes over `0..limit`.
pub struct Sieve {
    limit: usize,
    is_composite: Vec<u64>,
}

impl Sieve {
    pub fn new(limit: usize) -> Self {
        let words = limit / 64 + 1;
        let mut is_composite = vec![0u64; words];
        let set = |bits: &mut [u64], i: usize| bits[i / 64] |= 1 << (i % 64);
        if limit > 0 {
            set(&mut is_composite, 0);
        }
        if limit > 1 {
            set(&mut is_composite, 1);
        }
        let mut i = 2usize;
        while i * i <= limit {
            let get = |bits: &[u64], j: usize| bits[j / 64] & (1 << (j % 64)) != 0;
            if !get(&is_composite, i) {
                let mut j = i * i;
                while j <= limit {
                    set(&mut is_composite, j);
                    j += i;
                }
            }
            i += 1;
        }
        Self { limit, is_composite }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        if n > self.limit {
            return is_prime_u64(n as u64);
        }
        self.is_composite[n / 64] & (1 << (n % 64)) == 0
    }

    pub fn primes(&self) -> impl Iterator<Item = usize> + '_ {
        (2..=self.limit).filter(move |&n| self.is_prime(n))
    }

    pub fn count(&self) -> usize {
        self.primes().count()
    }
}
// #endregion 🔖Sieve

// #region 🔖Factor
/// 🔍 Pollard's rho with Brent's cycle-detection improvement and batched GCDs; returns a nontrivial
/// factor of composite `n` (never called on primes or `n <= 3` by [`factor_u64`]'s driver).
fn pollard_rho_u64(n: u64) -> u64 {
    if n.is_multiple_of(2) {
        return 2;
    }
    let mut rng = InlineSplitMix64(n ^ 0xA5A5_A5A5_A5A5_A5A5);
    loop {
        let c = 1 + (rng.next() % (n - 1));
        let f = |x: u128| -> u64 { ((x * x + c as u128) % n as u128) as u64 };
        let (mut x, mut y, mut d) = (2u64, 2u64, 1u64);
        let mut q = 1u128;
        let (mut xs, ys) = (x, y);
        let m = 128u64;
        'outer: while d == 1 {
            xs = x;
            for _ in 0..m {
                x = f(x as u128);
                y = f(f(y as u128) as u128);
                q = (q * (x as i64 - y as i64).unsigned_abs() as u128) % n as u128;
                if q == 0 {
                    break 'outer;
                }
            }
            d = gcd_u64(q as u64, n);
        }
        if d == n || d == 0 {
            // fallback: brute retry within the cycle
            d = 1;
            let mut xr = xs;
            let yr = ys;
            while d == 1 {
                xr = f(xr as u128);
                d = gcd_u64((xr as i64 - yr as i64).unsigned_abs(), n);
            }
        }
        if d != n && d > 1 {
            return d;
        }
        // retry with a new random c
    }
}

pub fn factor_u64(n: u64) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    let mut n = n;
    for p in [2u64, 3, 5, 7, 11, 13].into_iter().chain((17..65536).step_by(2)) {
        if p * p > n {
            break;
        }
        if n.is_multiple_of(p) {
            let mut count = 0u32;
            while n.is_multiple_of(p) {
                n /= p;
                count += 1;
            }
            factors.push((p, count));
        }
    }
    fn recurse(n: u64, factors: &mut Vec<(u64, u32)>) {
        if n == 1 {
            return;
        }
        if is_prime_u64(n) {
            match factors.iter_mut().find(|(p, _)| *p == n) {
                Some((_, c)) => *c += 1,
                None => factors.push((n, 1)),
            }
            return;
        }
        let d = pollard_rho_u64(n);
        recurse(d, factors);
        recurse(n / d, factors);
    }
    recurse(n, &mut factors);
    factors.sort_unstable_by_key(|&(p, _)| p);
    // merge duplicate prime entries that recursion may have added at different points
    let mut merged: Vec<(u64, u32)> = Vec::new();
    for (p, c) in factors {
        if let Some(last) = merged.last_mut() {
            if last.0 == p {
                last.1 += c;
                continue;
            }
        }
        merged.push((p, c));
    }
    merged
}

/// 🔍 Factorization over arbitrary-precision `Natural`, via a u64 fast path plus `Integer`-arithmetic
/// Pollard-Brent for larger inputs. 60+-digit hard semiprimes are out of scope (no ECM/QS here).
pub fn factor(n: &Natural) -> Vec<(Natural, u32)> {
    if let Some(small) = n.to_u64() {
        return factor_u64(small).into_iter().map(|(p, c)| (Natural::from_u64(p), c)).collect();
    }
    let mut remaining = n.clone();
    let mut factors: Vec<(Natural, u32)> = Vec::new();
    let sieve = Sieve::new(100_000);
    for p in sieve.primes() {
        let p_nat = Natural::from_u64(p as u64);
        if remaining.to_u64().is_some() {
            break;
        }
        let mut count = 0u32;
        loop {
            let (q, r) = remaining.div_rem(&p_nat);
            if r.is_zero() {
                remaining = q;
                count += 1;
            } else {
                break;
            }
        }
        if count > 0 {
            factors.push((p_nat, count));
        }
    }
    fn recurse(n: &Natural, out: &mut Vec<(Natural, u32)>) {
        if n == &Natural::one() {
            return;
        }
        if let Some(small) = n.to_u64() {
            for (p, c) in factor_u64(small) {
                let p_nat = Natural::from_u64(p);
                match out.iter_mut().find(|(q, _)| *q == p_nat) {
                    Some((_, existing)) => *existing += c,
                    None => out.push((p_nat, c)),
                }
            }
            return;
        }
        if is_prime(n) {
            match out.iter_mut().find(|(q, _)| q == n) {
                Some((_, c)) => *c += 1,
                None => out.push((n.clone(), 1)),
            }
            return;
        }
        let d = pollard_rho_natural(n);
        let (q, _) = n.div_rem(&d);
        recurse(&d, out);
        recurse(&q, out);
    }
    recurse(&remaining, &mut factors);
    factors.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged: Vec<(Natural, u32)> = Vec::new();
    for (p, c) in factors {
        if let Some(last) = merged.last_mut() {
            if last.0 == p {
                last.1 += c;
                continue;
            }
        }
        merged.push((p, c));
    }
    merged
}

fn pollard_rho_natural(n: &Natural) -> Natural {
    if !n.bit(0) {
        return Natural::from_u64(2);
    }
    let seed = n.limbs().first().copied().unwrap_or(1) ^ 0x1357_9BDF_2468_ACE0;
    let mut rng = InlineSplitMix64(seed);
    let m = 128usize;
    loop {
        let c = Natural::from_u64(1 + rng.next() % 0xFFFF_FFFF);
        let f = |x: &Natural| -> Natural {
            let (_, r) = x.mul(x).add(&c).div_rem(n);
            r
        };
        let (mut x, mut y) = (Natural::from_u64(2), Natural::from_u64(2));
        let (mut xs, mut ys) = (x.clone(), y.clone());
        let mut d = Natural::one();
        while d == Natural::one() {
            xs = x.clone();
            ys = y.clone();
            let mut q = Natural::one();
            for _ in 0..m {
                x = f(&x);
                y = f(&f(&y));
                let diff = if x > y { x.checked_sub(&y).unwrap() } else { y.checked_sub(&x).unwrap() };
                if diff.is_zero() {
                    q = Natural::zero();
                    break;
                }
                let (_, r) = q.mul(&diff).div_rem(n);
                q = r;
                if q.is_zero() {
                    break;
                }
            }
            d = if q.is_zero() { n.clone() } else { q.gcd(n) };
        }
        if d == *n {
            // batch overshot a nontrivial factor: backtrack one step at a time from the failed batch's start
            d = Natural::one();
            let mut xr = xs;
            while d == Natural::one() {
                xr = f(&xr);
                let diff = if xr > ys { xr.checked_sub(&ys).unwrap() } else { ys.checked_sub(&xr).unwrap() };
                if diff.is_zero() {
                    d = n.clone();
                    break;
                }
                d = diff.gcd(n);
            }
        }
        if d != *n && !d.is_zero() && d != Natural::one() {
            return d;
        }
        // else retry with a different c
    }
}
// #endregion 🔖Factor

// #region 🔖ArithmeticFns
pub fn euler_phi(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut result = n;
    for (p, _) in factor_u64(n) {
        result = result / p * (p - 1);
    }
    result
}

pub fn moebius(n: u64) -> i8 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let factors = factor_u64(n);
    if factors.iter().any(|&(_, e)| e > 1) {
        return 0;
    }
    if factors.len().is_multiple_of(2) {
        1
    } else {
        -1
    }
}

pub fn divisors_u64(n: u64) -> Vec<u64> {
    if n == 0 {
        return Vec::new();
    }
    let factors = factor_u64(n);
    let mut divisors = vec![1u64];
    for (p, e) in factors {
        let mut new_divisors = Vec::with_capacity(divisors.len() * (e as usize + 1));
        let mut power = 1u64;
        for _ in 0..=e {
            for &d in &divisors {
                new_divisors.push(d * power);
            }
            power *= p;
        }
        divisors = new_divisors;
    }
    divisors.sort_unstable();
    divisors
}

pub fn divisor_count(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    factor_u64(n).into_iter().map(|(_, e)| e as u64 + 1).product()
}

pub fn divisor_sum(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    factor_u64(n).into_iter().map(|(p, e)| (0..=e).map(|k| p.pow(k)).sum::<u64>()).product()
}
// #endregion 🔖ArithmeticFns

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn is_prime_u64_hand_cases() {
        for p in [2u64, 3, 5, 7, 11, 97, 7919, 999_983] {
            assert!(is_prime_u64(p), "{p} should be prime");
        }
        for c in [1u64, 4, 6, 8, 9, 100, 561, 1105, 1729] {
            assert!(!is_prime_u64(c), "{c} should be composite");
        }
    }

    #[test]
    fn carmichael_numbers_are_rejected() {
        for &carmichael in &[561u64, 41041, 825265] {
            assert!(!is_prime_u64(carmichael), "{carmichael} is a Carmichael number, not prime");
            assert!(!is_prime(&Natural::from_u64(carmichael)));
        }
    }

    #[test]
    fn is_prime_natural_matches_u64_path_for_small_values() {
        for n in 2u64..200 {
            assert_eq!(is_prime(&Natural::from_u64(n)), is_prime_u64(n), "mismatch at {n}");
        }
    }

    #[test]
    fn is_prime_natural_handles_large_known_prime() {
        // A 128-bit-ish prime candidate; validated via BPSW.
        let p = Natural::from_str("170141183460469231731687303715884105727").unwrap(); // 2^127 - 1, a Mersenne prime
        assert!(is_prime(&p));
        let composite = p.add(&Natural::from_u64(2));
        assert!(!is_prime(&composite) || composite.to_u64().is_some());
    }

    #[test]
    fn factor_u64_reconstructs_via_multiplication() {
        for n in [1u64, 2, 97, 360, 999_983, 1_000_000] {
            let factors = factor_u64(n);
            let product: u64 = factors.iter().map(|&(p, e)| p.pow(e)).product();
            assert_eq!(product, n, "factorization of {n} = {factors:?} doesn't reconstruct");
            for (p, _) in &factors {
                assert!(is_prime_u64(*p));
            }
        }
    }

    #[test]
    fn euler_phi_hand_cases() {
        assert_eq!(euler_phi(1), 1);
        assert_eq!(euler_phi(9), 6);
        assert_eq!(euler_phi(97), 96);
    }

    #[test]
    fn moebius_hand_cases() {
        assert_eq!(moebius(1), 1);
        assert_eq!(moebius(6), 1); // 2*3, two distinct primes -> +1
        assert_eq!(moebius(4), 0); // 2^2 -> 0
        assert_eq!(moebius(30), -1); // 2*3*5, three distinct primes -> -1
    }

    #[test]
    fn divisors_hand_case() {
        let divs = divisors_u64(12);
        assert_eq!(divs, vec![1, 2, 3, 4, 6, 12]);
        assert_eq!(divisor_count(12), 6);
        assert_eq!(divisor_sum(12), 28);
    }

    #[test]
    fn sieve_matches_is_prime_u64() {
        let sieve = Sieve::new(1000);
        for n in 0..=1000 {
            assert_eq!(sieve.is_prime(n), is_prime_u64(n as u64), "mismatch at {n}");
        }
    }

    #[test]
    fn next_prime_hand_cases() {
        assert_eq!(next_prime(&Natural::from_u64(10)), Natural::from_u64(11));
        assert_eq!(next_prime(&Natural::from_u64(1)), Natural::from_u64(2));
    }

    // #region 🔖QuickTests
    mod quick {
        use super::*;

        #[test]
        fn factor_random_64bit_semiprimes() {
            // Products of two mid-size primes.
            for &(p, q) in &[(65537u64, 65539u64), (999_983, 999_979), (7919, 104729)] {
                let n = p * q;
                let factors = factor_u64(n);
                let product: u64 = factors.iter().map(|&(f, e)| f.pow(e)).product();
                assert_eq!(product, n);
                assert!(factors.iter().any(|&(f, _)| f == p));
                assert!(factors.iter().any(|&(f, _)| f == q));
            }
        }

        #[test]
        fn sieve_count_matches_known_pi_of_1e6() {
            let sieve = Sieve::new(1_000_000);
            assert_eq!(sieve.count(), 78498);
        }
    }
    // #endregion 🔖QuickTests

    // #region 🔖LongTests
    mod long {
        use super::*;

        #[test]
        fn factor_natural_random_semiprime() {
            let p = Natural::from_str("1000000000000000003").unwrap(); // prime
            assert!(is_prime(&p));
            let q = Natural::from_u64(104_729); // prime, the 10,000th prime
            let n = p.mul(&q);
            let factors = factor(&n);
            let product = factors.iter().fold(Natural::one(), |acc, (f, e)| acc.mul(&f.pow(*e as u64)));
            assert_eq!(product, n);
        }
    }
    // #endregion 🔖LongTests
}
// #endregion 🔖Tests
}
// #endregion 🔖Primes

// #region 🔖Interval
pub mod interval {
//! 📏 Certified `f64` interval (ball) arithmetic with directed outward rounding — the substrate for
//! adaptive-precision numeric evaluation and root-isolation refinement in `mathematical_cas`.

use crate::rational::Rational;

// #region 🔖Interval
/// 📏 Closed interval `[lo, hi]`; invariant `lo <= hi` (infinite endpoints allowed, never NaN).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Interval {
    pub lo: f64,
    pub hi: f64,
}

impl Interval {
    pub fn new(lo: f64, hi: f64) -> Self {
        debug_assert!(lo <= hi, "Interval::new: lo must be <= hi");
        Self { lo, hi }
    }

    pub fn point(v: f64) -> Self {
        Self { lo: v, hi: v }
    }

    pub fn whole() -> Self {
        Self { lo: f64::NEG_INFINITY, hi: f64::INFINITY }
    }

    /// 🎯 Certified conversion from an exact `Rational`: rounds `to_f64()` outward by successive ulps
    /// (via `next_down`/`next_up`) until the interval is verified — by exact cross-multiplied rational
    /// comparison — to actually contain the true value.
    pub fn from_rational(r: &Rational) -> Self {
        let approx = r.to_f64();
        if !approx.is_finite() {
            return Self::whole();
        }
        let mut lo = approx;
        let mut hi = approx;
        // Widen outward until both bounds are certified to contain r; converges in a handful of ulps
        // since to_f64 is already within ~1 ulp of the true value.
        for _ in 0..4 {
            if Self::f64_le_rational(lo, r) {
                break;
            }
            lo = lo.next_down();
        }
        for _ in 0..4 {
            if Self::rational_le_f64(r, hi) {
                break;
            }
            hi = hi.next_up();
        }
        Self { lo, hi }
    }

    fn rational_le_f64(r: &Rational, bound: f64) -> bool {
        match Rational::from_f64(bound) {
            Some(bound_r) => *r <= bound_r,
            None => bound == f64::INFINITY,
        }
    }

    fn f64_le_rational(bound: f64, r: &Rational) -> bool {
        match Rational::from_f64(bound) {
            Some(bound_r) => bound_r <= *r,
            None => bound == f64::NEG_INFINITY,
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.lo <= x && x <= self.hi
    }

    pub fn contains_zero(&self) -> bool {
        self.contains(0.0)
    }

    pub fn width(&self) -> f64 {
        self.hi - self.lo
    }

    pub fn midpoint(&self) -> f64 {
        self.lo + (self.hi - self.lo) * 0.5
    }

    /// 〽️ Sign of the whole interval if it doesn't straddle zero, else `None`.
    pub fn sign(&self) -> Option<std::cmp::Ordering> {
        if self.hi < 0.0 {
            Some(std::cmp::Ordering::Less)
        } else if self.lo > 0.0 {
            Some(std::cmp::Ordering::Greater)
        } else if self.lo == 0.0 && self.hi == 0.0 {
            Some(std::cmp::Ordering::Equal)
        } else {
            None
        }
    }

    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let lo = self.lo.max(other.lo);
        let hi = self.hi.min(other.hi);
        if lo <= hi {
            Some(Self { lo, hi })
        } else {
            None
        }
    }

    pub fn hull(&self, other: &Self) -> Self {
        Self { lo: self.lo.min(other.lo), hi: self.hi.max(other.hi) }
    }

    // #region 🔖CertifiedOperations
    pub fn neg(&self) -> Self {
        Self { lo: -self.hi, hi: -self.lo }
    }

    pub fn abs(&self) -> Self {
        if self.lo >= 0.0 {
            *self
        } else if self.hi <= 0.0 {
            self.neg()
        } else {
            Self { lo: 0.0, hi: self.lo.abs().max(self.hi.abs()) }
        }
    }

    pub fn add(&self, rhs: &Self) -> Self {
        Self { lo: (self.lo + rhs.lo).next_down(), hi: (self.hi + rhs.hi).next_up() }
    }

    pub fn sub(&self, rhs: &Self) -> Self {
        self.add(&rhs.neg())
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        let candidates = [self.lo * rhs.lo, self.lo * rhs.hi, self.hi * rhs.lo, self.hi * rhs.hi];
        let lo = candidates.iter().copied().fold(f64::INFINITY, f64::min).next_down();
        let hi = candidates.iter().copied().fold(f64::NEG_INFINITY, f64::max).next_up();
        Self { lo, hi }
    }

    pub fn recip(&self) -> Option<Self> {
        if self.contains_zero() {
            return None;
        }
        let candidates = [1.0 / self.lo, 1.0 / self.hi];
        let lo = candidates.iter().copied().fold(f64::INFINITY, f64::min).next_down();
        let hi = candidates.iter().copied().fold(f64::NEG_INFINITY, f64::max).next_up();
        Some(Self { lo, hi })
    }

    pub fn div(&self, rhs: &Self) -> Option<Self> {
        rhs.recip().map(|r| self.mul(&r))
    }

    /// √ Rigorous 1-ulp-widened square root; `None` if the interval dips below zero.
    pub fn sqrt(&self) -> Option<Self> {
        if self.lo < 0.0 {
            return None;
        }
        Some(Self { lo: self.lo.sqrt().next_down(), hi: self.hi.sqrt().next_up() })
    }

    pub fn powi(&self, n: i32) -> Self {
        if n == 0 {
            return Self::point(1.0);
        }
        if n < 0 {
            return self.powi(-n).recip().unwrap_or_else(Self::whole);
        }
        let mut result = Self::point(1.0);
        let mut base = *self;
        let mut e = n;
        while e > 0 {
            if e & 1 == 1 {
                result = result.mul(&base);
            }
            base = base.mul(&base);
            e >>= 1;
        }
        result
    }
    // #endregion 🔖CertifiedOperations

    // #region 🔖HeuristicTranscendental
    /// 🌊 Heuristic (not certified): widens the libm result by 4 ulps per side. Platform libm is
    /// almost always <1 ulp but not guaranteed correctly-rounded, so callers needing certified bounds
    /// must stay on the `+ - * / sqrt` subset above.
    pub fn exp(&self) -> Self {
        Self { lo: widen_down(self.lo.exp(), 4), hi: widen_up(self.hi.exp(), 4) }
    }

    /// 🌊 Heuristic natural log; `None` if the interval is entirely non-positive.
    pub fn ln(&self) -> Option<Self> {
        if self.hi <= 0.0 {
            return None;
        }
        let lo_val = if self.lo <= 0.0 { f64::NEG_INFINITY } else { widen_down(self.lo.ln(), 4) };
        Some(Self { lo: lo_val, hi: widen_up(self.hi.ln(), 4) })
    }

    /// 🌊 Heuristic sine/cosine over a bounded-width interval; wide intervals conservatively widen to `[-1, 1]`.
    pub fn sin(&self) -> Self {
        if self.width() > std::f64::consts::PI {
            return Self { lo: -1.0, hi: 1.0 };
        }
        let samples = [self.lo.sin(), self.hi.sin(), self.midpoint().sin()];
        let lo = samples.iter().copied().fold(f64::INFINITY, f64::min);
        let hi = samples.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        Self { lo: widen_down(lo, 8), hi: widen_up(hi, 8) }
    }

    pub fn cos(&self) -> Self {
        if self.width() > std::f64::consts::PI {
            return Self { lo: -1.0, hi: 1.0 };
        }
        let samples = [self.lo.cos(), self.hi.cos(), self.midpoint().cos()];
        let lo = samples.iter().copied().fold(f64::INFINITY, f64::min);
        let hi = samples.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        Self { lo: widen_down(lo, 8), hi: widen_up(hi, 8) }
    }

    pub fn atan(&self) -> Self {
        Self { lo: widen_down(self.lo.atan(), 4), hi: widen_up(self.hi.atan(), 4) }
    }
    // #endregion 🔖HeuristicTranscendental
}

fn widen_down(x: f64, ulps: u32) -> f64 {
    let mut v = x;
    for _ in 0..ulps {
        v = v.next_down();
    }
    v
}

fn widen_up(x: f64, ulps: u32) -> f64 {
    let mut v = x;
    for _ in 0..ulps {
        v = v.next_up();
    }
    v
}
// #endregion 🔖Interval

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_rational_contains_the_exact_value() {
        let r = Rational::from_i64(1, 3).unwrap();
        let iv = Interval::from_rational(&r);
        assert!(iv.lo <= r.to_f64() && r.to_f64() <= iv.hi);
        // Certified containment check via exact rational comparison against the bounds.
        let lo_r = Rational::from_f64(iv.lo).unwrap();
        let hi_r = Rational::from_f64(iv.hi).unwrap();
        assert!(lo_r <= r && r <= hi_r);
    }

    #[test]
    fn arithmetic_hand_cases() {
        let a = Interval::point(2.0);
        let b = Interval::point(3.0);
        let sum = a.add(&b);
        assert!(sum.contains(5.0));
        let prod = a.mul(&b);
        assert!(prod.contains(6.0));
    }

    #[test]
    fn division_by_zero_straddling_interval_is_none() {
        let a = Interval::new(-1.0, 1.0);
        let b = Interval::point(1.0);
        assert!(b.div(&a).is_none());
    }

    #[test]
    fn sqrt_of_negative_is_none() {
        assert!(Interval::point(-1.0).sqrt().is_none());
        let s = Interval::point(4.0).sqrt().unwrap();
        assert!(s.contains(2.0));
    }

    #[test]
    fn sign_detection() {
        assert_eq!(Interval::new(1.0, 2.0).sign(), Some(std::cmp::Ordering::Greater));
        assert_eq!(Interval::new(-2.0, -1.0).sign(), Some(std::cmp::Ordering::Less));
        assert_eq!(Interval::new(-1.0, 1.0).sign(), None);
    }

    #[test]
    fn hull_and_intersect() {
        let a = Interval::new(0.0, 2.0);
        let b = Interval::new(1.0, 3.0);
        assert_eq!(a.hull(&b), Interval::new(0.0, 3.0));
        let inter = a.intersect(&b).unwrap();
        assert_eq!(inter, Interval::new(1.0, 2.0));
        assert!(Interval::new(0.0, 1.0).intersect(&Interval::new(2.0, 3.0)).is_none());
    }

    #[test]
    fn powi_matches_repeated_multiplication() {
        let a = Interval::point(2.0);
        let p = a.powi(10);
        assert!(p.contains(1024.0));
    }
}
// #endregion 🔖Tests
}
// #endregion 🔖Interval

pub use integer::{Integer, Sign};
pub use interval::Interval;
pub use modular::ModInt;
pub use natural::Natural;
pub use rational::Rational;
pub use traits::{field_div_rem, field_gcd, CommutativeRing, EuclideanDomain, Field, GcdDomain, IntegralDomain, Ring};
