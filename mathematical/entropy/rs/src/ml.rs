//! 🤖 Machine-learning uncertainty measures: per-sample predictive entropy over classifier
//! outputs, BALD (epistemic) mutual information from ensemble predictions, and expected
//! calibration error.

use crate::numeric::x_ln_x;
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase};

// #region 🔖Predictive
/// 🤖 Shannon entropy of each row of a row-major `[n_samples x n_classes]` probability batch.
pub fn predictive_entropy(probs: &[f64], n_classes: usize, base: LogBase) -> Result<Vec<Estimate>, EntropyError> {
    base.validate()?;
    if n_classes == 0 {
        return Err(EntropyError::InvalidConfig { field: "n_classes", reason: "must be at least 1" });
    }
    if probs.is_empty() || probs.len() % n_classes != 0 {
        return Err(EntropyError::ShapeMismatch { what: "probs", expected: n_classes, actual: probs.len() % n_classes.max(1) });
    }
    let n_samples = probs.len() / n_classes;
    let mut out = Vec::with_capacity(n_samples);
    for i in 0..n_samples {
        let row = &probs[i * n_classes..(i + 1) * n_classes];
        let p = crate::counts::validate_probabilities(row, crate::Tolerances::default())?;
        let nats = -p.iter().map(|&v| x_ln_x(v)).sum::<f64>();
        out.push(Estimate {
            value: base.from_nats(nats),
            base,
            method: "predictive_entropy",
            n: n_classes,
            n_effective: n_classes as f64,
            std_error: None,
            ci: None::<ConfidenceInterval>,
            warnings: Vec::new(),
            diagnostics: Vec::new(),
        });
    }
    Ok(out)
}
// #endregion 🔖Predictive

// #region 🔖Bald
/// 🤖 BALD mutual information per sample: `H(mean_over_members(p)) - mean_over_members(H(p))`,
/// splitting total predictive uncertainty into epistemic (this value) and aleatoric (the
/// subtracted mean-member-entropy term) components. `ensemble_probs` is row-major
/// `[n_samples][n_members][n_classes]` flattened.
pub fn bald_mutual_information(ensemble_probs: &[f64], n_members: usize, n_classes: usize, base: LogBase) -> Result<Vec<Estimate>, EntropyError> {
    base.validate()?;
    if n_members == 0 || n_classes == 0 {
        return Err(EntropyError::InvalidConfig { field: "n_members/n_classes", reason: "must be at least 1" });
    }
    let per_sample_len = n_members * n_classes;
    if ensemble_probs.is_empty() || ensemble_probs.len() % per_sample_len != 0 {
        return Err(EntropyError::ShapeMismatch { what: "ensemble_probs", expected: per_sample_len, actual: ensemble_probs.len() % per_sample_len.max(1) });
    }
    let n_samples = ensemble_probs.len() / per_sample_len;
    let mut out = Vec::with_capacity(n_samples);
    for s in 0..n_samples {
        let sample = &ensemble_probs[s * per_sample_len..(s + 1) * per_sample_len];
        let mut mean_probs = vec![0.0_f64; n_classes];
        let mut mean_member_entropy_nats = 0.0_f64;
        for m in 0..n_members {
            let member = &sample[m * n_classes..(m + 1) * n_classes];
            let p = crate::counts::validate_probabilities(member, crate::Tolerances::default())?;
            for c in 0..n_classes {
                mean_probs[c] += p[c] / n_members as f64;
            }
            mean_member_entropy_nats += -p.iter().map(|&v| x_ln_x(v)).sum::<f64>() / n_members as f64;
        }
        let mean_probs = crate::counts::validate_probabilities(&mean_probs, crate::Tolerances::default())?;
        let predictive_nats = -mean_probs.iter().map(|&v| x_ln_x(v)).sum::<f64>();
        let bald_nats = crate::numeric::clamp_near_zero(predictive_nats - mean_member_entropy_nats, 1e-9);

        out.push(Estimate {
            value: base.from_nats(bald_nats),
            base,
            method: "bald_mutual_information",
            n: n_members,
            n_effective: n_members as f64,
            std_error: None,
            ci: None::<ConfidenceInterval>,
            warnings: Vec::new(),
            diagnostics: vec![("predictive_entropy", base.from_nats(predictive_nats)), ("mean_member_entropy", base.from_nats(mean_member_entropy_nats))],
        });
    }
    Ok(out)
}
// #endregion 🔖Bald

// #region 🔖Calibration
/// 🤖 Expected calibration error: bins predictions by confidence into `n_bins` equal-width bins
/// in `[0,1]`, and reports the confidence-weighted average `|accuracy - confidence|` per bin.
pub fn expected_calibration_error(confidences: &[f64], correct: &[bool], n_bins: usize) -> Result<f64, EntropyError> {
    if confidences.len() != correct.len() {
        return Err(EntropyError::LengthMismatch { expected: confidences.len(), actual: correct.len() });
    }
    if confidences.is_empty() {
        return Err(EntropyError::EmptyInput { what: "confidences" });
    }
    if n_bins == 0 {
        return Err(EntropyError::InvalidConfig { field: "n_bins", reason: "must be at least 1" });
    }
    for (i, &c) in confidences.iter().enumerate() {
        if !(0.0..=1.0).contains(&c) {
            return Err(EntropyError::InvalidProbability { index: i, value: c });
        }
    }
    let mut bin_conf_sum = vec![0.0_f64; n_bins];
    let mut bin_correct_sum = vec![0.0_f64; n_bins];
    let mut bin_count = vec![0.0_f64; n_bins];
    for (&c, &ok) in confidences.iter().zip(correct.iter()) {
        let bin = ((c * n_bins as f64) as usize).min(n_bins - 1);
        bin_conf_sum[bin] += c;
        bin_correct_sum[bin] += if ok { 1.0 } else { 0.0 };
        bin_count[bin] += 1.0;
    }
    let n = confidences.len() as f64;
    let mut ece = 0.0_f64;
    for b in 0..n_bins {
        if bin_count[b] <= 0.0 {
            continue;
        }
        let avg_conf = bin_conf_sum[b] / bin_count[b];
        let accuracy = bin_correct_sum[b] / bin_count[b];
        ece += (bin_count[b] / n) * (accuracy - avg_conf).abs();
    }
    Ok(ece)
}
// #endregion 🔖Calibration

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predictive_entropy_of_confident_prediction_is_zero() {
        let probs = [1.0, 0.0, 0.0];
        let est = predictive_entropy(&probs, 3, LogBase::Bits).unwrap();
        assert!(est[0].value.abs() < 1e-9);
    }

    #[test]
    fn predictive_entropy_of_uniform_is_max() {
        let probs = [0.25, 0.25, 0.25, 0.25];
        let est = predictive_entropy(&probs, 4, LogBase::Bits).unwrap();
        assert!((est[0].value - 2.0).abs() < 1e-9);
    }

    #[test]
    fn predictive_entropy_batch_shape() {
        let probs = [0.5, 0.5, 0.9, 0.1, 0.25, 0.75];
        let est = predictive_entropy(&probs, 2, LogBase::Bits).unwrap();
        assert_eq!(est.len(), 3);
    }

    #[test]
    fn bald_of_unanimous_ensemble_is_near_zero() {
        // 🔐 every member gives the same confident prediction: no epistemic disagreement.
        let ensemble = [1.0, 0.0, 1.0, 0.0, 1.0, 0.0];
        let est = bald_mutual_information(&ensemble, 3, 2, LogBase::Bits).unwrap();
        assert!(est[0].value.abs() < 1e-9);
    }

    #[test]
    fn bald_of_disagreeing_ensemble_is_positive() {
        // 🔐 members confidently disagree with each other but each is individually confident.
        let ensemble = [1.0, 0.0, 0.0, 1.0];
        let est = bald_mutual_information(&ensemble, 2, 2, LogBase::Bits).unwrap();
        assert!(est[0].value > 0.9, "got {}", est[0].value);
    }

    #[test]
    fn ece_of_perfectly_calibrated_predictions_is_zero() {
        let mut rng = crate::numeric::Xorshift64::new(1);
        let n = 5000;
        let confidences: Vec<f64> = (0..n).map(|_| rng.next_f64()).collect();
        let correct: Vec<bool> = confidences.iter().map(|&c| rng.next_f64() < c).collect();
        let ece = expected_calibration_error(&confidences, &correct, 10).unwrap();
        assert!(ece < 0.05, "got {ece}");
    }

    #[test]
    fn ece_of_badly_miscalibrated_predictions_is_large() {
        let confidences = vec![0.95; 100];
        let correct = vec![false; 100];
        let ece = expected_calibration_error(&confidences, &correct, 10).unwrap();
        assert!(ece > 0.8, "got {ece}");
    }

    #[test]
    fn ece_rejects_length_mismatch() {
        assert!(expected_calibration_error(&[0.5], &[], 5).is_err());
    }

    #[test]
    fn ece_rejects_out_of_range_confidence() {
        assert!(expected_calibration_error(&[1.5], &[true], 5).is_err());
    }
}
// #endregion 🔖Tests
