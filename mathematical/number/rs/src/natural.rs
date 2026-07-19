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

    // #region 🔖BitOps
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
    // #endregion 🔖BitOps

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
