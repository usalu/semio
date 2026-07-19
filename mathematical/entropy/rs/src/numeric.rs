//! 🔬 Hand-rolled numerical primitives shared by every estimator: special functions (ln-gamma,
//! digamma), stable summation, and a deterministic PRNG. Zero external dependencies.

// #region 🔖SpecialFunctions
const LANCZOS_G: f64 = 7.0;
const LANCZOS_COEFFICIENTS: [f64; 9] = [
    0.999_999_999_999_809_93,
    676.520_368_121_885_1,
    -1259.139_216_722_402_8,
    771.323_428_777_653_13,
    -176.615_029_162_140_59,
    12.507_343_278_686_905,
    -0.138_571_095_265_720_12,
    9.984_369_578_019_572e-6,
    1.505_632_735_149_312e-7,
];

/// 🔬 Natural log of the gamma function, accurate to ~1e-13 relative error for `x > 0`.
/// Lanczos approximation, g = 7, n = 9 (Numerical Recipes coefficient set).
pub fn ln_gamma(x: f64) -> f64 {
    if x < 0.5 {
        // 🔬 Reflection formula: Gamma(x) * Gamma(1-x) = pi / sin(pi*x).
        let sin_term = (core::f64::consts::PI * x).sin();
        core::f64::consts::PI.ln() - sin_term.abs().ln() - ln_gamma(1.0 - x)
    } else {
        let x = x - 1.0;
        let mut a = LANCZOS_COEFFICIENTS[0];
        for (i, c) in LANCZOS_COEFFICIENTS.iter().enumerate().skip(1) {
            a += c / (x + i as f64);
        }
        let t = x + LANCZOS_G + 0.5;
        0.5 * (2.0 * core::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
    }
}

/// 🔬 Gamma function via `exp(ln_gamma(x))`. Prefer [`ln_gamma`] directly when only the log is
/// needed (avoids overflow for large `x`).
pub fn gamma(x: f64) -> f64 {
    ln_gamma(x).exp()
}

/// 🔬 Digamma function `psi(x) = d/dx ln(Gamma(x))`. Uses the recurrence `psi(x) = psi(x+1) -
/// 1/x` to shift small `x` into the region where the asymptotic series converges well (`x >=
/// 6`), then the standard asymptotic expansion in `1/x^2`.
pub fn digamma(mut x: f64) -> f64 {
    let mut result = 0.0;
    while x < 6.0 {
        result -= 1.0 / x;
        x += 1.0;
    }
    let inv = 1.0 / x;
    let inv2 = inv * inv;
    result += x.ln() - 0.5 * inv
        - inv2 * (1.0 / 12.0 - inv2 * (1.0 / 120.0 - inv2 * (1.0 / 252.0 - inv2 / 240.0)));
    result
}

/// 🔬 Trigamma function `psi'(x)`, the derivative of [`digamma`]. Same shift-and-asymptotic
/// strategy as `digamma`.
pub fn trigamma(mut x: f64) -> f64 {
    let mut result = 0.0;
    while x < 6.0 {
        result += 1.0 / (x * x);
        x += 1.0;
    }
    let inv = 1.0 / x;
    let inv2 = inv * inv;
    result += inv * (1.0 + inv * (0.5 + inv * (1.0 / 6.0 - inv2 * (1.0 / 30.0 - inv2 / 42.0))));
    result
}

/// 🔬 Error function via Abramowitz & Stegun 7.1.26 (max abs error ~1.5e-7), refined with one
/// Newton step against [`erfc`]'s complementary identity is unnecessary at this tolerance for
/// entropy-estimator purposes (bandwidth selection, normal CDF).
pub fn erf(x: f64) -> f64 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let a1 = 0.254_829_592;
    let a2 = -0.284_496_736;
    let a3 = 1.421_413_741;
    let a4 = -1.453_152_027;
    let a5 = 1.061_405_429;
    let p = 0.327_591_1;
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();
    sign * y
}

/// 🔬 Complementary error function `1 - erf(x)`.
pub fn erfc(x: f64) -> f64 {
    1.0 - erf(x)
}

/// 🔬 Standard normal CDF `Phi(x)`.
pub fn normal_cdf(x: f64) -> f64 {
    0.5 * erfc(-x / core::f64::consts::SQRT_2)
}

/// 🔬 Inverse standard normal CDF (quantile function), Acklam's rational approximation
/// (relative error < 1.15e-9 across `(0, 1)`), refined by one Halley step.
pub fn inverse_normal_cdf(p: f64) -> f64 {
    if !(0.0..=1.0).contains(&p) {
        return f64::NAN;
    }
    if p <= 0.0 {
        return f64::NEG_INFINITY;
    }
    if p >= 1.0 {
        return f64::INFINITY;
    }
    const A: [f64; 6] = [
        -3.969_683_028_665_376e+01,
        2.209_460_984_245_205e+02,
        -2.759_285_104_469_687e+02,
        1.383_577_518_672_690e+02,
        -3.066_479_806_614_716e+01,
        2.506_628_277_459_239e+00,
    ];
    const B: [f64; 5] = [
        -5.447_609_879_822_406e+01,
        1.615_858_368_580_409e+02,
        -1.556_989_798_598_866e+02,
        6.680_131_188_771_972e+01,
        -1.328_068_155_288_572e+01,
    ];
    const C: [f64; 6] = [
        -7.784_894_002_430_293e-03,
        -3.223_964_580_411_365e-01,
        -2.400_758_277_161_838e+00,
        -2.549_732_539_343_734e+00,
        4.374_664_141_464_968e+00,
        2.938_163_982_698_783e+00,
    ];
    const D: [f64; 4] = [
        7.784_695_709_041_462e-03,
        3.224_671_290_700_398e-01,
        2.445_134_137_142_996e+00,
        3.754_408_661_907_416e+00,
    ];
    const P_LOW: f64 = 0.024_25;
    let x = if p < P_LOW {
        let q = (-2.0 * p.ln()).sqrt();
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else if p <= 1.0 - P_LOW {
        let q = p - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    };
    // 🔬 One Halley refinement step against the CDF for full double precision.
    let e = 0.5 * erfc(-x / core::f64::consts::SQRT_2) - p;
    let u = e * (2.0 * core::f64::consts::PI).sqrt() * (x * x / 2.0).exp();
    x - u / (1.0 + x * u / 2.0)
}

/// 🔬 Regularized lower incomplete gamma function `P(a, x)`, via series expansion for `x < a+1`
/// and a continued fraction for `x >= a+1` (standard Numerical Recipes split).
pub fn regularized_lower_incomplete_gamma(a: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x < a + 1.0 {
        let mut term = 1.0 / a;
        let mut sum = term;
        let mut n = a;
        for _ in 0..500 {
            n += 1.0;
            term *= x / n;
            sum += term;
            if term.abs() < sum.abs() * 1e-15 {
                break;
            }
        }
        sum * (-x + a * x.ln() - ln_gamma(a)).exp()
    } else {
        1.0 - regularized_upper_incomplete_gamma_cf(a, x)
    }
}

fn regularized_upper_incomplete_gamma_cf(a: f64, x: f64) -> f64 {
    let tiny = 1e-300;
    let mut b = x + 1.0 - a;
    let mut c = 1.0 / tiny;
    let mut d = 1.0 / b;
    let mut h = d;
    for i in 1..500 {
        let an = -(i as f64) * (i as f64 - a);
        b += 2.0;
        d = an * d + b;
        if d.abs() < tiny {
            d = tiny;
        }
        c = b + an / c;
        if c.abs() < tiny {
            c = tiny;
        }
        d = 1.0 / d;
        let delta = d * c;
        h *= delta;
        if (delta - 1.0).abs() < 1e-15 {
            break;
        }
    }
    (-x + a * x.ln() - ln_gamma(a)).exp() * h
}

/// 🔬 Regularized upper incomplete gamma function `Q(a, x) = 1 - P(a, x)`.
pub fn regularized_upper_incomplete_gamma(a: f64, x: f64) -> f64 {
    1.0 - regularized_lower_incomplete_gamma(a, x)
}

/// 🔬 Log-factorial via `ln_gamma(n + 1)`, cached for small `n` to avoid recomputation in hot
/// bias-correction loops (Grassberger, Schurmann-Grassberger).
pub struct LogFactorialCache {
    cache: Vec<f64>,
}

impl LogFactorialCache {
    /// 🔬 Precomputes `ln(k!)` for `k` in `0..=max_n`.
    pub fn new(max_n: usize) -> Self {
        let mut cache = Vec::with_capacity(max_n + 1);
        cache.push(0.0);
        for k in 1..=max_n {
            cache.push(cache[k - 1] + (k as f64).ln());
        }
        Self { cache }
    }

    /// 🔬 `ln(n!)`, falling back to `ln_gamma(n + 1)` beyond the cached range.
    pub fn get(&self, n: usize) -> f64 {
        match self.cache.get(n) {
            Some(&v) => v,
            None => ln_gamma(n as f64 + 1.0),
        }
    }
}
// #endregion 🔖SpecialFunctions

// #region 🔖StableSummation
/// 🔬 Neumaier's improved Kahan compensated summation: tracks a running compensation term so
/// that summing many small values alongside a few large ones does not lose precision.
pub fn neumaier_sum(values: impl IntoIterator<Item = f64>) -> f64 {
    let mut sum = 0.0_f64;
    let mut compensation = 0.0_f64;
    for value in values {
        let t = sum + value;
        if sum.abs() >= value.abs() {
            compensation += (sum - t) + value;
        } else {
            compensation += (value - t) + sum;
        }
        sum = t;
    }
    sum + compensation
}

/// 🔬 Pairwise (divide-and-conquer) summation: `O(log n)` error growth instead of `O(n)` for
/// naive left-to-right summation, useful for large flat arrays where compensated summation's
/// per-element overhead is undesirable.
pub fn pairwise_sum(values: &[f64]) -> f64 {
    const BASE_CASE: usize = 128;
    if values.len() <= BASE_CASE {
        return neumaier_sum(values.iter().copied());
    }
    let mid = values.len() / 2;
    pairwise_sum(&values[..mid]) + pairwise_sum(&values[mid..])
}

/// 🔬 Numerically stable `x * ln(x)`, defined as `0` at `x == 0` (the standard entropy
/// convention `0 log 0 = 0`, taken as a limit).
pub fn x_ln_x(x: f64) -> f64 {
    if x <= 0.0 {
        0.0
    } else {
        x * x.ln()
    }
}

/// 🔬 `log(sum(exp(values)))` computed without overflow by factoring out the maximum element.
/// Returns `f64::NEG_INFINITY` for an empty input.
pub fn log_sum_exp(values: &[f64]) -> f64 {
    let Some(&max) = values.iter().max_by(|a, b| a.total_cmp(b)) else {
        return f64::NEG_INFINITY;
    };
    if !max.is_finite() {
        return max;
    }
    let sum = neumaier_sum(values.iter().map(|&v| (v - max).exp()));
    max + sum.ln()
}
// #endregion 🔖StableSummation

// #region 🔖Rng
/// 🎲 Deterministic xorshift64* PRNG. Never seeded from wall-clock time — every call site that
/// needs randomness (bootstrap, surrogates, jitter, tests) takes an explicit `u64` seed so
/// results are exactly reproducible.
pub struct Xorshift64 {
    state: u64,
}

impl Xorshift64 {
    /// 🎲 Seeds the generator. A seed of `0` is remapped to a fixed nonzero constant since
    /// xorshift's all-zero state is a fixed point.
    pub fn new(seed: u64) -> Self {
        Self { state: if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed } }
    }

    /// 🎲 Next raw 64-bit output.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// 🎲 Uniform `f64` in `[0, 1)`.
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// 🎲 Uniform integer in `[0, bound)` via Lemire's rejection-free-in-practice method (biased
    /// only by a negligible `2^-64` amount, acceptable for permutation/bootstrap indices).
    pub fn next_below(&mut self, bound: usize) -> usize {
        if bound == 0 {
            return 0;
        }
        ((self.next_u64() as u128 * bound as u128) >> 64) as usize
    }

    /// 🎲 One standard-normal sample via Box-Muller (caller pays two uniform draws per call;
    /// simplicity over the two-sample-per-call optimization since callers rarely need high
    /// throughput Gaussian streams).
    pub fn next_gaussian(&mut self) -> f64 {
        let u1 = (self.next_f64()).max(f64::MIN_POSITIVE);
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (2.0 * core::f64::consts::PI * u2).cos()
    }

    /// 🎲 In-place Fisher-Yates shuffle.
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            let j = self.next_below(i + 1);
            slice.swap(i, j);
        }
    }
}
// #endregion 🔖Rng

// #region 🔖Hygiene
/// 🧹 Overflow-safe product of bin-count-like dimensions, returning `None` on overflow rather
/// than silently wrapping.
pub fn checked_state_count(dims: &[usize]) -> Option<u128> {
    dims.iter().try_fold(1u128, |acc, &d| acc.checked_mul(d as u128))
}

/// 🧹 Clips a near-zero-but-negative eigenvalue/entropy estimate to exactly `0.0` when it falls
/// within `tolerance` of zero; leaves values further negative untouched (callers decide whether
/// that indicates a real error).
pub fn clamp_near_zero(value: f64, tolerance: f64) -> f64 {
    if value < 0.0 && value >= -tolerance {
        0.0
    } else {
        value
    }
}
// #endregion 🔖Hygiene

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ln_gamma_matches_known_factorials() {
        for n in 1..10u32 {
            let expected = (1..n).map(|k| k as f64).product::<f64>().ln();
            assert!((ln_gamma(n as f64) - expected).abs() < 1e-9, "n={n}");
        }
    }

    #[test]
    fn ln_gamma_half_matches_sqrt_pi() {
        let expected = core::f64::consts::PI.sqrt().ln();
        assert!((ln_gamma(0.5) - expected).abs() < 1e-9);
    }

    #[test]
    fn digamma_matches_known_value_at_one() {
        // 🔬 psi(1) = -gamma (Euler-Mascheroni constant).
        let euler_mascheroni = 0.577_215_664_901_532_9;
        assert!((digamma(1.0) - (-euler_mascheroni)).abs() < 1e-9);
    }

    #[test]
    fn digamma_recurrence_holds() {
        let x = 3.7;
        assert!((digamma(x + 1.0) - (digamma(x) + 1.0 / x)).abs() < 1e-9);
    }

    #[test]
    fn trigamma_matches_pi_squared_over_six_at_one() {
        assert!((trigamma(1.0) - core::f64::consts::PI.powi(2) / 6.0).abs() < 1e-8);
    }

    #[test]
    fn erf_endpoints() {
        assert!((erf(0.0)).abs() < 1e-12);
        assert!((erf(10.0) - 1.0).abs() < 1e-12);
        assert!((erf(-10.0) + 1.0).abs() < 1e-12);
    }

    #[test]
    fn normal_cdf_at_zero_is_half() {
        assert!((normal_cdf(0.0) - 0.5).abs() < 1e-12);
    }

    #[test]
    fn inverse_normal_cdf_roundtrips_normal_cdf() {
        for p in [0.001, 0.01, 0.1, 0.3, 0.5, 0.7, 0.9, 0.99, 0.999] {
            let x = inverse_normal_cdf(p);
            let back = normal_cdf(x);
            assert!((back - p).abs() < 1e-8, "p={p} back={back}");
        }
    }

    #[test]
    fn incomplete_gamma_full_integral_is_one() {
        assert!((regularized_lower_incomplete_gamma(2.5, 1e6) - 1.0).abs() < 1e-9);
        assert!(regularized_upper_incomplete_gamma(2.5, 1e6) < 1e-9);
    }

    #[test]
    fn log_factorial_cache_matches_ln_gamma() {
        let cache = LogFactorialCache::new(20);
        for n in 0..20 {
            assert!((cache.get(n) - ln_gamma(n as f64 + 1.0)).abs() < 1e-9);
        }
    }

    #[test]
    fn neumaier_sum_exact_on_simple_case() {
        assert_eq!(neumaier_sum([1.0, 2.0, 3.0]), 6.0);
    }

    #[test]
    fn neumaier_more_accurate_than_naive_for_ill_conditioned_sum() {
        let mut values = vec![1e16, 1.0, -1e16];
        let naive: f64 = values.iter().sum();
        let stable = neumaier_sum(values.iter().copied());
        assert_eq!(naive, 0.0); // 🔬 naive loses the 1.0 entirely
        assert!((stable - 1.0).abs() < 1e-9);
        values.reverse();
        assert!((neumaier_sum(values) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn pairwise_sum_matches_neumaier_on_large_array() {
        let values: Vec<f64> = (0..10_000).map(|i| (i as f64).sin()).collect();
        let a = pairwise_sum(&values);
        let b = neumaier_sum(values.iter().copied());
        assert!((a - b).abs() < 1e-6);
    }

    #[test]
    fn x_ln_x_zero_at_zero() {
        assert_eq!(x_ln_x(0.0), 0.0);
        assert!((x_ln_x(1.0) - 0.0).abs() < 1e-12);
        assert!(x_ln_x(core::f64::consts::E) > 0.0);
    }

    #[test]
    fn log_sum_exp_matches_naive_for_moderate_values() {
        let values: [f64; 4] = [0.1, 0.5, -0.3, 0.2];
        let naive = values.iter().map(|v: &f64| v.exp()).sum::<f64>().ln();
        assert!((log_sum_exp(&values) - naive).abs() < 1e-9);
    }

    #[test]
    fn log_sum_exp_avoids_overflow() {
        let values = [1000.0, 1000.5, 999.0];
        let result = log_sum_exp(&values);
        assert!(result.is_finite());
        assert!(result > 1000.0 && result < 1002.0);
    }

    #[test]
    fn xorshift_is_deterministic_for_fixed_seed() {
        let mut a = Xorshift64::new(42);
        let mut b = Xorshift64::new(42);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn xorshift_uniform_range_respected() {
        let mut rng = Xorshift64::new(1234);
        for _ in 0..1000 {
            let v = rng.next_f64();
            assert!((0.0..1.0).contains(&v));
            let b = rng.next_below(7);
            assert!(b < 7);
        }
    }

    #[test]
    fn shuffle_is_a_permutation() {
        let mut rng = Xorshift64::new(99);
        let mut data: Vec<u32> = (0..50).collect();
        rng.shuffle(&mut data);
        let mut sorted = data.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..50).collect::<Vec<u32>>());
    }

    #[test]
    fn checked_state_count_detects_overflow() {
        assert_eq!(checked_state_count(&[2, 3, 4]), Some(24));
        assert_eq!(checked_state_count(&[usize::MAX, 2]), None);
    }

    #[test]
    fn clamp_near_zero_behavior() {
        assert_eq!(clamp_near_zero(-1e-14, 1e-12), 0.0);
        assert_eq!(clamp_near_zero(-1.0, 1e-12), -1.0);
        assert_eq!(clamp_near_zero(5.0, 1e-12), 5.0);
    }

    mod quick {
        use super::*;

        #[test]
        fn xorshift_gaussian_mean_and_variance_converge() {
            let mut rng = Xorshift64::new(7);
            let n = 20_000;
            let samples: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
            let mean = samples.iter().sum::<f64>() / n as f64;
            let var = samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;
            assert!(mean.abs() < 0.05, "mean={mean}");
            assert!((var - 1.0).abs() < 0.05, "var={var}");
        }

        #[test]
        fn digamma_asymptotic_region_matches_recurrence_shifted_from_small_x() {
            for x in [0.1, 0.5, 1.5, 5.9, 6.1, 50.0, 500.0] {
                let direct = digamma(x);
                let via_recurrence = digamma(x + 1.0) - 1.0 / x;
                assert!((direct - via_recurrence).abs() < 1e-8, "x={x}");
            }
        }
    }
}
// #endregion 🔖Tests
