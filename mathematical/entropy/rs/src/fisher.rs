//! 📉 Fisher information and information-criterion model-selection scores. Fisher information is
//! recovered numerically from a supplied log-likelihood via a central second difference; the
//! criteria (AIC/AICc/BIC/HQC/MDL) are closed-form penalized-likelihood scores taking an
//! already-computed `ln_L` and parameter/sample counts — no external crate, no `Estimate` wrapper
//! since these are exact-given-inputs, not estimated-from-samples quantities.

use crate::EntropyError;

// #region 🔖FisherInformation
/// 📉 Numerical (observed) Fisher information via a central second-difference of the supplied
/// log-likelihood function at `theta`: `-(L(theta+h) - 2*L(theta) + L(theta-h)) / h^2`, with a
/// fixed step `h = 1e-4 * theta.abs().max(1.0)` (scale-aware so it works across parameter
/// magnitudes).
pub fn fisher_information(log_likelihood: impl Fn(f64) -> f64, theta: f64) -> f64 {
    let h = 1e-4 * theta.abs().max(1.0);
    let forward = log_likelihood(theta + h);
    let center = log_likelihood(theta);
    let backward = log_likelihood(theta - h);
    -(forward - 2.0 * center + backward) / (h * h)
}
// #endregion 🔖FisherInformation

// #region 🔖Criteria
/// 📉 Akaike information criterion: `2*k - 2*ln_L`.
pub fn aic(log_likelihood: f64, num_params: usize) -> f64 {
    2.0 * num_params as f64 - 2.0 * log_likelihood
}

/// 📉 Corrected AIC for small sample sizes: `aic + (2*k*(k+1)) / (n - k - 1)`. Errors with
/// [`EntropyError::InvalidConfig`] if `n <= k + 1` (undefined).
pub fn aicc(log_likelihood: f64, num_params: usize, n: usize) -> Result<f64, EntropyError> {
    let k = num_params as f64;
    let n = n as f64;
    if n <= k + 1.0 {
        return Err(EntropyError::InvalidConfig {
            field: "n",
            reason: "AICc is undefined when n <= num_params + 1",
        });
    }
    Ok(aic(log_likelihood, num_params) + (2.0 * k * (k + 1.0)) / (n - k - 1.0))
}

/// 📉 Bayesian information criterion: `k * ln(n) - 2*ln_L`.
pub fn bic(log_likelihood: f64, num_params: usize, n: usize) -> f64 {
    num_params as f64 * (n as f64).ln() - 2.0 * log_likelihood
}

/// 📉 Hannan-Quinn criterion: `2*k*ln(ln(n)) - 2*ln_L`. For `n <= e` (so `ln(ln(n))` is undefined
/// or non-positive) the penalty term is treated as `0.0` rather than producing NaN/negative
/// infinity — a small-`n` edge case, not a claim that HQC is well-defined there.
pub fn hqc(log_likelihood: f64, num_params: usize, n: usize) -> f64 {
    let penalty = if (n as f64) <= core::f64::consts::E {
        0.0
    } else {
        2.0 * num_params as f64 * (n as f64).ln().ln()
    };
    penalty - 2.0 * log_likelihood
}

/// 📉 A simple two-part-code minimum-description-length approximation:
/// `-ln_L + 0.5 * k * ln(n)` (equivalent to BIC/2 in the log-likelihood term's convention).
pub fn mdl(log_likelihood: f64, num_params: usize, n: usize) -> f64 {
    -log_likelihood + 0.5 * num_params as f64 * (n as f64).ln()
}
// #endregion 🔖Criteria

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aic_matches_hand_computed_value() {
        assert!((aic(-10.0, 3) - 26.0).abs() < 1e-12);
    }

    #[test]
    fn bic_matches_hand_computed_value() {
        let n = 100;
        let expected = 3.0 * (100.0_f64).ln() - 2.0 * -10.0;
        assert!((bic(-10.0, 3, n) - expected).abs() < 1e-12);
    }

    #[test]
    fn mdl_matches_hand_computed_value() {
        let n = 100;
        let expected = 10.0 + 0.5 * 3.0 * (100.0_f64).ln();
        assert!((mdl(-10.0, 3, n) - expected).abs() < 1e-12);
    }

    #[test]
    fn aic_increases_with_num_params() {
        let ln_l = -50.0;
        let mut prev = aic(ln_l, 1);
        for k in 2..10 {
            let cur = aic(ln_l, k);
            assert!(cur > prev, "k={k}");
            prev = cur;
        }
    }

    #[test]
    fn bic_increases_with_num_params() {
        let ln_l = -50.0;
        let n = 200;
        let mut prev = bic(ln_l, 1, n);
        for k in 2..10 {
            let cur = bic(ln_l, k, n);
            assert!(cur > prev, "k={k}");
            prev = cur;
        }
    }

    #[test]
    fn hqc_increases_with_num_params() {
        let ln_l = -50.0;
        let n = 200;
        let mut prev = hqc(ln_l, 1, n);
        for k in 2..10 {
            let cur = hqc(ln_l, k, n);
            assert!(cur > prev, "k={k}");
            prev = cur;
        }
    }

    #[test]
    fn mdl_increases_with_num_params() {
        let ln_l = -50.0;
        let n = 200;
        let mut prev = mdl(ln_l, 1, n);
        for k in 2..10 {
            let cur = mdl(ln_l, k, n);
            assert!(cur > prev, "k={k}");
            prev = cur;
        }
    }

    #[test]
    fn aicc_approaches_aic_for_large_n() {
        let ln_l = -50.0;
        let k = 3;
        let n = 1_000_000;
        let a = aic(ln_l, k);
        let ac = aicc(ln_l, k, n).unwrap();
        assert!((ac - a).abs() < 1e-6, "aicc={ac} aic={a}");
    }

    #[test]
    fn aicc_errs_when_n_leq_k_plus_one() {
        assert!(matches!(aicc(-10.0, 3, 4), Err(EntropyError::InvalidConfig { .. })));
        assert!(matches!(aicc(-10.0, 3, 3), Err(EntropyError::InvalidConfig { .. })));
    }

    #[test]
    fn aicc_ok_when_n_greater_than_k_plus_one() {
        assert!(aicc(-10.0, 3, 5).is_ok());
    }

    #[test]
    fn bic_penalizes_more_than_aic_when_ln_n_exceeds_two() {
        // 🔐 BIC's penalty is k*ln(n) vs AIC's 2*k; ln(n) > 2 (n > e^2 ~= 7.39) means BIC > AIC
        // for the same ln_L and k, since both share the -2*ln_L term.
        let ln_l = -50.0;
        let k = 4;
        let n = 100; // ln(100) ~= 4.6 > 2
        assert!((n as f64).ln() > 2.0);
        assert!(bic(ln_l, k, n) > aic(ln_l, k));
    }

    #[test]
    fn hqc_small_n_edge_case_has_zero_penalty() {
        let ln_l = -5.0;
        for n in [1usize, 2] {
            assert!((n as f64) <= core::f64::consts::E);
            let expected = -2.0 * ln_l;
            assert!((hqc(ln_l, 3, n) - expected).abs() < 1e-12, "n={n}");
        }
    }

    #[test]
    fn fisher_information_matches_gaussian_closed_form() {
        // 🔐 log_lik(theta) = -0.5 * sum((x_i - theta)^2) / sigma^2 has exact Fisher information
        // n / sigma^2 for the mean parameter, independent of the sample itself.
        let sigma: f64 = 2.0;
        let xs: Vec<f64> = vec![1.0, 2.5, -0.7, 3.3, 0.1, 4.2, -1.1, 2.0];
        let n = xs.len();
        let log_lik = |theta: f64| -> f64 {
            -0.5 * xs.iter().map(|x| (x - theta).powi(2)).sum::<f64>() / (sigma * sigma)
        };
        let theta0 = 1.3;
        let observed = fisher_information(log_lik, theta0);
        let expected = n as f64 / (sigma * sigma);
        let rel_err = (observed - expected).abs() / expected;
        assert!(rel_err < 1e-3, "observed={observed} expected={expected} rel_err={rel_err}");
    }

    #[test]
    fn fisher_information_zero_for_flat_log_likelihood() {
        let flat = |_theta: f64| -> f64 { 42.0 };
        let observed = fisher_information(flat, 5.0);
        assert!(observed.abs() < 1e-6, "observed={observed}");
    }
}
// #endregion 🔖Tests
