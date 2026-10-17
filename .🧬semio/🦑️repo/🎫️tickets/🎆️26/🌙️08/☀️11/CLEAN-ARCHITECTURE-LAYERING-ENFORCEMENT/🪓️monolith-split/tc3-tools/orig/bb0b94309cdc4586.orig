//! 💍️ Abstract algebra hierarchy shared by every exact numeric/symbolic crate downstream:
//! generic matrices, generic polynomials, and CAS expression coefficients are all written
//! once, generically, against these traits.

// #region 🔖️Ring
/// 💍️ Ring with multiplicative identity. Methods take `&self`/`&Self` throughout (not `Copy`/by-value)
/// because the concrete types that matter most (`Integer`, `PolyU<C>`, `Expr`) are heap-allocated and
/// expensive to move implicitly.
pub trait Ring: Clone + PartialEq + std::fmt::Debug {
    /// 0⃣ Additive identity.
    fn zero() -> Self;
    /// 1⃣ Multiplicative identity.
    fn one() -> Self;
    fn add(&self, rhs: &Self) -> Self;
    fn neg(&self) -> Self;
    fn mul(&self, rhs: &Self) -> Self;
    /// ➖️ Default via `add(neg)`; override when direct subtraction avoids an allocation.
    fn sub(&self, rhs: &Self) -> Self {
        self.add(&rhs.neg())
    }
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
    fn is_one(&self) -> bool {
        *self == Self::one()
    }
    /// 🔢️ Embeds a signed machine integer via repeated doubling; concrete types override with a direct
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
    /// 🔋️ Square-and-multiply exponentiation; correct for any ring (including `exp == 0`, giving `one()`).
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
    /// 🧬️ Additive order of `one()` (0 in characteristic zero); an instance method — not a constant —
    /// because `ModInt`'s characteristic is a runtime modulus, not a compile-time property of the type.
    fn characteristic(&self) -> u64;
}

/// 💍️ Marker for rings whose multiplication commutes; nothing here relies on it structurally, but it
/// documents the contract every ring in this stack actually satisfies and gates commutative-only algorithms.
pub trait CommutativeRing: Ring {}
// #endregion 🔖️Ring

// #region 🔖️IntegralDomain
/// 🚫️ Commutative ring with no zero divisors: `exact_div` is the load-bearing operation for
/// fraction-free algorithms (Bareiss elimination, polynomial pseudo-division) that only ever divide
/// when the result is provably exact.
pub trait IntegralDomain: CommutativeRing {
    /// ➗️ `Some(self / rhs)` if `rhs` divides `self` exactly, else `None`. `rhs` must be nonzero.
    fn exact_div(&self, rhs: &Self) -> Option<Self>;
}
// #endregion 🔖️IntegralDomain

// #region 🔖️GcdDomain
pub trait GcdDomain: IntegralDomain {
    fn gcd(&self, rhs: &Self) -> Self;
    /// ➗️ Default `self * rhs / gcd(self, rhs)`; degenerate (zero) inputs return `zero()`.
    fn lcm(&self, rhs: &Self) -> Self {
        if self.is_zero() || rhs.is_zero() {
            return Self::zero();
        }
        let g = self.gcd(rhs);
        self.mul(rhs).exact_div(&g).expect("gcd(a, b) divides a * b exactly by definition")
    }
}
// #endregion 🔖️GcdDomain

// #region 🔖️EuclideanDomain
pub trait EuclideanDomain: GcdDomain {
    /// ➗️ `(quotient, remainder)` such that `self == quotient * rhs + remainder`. `rhs` must be nonzero.
    fn div_rem(&self, rhs: &Self) -> (Self, Self)
    where
        Self: Sized;
}

/// ➗️ Trivial Euclidean-domain `div_rem` for any `Field`: remainder is always zero. Free function
/// (not a default trait method) because a blanket `impl<T: Field> EuclideanDomain for T` would
/// conflict with concrete manual impls under Rust's orphan/coherence rules.
pub fn field_div_rem<T: Field>(a: &T, b: &T) -> (T, T) {
    (a.div(b).expect("field_div_rem: divisor must be nonzero"), T::zero())
}

/// 🚫️ Trivial GCD-domain `gcd` for any `Field`: every nonzero element is a unit, so the gcd of any two
/// non-both-zero elements is `one()` (and `zero()` when both inputs are zero).
pub fn field_gcd<T: Field>(a: &T, b: &T) -> T {
    if a.is_zero() && b.is_zero() {
        T::zero()
    } else {
        T::one()
    }
}
// #endregion 🔖️EuclideanDomain

// #region 🔖️Field
pub trait Field: EuclideanDomain {
    /// ➗️ Multiplicative inverse; `None` only for zero.
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
// #endregion 🔖️Field

// #region 🔖️PrimitiveImpls
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

/// 🌊️ `f64` as an (approximate) field: convenient for numeric-evaluation code paths that want to be
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
// #endregion 🔖️PrimitiveImpls

// #region 🔖️Tests
#[cfg(test)]
include!("../🧪️tests/🔬️traits-unit/🦀️.rs");
// #endregion 🔖️Tests
