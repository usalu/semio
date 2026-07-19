//! 📶 Multiscale entropy: coarse-grain a series at increasing scales and report a chosen
//! regularity/ordinal entropy at each scale, summarized by a complexity index (mean entropy
//! across valid scales).

use crate::ordinal::{dispersion_entropy, permutation_entropy, DispersionConfig};
use crate::regularity::{fuzzy_entropy, sample_entropy, RegularityConfig};
use crate::symbolic::OrdinalConfig;
use crate::{EntropyError, Estimate, LogBase};

// #region 🔖Grain
/// 📶 How each scale's coarse-grained series is derived from non-overlapping windows of the
/// original (or previous-scale) series.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Grain {
    Mean,
    Median,
    Variance,
    StdDev,
}

fn coarse_grain(x: &[f64], scale: usize, grain: Grain) -> Vec<f64> {
    if scale == 1 {
        return x.to_vec();
    }
    let n_out = x.len() / scale;
    (0..n_out)
        .map(|j| {
            let window = &x[j * scale..(j + 1) * scale];
            match grain {
                Grain::Mean => window.iter().sum::<f64>() / scale as f64,
                Grain::Median => {
                    let mut sorted = window.to_vec();
                    sorted.sort_by(|a, b| a.total_cmp(b));
                    let mid = sorted.len() / 2;
                    if sorted.len() % 2 == 0 {
                        0.5 * (sorted[mid - 1] + sorted[mid])
                    } else {
                        sorted[mid]
                    }
                }
                Grain::Variance | Grain::StdDev => {
                    let mean = window.iter().sum::<f64>() / scale as f64;
                    let var = window.iter().map(|&v| (v - mean).powi(2)).sum::<f64>() / scale as f64;
                    if matches!(grain, Grain::StdDev) {
                        var.sqrt()
                    } else {
                        var
                    }
                }
            }
        })
        .collect()
}
// #endregion 🔖Grain

// #region 🔖Inner
/// 📶 Which per-scale entropy [`multiscale_entropy`] computes on each coarse-grained series. An
/// enum (not a trait object) so the whole pipeline stays monomorphic and exhaustively matchable.
#[derive(Clone, PartialEq, Debug)]
pub enum MsInner {
    SampleEntropy(RegularityConfig),
    FuzzyEntropy(RegularityConfig),
    Permutation(OrdinalConfig),
    Dispersion(DispersionConfig),
}

fn run_inner(x: &[f64], inner: &MsInner, base: LogBase) -> Result<Estimate, EntropyError> {
    match inner {
        MsInner::SampleEntropy(cfg) => sample_entropy(x, *cfg, base),
        MsInner::FuzzyEntropy(cfg) => fuzzy_entropy(x, *cfg, base),
        MsInner::Permutation(cfg) => permutation_entropy(x, *cfg, base),
        MsInner::Dispersion(cfg) => dispersion_entropy(x, *cfg, base),
    }
}
// #endregion 🔖Inner

// #region 🔖Dispatch
/// 📶 Configuration for [`multiscale_entropy`].
#[derive(Clone, PartialEq, Debug)]
pub struct MultiscaleConfig {
    pub scales: usize,
    pub grain: Grain,
    pub inner: MsInner,
}

impl MultiscaleConfig {
    pub fn new(scales: usize, grain: Grain, inner: MsInner) -> Result<Self, EntropyError> {
        if scales == 0 {
            return Err(EntropyError::InvalidConfig { field: "scales", reason: "must be at least 1" });
        }
        Ok(Self { scales, grain, inner })
    }
}

/// 📶 Per-scale entropies plus a summary complexity index (mean entropy across valid scales).
#[derive(Clone, Debug)]
pub struct MultiscaleResult {
    pub per_scale: Vec<Estimate>,
    pub complexity_index: f64,
    pub scales: Vec<usize>,
}

/// 📶 Coarse-grains `x` at scales `1..=cfg.scales` and computes `cfg.inner`'s entropy at each,
/// stopping early (without error) once a scale's coarse-grained series becomes too short for the
/// inner method — [`MultiscaleResult::scales`] reports exactly which scales succeeded.
pub fn multiscale_entropy(x: &[f64], cfg: MultiscaleConfig, base: LogBase) -> Result<MultiscaleResult, EntropyError> {
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what: "x" });
    }
    let mut per_scale = Vec::new();
    let mut scales = Vec::new();
    for scale in 1..=cfg.scales {
        let coarse = coarse_grain(x, scale, cfg.grain);
        if coarse.len() < 4 {
            break;
        }
        match run_inner(&coarse, &cfg.inner, base) {
            Ok(est) => {
                per_scale.push(est);
                scales.push(scale);
            }
            Err(_) if scale > 1 => break,
            Err(e) => return Err(e),
        }
    }
    if per_scale.is_empty() {
        return Err(EntropyError::InsufficientData { what: "multiscale_entropy", needed: 4, actual: x.len() });
    }
    let complexity_index = per_scale.iter().map(|e| e.value).sum::<f64>() / per_scale.len() as f64;
    Ok(MultiscaleResult { per_scale, complexity_index, scales })
}
// #endregion 🔖Dispatch

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiscale_config_rejects_zero_scales() {
        let inner = MsInner::Permutation(OrdinalConfig::new(3, 1).unwrap());
        assert!(MultiscaleConfig::new(0, Grain::Mean, inner).is_err());
    }

    #[test]
    fn coarse_grain_mean_matches_hand_computation() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let coarse = coarse_grain(&x, 2, Grain::Mean);
        assert_eq!(coarse, vec![1.5, 3.5, 5.5]);
    }

    #[test]
    fn coarse_grain_scale_one_is_identity() {
        let x = vec![1.0, 2.0, 3.0];
        assert_eq!(coarse_grain(&x, 1, Grain::Mean), x);
    }

    #[test]
    fn multiscale_entropy_reports_requested_scales_for_long_series() {
        let mut rng = crate::numeric::Xorshift64::new(1);
        let x: Vec<f64> = (0..3000).map(|_| rng.next_f64()).collect();
        let inner = MsInner::Permutation(OrdinalConfig::new(3, 1).unwrap());
        let cfg = MultiscaleConfig::new(5, Grain::Mean, inner).unwrap();
        let result = multiscale_entropy(&x, cfg, LogBase::Bits).unwrap();
        assert_eq!(result.scales, vec![1, 2, 3, 4, 5]);
        assert_eq!(result.per_scale.len(), 5);
    }

    #[test]
    fn multiscale_entropy_stops_early_for_short_series() {
        let x: Vec<f64> = (0..20).map(|i| i as f64).collect();
        let inner = MsInner::Permutation(OrdinalConfig::new(3, 1).unwrap());
        let cfg = MultiscaleConfig::new(20, Grain::Mean, inner).unwrap();
        let result = multiscale_entropy(&x, cfg, LogBase::Bits).unwrap();
        assert!(result.scales.len() < 20);
    }

    mod quick {
        use super::*;

        #[test]
        fn white_noise_multiscale_entropy_differs_from_pink_like_noise() {
            let mut rng = crate::numeric::Xorshift64::new(2);
            let n = 4000;
            let white: Vec<f64> = (0..n).map(|_| rng.next_gaussian()).collect();
            // 🔐 a crude 1/f-like signal via running-sum (integrated white noise).
            let mut pink = vec![0.0; n];
            let mut acc = 0.0;
            for i in 0..n {
                acc = 0.98 * acc + rng.next_gaussian();
                pink[i] = acc;
            }
            let inner = MsInner::SampleEntropy(RegularityConfig::new(2, crate::Tolerance::Auto).unwrap());
            let cfg = MultiscaleConfig::new(4, Grain::Mean, inner).unwrap();
            let white_result = multiscale_entropy(&white, cfg.clone(), LogBase::Nats).unwrap();
            let pink_result = multiscale_entropy(&pink, cfg, LogBase::Nats).unwrap();
            assert!((white_result.complexity_index - pink_result.complexity_index).abs() > 1e-6);
        }
    }
}
// #endregion 🔖Tests
