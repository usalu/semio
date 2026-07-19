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
        while a % 2 == 0 {
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
    while q % 2 == 0 {
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
    let mut r = mod_pow(a, (q + 1) / 2, p);
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
        while order % p == 0 && mod_pow(a, order / p, m) == 1 % m {
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
/// element produced by `Ring::zero()/one()/from_i64` before any modulus is known from context; binary
/// operations unify an unbound operand onto the other side's (bound) modulus, and `debug_assert` that
/// two differently-bound operands never meet (that would be a caller bug, not a valid computation).
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

    fn unify(self, other: Self) -> (u64, u64, u64) {
        if self.modulus == 0 {
            (self.value, other.value, other.modulus)
        } else if other.modulus == 0 {
            (self.value, other.value, self.modulus)
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
            Self { value: a + b, modulus: 0 }
        } else {
            Self { value: mod_add(a, b, m), modulus: m }
        }
    }
    fn neg(&self) -> Self {
        if self.modulus == 0 {
            Self { value: self.value, modulus: 0 }
        } else if self.value == 0 {
            *self
        } else {
            Self { value: self.modulus - self.value, modulus: self.modulus }
        }
    }
    fn mul(&self, rhs: &Self) -> Self {
        let (a, b, m) = self.unify(*rhs);
        if m == 0 {
            Self { value: a * b, modulus: 0 }
        } else {
            Self { value: mod_mul(a, b, m), modulus: m }
        }
    }
    fn is_zero(&self) -> bool {
        self.value == 0
    }
    fn from_i64(value: i64) -> Self {
        Self { value: value.unsigned_abs(), modulus: 0 }
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
    fn modint_field_ops() {
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
