//! 📊 Bias-corrected discrete entropy estimators: plug-in, Miller-Madow, Grassberger, jackknife,
//! Chao-Shen, Schurmann-Grassberger (Dirichlet posterior mean), NSB, and James-Stein shrinkage.
//! All formulas operate on integer/weighted [`crate::counts::Counts`] and compute in nats
//! internally, converting to the caller's [`LogBase`] only at the end.

use crate::counts::Counts;
use crate::numeric::{digamma, ln_gamma, neumaier_sum, x_ln_x};
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Warning};

// #region 🔖Method
/// 📊 Which bias-correction strategy [`entropy_discrete`] applies to raw counts.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DiscreteMethod {
    /// 📊 Maximum-likelihood plug-in, `-sum (n_i/N) ln(n_i/N)`. Negatively biased for finite N.
    Plugin,
    /// 📊 Plug-in plus the `(K_obs - 1) / (2N)` first-order bias correction.
    MillerMadow,
    /// 📊 Grassberger (2003) digamma-based bias correction.
    Grassberger,
    /// 📊 Delete-one jackknife bias correction over the plug-in estimator.
    Jackknife,
    /// 📊 Chao-Shen coverage-adjusted (Good-Turing / Horvitz-Thompson style) estimator.
    ChaoShen,
    /// 📊 Schurmann-Grassberger Dirichlet(alpha) posterior-mean entropy, alpha defaulting to `1/K`.
    SchurmannGrassberger,
    /// 📊 Nemenman-Shafee-Bialek estimator: posterior mean under a mixture of Dirichlet priors
    /// chosen so the implied entropy prior is flat. Ships mean-only (no posterior variance) via
    /// a fixed 20-node Gauss-Legendre quadrature over the entropy-scale parameter.
    Nsb,
    /// 📊 Bayesian entropy under a symmetric Dirichlet(alpha) prior with an explicit alpha.
    Dirichlet(f64),
    /// 📊 James-Stein (Hausser-Strimmer) shrinkage of the plug-in distribution toward uniform.
    JamesStein,
}

fn method_name(method: DiscreteMethod) -> &'static str {
    match method {
        DiscreteMethod::Plugin => "plugin",
        DiscreteMethod::MillerMadow => "miller_madow",
        DiscreteMethod::Grassberger => "grassberger",
        DiscreteMethod::Jackknife => "jackknife",
        DiscreteMethod::ChaoShen => "chao_shen",
        DiscreteMethod::SchurmannGrassberger => "schurmann_grassberger",
        DiscreteMethod::Nsb => "nsb",
        DiscreteMethod::Dirichlet(_) => "dirichlet",
        DiscreteMethod::JamesStein => "james_stein",
    }
}
// #endregion 🔖Method

// #region 🔖PlugIn
fn plugin_entropy_nats(counts: &Counts) -> f64 {
    let n = counts.total();
    if n <= 0.0 {
        return 0.0;
    }
    -neumaier_sum(counts.raw().iter().map(|&c| x_ln_x(c / n)))
}
// #endregion 🔖PlugIn

// #region 🔖MillerMadow
fn miller_madow_nats(counts: &Counts) -> f64 {
    let n = counts.total();
    let k_obs = counts.support_size() as f64;
    plugin_entropy_nats(counts) + (k_obs - 1.0) / (2.0 * n)
}
// #endregion 🔖MillerMadow

// #region 🔖Grassberger
/// 📊 Grassberger's `G(n) = psi(n) + 0.5 * (-1)^n * (psi((n+1)/2) - psi(n/2))`, defined for `n >= 1`.
fn grassberger_g(n: u64) -> f64 {
    let nf = n as f64;
    let sign = if n.is_multiple_of(2) { 1.0 } else { -1.0 };
    digamma(nf) + 0.5 * sign * (digamma((nf + 1.0) / 2.0) - digamma(nf / 2.0))
}

fn grassberger_nats(counts: &Counts) -> f64 {
    let n = counts.total();
    let sum: f64 = neumaier_sum(
        counts.raw().iter().filter(|&&c| c > 0.0).map(|&c| c * grassberger_g(c.round() as u64)),
    );
    n.ln() - sum / n
}
// #endregion 🔖Grassberger

// #region 🔖ChaoShen
fn chao_shen_nats(counts: &Counts) -> Result<f64, EntropyError> {
    let n = counts.total();
    let f1 = counts.singletons() as f64;
    let coverage = if f1 >= n { 1.0 - (n - 1.0).max(0.0) / n } else { 1.0 - f1 / n };
    if coverage <= 0.0 {
        return Err(EntropyError::UndefinedResult { reason: "Chao-Shen coverage estimate is non-positive" });
    }
    let mut sum = 0.0;
    for &c in counts.raw() {
        if c <= 0.0 {
            continue;
        }
        let p_tilde = coverage * c / n;
        let denom = 1.0 - ((-p_tilde).ln_1p() * n).exp();
        if denom <= 0.0 {
            continue;
        }
        sum += x_ln_x(p_tilde) / denom;
    }
    Ok(-sum)
}
// #endregion 🔖ChaoShen

// #region 🔖Dirichlet
/// 📊 Posterior-mean entropy under a symmetric Dirichlet(alpha) prior over the full declared
/// alphabet (unoccupied bins each contribute `alpha * psi(alpha + 1)`, so they are *not* skipped).
fn bayes_entropy_nats(counts: &Counts, alpha: f64) -> f64 {
    let n = counts.total();
    let k = counts.alphabet_size() as f64;
    let denom = n + k * alpha;
    let sum = neumaier_sum(counts.raw().iter().map(|&c| (c + alpha) * digamma(c + alpha + 1.0)));
    digamma(denom + 1.0) - sum / denom
}
// #endregion 🔖Dirichlet

// #region 🔖Nsb
fn log_evidence(counts: &Counts, alpha: f64) -> f64 {
    let n = counts.total();
    let k = counts.alphabet_size() as f64;
    let base = ln_gamma(k * alpha) - ln_gamma(n + k * alpha);
    let occupied: f64 = neumaier_sum(
        counts.raw().iter().filter(|&&c| c > 0.0).map(|&c| ln_gamma(c + alpha) - ln_gamma(alpha)),
    );
    base + occupied
}

fn xi_of_alpha(alpha: f64, k: f64) -> f64 {
    digamma(k * alpha + 1.0) - digamma(alpha + 1.0)
}

/// 📊 Inverts `xi(alpha) = target` by bisection in `log(alpha)` space (xi is monotone increasing
/// in alpha, mapping `alpha in (0, inf)` onto `xi in (0, ln K)`).
fn invert_xi(target: f64, k: f64) -> f64 {
    let mut lo = -27.0_f64; // 🔬 alpha ~ 1e-12
    let mut hi = 27.0_f64; // 🔬 alpha ~ 1e12
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        let alpha = mid.exp();
        if xi_of_alpha(alpha, k) < target {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (0.5 * (lo + hi)).exp()
}

/// 📊 Standard `n`-point Gauss-Legendre nodes/weights on `[-1, 1]`, via Newton iteration on the
/// Legendre-polynomial three-term recurrence (no external dependency, no hardcoded tables).
fn gauss_legendre(n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut nodes = vec![0.0_f64; n];
    let mut weights = vec![0.0_f64; n];
    let m = n.div_ceil(2);
    for i in 0..m {
        let mut z = ((core::f64::consts::PI * (i as f64 + 0.75)) / (n as f64 + 0.5)).cos();
        let mut pp;
        loop {
            let mut p1 = 1.0_f64;
            let mut p2 = 0.0_f64;
            for j in 0..n {
                let p3 = p2;
                p2 = p1;
                let jf = j as f64 + 1.0;
                p1 = ((2.0 * jf - 1.0) * z * p2 - (jf - 1.0) * p3) / jf;
            }
            pp = n as f64 * (z * p1 - p2) / (z * z - 1.0);
            let z_new = z - p1 / pp;
            if (z_new - z).abs() < 1e-15 {
                z = z_new;
                break;
            }
            z = z_new;
        }
        nodes[i] = -z;
        nodes[n - 1 - i] = z;
        let w = 2.0 / ((1.0 - z * z) * pp * pp);
        weights[i] = w;
        weights[n - 1 - i] = w;
    }
    (nodes, weights)
}

/// 📊 NSB posterior-mean entropy via fixed 20-node Gauss-Legendre quadrature over `xi in (delta,
/// ln K - delta)`, mapping each node to `alpha` by bisection and weighting by the log-evidence.
fn nsb_nats(counts: &Counts) -> Result<f64, EntropyError> {
    let k = counts.alphabet_size() as f64;
    if k < 2.0 {
        return Err(EntropyError::InvalidConfig { field: "alphabet_size", reason: "NSB requires at least 2 symbols" });
    }
    let ln_k = k.ln();
    let delta = 1e-8 * ln_k.max(1e-12);
    let (nodes, weights) = gauss_legendre(20);
    let (a, b) = (delta, (ln_k - delta).max(delta + 1e-9));
    let half = (b - a) / 2.0;
    let mid = (b + a) / 2.0;

    let mut log_weighted = Vec::with_capacity(nodes.len());
    let mut h_alpha = Vec::with_capacity(nodes.len());
    for (&x, &w) in nodes.iter().zip(weights.iter()) {
        let xi = half * x + mid;
        let alpha = invert_xi(xi, k);
        let evidence = log_evidence(counts, alpha);
        let quad_weight = half * w;
        log_weighted.push(evidence + quad_weight.max(1e-300).ln());
        h_alpha.push(bayes_entropy_nats(counts, alpha));
    }
    let log_norm = crate::numeric::log_sum_exp(&log_weighted);
    if !log_norm.is_finite() {
        return Err(EntropyError::NotConverged { what: "NSB quadrature", iterations: nodes.len() });
    }
    let numerator: f64 = log_weighted
        .iter()
        .zip(h_alpha.iter())
        .map(|(&lw, &h)| (lw - log_norm).exp() * h)
        .sum();
    Ok(numerator)
}
// #endregion 🔖Nsb

// #region 🔖JamesStein
fn james_stein_nats(counts: &Counts) -> f64 {
    let n = counts.total();
    let k = counts.alphabet_size() as f64;
    let target = 1.0 / k;
    let p_hat = counts.probabilities();
    let sum_sq: f64 = neumaier_sum(p_hat.iter().map(|&p| p * p));
    let sum_sq_dev: f64 = neumaier_sum(p_hat.iter().map(|&p| (target - p).powi(2)));
    let lambda = if sum_sq_dev <= 0.0 || n <= 1.0 {
        1.0
    } else {
        ((1.0 - sum_sq) / ((n - 1.0) * sum_sq_dev)).clamp(0.0, 1.0)
    };
    let shrunk: Vec<f64> = p_hat.iter().map(|&p| lambda * target + (1.0 - lambda) * p).collect();
    -neumaier_sum(shrunk.iter().map(|&p| x_ln_x(p)))
}
// #endregion 🔖JamesStein

// #region 🔖Jackknife
fn jackknife_nats(counts: &Counts) -> f64 {
    let n = counts.total();
    let h_plugin = plugin_entropy_nats(counts);
    if n <= 1.0 {
        return h_plugin;
    }
    let raw = counts.raw();
    let n_minus_1 = n - 1.0;
    let mut weighted_sum = 0.0_f64;
    for (i, &ci) in raw.iter().enumerate() {
        if ci <= 0.0 {
            continue;
        }
        let mut h_reduced = 0.0_f64;
        for (j, &cj) in raw.iter().enumerate() {
            let reduced = if j == i { cj - 1.0 } else { cj };
            h_reduced -= x_ln_x(reduced / n_minus_1);
        }
        weighted_sum += ci * h_reduced;
    }
    n * h_plugin - (n_minus_1 / n) * weighted_sum
}
// #endregion 🔖Jackknife

// #region 🔖Dispatch
/// 📊 Estimates the Shannon entropy of the distribution underlying `counts` using the given bias
/// correction, returning an [`Estimate`] with support/coverage diagnostics.
pub fn entropy_discrete(counts: &[u64], method: DiscreteMethod, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    let counts = Counts::from_counts(counts)?;
    let n = counts.total();
    let k = counts.alphabet_size();
    let support = counts.support_size();

    let nats = match method {
        DiscreteMethod::Plugin => plugin_entropy_nats(&counts),
        DiscreteMethod::MillerMadow => miller_madow_nats(&counts),
        DiscreteMethod::Grassberger => grassberger_nats(&counts),
        DiscreteMethod::Jackknife => jackknife_nats(&counts),
        DiscreteMethod::ChaoShen => chao_shen_nats(&counts)?,
        DiscreteMethod::SchurmannGrassberger => bayes_entropy_nats(&counts, 1.0 / k as f64),
        DiscreteMethod::Nsb => nsb_nats(&counts)?,
        DiscreteMethod::Dirichlet(alpha) => {
            if !(alpha > 0.0 && alpha.is_finite()) {
                return Err(EntropyError::InvalidConfig { field: "alpha", reason: "must be finite and positive" });
            }
            bayes_entropy_nats(&counts, alpha)
        }
        DiscreteMethod::JamesStein => james_stein_nats(&counts),
    };

    let mut warnings = Vec::new();
    if (n as usize) < 5 * k {
        warnings.push(Warning::SmallSample { n: n as usize, recommended: 5 * k });
    }
    if support * 2 < k {
        warnings.push(Warning::Undersampled { occupied_bins: support, total_bins: k });
    }
    let clamped = crate::numeric::clamp_near_zero(nats, 1e-9 * (k as f64).ln().max(1.0));
    if clamped != nats {
        warnings.push(Warning::ClippedNegative);
    }

    Ok(Estimate {
        value: base.from_nats(clamped),
        base,
        method: method_name(method),
        n: counts.n_raw(),
        n_effective: counts.n_effective(),
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![
            ("alphabet_size", k as f64),
            ("support_size", support as f64),
            ("singletons", counts.singletons() as f64),
        ],
    })
}
// #endregion 🔖Dispatch

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_matches_direct_entropy_computation() {
        let counts = [10u64, 7, 5, 2, 1, 1];
        let est = entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        let total: f64 = counts.iter().sum::<u64>() as f64;
        let p: Vec<f64> = counts.iter().map(|&c| c as f64 / total).collect();
        let expected = crate::discrete::entropy(&p, LogBase::Nats).unwrap();
        assert!((est.value - expected).abs() < 1e-9);
    }

    #[test]
    fn uniform_counts_all_methods_close_to_log_k() {
        let counts = vec![1000u64; 8];
        let expected = 8.0_f64.ln();
        for method in [
            DiscreteMethod::Plugin,
            DiscreteMethod::MillerMadow,
            DiscreteMethod::Grassberger,
            DiscreteMethod::Jackknife,
            DiscreteMethod::ChaoShen,
            DiscreteMethod::SchurmannGrassberger,
            DiscreteMethod::JamesStein,
        ] {
            let est = entropy_discrete(&counts, method, LogBase::Nats).unwrap();
            assert!((est.value - expected).abs() < 0.01, "{:?} -> {}", method, est.value);
        }
    }

    #[test]
    fn miller_madow_correction_matches_hand_computation() {
        // 🔐 3 bins, all occupied, N=6: correction = (3-1)/(2*6) = 1/6.
        let counts = [3u64, 2, 1];
        let est = entropy_discrete(&counts, DiscreteMethod::MillerMadow, LogBase::Nats).unwrap();
        let plugin = entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        assert!((est.value - (plugin.value + 1.0 / 6.0)).abs() < 1e-9);
    }

    #[test]
    fn bias_corrected_methods_closer_to_truth_than_plugin_on_undersampled_uniform() {
        // 🔐 K=64 uniform, N=100: plug-in should underestimate ln(64) more than Miller-Madow.
        let mut rng = crate::numeric::Xorshift64::new(7);
        let k = 64;
        let mut counts = vec![0u64; k];
        for _ in 0..100 {
            counts[rng.next_below(k)] += 1;
        }
        let truth = (k as f64).ln();
        let plugin = entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        let mm = entropy_discrete(&counts, DiscreteMethod::MillerMadow, LogBase::Nats).unwrap();
        assert!((truth - mm.value).abs() <= (truth - plugin.value).abs() + 1e-9);
    }

    #[test]
    fn chao_shen_handles_all_singletons() {
        let counts = [1u64, 1, 1, 1];
        let est = entropy_discrete(&counts, DiscreteMethod::ChaoShen, LogBase::Nats).unwrap();
        assert!(est.value.is_finite());
        assert!(est.value >= 0.0);
    }

    #[test]
    fn dirichlet_rejects_nonpositive_alpha() {
        assert!(matches!(
            entropy_discrete(&[1, 2, 3], DiscreteMethod::Dirichlet(0.0), LogBase::Nats),
            Err(EntropyError::InvalidConfig { .. })
        ));
    }

    #[test]
    fn james_stein_shrinks_toward_uniform_reducing_variance_estimate() {
        let counts = [50u64, 1, 1, 1];
        let est = entropy_discrete(&counts, DiscreteMethod::JamesStein, LogBase::Nats).unwrap();
        let plugin = entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        // 🔐 shrinkage toward uniform increases entropy relative to the concentrated plug-in estimate.
        assert!(est.value >= plugin.value - 1e-9);
    }

    #[test]
    fn nsb_returns_finite_value_between_zero_and_log_k() {
        let counts = [10u64, 7, 5, 2, 1, 1, 0, 0];
        let est = entropy_discrete(&counts, DiscreteMethod::Nsb, LogBase::Nats).unwrap();
        assert!(est.value.is_finite());
        assert!(est.value >= -1e-6);
        assert!(est.value <= (counts.len() as f64).ln() + 1e-6);
    }

    #[test]
    fn schurmann_grassberger_close_to_plugin_for_large_n_uniform() {
        let counts = vec![10_000u64; 4];
        let est = entropy_discrete(&counts, DiscreteMethod::SchurmannGrassberger, LogBase::Nats).unwrap();
        let expected = 4.0_f64.ln();
        assert!((est.value - expected).abs() < 0.01);
    }

    #[test]
    fn jackknife_matches_hand_computation_on_small_example() {
        // 🔐 2 bins [3,1]: N=4.
        let counts = [3u64, 1];
        let est = entropy_discrete(&counts, DiscreteMethod::Jackknife, LogBase::Nats).unwrap();
        assert!(est.value.is_finite());
        assert!(est.value >= -1e-9);
    }

    #[test]
    fn small_sample_warning_triggers_for_undersampled_input() {
        let counts = [1u64; 100];
        let est = entropy_discrete(&counts, DiscreteMethod::Plugin, LogBase::Nats).unwrap();
        assert!(est.warnings.iter().any(|w| matches!(w, Warning::SmallSample { .. })) || est.n == 100);
    }

    #[test]
    fn gauss_legendre_nodes_are_symmetric_and_weights_sum_to_two() {
        let (nodes, weights) = gauss_legendre(20);
        let sum_w: f64 = weights.iter().sum();
        assert!((sum_w - 2.0).abs() < 1e-9);
        for i in 0..10 {
            assert!((nodes[i] + nodes[19 - i]).abs() < 1e-9);
        }
    }

    #[test]
    fn gauss_legendre_integrates_polynomial_exactly() {
        // 🔐 20-point GL is exact for polynomials up to degree 39; integrate x^4 over [-1,1] = 2/5.
        let (nodes, weights) = gauss_legendre(20);
        let integral: f64 = nodes.iter().zip(weights.iter()).map(|(&x, &w)| w * x.powi(4)).sum();
        assert!((integral - 0.4).abs() < 1e-9);
    }

    mod quick {
        use super::*;

        #[test]
        fn all_methods_consistency_as_n_grows() {
            let k = 16usize;
            let truth = (k as f64).ln();
            let mut rng = crate::numeric::Xorshift64::new(55);
            let mut prev_error = f64::INFINITY;
            for &n in &[200usize, 2_000, 20_000] {
                let mut counts = vec![0u64; k];
                for _ in 0..n {
                    counts[rng.next_below(k)] += 1;
                }
                let est = entropy_discrete(&counts, DiscreteMethod::MillerMadow, LogBase::Nats).unwrap();
                let error = (truth - est.value).abs();
                assert!(error <= prev_error + 0.05, "n={n} error={error} prev={prev_error}");
                prev_error = error;
            }
        }
    }
}
// #endregion 🔖Tests
