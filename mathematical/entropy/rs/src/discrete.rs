//! 📐 Plug-in (exact-given-the-distribution) discrete entropy family: Shannon, Rényi, Tsallis,
//! Hartley, collision/min-entropy, Sharma-Mittal, Kaniadakis, cross/joint/conditional entropy.
//! Every function here takes a *given* probability vector and returns `f64` directly (no
//! [`crate::Estimate`]) — estimation from raw samples lives in `estimators.rs`.

use crate::counts::validate_probabilities;
use crate::numeric::{neumaier_sum, x_ln_x};
use crate::{EntropyError, LogBase, Tolerances};

// #region 🔖Shannon
/// 📐 Shannon entropy `H(p) = -sum p_i log p_i`, in `base`.
pub fn entropy(p: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    base.validate()?;
    let p = validate_probabilities(p, Tolerances::default())?;
    let nats = -neumaier_sum(p.iter().map(|&pi| x_ln_x(pi)));
    Ok(base.from_nats(nats))
}

/// 📐 Binary entropy `H(p) = -p log p - (1-p) log(1-p)` for a single Bernoulli parameter.
pub fn binary_entropy(p: f64, base: LogBase) -> Result<f64, EntropyError> {
    entropy(&[p, 1.0 - p], base)
}

/// 📐 Joint Shannon entropy `H(X,Y)` of a flattened row-major joint probability matrix.
pub fn joint_entropy(joint_p: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    entropy(joint_p, base)
}

/// 📐 Conditional entropy `H(Y|X) = H(X,Y) - H(X)` from a flattened row-major joint probability
/// matrix with `rows` values of `X` and `cols` values of `Y`.
pub fn conditional_entropy(joint_p: &[f64], rows: usize, cols: usize, base: LogBase) -> Result<f64, EntropyError> {
    if joint_p.len() != rows * cols {
        return Err(EntropyError::ShapeMismatch { what: "joint_p", expected: rows * cols, actual: joint_p.len() });
    }
    let h_xy = joint_entropy(joint_p, base)?;
    let marginal_x: Vec<f64> = (0..rows).map(|r| (0..cols).map(|c| joint_p[r * cols + c]).sum()).collect();
    let h_x = entropy(&marginal_x, base)?;
    Ok(h_xy - h_x)
}

/// 📐 Cross-entropy `H(p, q) = -sum p_i log q_i`. Returns `f64::INFINITY` when `p_i > 0` and
/// `q_i == 0` for some `i` (mathematically correct: the code built for `q` cannot represent
/// that symbol).
pub fn cross_entropy(p: &[f64], q: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    base.validate()?;
    if p.len() != q.len() {
        return Err(EntropyError::LengthMismatch { expected: p.len(), actual: q.len() });
    }
    let p = validate_probabilities(p, Tolerances::default())?;
    let q = validate_probabilities(q, Tolerances::default())?;
    let mut nats = 0.0;
    for (&pi, &qi) in p.iter().zip(q.iter()) {
        if pi <= 0.0 {
            continue;
        }
        if qi <= 0.0 {
            return Ok(f64::INFINITY);
        }
        nats -= pi * qi.ln();
    }
    Ok(base.from_nats(nats))
}
// #endregion 🔖Shannon

// #region 🔖Generalized
/// 📐 Hartley entropy `log(k)` of a support of size `k` (the entropy of a uniform distribution
/// over `k` symbols; also the `alpha -> 0` limit of Rényi entropy).
pub fn hartley_entropy(support_size: usize, base: LogBase) -> Result<f64, EntropyError> {
    base.validate()?;
    if support_size == 0 {
        return Err(EntropyError::EmptyInput { what: "support" });
    }
    Ok(base.from_nats((support_size as f64).ln()))
}

/// 📐 Rényi entropy of order `alpha`: `H_alpha(p) = 1/(1-alpha) * log(sum p_i^alpha)`.
/// `alpha == 1` is mathematically the Shannon limit and must be requested via [`entropy`]
/// directly rather than this formula's removable singularity — this function returns
/// [`EntropyError::UndefinedResult`] for `alpha` exactly `1.0`. `alpha == 0` returns the
/// [`hartley_entropy`] of the occupied support (`0^0` is conventionally excluded from the sum).
pub fn renyi_entropy(p: &[f64], alpha: f64, base: LogBase) -> Result<f64, EntropyError> {
    base.validate()?;
    if !alpha.is_finite() {
        return Err(EntropyError::InvalidConfig { field: "alpha", reason: "must be finite" });
    }
    let p = validate_probabilities(p, Tolerances::default())?;
    if alpha == 1.0 {
        return Err(EntropyError::UndefinedResult { reason: "Renyi entropy at alpha=1 is the Shannon limit; call entropy() instead" });
    }
    if alpha == 0.0 {
        return hartley_entropy(p.iter().filter(|&&pi| pi > 0.0).count(), base);
    }
    let sum_alpha = neumaier_sum(p.iter().filter(|&&pi| pi > 0.0).map(|&pi| pi.powf(alpha)));
    if sum_alpha <= 0.0 {
        return Err(EntropyError::UndefinedResult { reason: "sum of p^alpha is non-positive" });
    }
    let nats = sum_alpha.ln() / (1.0 - alpha);
    Ok(base.from_nats(nats))
}

/// 📐 Collision entropy: Rényi entropy at `alpha = 2`, `-log(sum p_i^2)`.
pub fn collision_entropy(p: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    renyi_entropy(p, 2.0, base)
}

/// 📐 Min-entropy: the `alpha -> infinity` limit of Rényi entropy, `-log(max_i p_i)`.
pub fn min_entropy(p: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    base.validate()?;
    let p = validate_probabilities(p, Tolerances::default())?;
    let max_p = p.iter().copied().fold(0.0_f64, f64::max);
    if max_p <= 0.0 {
        return Err(EntropyError::UndefinedResult { reason: "all probabilities are zero" });
    }
    Ok(base.from_nats(-max_p.ln()))
}

/// 📐 Tsallis entropy of entropic index `q`: `S_q(p) = (1 - sum p_i^q) / (q - 1)`. Unitless by
/// convention (no logarithm base); the `q -> 1` limit equals Shannon entropy in nats.
pub fn tsallis_entropy(p: &[f64], q: f64) -> Result<f64, EntropyError> {
    if !q.is_finite() {
        return Err(EntropyError::InvalidConfig { field: "q", reason: "must be finite" });
    }
    let p = validate_probabilities(p, Tolerances::default())?;
    if (q - 1.0).abs() < 1e-12 {
        return Ok(-neumaier_sum(p.iter().map(|&pi| x_ln_x(pi))));
    }
    let sum_q = neumaier_sum(p.iter().filter(|&&pi| pi > 0.0).map(|&pi| pi.powf(q)));
    Ok((1.0 - sum_q) / (q - 1.0))
}

/// 📐 Sharma-Mittal entropy, a two-parameter family generalizing both Rényi (`beta -> 1`) and
/// Tsallis (`beta == alpha`): `SM_{alpha,beta}(p) = (1/(1-beta)) * [(sum p_i^alpha)^((1-beta)/(1-alpha)) - 1]`.
pub fn sharma_mittal_entropy(p: &[f64], alpha: f64, beta: f64) -> Result<f64, EntropyError> {
    if !alpha.is_finite() || !beta.is_finite() {
        return Err(EntropyError::InvalidConfig { field: "alpha/beta", reason: "must be finite" });
    }
    if (alpha - 1.0).abs() < 1e-12 || (beta - 1.0).abs() < 1e-12 {
        return Err(EntropyError::UndefinedResult { reason: "Sharma-Mittal requires alpha != 1 and beta != 1" });
    }
    let p = validate_probabilities(p, Tolerances::default())?;
    let sum_alpha = neumaier_sum(p.iter().filter(|&&pi| pi > 0.0).map(|&pi| pi.powf(alpha)));
    if sum_alpha <= 0.0 {
        return Err(EntropyError::UndefinedResult { reason: "sum of p^alpha is non-positive" });
    }
    let exponent = (1.0 - beta) / (1.0 - alpha);
    Ok((sum_alpha.powf(exponent) - 1.0) / (1.0 - beta))
}

/// 📐 Kaniadakis (kappa-) entropy: `S_kappa(p) = -sum p_i * (p_i^kappa - p_i^{-kappa}) / (2 kappa)`,
/// defined for `kappa` in `(-1, 1) \ {0}`; the `kappa -> 0` limit is Shannon entropy in nats.
pub fn kaniadakis_entropy(p: &[f64], kappa: f64) -> Result<f64, EntropyError> {
    // 🔐 `kappa.is_nan()` is explicit here (rather than relying on `!(kappa.abs() < 1.0)`'s
    // NaN-through-negation behavior) so a NaN input is rejected exactly as before.
    if kappa.is_nan() || kappa.abs() >= 1.0 {
        return Err(EntropyError::InvalidConfig { field: "kappa", reason: "must satisfy |kappa| < 1" });
    }
    let p = validate_probabilities(p, Tolerances::default())?;
    if kappa.abs() < 1e-12 {
        return Ok(-neumaier_sum(p.iter().map(|&pi| x_ln_x(pi))));
    }
    let terms = p.iter().filter(|&&pi| pi > 0.0).map(|&pi| pi * (pi.powf(kappa) - pi.powf(-kappa)) / (2.0 * kappa));
    Ok(-neumaier_sum(terms))
}
// #endregion 🔖Generalized

// #region 🔖Normalized
/// 📐 Shannon entropy divided by the Hartley entropy of the *declared* alphabet size (`p.len()`),
/// giving a value in `[0, 1]` regardless of how concentrated `p` is.
pub fn normalized_entropy(p: &[f64], base: LogBase) -> Result<f64, EntropyError> {
    let h = entropy(p, base)?;
    let h_max = hartley_entropy(p.len(), base)?;
    if h_max <= 0.0 {
        return Ok(0.0);
    }
    Ok(h / h_max)
}
// #endregion 🔖Normalized

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_entropy_equals_log_support_size() {
        for k in [2usize, 3, 8, 16] {
            let p = vec![1.0 / k as f64; k];
            let h = entropy(&p, LogBase::Nats).unwrap();
            assert!((h - (k as f64).ln()).abs() < 1e-9, "k={k}");
        }
    }

    #[test]
    fn fair_coin_entropy_is_one_bit() {
        let h = binary_entropy(0.5, LogBase::Bits).unwrap();
        assert!((h - 1.0).abs() < 1e-9);
    }

    #[test]
    fn deterministic_distribution_entropy_is_zero() {
        let h = entropy(&[1.0, 0.0, 0.0], LogBase::Bits).unwrap();
        assert!(h.abs() < 1e-12);
    }

    #[test]
    fn entropy_rejects_empty() {
        assert!(matches!(entropy(&[], LogBase::Bits), Err(EntropyError::EmptyInput { .. })));
    }

    #[test]
    fn cross_entropy_of_identical_distributions_equals_entropy() {
        let p = [0.2, 0.3, 0.5];
        let h = entropy(&p, LogBase::Bits).unwrap();
        let ce = cross_entropy(&p, &p, LogBase::Bits).unwrap();
        assert!((h - ce).abs() < 1e-9);
    }

    #[test]
    fn cross_entropy_infinite_on_support_mismatch() {
        let p = [0.5, 0.5];
        let q = [1.0, 0.0];
        assert_eq!(cross_entropy(&p, &q, LogBase::Bits).unwrap(), f64::INFINITY);
    }

    #[test]
    fn chain_rule_holds_for_joint_entropy() {
        // 🔐 X,Y independent uniform over {0,1}: H(X,Y) = H(X) + H(Y|X) = 2 bits.
        let joint = [0.25, 0.25, 0.25, 0.25];
        let h_xy = joint_entropy(&joint, LogBase::Bits).unwrap();
        let h_x = entropy(&[0.5, 0.5], LogBase::Bits).unwrap();
        let h_y_given_x = conditional_entropy(&joint, 2, 2, LogBase::Bits).unwrap();
        assert!((h_xy - (h_x + h_y_given_x)).abs() < 1e-9);
        assert!((h_xy - 2.0).abs() < 1e-9);
    }

    #[test]
    fn conditional_entropy_zero_when_y_determined_by_x() {
        // 🔐 Y = X exactly: H(Y|X) = 0.
        let joint = [0.5, 0.0, 0.0, 0.5];
        let h = conditional_entropy(&joint, 2, 2, LogBase::Bits).unwrap();
        assert!(h.abs() < 1e-9);
    }

    #[test]
    fn hartley_entropy_matches_log_k() {
        assert!((hartley_entropy(8, LogBase::Bits).unwrap() - 3.0).abs() < 1e-9);
    }

    #[test]
    fn renyi_rejects_alpha_exactly_one() {
        assert!(matches!(
            renyi_entropy(&[0.5, 0.5], 1.0, LogBase::Bits),
            Err(EntropyError::UndefinedResult { .. })
        ));
    }

    #[test]
    fn renyi_alpha_zero_equals_hartley_over_support() {
        let p = [0.5, 0.5, 0.0];
        let h = renyi_entropy(&p, 0.0, LogBase::Bits).unwrap();
        assert!((h - 1.0).abs() < 1e-9); // support size 2 -> log2(2) = 1
    }

    #[test]
    fn renyi_alpha_two_equals_collision_entropy() {
        let p = [0.5, 0.25, 0.25];
        let a = renyi_entropy(&p, 2.0, LogBase::Bits).unwrap();
        let b = collision_entropy(&p, LogBase::Bits).unwrap();
        assert!((a - b).abs() < 1e-12);
    }

    #[test]
    fn renyi_continuity_near_alpha_one_approaches_shannon() {
        let p = [0.2, 0.3, 0.5];
        let shannon = entropy(&p, LogBase::Nats).unwrap();
        let near = renyi_entropy(&p, 1.0 + 1e-6, LogBase::Nats).unwrap();
        assert!((near - shannon).abs() < 1e-4);
    }

    #[test]
    fn min_entropy_matches_negative_log_max_probability() {
        let p = [0.6, 0.3, 0.1];
        let h = min_entropy(&p, LogBase::Bits).unwrap();
        assert!((h - (-(0.6_f64.log2()))).abs() < 1e-9);
    }

    #[test]
    fn tsallis_limit_at_q_one_equals_shannon_nats() {
        let p = [0.2, 0.3, 0.5];
        let shannon = entropy(&p, LogBase::Nats).unwrap();
        let tsallis = tsallis_entropy(&p, 1.0).unwrap();
        assert!((tsallis - shannon).abs() < 1e-9);
    }

    #[test]
    fn tsallis_uniform_matches_closed_form() {
        // 🔐 S_q(uniform_k) = (1 - k^(1-q)) / (q - 1)
        let k = 4;
        let p = vec![1.0 / k as f64; k];
        let q = 2.0;
        let expected = (1.0 - (k as f64).powf(1.0 - q)) / (q - 1.0);
        assert!((tsallis_entropy(&p, q).unwrap() - expected).abs() < 1e-9);
    }

    #[test]
    fn kaniadakis_limit_at_kappa_zero_equals_shannon_nats() {
        let p = [0.2, 0.3, 0.5];
        let shannon = entropy(&p, LogBase::Nats).unwrap();
        let kaniadakis = kaniadakis_entropy(&p, 0.0).unwrap();
        assert!((kaniadakis - shannon).abs() < 1e-9);
    }

    #[test]
    fn sharma_mittal_reduces_to_renyi_as_beta_approaches_one() {
        let p = [0.2, 0.3, 0.5];
        let alpha = 2.0;
        let renyi = renyi_entropy(&p, alpha, LogBase::Nats).unwrap();
        let sm = sharma_mittal_entropy(&p, alpha, 1.0 + 1e-7).unwrap();
        assert!((sm - renyi).abs() < 1e-4);
    }

    #[test]
    fn normalized_entropy_of_uniform_is_one() {
        let p = [0.25, 0.25, 0.25, 0.25];
        assert!((normalized_entropy(&p, LogBase::Bits).unwrap() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn normalized_entropy_of_deterministic_is_zero() {
        let p = [1.0, 0.0, 0.0, 0.0];
        assert!(normalized_entropy(&p, LogBase::Bits).unwrap().abs() < 1e-12);
    }

    #[test]
    fn entropy_non_negative_for_random_distributions() {
        let mut rng = crate::numeric::Xorshift64::new(2024);
        for _ in 0..200 {
            let k = 2 + (rng.next_below(5));
            let mut raw: Vec<f64> = (0..k).map(|_| rng.next_f64()).collect();
            let sum: f64 = raw.iter().sum();
            for v in &mut raw {
                *v /= sum;
            }
            let h = entropy(&raw, LogBase::Bits).unwrap();
            assert!(h >= -1e-9);
            assert!(h <= (k as f64).log2() + 1e-9);
        }
    }

    mod quick {
        use super::*;

        #[test]
        fn renyi_is_monotone_nonincreasing_in_alpha() {
            let p = [0.6, 0.3, 0.1];
            let alphas = [-2.0, -0.5, 0.5, 2.0, 5.0, 20.0];
            let mut prev = renyi_entropy(&p, alphas[0], LogBase::Nats).unwrap();
            for &a in &alphas[1..] {
                let h = renyi_entropy(&p, a, LogBase::Nats).unwrap();
                assert!(h <= prev + 1e-9, "alpha={a} h={h} prev={prev}");
                prev = h;
            }
        }
    }
}
// #endregion 🔖Tests
