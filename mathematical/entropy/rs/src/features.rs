//! 📋 Batch feature extraction and estimator-selection automation: a deterministically-ordered
//! named registry of standard entropy features over a single raw series, plus simple heuristics
//! for picking bin counts and kNN neighbor counts.

use crate::regularity::{sample_entropy, RegularityConfig};
use crate::symbolic::OrdinalConfig;
use crate::{BinsSpec, EntropyError, Estimate, LogBase, Tolerance};

// #region 🔖Feature
/// 📋 One named entry of a [`FeatureRegistry::compute`] result.
#[derive(Clone, Debug)]
pub struct Feature {
    pub name: &'static str,
    pub estimate: Estimate,
}
// #endregion 🔖Feature

// #region 🔖StandardFeatures
fn feature_histogram_entropy(x: &[f64]) -> Result<Estimate, EntropyError> {
    crate::continuous::entropy_continuous(x, &crate::continuous::ContinuousMethod::Histogram(BinsSpec::Sturges), LogBase::Nats)
}

fn feature_sample_entropy(x: &[f64]) -> Result<Estimate, EntropyError> {
    let cfg = RegularityConfig::new(2, Tolerance::Auto)?;
    sample_entropy(x, cfg, LogBase::Nats)
}

fn feature_permutation_entropy(x: &[f64]) -> Result<Estimate, EntropyError> {
    let cfg = OrdinalConfig::new(3, 1)?;
    crate::ordinal::permutation_entropy(x, cfg, LogBase::Nats)
}

fn feature_spectral_entropy(x: &[f64]) -> Result<Estimate, EntropyError> {
    crate::spectral::spectral_entropy(x, crate::spectral::SpectralConfig::default())
}

fn feature_lempel_ziv(x: &[f64]) -> Result<Estimate, EntropyError> {
    let (_, sd) = {
        let n = x.len() as f64;
        let mean = x.iter().sum::<f64>() / n;
        let var = x.iter().map(|&v| (v - mean).powi(2)).sum::<f64>() / n;
        (mean, var.sqrt())
    };
    let symbols: Vec<u32> = if sd > 0.0 {
        let mean = x.iter().sum::<f64>() / x.len() as f64;
        x.iter().map(|&v| if v > mean { 1 } else { 0 }).collect()
    } else {
        vec![0; x.len()]
    };
    crate::lz::lempel_ziv_complexity(&symbols, true)
}
// #endregion 🔖StandardFeatures

// #region 🔖Registry
type FeatureFn = fn(&[f64]) -> Result<Estimate, EntropyError>;

/// 📋 A deterministically-ordered set of named entropy features to compute over a single raw
/// series in one pass.
pub struct FeatureRegistry {
    entries: Vec<(&'static str, FeatureFn)>,
}

impl FeatureRegistry {
    /// 📋 The built-in standard feature set: histogram entropy, sample entropy, permutation
    /// entropy, spectral entropy, and normalized Lempel-Ziv complexity (over a median-split
    /// binary encoding).
    pub fn standard() -> Self {
        Self {
            entries: vec![
                ("histogram_entropy", feature_histogram_entropy as FeatureFn),
                ("sample_entropy", feature_sample_entropy as FeatureFn),
                ("permutation_entropy", feature_permutation_entropy as FeatureFn),
                ("spectral_entropy", feature_spectral_entropy as FeatureFn),
                ("lempel_ziv_complexity", feature_lempel_ziv as FeatureFn),
            ],
        }
    }

    /// 📋 Registers an additional named feature function, appended after the existing entries
    /// (preserving deterministic ordering).
    pub fn with_feature(mut self, name: &'static str, f: FeatureFn) -> Self {
        self.entries.push((name, f));
        self
    }

    /// 📋 Computes every registered feature over `x`, in registration order. The first feature
    /// that errors short-circuits the whole batch (no partial/best-effort results) — callers
    /// that want per-feature failure isolation should call individual feature functions directly.
    pub fn compute(&self, x: &[f64]) -> Result<Vec<Feature>, EntropyError> {
        self.entries.iter().map(|&(name, f)| Ok(Feature { name, estimate: f(x)? })).collect()
    }
}
// #endregion 🔖Registry

// #region 🔖Automation
/// 📋 Suggests a histogram binning rule for `x`: Freedman-Diaconis when the interquartile range
/// is positive (robust to outliers), falling back to Sturges' rule otherwise (e.g. heavily
/// discrete/degenerate data where IQR collapses to zero).
pub fn suggest_bins(x: &[f64]) -> BinsSpec {
    if x.len() < 2 {
        return BinsSpec::Fixed(1);
    }
    let mut sorted = x.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let q1 = sorted[(sorted.len() as f64 * 0.25) as usize];
    let q3 = sorted[((sorted.len() as f64 * 0.75) as usize).min(sorted.len() - 1)];
    if q3 - q1 > 0.0 {
        BinsSpec::FreedmanDiaconis
    } else {
        BinsSpec::Sturges
    }
}

/// 📋 Suggests a kNN neighbor count `k` for continuous estimators (Kozachenko-Leonenko/KSG):
/// `round(sqrt(n) / 2)`, clamped to `[3, 20]` and to at most `n / 4` for small samples (the
/// standard practical bias/variance compromise for these estimators).
pub fn suggest_knn_k(n: usize) -> usize {
    if n == 0 {
        return 3;
    }
    let raw = ((n as f64).sqrt() / 2.0).round() as usize;
    raw.clamp(3, 20).min((n / 4).max(1))
}
// #endregion 🔖Automation

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_registry_computes_all_features_in_order() {
        let mut rng = crate::numeric::Xorshift64::new(1);
        let x: Vec<f64> = (0..2000).map(|_| rng.next_gaussian()).collect();
        let registry = FeatureRegistry::standard();
        let features = registry.compute(&x).unwrap();
        let names: Vec<&str> = features.iter().map(|f| f.name).collect();
        assert_eq!(names, vec!["histogram_entropy", "sample_entropy", "permutation_entropy", "spectral_entropy", "lempel_ziv_complexity"]);
        for f in &features {
            assert!(f.estimate.value.is_finite(), "{} produced non-finite value", f.name);
        }
    }

    #[test]
    fn with_feature_appends_after_standard_entries() {
        let registry = FeatureRegistry::standard().with_feature("custom", feature_histogram_entropy as FeatureFn);
        let mut rng = crate::numeric::Xorshift64::new(2);
        let x: Vec<f64> = (0..1500).map(|_| rng.next_gaussian()).collect();
        let features = registry.compute(&x).unwrap();
        assert_eq!(features.last().unwrap().name, "custom");
    }

    #[test]
    fn compute_propagates_first_error() {
        let registry = FeatureRegistry::standard();
        let constant = vec![1.0; 500];
        assert!(registry.compute(&constant).is_err());
    }

    #[test]
    fn suggest_bins_prefers_freedman_diaconis_for_spread_data() {
        let mut rng = crate::numeric::Xorshift64::new(3);
        let x: Vec<f64> = (0..500).map(|_| rng.next_gaussian()).collect();
        assert!(matches!(suggest_bins(&x), BinsSpec::FreedmanDiaconis));
    }

    #[test]
    fn suggest_bins_falls_back_to_sturges_for_degenerate_iqr() {
        let mut x = vec![0.0; 100];
        x[0] = 1000.0; // 🔐 a single outlier keeps the IQR at zero
        assert!(matches!(suggest_bins(&x), BinsSpec::Sturges));
    }

    #[test]
    fn suggest_knn_k_scales_with_sample_size_and_stays_bounded() {
        assert_eq!(suggest_knn_k(0), 3);
        assert!(suggest_knn_k(16) <= 4);
        let k_large = suggest_knn_k(1_000_000);
        assert!((3..=20).contains(&k_large));
    }
}
// #endregion 🔖Tests
