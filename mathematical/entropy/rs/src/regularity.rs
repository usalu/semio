//! 🔁 Regularity/complexity measures over a single scalar time series: Approximate Entropy
//! (ApEn), Sample Entropy (SampEn), and Fuzzy Entropy (FuzzyEn). All three compare
//! time-delay-embedded template vectors (via [`crate::symbolic::embed`]) under a Chebyshev
//! tolerance radius `r` and differ only in how "matching" is counted and whether self-matches
//! are included — see each function's docstring for the exact convention.

use crate::numeric::neumaier_sum;
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Tolerance, Warning};

// #region 🔖Config
/// 🔁 Shared knobs for the ApEn/SampEn/FuzzyEn family: embedding dimension `m` and tolerance
/// radius `r`. The companion dimension `m + 1` is always derived internally.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RegularityConfig {
    pub m: usize,
    pub r: Tolerance,
}

impl RegularityConfig {
    /// 🔁 Validates `m >= 1` (a zero-length template is not a meaningful embedding). `r` is
    /// resolved later, once data is available, via [`resolve_tolerance`].
    pub fn new(m: usize, r: Tolerance) -> Result<Self, EntropyError> {
        if m < 1 {
            return Err(EntropyError::InvalidConfig {
                field: "m",
                reason: "embedding dimension must be at least 1",
            });
        }
        Ok(Self { m, r })
    }
}

/// 🔁 Sample standard deviation (`n - 1` denominator). Rejects a degenerate (constant, or
/// too-short-to-vary) series since it can never anchor a meaningful proportional tolerance.
fn sample_sd(x: &[f64]) -> Result<f64, EntropyError> {
    let n = x.len() as f64;
    if x.len() < 2 {
        return Err(EntropyError::DegenerateInput { what: "series has fewer than 2 points" });
    }
    let mean = neumaier_sum(x.iter().copied()) / n;
    let variance = neumaier_sum(x.iter().map(|&v| (v - mean).powi(2))) / (n - 1.0);
    if variance <= 0.0 {
        return Err(EntropyError::DegenerateInput { what: "constant series has zero standard deviation" });
    }
    Ok(variance.sqrt())
}

/// 🔁 Resolves a [`Tolerance`] policy into a concrete positive Chebyshev radius for `x`.
fn resolve_tolerance(x: &[f64], r: Tolerance) -> Result<f64, EntropyError> {
    match r {
        Tolerance::Absolute(v) => {
            if !(v.is_finite() && v > 0.0) {
                return Err(EntropyError::InvalidConfig { field: "r", reason: "absolute tolerance must be finite and positive" });
            }
            Ok(v)
        }
        Tolerance::RelativeToSd(k) => {
            if !(k.is_finite() && k > 0.0) {
                return Err(EntropyError::InvalidConfig { field: "r", reason: "sd multiplier must be finite and positive" });
            }
            Ok(k * sample_sd(x)?)
        }
        Tolerance::Auto => Ok(0.2 * sample_sd(x)?),
    }
}
// #endregion 🔖Config

// #region 🔖Distance
/// 🔁 Chebyshev (`L-infinity`) distance between two equal-length template vectors.
fn chebyshev(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(&x, &y)| (x - y).abs()).fold(0.0_f64, f64::max)
}

/// 🔁 Template minus its own mean (the Chen et al. fuzzy-entropy de-trending step).
fn demean(t: &[f64]) -> Vec<f64> {
    let mean = neumaier_sum(t.iter().copied()) / t.len() as f64;
    t.iter().map(|&v| v - mean).collect()
}

fn small_sample_warning(n: usize) -> Option<Warning> {
    (n < 100).then_some(Warning::SmallSample { n, recommended: 100 })
}
// #endregion 🔖Distance

// #region 🔖ApproximateEntropy
/// 🔁 `Phi(dim) = mean_i(ln(C_i))` where `C_i` counts matches INCLUDING the self-match `j == i` —
/// the defining (if statistically awkward) ApEn convention that SampEn below deliberately drops.
fn apen_phi(templates: &[Vec<f64>], r: f64) -> f64 {
    let n = templates.len() as f64;
    let ln_c: Vec<f64> = templates
        .iter()
        .map(|ti| {
            let matches = templates.iter().filter(|tj| chebyshev(ti, tj) <= r).count() as f64;
            (matches / n).ln()
        })
        .collect();
    neumaier_sum(ln_c) / n
}

/// 🔁 Approximate Entropy (Pincus 1991): `ApEn = Phi(m) - Phi(m + 1)`, in `base`. Self-matches are
/// counted, which biases ApEn low and dependent on `m` in a way SampEn was designed to fix.
pub fn approximate_entropy(x: &[f64], cfg: RegularityConfig, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if x.len() < cfg.m + 2 {
        return Err(EntropyError::InsufficientData {
            what: "approximate_entropy",
            needed: cfg.m + 2,
            actual: x.len(),
        });
    }
    let r = resolve_tolerance(x, cfg.r)?;
    let templates_m = crate::symbolic::embed(x, cfg.m, 1)?;
    let templates_m1 = crate::symbolic::embed(x, cfg.m + 1, 1)?;
    let apen_nats = apen_phi(&templates_m, r) - apen_phi(&templates_m1, r);

    let mut warnings = Vec::new();
    warnings.extend(small_sample_warning(x.len()));

    Ok(Estimate {
        value: base.from_nats(apen_nats),
        base,
        method: "approximate_entropy",
        n: x.len(),
        n_effective: x.len() as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("m", cfg.m as f64), ("r", r)],
    })
}
// #endregion 🔖ApproximateEntropy

// #region 🔖SharedEmbedding
/// 🔁 Embeds at `m + 1` first to fix the valid start-index range, then embeds at `m` and
/// truncates to that SAME range, so SampEn/FuzzyEn compare the two lengths over identical
/// windows rather than the (larger) index range `m`-only embedding would otherwise allow.
fn shared_templates(x: &[f64], m: usize) -> Result<(Vec<Vec<f64>>, Vec<Vec<f64>>), EntropyError> {
    let templates_m1 = crate::symbolic::embed(x, m + 1, 1)?;
    let k = templates_m1.len();
    let mut templates_m = crate::symbolic::embed(x, m, 1)?;
    templates_m.truncate(k);
    Ok((templates_m, templates_m1))
}
// #endregion 🔖SharedEmbedding

// #region 🔖SampleEntropy
/// 🔁 Sample Entropy (Richman & Moorman 2000): `SampEn = -ln(A / B)`, where `B` counts
/// length-`m` matches and `A` counts length-`(m + 1)` matches among the SAME index pairs
/// `i < j` (self-matches structurally excluded, unlike ApEn). `B == 0` is undefined; `A == 0`
/// is reported as `+infinity` with a diagnostic warning rather than erroring.
pub fn sample_entropy(x: &[f64], cfg: RegularityConfig, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if x.len() < cfg.m + 2 {
        return Err(EntropyError::InsufficientData { what: "sample_entropy", needed: cfg.m + 2, actual: x.len() });
    }
    let r = resolve_tolerance(x, cfg.r)?;
    let (templates_m, templates_m1) = shared_templates(x, cfg.m)?;
    let k = templates_m1.len();

    let mut a: u64 = 0;
    let mut b: u64 = 0;
    for i in 0..k {
        for j in (i + 1)..k {
            if chebyshev(&templates_m[i], &templates_m[j]) <= r {
                b += 1;
                if chebyshev(&templates_m1[i], &templates_m1[j]) <= r {
                    a += 1;
                }
            }
        }
    }

    if b == 0 {
        return Err(EntropyError::UndefinedResult { reason: "no length-m template matches; sample entropy is undefined" });
    }

    let mut warnings = Vec::new();
    warnings.extend(small_sample_warning(x.len()));
    let sampen_nats = if a == 0 {
        warnings.push(Warning::NotConvergedSoft { what: "no length-(m+1) matches; SampEn is infinite" });
        f64::INFINITY
    } else {
        -((a as f64) / (b as f64)).ln()
    };

    Ok(Estimate {
        value: base.from_nats(sampen_nats),
        base,
        method: "sample_entropy",
        n: x.len(),
        n_effective: x.len() as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("m", cfg.m as f64), ("r", r)],
    })
}
// #endregion 🔖SampleEntropy

// #region 🔖FuzzyEntropy
/// 🔁 `Phi(dim) = (1 / (K*(K-1))) * sum_i sum_{j != i} mu(Chebyshev(T_i - mean(T_i), T_j -
/// mean(T_j)))` with Gaussian membership `mu(d) = exp(-(d/r)^2)`, replacing SampEn/ApEn's hard
/// `<= r` indicator with a smooth one (Chen et al. 2007).
fn fuzzy_phi(templates: &[Vec<f64>], r: f64) -> f64 {
    let k = templates.len();
    let demeaned: Vec<Vec<f64>> = templates.iter().map(|t| demean(t)).collect();
    let mut terms = Vec::with_capacity(k.saturating_mul(k.saturating_sub(1)));
    for i in 0..k {
        for j in 0..k {
            if i == j {
                continue;
            }
            let d = chebyshev(&demeaned[i], &demeaned[j]);
            terms.push((-(d / r).powi(2)).exp());
        }
    }
    neumaier_sum(terms) / (k as f64 * (k as f64 - 1.0))
}

/// 🔁 Fuzzy Entropy: like [`sample_entropy`] but with a Gaussian-membership match indicator and
/// per-template mean removal, `FuzzyEn = ln(Phi(m)) - ln(Phi(m + 1))`, in `base`.
pub fn fuzzy_entropy(x: &[f64], cfg: RegularityConfig, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if x.len() < cfg.m + 2 {
        return Err(EntropyError::InsufficientData { what: "fuzzy_entropy", needed: cfg.m + 2, actual: x.len() });
    }
    let r = resolve_tolerance(x, cfg.r)?;
    let (templates_m, templates_m1) = shared_templates(x, cfg.m)?;
    let phi_m = fuzzy_phi(&templates_m, r);
    let phi_m1 = fuzzy_phi(&templates_m1, r);
    if phi_m <= 0.0 || phi_m1 <= 0.0 {
        return Err(EntropyError::UndefinedResult { reason: "fuzzy membership sum is non-positive" });
    }
    let fuzzyen_nats = phi_m.ln() - phi_m1.ln();

    let mut warnings = Vec::new();
    warnings.extend(small_sample_warning(x.len()));

    Ok(Estimate {
        value: base.from_nats(fuzzyen_nats),
        base,
        method: "fuzzy_entropy",
        n: x.len(),
        n_effective: x.len() as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("m", cfg.m as f64), ("r", r)],
    })
}
// #endregion 🔖FuzzyEntropy

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::numeric::Xorshift64;

    fn sine_series(n: usize, period: f64) -> Vec<f64> {
        (0..n).map(|i| (2.0 * core::f64::consts::PI * i as f64 / period).sin()).collect()
    }

    fn white_noise_series(n: usize, seed: u64) -> Vec<f64> {
        let mut rng = Xorshift64::new(seed);
        (0..n).map(|_| rng.next_gaussian()).collect()
    }

    #[test]
    fn regularity_config_rejects_m_zero() {
        assert!(matches!(
            RegularityConfig::new(0, Tolerance::Auto),
            Err(EntropyError::InvalidConfig { field: "m", .. })
        ));
        assert!(RegularityConfig::new(1, Tolerance::Auto).is_ok());
    }

    #[test]
    fn resolve_tolerance_auto_matches_hand_computation() {
        // 🔐 [1,2,3,4,5]: mean=3, sample variance=(4+1+0+1+4)/4=2.5, sd=sqrt(2.5).
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        let expected = 0.2 * 2.5_f64.sqrt();
        let r = resolve_tolerance(&x, Tolerance::Auto).unwrap();
        assert!((r - expected).abs() < 1e-12, "r={r} expected={expected}");
    }

    #[test]
    fn resolve_tolerance_rejects_constant_series() {
        let x = [5.0; 10];
        assert!(matches!(resolve_tolerance(&x, Tolerance::Auto), Err(EntropyError::DegenerateInput { .. })));
        assert!(matches!(
            resolve_tolerance(&x, Tolerance::RelativeToSd(1.0)),
            Err(EntropyError::DegenerateInput { .. })
        ));
    }

    #[test]
    fn sample_entropy_rejects_very_short_series() {
        let cfg = RegularityConfig::new(2, Tolerance::Auto).unwrap();
        let x = [1.0, 2.0, 3.0];
        assert!(matches!(
            sample_entropy(&x, cfg, LogBase::Nats),
            Err(EntropyError::InsufficientData { .. })
        ));
    }

    #[test]
    fn sample_entropy_reports_infinity_when_no_higher_order_matches_exist() {
        // 🔐 Hand-verified: at m=1 the pairs (0,1),(0,3),(1,3) match within r=1.0, but every one
        // of those pairs diverges by 4-8 at m+1=2, so A=0 while B=3 — SampEn must be +infinity.
        let x = [1.0, 1.0, 5.0, 1.0, 9.0];
        let cfg = RegularityConfig::new(1, Tolerance::Absolute(1.0)).unwrap();
        let est = sample_entropy(&x, cfg, LogBase::Nats).unwrap();
        assert_eq!(est.value, f64::INFINITY);
        assert!(est.warnings.iter().any(|w| matches!(w, Warning::NotConvergedSoft { .. })));
    }

    #[test]
    fn near_constant_signal_under_generous_tolerance_has_near_zero_regularity_entropy() {
        // 🔐 A constant plus a 1e-10-scale perturbation, compared against a tolerance orders of
        // magnitude larger than the perturbation: virtually every template matches every other
        // template at both m and m+1, so Phi(m) ~= Phi(m+1) and all three measures collapse to
        // ~0 — the "this series is maximally predictable at this resolution" case.
        let mut rng = Xorshift64::new(11);
        let x: Vec<f64> = (0..300).map(|_| 5.0 + 1e-10 * rng.next_f64()).collect();
        let cfg = RegularityConfig::new(2, Tolerance::Absolute(0.5)).unwrap();
        let apen = approximate_entropy(&x, cfg, LogBase::Nats).unwrap();
        let sampen = sample_entropy(&x, cfg, LogBase::Nats).unwrap();
        assert!(apen.value.abs() < 1e-6, "apen={}", apen.value);
        assert!(sampen.value.abs() < 1e-6, "sampen={}", sampen.value);
    }

    #[test]
    fn regular_sine_has_much_lower_regularity_entropy_than_white_noise() {
        // 🔐 THE canonical ApEn/SampEn/FuzzyEn sanity check: a smooth periodic signal is far more
        // "regular" (predictable from its own past) than i.i.d. noise of the same length, so all
        // three measures must be substantially lower on the sine than on the noise.
        let n = 1000;
        let sine = sine_series(n, 50.0);
        let noise = white_noise_series(n, 42);
        let cfg = RegularityConfig::new(2, Tolerance::Auto).unwrap();

        let apen_sine = approximate_entropy(&sine, cfg, LogBase::Nats).unwrap().value;
        let apen_noise = approximate_entropy(&noise, cfg, LogBase::Nats).unwrap().value;
        assert!(apen_sine < apen_noise - 0.5, "apen_sine={apen_sine} apen_noise={apen_noise}");

        let sampen_sine = sample_entropy(&sine, cfg, LogBase::Nats).unwrap().value;
        let sampen_noise = sample_entropy(&noise, cfg, LogBase::Nats).unwrap().value;
        assert!(sampen_sine < sampen_noise - 0.5, "sampen_sine={sampen_sine} sampen_noise={sampen_noise}");

        let fuzzyen_sine = fuzzy_entropy(&sine, cfg, LogBase::Nats).unwrap().value;
        let fuzzyen_noise = fuzzy_entropy(&noise, cfg, LogBase::Nats).unwrap().value;
        assert!(fuzzyen_sine < fuzzyen_noise - 0.5, "fuzzyen_sine={fuzzyen_sine} fuzzyen_noise={fuzzyen_noise}");
    }

    #[test]
    fn approximate_entropy_small_sample_warning() {
        let cfg = RegularityConfig::new(2, Tolerance::Auto).unwrap();
        let x = sine_series(30, 10.0);
        let est = approximate_entropy(&x, cfg, LogBase::Nats).unwrap();
        assert!(est.warnings.iter().any(|w| matches!(w, Warning::SmallSample { .. })));
    }

    #[test]
    fn base_conversion_is_consistent_across_apen_sampen_fuzzyen() {
        let cfg = RegularityConfig::new(2, Tolerance::Auto).unwrap();
        let x = sine_series(200, 25.0);
        for est in [
            approximate_entropy(&x, cfg, LogBase::Bits).unwrap(),
            sample_entropy(&x, cfg, LogBase::Bits).unwrap(),
            fuzzy_entropy(&x, cfg, LogBase::Bits).unwrap(),
        ] {
            let nats = est.nats();
            let back = LogBase::convert(nats, LogBase::Nats, LogBase::Bits);
            assert!((back - est.value).abs() < 1e-9);
        }
    }

    mod quick {
        use super::*;

        #[test]
        fn fuzzy_entropy_finite_and_defined_for_moderate_series() {
            let cfg = RegularityConfig::new(2, Tolerance::Auto).unwrap();
            let x = white_noise_series(500, 7);
            let est = fuzzy_entropy(&x, cfg, LogBase::Nats).unwrap();
            assert!(est.value.is_finite());
        }
    }
}
// #endregion 🔖Tests
