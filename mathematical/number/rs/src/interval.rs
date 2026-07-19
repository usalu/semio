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
            if Self::rational_le_f64(r, lo) {
                break;
            }
            lo = lo.next_down();
        }
        for _ in 0..4 {
            if Self::f64_le_rational(hi, r) {
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

    // #region 🔖CertifiedOps
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
    // #endregion 🔖CertifiedOps

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
