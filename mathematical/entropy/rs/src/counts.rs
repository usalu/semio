//! 🧮 Frequency counting and probability-vector validation: the shared foundation every discrete
//! entropy/estimator/divergence function builds on.

use crate::{EntropyError, Tolerances};
use std::collections::HashMap;

// #region 🔖Counts
/// 🧮 A dense frequency table over a `0..alphabet_size` symbol alphabet, plus the total weight
/// observed (integer counts have `total == sum(counts)`; weighted data can have fractional
/// `total` and a smaller `n_effective`).
#[derive(Clone, PartialEq, Debug)]
pub struct Counts {
    counts: Vec<f64>,
    total: f64,
    n_raw: usize,
}

impl Counts {
    /// 🧮 Builds a dense count table from `u32` symbols over `0..alphabet_size`.
    pub fn from_symbols(symbols: &[u32], alphabet_size: usize) -> Result<Self, EntropyError> {
        if symbols.is_empty() {
            return Err(EntropyError::EmptyInput { what: "symbols" });
        }
        let mut counts = vec![0.0_f64; alphabet_size];
        for (i, &s) in symbols.iter().enumerate() {
            let idx = s as usize;
            if idx >= alphabet_size {
                return Err(EntropyError::ShapeMismatch { what: "symbol index", expected: alphabet_size, actual: idx + 1 });
            }
            counts[idx] += 1.0;
            let _ = i;
        }
        let total = symbols.len() as f64;
        Ok(Self { counts, total, n_raw: symbols.len() })
    }

    /// 🧮 Builds a dense count table from weighted symbol observations. Weights must be finite
    /// and non-negative.
    pub fn from_weighted(symbols: &[u32], weights: &[f64], alphabet_size: usize) -> Result<Self, EntropyError> {
        if symbols.len() != weights.len() {
            return Err(EntropyError::LengthMismatch { expected: symbols.len(), actual: weights.len() });
        }
        if symbols.is_empty() {
            return Err(EntropyError::EmptyInput { what: "symbols" });
        }
        let mut counts = vec![0.0_f64; alphabet_size];
        let mut total = 0.0;
        for (i, (&s, &w)) in symbols.iter().zip(weights.iter()).enumerate() {
            if !w.is_finite() {
                return Err(EntropyError::NonFinite { what: "weights", index: i });
            }
            if w < 0.0 {
                return Err(EntropyError::InvalidProbability { index: i, value: w });
            }
            let idx = s as usize;
            if idx >= alphabet_size {
                return Err(EntropyError::ShapeMismatch { what: "symbol index", expected: alphabet_size, actual: idx + 1 });
            }
            counts[idx] += w;
            total += w;
        }
        Ok(Self { counts, total, n_raw: symbols.len() })
    }

    /// 🧮 Builds counts directly from a raw non-negative count vector (e.g. already-tabulated
    /// external data).
    pub fn from_counts(raw: &[u64]) -> Result<Self, EntropyError> {
        if raw.is_empty() {
            return Err(EntropyError::EmptyInput { what: "counts" });
        }
        let counts: Vec<f64> = raw.iter().map(|&c| c as f64).collect();
        let total = counts.iter().sum();
        let n_raw = raw.iter().sum::<u64>() as usize;
        Ok(Self { counts, total, n_raw })
    }

    pub fn alphabet_size(&self) -> usize {
        self.counts.len()
    }

    pub fn total(&self) -> f64 {
        self.total
    }

    /// 🧮 Raw number of observations consumed (ignores weighting).
    pub fn n_raw(&self) -> usize {
        self.n_raw
    }

    /// 🧮 Effective sample size `(sum w)^2 / sum(w^2)`, equal to `n_raw` for unweighted/integer
    /// counts.
    pub fn n_effective(&self) -> f64 {
        let sum_sq: f64 = self.counts.iter().map(|c| c * c).sum();
        if sum_sq <= 0.0 {
            0.0
        } else {
            self.total * self.total / sum_sq
        }
    }

    pub fn raw(&self) -> &[f64] {
        &self.counts
    }

    /// 🧮 Number of symbols with strictly positive count (the occupied support size).
    pub fn support_size(&self) -> usize {
        self.counts.iter().filter(|&&c| c > 0.0).count()
    }

    /// 🧮 Number of symbols observed exactly once (singletons), used by Chao-Shen/Good-Turing
    /// coverage diagnostics.
    pub fn singletons(&self) -> usize {
        self.counts.iter().filter(|&&c| (c - 1.0).abs() < 1e-12).count()
    }

    /// 🧮 Number of symbols observed exactly twice (doubletons).
    pub fn doubletons(&self) -> usize {
        self.counts.iter().filter(|&&c| (c - 2.0).abs() < 1e-12).count()
    }

    /// 🧮 Maximum-likelihood plug-in probability vector `count_i / total`.
    pub fn probabilities(&self) -> Vec<f64> {
        if self.total <= 0.0 {
            return vec![0.0; self.counts.len()];
        }
        self.counts.iter().map(|&c| c / self.total).collect()
    }

    /// 🧮 Applies a smoothing prior and returns the resulting posterior probability vector.
    pub fn smoothed_probabilities(&self, prior: SmoothingPrior) -> Vec<f64> {
        let k = self.counts.len() as f64;
        let pseudo = match prior {
            SmoothingPrior::None => 0.0,
            SmoothingPrior::Laplace => 1.0,
            SmoothingPrior::Lidstone(a) => a,
            SmoothingPrior::Jeffreys => 0.5,
            SmoothingPrior::Dirichlet(a) => a,
        };
        let denom = self.total + pseudo * k;
        if denom <= 0.0 {
            return vec![1.0 / k; self.counts.len()];
        }
        self.counts.iter().map(|&c| (c + pseudo) / denom).collect()
    }
}

/// 🧮 A prior used to smooth raw counts into a posterior probability vector before plug-in
/// estimation (Laplace/Lidstone/Jeffreys/Dirichlet families).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SmoothingPrior {
    None,
    /// 🧮 Add-one smoothing (Lidstone with `alpha = 1`).
    Laplace,
    /// 🧮 Add-`alpha` smoothing.
    Lidstone(f64),
    /// 🧮 Add-1/2 smoothing (Krichevsky-Trofimov / Jeffreys prior).
    Jeffreys,
    /// 🧮 Symmetric Dirichlet prior with concentration `alpha` per cell.
    Dirichlet(f64),
}

/// 🧮 A dense joint frequency table over two symbol alphabets, from which marginals and
/// conditionals are derived without re-scanning the original data.
#[derive(Clone, PartialEq, Debug)]
pub struct JointCounts {
    counts: Vec<f64>,
    rows: usize,
    cols: usize,
    total: f64,
}

impl JointCounts {
    /// 🧮 Builds a joint table from two equal-length symbol sequences.
    pub fn from_pairs(x: &[u32], y: &[u32], x_size: usize, y_size: usize) -> Result<Self, EntropyError> {
        if x.len() != y.len() {
            return Err(EntropyError::LengthMismatch { expected: x.len(), actual: y.len() });
        }
        if x.is_empty() {
            return Err(EntropyError::EmptyInput { what: "pairs" });
        }
        let mut counts = vec![0.0_f64; x_size * y_size];
        for (&xi, &yi) in x.iter().zip(y.iter()) {
            let (xi, yi) = (xi as usize, yi as usize);
            if xi >= x_size || yi >= y_size {
                return Err(EntropyError::ShapeMismatch { what: "joint symbol index", expected: x_size.max(y_size), actual: xi.max(yi) + 1 });
            }
            counts[xi * y_size + yi] += 1.0;
        }
        let total = x.len() as f64;
        Ok(Self { counts, rows: x_size, cols: y_size, total })
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn total(&self) -> f64 {
        self.total
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.counts[row * self.cols + col]
    }

    /// 🧮 Row-major joint probability matrix, flattened.
    pub fn joint_probabilities(&self) -> Vec<f64> {
        if self.total <= 0.0 {
            return vec![0.0; self.counts.len()];
        }
        self.counts.iter().map(|&c| c / self.total).collect()
    }

    /// 🧮 Marginal probability vector over rows (the `x` variable).
    pub fn marginal_x(&self) -> Vec<f64> {
        (0..self.rows)
            .map(|r| (0..self.cols).map(|c| self.get(r, c)).sum::<f64>() / self.total.max(1.0))
            .collect()
    }

    /// 🧮 Marginal probability vector over columns (the `y` variable).
    pub fn marginal_y(&self) -> Vec<f64> {
        (0..self.cols)
            .map(|c| (0..self.rows).map(|r| self.get(r, c)).sum::<f64>() / self.total.max(1.0))
            .collect()
    }

    /// 🧮 Flattened counts as a [`Counts`] over the joint alphabet `rows * cols`, e.g. for
    /// applying a discrete bias-corrected estimator to the joint distribution directly.
    pub fn as_counts(&self) -> Counts {
        Counts { counts: self.counts.clone(), total: self.total, n_raw: self.total as usize }
    }
}
// #endregion 🔖Counts

// #region 🔖Validation
/// 🧮 Validates and (if within tolerance) renormalizes a probability vector: rejects `NaN`/`Inf`,
/// rejects probabilities more negative than `tolerances.negative_probability`, clamps tiny
/// negatives to zero, and renormalizes when `|sum - 1| <= tolerances.renormalize_sum`.
pub fn validate_probabilities(p: &[f64], tolerances: Tolerances) -> Result<Vec<f64>, EntropyError> {
    if p.is_empty() {
        return Err(EntropyError::EmptyInput { what: "probabilities" });
    }
    let mut out = Vec::with_capacity(p.len());
    for (i, &v) in p.iter().enumerate() {
        if !v.is_finite() {
            return Err(EntropyError::NonFinite { what: "probabilities", index: i });
        }
        if v < tolerances.negative_probability {
            return Err(EntropyError::InvalidProbability { index: i, value: v });
        }
        out.push(v.max(0.0));
    }
    let sum: f64 = crate::numeric::neumaier_sum(out.iter().copied());
    if (sum - 1.0).abs() > tolerances.renormalize_sum {
        return Err(EntropyError::NotNormalized { sum });
    }
    if sum > 0.0 {
        for v in &mut out {
            *v /= sum;
        }
    }
    Ok(out)
}

/// 🧮 Maps arbitrary hashable category labels to a dense `0..k` integer alphabet, in first-seen
/// order (deterministic given a fixed input order).
pub fn encode_categories<T: std::hash::Hash + Eq + Clone>(labels: &[T]) -> (Vec<u32>, usize) {
    let mut map: HashMap<T, u32> = HashMap::new();
    let mut symbols = Vec::with_capacity(labels.len());
    for label in labels {
        let next_id = map.len() as u32;
        let id = *map.entry(label.clone()).or_insert(next_id);
        symbols.push(id);
    }
    (symbols, map.len())
}
// #endregion 🔖Validation

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_symbols_counts_correctly() {
        let counts = Counts::from_symbols(&[0, 1, 1, 2, 0, 0], 3).unwrap();
        assert_eq!(counts.raw(), &[3.0, 2.0, 1.0]);
        assert_eq!(counts.total(), 6.0);
        assert_eq!(counts.n_raw(), 6);
    }

    #[test]
    fn from_symbols_rejects_empty() {
        assert!(matches!(Counts::from_symbols(&[], 3), Err(EntropyError::EmptyInput { .. })));
    }

    #[test]
    fn from_symbols_rejects_out_of_range() {
        assert!(matches!(Counts::from_symbols(&[0, 5], 3), Err(EntropyError::ShapeMismatch { .. })));
    }

    #[test]
    fn probabilities_normalize_to_one() {
        let counts = Counts::from_symbols(&[0, 1, 1, 2], 3).unwrap();
        let p = counts.probabilities();
        assert!((p.iter().sum::<f64>() - 1.0).abs() < 1e-12);
        assert!((p[1] - 0.5).abs() < 1e-12);
    }

    #[test]
    fn weighted_counts_effective_sample_size() {
        let counts = Counts::from_weighted(&[0, 1], &[1.0, 1.0], 2).unwrap();
        assert!((counts.n_effective() - 2.0).abs() < 1e-9);
        let skewed = Counts::from_weighted(&[0, 1], &[10.0, 0.001], 2).unwrap();
        assert!(skewed.n_effective() < 1.1);
    }

    #[test]
    fn support_and_singleton_diagnostics() {
        let counts = Counts::from_symbols(&[0, 0, 1, 2, 2, 2], 4).unwrap();
        assert_eq!(counts.support_size(), 3);
        assert_eq!(counts.singletons(), 1);
        assert_eq!(counts.doubletons(), 1);
    }

    #[test]
    fn laplace_smoothing_adds_pseudocount() {
        let counts = Counts::from_symbols(&[0, 0, 1], 2).unwrap();
        let p = counts.smoothed_probabilities(SmoothingPrior::Laplace);
        // (2+1)/(3+2), (1+1)/(3+2)
        assert!((p[0] - 0.6).abs() < 1e-12);
        assert!((p[1] - 0.4).abs() < 1e-12);
    }

    #[test]
    fn joint_counts_marginals_match_independent_construction() {
        let x = [0, 0, 1, 1];
        let y = [0, 1, 0, 1];
        let joint = JointCounts::from_pairs(&x, &y, 2, 2).unwrap();
        assert_eq!(joint.marginal_x(), vec![0.5, 0.5]);
        assert_eq!(joint.marginal_y(), vec![0.5, 0.5]);
        assert_eq!(joint.total(), 4.0);
    }

    #[test]
    fn validate_probabilities_renormalizes_within_tolerance() {
        let p = validate_probabilities(&[0.5, 0.5 + 1e-10], Tolerances::default()).unwrap();
        assert!((p.iter().sum::<f64>() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn validate_probabilities_rejects_large_deviation() {
        assert!(matches!(
            validate_probabilities(&[0.5, 0.2], Tolerances::default()),
            Err(EntropyError::NotNormalized { .. })
        ));
    }

    #[test]
    fn validate_probabilities_rejects_nan() {
        assert!(matches!(
            validate_probabilities(&[0.5, f64::NAN], Tolerances::default()),
            Err(EntropyError::NonFinite { .. })
        ));
    }

    #[test]
    fn encode_categories_is_first_seen_order() {
        let (symbols, k) = encode_categories(&["b", "a", "b", "c"]);
        assert_eq!(symbols, vec![0, 1, 0, 2]);
        assert_eq!(k, 3);
    }
}
// #endregion 🔖Tests
