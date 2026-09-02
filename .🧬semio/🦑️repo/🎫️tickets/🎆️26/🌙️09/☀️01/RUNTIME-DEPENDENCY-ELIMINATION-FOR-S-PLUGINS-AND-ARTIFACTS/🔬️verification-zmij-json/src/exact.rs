//! Exact-arithmetic correctly-rounded decimal digit generation, used to verify (and, if proven
//! correct, replace) the "probe the neighboring digit" heuristic that turned out to be unsound
//! for large-magnitude values (the round-trip basin can contain more than two adjacent
//! minimal-length grid points there, so "does a neighbor also round-trip" does not imply "the
//! true value sits exactly halfway between them").
//!
//! This computes round-half-to-even at a FIXED, externally supplied digit count and decimal
//! exponent (both already correctly determined by Rust's own shortest-round-trip `{:e}`), using
//! exact big-integer arithmetic over the value's exact rational form `mantissa * 2^binary_exponent`.
//! No third-party bignum crate — a minimal base-2^32 unsigned bignum, sized for this one job.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Big(Vec<u32>);

impl Big {
    pub fn from_u64(v: u64) -> Self {
        let lo = v as u32;
        let hi = (v >> 32) as u32;
        let mut limbs = vec![lo, hi];
        Self::trim(&mut limbs);
        Big(limbs)
    }

    fn trim(limbs: &mut Vec<u32>) {
        while limbs.len() > 1 && *limbs.last().unwrap() == 0 {
            limbs.pop();
        }
    }

    pub fn is_zero(&self) -> bool {
        self.0.len() == 1 && self.0[0] == 0
    }

    pub fn mul_small(&mut self, m: u32) {
        let mut carry: u64 = 0;
        for limb in self.0.iter_mut() {
            let product = (*limb as u64) * (m as u64) + carry;
            *limb = product as u32;
            carry = product >> 32;
        }
        if carry > 0 {
            self.0.push(carry as u32);
        }
        Self::trim(&mut self.0);
    }

    pub fn mul_pow5(&mut self, mut exp: u32) {
        while exp > 0 {
            let chunk = exp.min(13);
            self.mul_small(5u32.pow(chunk));
            exp -= chunk;
        }
    }

    pub fn shl(&mut self, bits: u32) {
        if bits == 0 {
            return;
        }
        let limb_shift = (bits / 32) as usize;
        let bit_shift = bits % 32;
        let mut result = vec![0u32; self.0.len() + limb_shift + 1];
        for (i, &limb) in self.0.iter().enumerate() {
            let value = limb as u64;
            if bit_shift == 0 {
                result[i + limb_shift] |= value as u32;
            } else {
                let shifted = value << bit_shift;
                result[i + limb_shift] |= shifted as u32;
                result[i + limb_shift + 1] |= (shifted >> 32) as u32;
            }
        }
        Self::trim(&mut result);
        self.0 = result;
    }

    fn bit_length(&self) -> u32 {
        let top = *self.0.last().unwrap();
        if top == 0 {
            0
        } else {
            (self.0.len() as u32 - 1) * 32 + (32 - top.leading_zeros())
        }
    }

    fn get_bit(&self, i: u32) -> bool {
        let limb = i / 32;
        let bit = i % 32;
        if (limb as usize) >= self.0.len() {
            false
        } else {
            (self.0[limb as usize] >> bit) & 1 == 1
        }
    }

    fn cmp(&self, other: &Big) -> std::cmp::Ordering {
        if self.0.len() != other.0.len() {
            return self.0.len().cmp(&other.0.len());
        }
        for i in (0..self.0.len()).rev() {
            if self.0[i] != other.0[i] {
                return self.0[i].cmp(&other.0[i]);
            }
        }
        std::cmp::Ordering::Equal
    }

    fn sub_assign(&mut self, other: &Big) {
        let mut borrow: i64 = 0;
        for i in 0..self.0.len() {
            let a = self.0[i] as i64;
            let b = if i < other.0.len() { other.0[i] as i64 } else { 0 };
            let mut diff = a - b - borrow;
            if diff < 0 {
                diff += 1 << 32;
                borrow = 1;
            } else {
                borrow = 0;
            }
            self.0[i] = diff as u32;
        }
        Self::trim(&mut self.0);
    }

    fn shl1(&mut self) {
        let mut carry = 0u32;
        for limb in self.0.iter_mut() {
            let new_carry = *limb >> 31;
            *limb = (*limb << 1) | carry;
            carry = new_carry;
        }
        if carry != 0 {
            self.0.push(carry);
        }
    }

    fn or_bit0(&mut self, bit: bool) {
        if bit {
            self.0[0] |= 1;
        }
    }

    /// Schoolbook binary long division: returns (quotient, remainder), `self / other`.
    pub fn div_rem(&self, other: &Big) -> (Big, Big) {
        assert!(!other.is_zero());
        let bits = self.bit_length();
        let mut quotient = Big(vec![0u32; ((bits / 32) + 1) as usize]);
        let mut remainder = Big(vec![0u32]);
        for i in (0..bits).rev() {
            remainder.shl1();
            remainder.or_bit0(self.get_bit(i));
            if remainder.cmp(other) != std::cmp::Ordering::Less {
                remainder.sub_assign(other);
                let limb = (i / 32) as usize;
                let bit = i % 32;
                if limb >= quotient.0.len() {
                    quotient.0.resize(limb + 1, 0);
                }
                quotient.0[limb] |= 1 << bit;
            }
        }
        Self::trim(&mut quotient.0);
        Self::trim(&mut remainder.0);
        (quotient, remainder)
    }

    pub fn double(&self) -> Big {
        let mut c = self.clone();
        c.shl1();
        c
    }

    pub fn to_decimal_string(&self) -> String {
        if self.is_zero() {
            return "0".to_string();
        }
        let mut digits_le: Vec<u8> = Vec::new();
        let mut cur = self.clone();
        let ten_pow9 = Big::from_u64(1_000_000_000u64);
        while !cur.is_zero() {
            let (q, r) = cur.div_rem(&ten_pow9);
            let mut chunk = if r.0.is_empty() { 0u64 } else { r.0.iter().rev().fold(0u64, |acc, &limb| (acc << 32) | limb as u64) };
            for _ in 0..9 {
                digits_le.push((chunk % 10) as u8);
                chunk /= 10;
            }
            cur = q;
        }
        while digits_le.len() > 1 && *digits_le.last().unwrap() == 0 {
            digits_le.pop();
        }
        digits_le.iter().rev().map(|d| (b'0' + d) as char).collect()
    }
}

/// 🎯️ Decomposes an `f64` into its exact `mantissa * 2^binary_exponent` form (mantissa carries
/// the implicit bit for normals; subnormals keep the raw 52-bit significand at the fixed minimum
/// exponent). `value` must be finite and non-zero.
pub fn decompose(value: f64) -> (u64, i32) {
    let bits = value.to_bits();
    let raw_exp = ((bits >> 52) & 0x7FF) as i32;
    let raw_mantissa = bits & 0x000F_FFFF_FFFF_FFFF;
    if raw_exp == 0 {
        (raw_mantissa, 1 - 1023 - 52)
    } else {
        (raw_mantissa | (1u64 << 52), raw_exp - 1023 - 52)
    }
}

/// ✅️ Computes the correctly-rounded (round-half-to-even) `digit_count`-digit decimal
/// significand for `value`, given the leading-digit decimal exponent `decimal_exponent` (both
/// already established by Rust's own shortest-round-trip formatter — this function only
/// re-derives the DIGITS at that fixed precision, exactly). Returns `(digits, exponent_adjust)`
/// where `exponent_adjust` is `1` if rounding carried all the way through (e.g. "999" -> "1000"
/// truncated back to "100" with the exponent bumped), else `0`.
pub fn correctly_rounded_digits(value: f64, decimal_exponent: i32, digit_count: usize) -> (Vec<u8>, i32) {
    let (mantissa, binary_exponent) = decompose(value.abs());
    let q = decimal_exponent - (digit_count as i32 - 1);
    let e2 = binary_exponent - q;
    let e5 = -q;

    let mut numerator = Big::from_u64(mantissa);
    if e5 > 0 {
        numerator.mul_pow5(e5 as u32);
    }
    if e2 > 0 {
        numerator.shl(e2 as u32);
    }
    let mut denominator = Big::from_u64(1);
    if e5 < 0 {
        denominator.mul_pow5((-e5) as u32);
    }
    if e2 < 0 {
        denominator.shl((-e2) as u32);
    }

    let (mut quotient, remainder) = numerator.div_rem(&denominator);
    let doubled_remainder = remainder.double();
    let cmp = doubled_remainder.cmp(&denominator);
    let quotient_is_odd = quotient.0[0] & 1 == 1;
    let round_up = match cmp {
        std::cmp::Ordering::Greater => true,
        std::cmp::Ordering::Less => false,
        std::cmp::Ordering::Equal => quotient_is_odd,
    };
    if round_up {
        quotient.mul_small(1);
        let one = Big::from_u64(1);
        let mut q2 = quotient.clone();
        // add 1
        let mut carry = 1u64;
        for limb in q2.0.iter_mut() {
            let sum = *limb as u64 + carry;
            *limb = sum as u32;
            carry = sum >> 32;
            if carry == 0 {
                break;
            }
        }
        if carry > 0 {
            q2.0.push(carry as u32);
        }
        let _ = one;
        quotient = q2;
    }

    let mut digit_string = quotient.to_decimal_string();
    let mut exponent_adjust = 0;
    if digit_string.len() > digit_count {
        assert_eq!(digit_string.len(), digit_count + 1, "rounding should carry at most one extra digit");
        assert!(digit_string.ends_with('0'));
        digit_string.pop();
        exponent_adjust = 1;
    }
    while digit_string.len() < digit_count {
        digit_string.insert(0, '0');
    }
    (digit_string.into_bytes(), exponent_adjust)
}
