//! 🌱 Real root isolation for integer polynomials via Sturm sequences: sign-change counting gives
//! rigorous isolating rational intervals for every real root, refined by bisection.

use crate::univariate::PolyU;
use mathematical_number::{Integer, Rational};

// #region 🔖RootBounds
/// 📏 Cauchy's bound: every real root of `f` has absolute value `<= 1 + max(|a_i|)/|lc|`.
pub fn cauchy_root_bound(f: &PolyU<Integer>) -> Rational {
    let Some(lc) = f.leading_coeff() else { return Rational::zero() };
    let lc_abs = Rational::from_integer(Integer::from_natural(lc.abs()));
    let max_other = f.coeffs()[..f.coeffs().len() - 1].iter().map(|c| Rational::from_integer(Integer::from_natural(c.abs()))).fold(Rational::zero(), |acc, v| if v > acc { v } else { acc });
    Rational::one().add(&max_other.div(&lc_abs).unwrap_or_else(Rational::zero))
}
// #endregion 🔖RootBounds

// #region 🔖Sturm
/// 🔗 The signed polynomial-remainder sequence `f, f', -rem(f,f'), -rem(f',...), ...` (primitive-part
/// normalized at each step to control coefficient growth), used to count real roots via sign changes.
pub fn sturm_sequence(f: &PolyU<Integer>) -> Vec<PolyU<Integer>> {
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

fn sign_changes(seq: &[PolyU<Integer>], point: &Rational) -> usize {
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

fn eval_rational(f: &PolyU<Integer>, point: &Rational) -> Rational {
    let mut result = Rational::zero();
    for c in f.coeffs().iter().rev() {
        result = result.mul(point).add(&Rational::from_integer(c.clone()));
    }
    result
}

/// 🔢 Number of distinct real roots of (the squarefree part of) `f` in the half-open interval `(lo, hi]`.
pub fn count_roots_in(seq: &[PolyU<Integer>], lo: &Rational, hi: &Rational) -> usize {
    sign_changes(seq, lo).saturating_sub(sign_changes(seq, hi))
}

/// ✂️ Isolates every distinct real root of `f` into a sorted list of half-open rational intervals
/// `(lo, hi]`, each containing exactly one root. Operates on the squarefree part (repeated roots of
/// the original `f` collapse to one isolated interval, matching Sturm's theorem's requirement of a
/// squarefree input).
pub fn isolate_real_roots(f: &PolyU<Integer>) -> Vec<(Rational, Rational)> {
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

fn to_squarefree_integer(f: &PolyU<Integer>) -> PolyU<Integer> {
    let rational = PolyU::from_coeffs(f.coeffs().iter().map(|c| Rational::from_integer(c.clone())).collect());
    let parts = rational.squarefree_decomposition();
    let mut result = PolyU::<Rational>::one();
    for (part, _) in parts {
        result = result.mul(&part);
    }
    let denom_lcm = result.coeffs().iter().fold(mathematical_number::Natural::one(), |acc, c| acc.mul(c.denom()).div_rem(&acc.gcd(c.denom())).0);
    PolyU::from_coeffs(result.coeffs().iter().map(|c| c.mul(&Rational::from_integer(Integer::from_natural(denom_lcm.clone()))).trunc()).collect()).primitive_part()
}

/// 🔬 Bisects `(lo, hi]` (assumed to isolate exactly one root of `f`) down to the given `width`,
/// preserving the sign-change invariant at each step.
pub fn refine_root(f: &PolyU<Integer>, lo: &Rational, hi: &Rational, width: &Rational) -> (Rational, Rational) {
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
// #endregion 🔖Sturm

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn ipoly(coeffs: Vec<i64>) -> PolyU<Integer> {
        PolyU::from_coeffs(coeffs.into_iter().map(Integer::from_i64).collect())
    }

    #[test]
    fn isolate_roots_of_simple_quadratic() {
        let f = ipoly(vec![-2, 0, 1]); // x^2 - 2, roots +-sqrt(2)
        let intervals = isolate_real_roots(&f);
        assert_eq!(intervals.len(), 2);
        for (lo, hi) in &intervals {
            let seq = sturm_sequence(&f);
            assert_eq!(count_roots_in(&seq, lo, hi), 1);
        }
    }

    #[test]
    fn isolate_roots_matches_known_integer_roots() {
        // (x-1)(x-3)(x+2)
        let f = ipoly(vec![6, -1, -4, 1]);
        let intervals = isolate_real_roots(&f);
        assert_eq!(intervals.len(), 3);
    }

    #[test]
    fn refine_root_converges_to_sqrt2() {
        let f = ipoly(vec![-2, 0, 1]);
        let intervals = isolate_real_roots(&f);
        let positive = intervals.iter().find(|(lo, hi)| lo.is_zero() || (!lo.numer().is_negative() && !hi.numer().is_negative())).cloned().unwrap();
        let width = Rational::from_i64(1, 1_000_000).unwrap();
        let (lo, hi) = refine_root(&f, &positive.0, &positive.1, &width);
        let approx = (lo.to_f64() + hi.to_f64()) / 2.0;
        assert!((approx - std::f64::consts::SQRT_2).abs() < 1e-5);
    }

    #[test]
    fn cauchy_bound_contains_all_roots() {
        let f = ipoly(vec![6, -1, -4, 1]); // roots -2, 1, 3
        let bound = cauchy_root_bound(&f);
        assert!(bound >= Rational::from_i64(3, 1).unwrap());
    }

    #[test]
    fn wilkinson_like_small_case_root_count() {
        // (x-1)(x-2)(x-3)(x-4)
        let f = ipoly(vec![-1, 1]).mul(&ipoly(vec![-2, 1])).mul(&ipoly(vec![-3, 1])).mul(&ipoly(vec![-4, 1]));
        let intervals = isolate_real_roots(&f);
        assert_eq!(intervals.len(), 4);
    }

    #[test]
    fn zero_polynomial_and_constant_have_no_roots() {
        assert!(isolate_real_roots(&ipoly(vec![])).is_empty());
        assert!(isolate_real_roots(&ipoly(vec![5])).is_empty());
    }
}
// #endregion 🔖Tests
