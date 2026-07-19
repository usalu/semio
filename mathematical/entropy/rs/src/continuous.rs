//! 📈 Differential (continuous) entropy estimators: histogram, Gaussian KDE (leave-one-out
//! plug-in), Kozachenko-Leonenko kNN, Vasicek/Correa m-spacing, and the Gaussian closed form.

use crate::knn::KdTree;
use crate::numeric::{digamma, log_sum_exp, neumaier_sum, x_ln_x};
use crate::{BinsSpec, ConfidenceInterval, EntropyError, Estimate, LogBase, Metric, Warning};

// #region 🔖Shared
fn mean_and_sd(x: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let mean = neumaier_sum(x.iter().copied()) / n;
    let var = neumaier_sum(x.iter().map(|&v| (v - mean).powi(2))) / n;
    (mean, var.sqrt())
}

fn validate_series(x: &[f64], what: &'static str) -> Result<(), EntropyError> {
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what });
    }
    for (i, &v) in x.iter().enumerate() {
        if !v.is_finite() {
            return Err(EntropyError::NonFinite { what, index: i });
        }
    }
    Ok(())
}

/// 📈 Default spacing-estimator window `m = round(sqrt(N))`, clamped to `[1, N/2 - 1]`.
fn default_spacing_m(n: usize) -> usize {
    let m = (n as f64).sqrt().round() as usize;
    m.clamp(1, (n / 2).saturating_sub(1).max(1))
}
// #endregion 🔖Shared

// #region 🔖Kernel
/// 📈 Kernel family for [`KdeDensity`].
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum Kernel {
    #[default]
    Gaussian,
    Epanechnikov,
}

impl Kernel {
    fn log_density_contribution(self, scaled_diff: f64) -> f64 {
        match self {
            Kernel::Gaussian => -0.5 * scaled_diff * scaled_diff,
            Kernel::Epanechnikov => {
                if scaled_diff.abs() >= 5.0_f64.sqrt() {
                    f64::NEG_INFINITY
                } else {
                    (0.75 * (1.0 - scaled_diff * scaled_diff / 5.0) / 5.0_f64.sqrt()).max(1e-300).ln()
                }
            }
        }
    }
}

/// 📈 Bandwidth selection rule for [`KdeDensity`].
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum Bandwidth {
    #[default]
    Silverman,
    Scott,
    Fixed(f64),
}
// #endregion 🔖Kernel

// #region 🔖Kde
/// 📈 Configuration for [`KdeDensity::fit`].
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct KdeConfig {
    pub kernel: Kernel,
    pub bandwidth: Bandwidth,
}

/// 📈 A fitted 1-D Gaussian/Epanechnikov kernel density estimate. Fit once, query `pdf` or
/// `entropy` many times.
pub struct KdeDensity {
    data: Vec<f64>,
    kernel: Kernel,
    h: f64,
}

impl KdeDensity {
    /// 📈 Fits a KDE to `x`, selecting bandwidth `h` per `cfg.bandwidth` if not [`Bandwidth::Fixed`].
    pub fn fit(x: &[f64], cfg: KdeConfig) -> Result<Self, EntropyError> {
        validate_series(x, "kde input")?;
        if x.len() < 2 {
            return Err(EntropyError::InsufficientData { what: "KDE", needed: 2, actual: x.len() });
        }
        let (_, sd) = mean_and_sd(x);
        if sd <= 0.0 {
            return Err(EntropyError::DegenerateInput { what: "constant series has zero bandwidth" });
        }
        let n = x.len() as f64;
        let h = match cfg.bandwidth {
            Bandwidth::Silverman => sd * (4.0 / (3.0 * n)).powf(0.2),
            Bandwidth::Scott => sd * n.powf(-0.2),
            Bandwidth::Fixed(h) => {
                if !(h > 0.0 && h.is_finite()) {
                    return Err(EntropyError::InvalidConfig { field: "bandwidth", reason: "must be finite and positive" });
                }
                h
            }
        };
        Ok(Self { data: x.to_vec(), kernel: cfg.kernel, h })
    }

    /// 📈 Density estimate at `x` using the full sample (not leave-one-out).
    pub fn pdf(&self, x: f64) -> f64 {
        let n = self.data.len() as f64;
        let log_terms: Vec<f64> = self
            .data
            .iter()
            .map(|&xi| self.kernel.log_density_contribution((x - xi) / self.h))
            .collect();
        (log_sum_exp(&log_terms) - n.ln() - self.h.ln()).exp()
    }

    /// 📈 Leave-one-out differential entropy plug-in: `-1/N * sum ln f_{-i}(x_i)`, which removes
    /// the systematic downward bias the self-term introduces in plain resubstitution.
    pub fn entropy(&self, base: LogBase) -> Result<Estimate, EntropyError> {
        base.validate()?;
        let n = self.data.len();
        let nf = n as f64;
        let mut sum_log_density = 0.0_f64;
        for i in 0..n {
            let log_terms: Vec<f64> = (0..n)
                .filter(|&j| j != i)
                .map(|j| self.kernel.log_density_contribution((self.data[i] - self.data[j]) / self.h))
                .collect();
            let log_density = log_sum_exp(&log_terms) - (nf - 1.0).ln() - self.h.ln();
            sum_log_density += log_density;
        }
        let nats = -sum_log_density / nf;
        Ok(Estimate {
            value: base.from_nats(nats),
            base,
            method: "kde_loo",
            n,
            n_effective: nf,
            std_error: None,
            ci: None::<ConfidenceInterval>,
            warnings: Vec::new(),
            diagnostics: vec![("bandwidth", self.h)],
        })
    }
}
// #endregion 🔖Kde

// #region 🔖Histogram
fn histogram_entropy_nats(x: &[f64], bins: &BinsSpec) -> Result<(f64, usize), EntropyError> {
    let n = x.len() as f64;
    let (min, max) = x.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
    if !(max > min) {
        return Err(EntropyError::DegenerateInput { what: "constant series has zero range" });
    }
    let edges: Vec<f64> = match bins {
        BinsSpec::Edges(e) => e.clone(),
        _ => {
            let k = match bins {
                BinsSpec::Fixed(k) => *k,
                BinsSpec::Sturges => (1.0 + (x.len() as f64).log2()).ceil() as usize,
                BinsSpec::Scott => {
                    let (_, sd) = mean_and_sd(x);
                    let width = 3.49 * sd / (x.len() as f64).powf(1.0 / 3.0);
                    (((max - min) / width.max(1e-12)).ceil() as usize).max(1)
                }
                BinsSpec::FreedmanDiaconis => {
                    let mut sorted = x.to_vec();
                    sorted.sort_by(|a, b| a.total_cmp(b));
                    let q1 = sorted[(sorted.len() as f64 * 0.25) as usize];
                    let q3 = sorted[(sorted.len() as f64 * 0.75).min(sorted.len() as f64 - 1.0) as usize];
                    let width = 2.0 * (q3 - q1) / (x.len() as f64).powf(1.0 / 3.0);
                    (((max - min) / width.max(1e-12)).ceil() as usize).max(1)
                }
                BinsSpec::Doane => {
                    let (_, sd) = mean_and_sd(x);
                    let n = x.len() as f64;
                    let mean = neumaier_sum(x.iter().copied()) / n;
                    let skew = if sd > 0.0 {
                        neumaier_sum(x.iter().map(|&v| ((v - mean) / sd).powi(3))) / n
                    } else {
                        0.0
                    };
                    let sigma_g1 = (6.0 * (n - 2.0) / ((n + 1.0) * (n + 3.0))).sqrt();
                    let k = 1.0 + (x.len() as f64).log2() + (1.0 + skew.abs() / sigma_g1.max(1e-12)).log2();
                    (k.ceil() as usize).max(1)
                }
                BinsSpec::Edges(_) => unreachable!(),
            }
            .max(1);
            (0..=k).map(|i| min + (max - min) * i as f64 / k as f64).collect()
        }
    };
    if edges.len() < 2 {
        return Err(EntropyError::InvalidConfig { field: "bins", reason: "need at least 2 edges" });
    }
    let k = edges.len() - 1;
    let mut counts = vec![0.0_f64; k];
    for &v in x {
        let mut bin = match edges.binary_search_by(|e| e.total_cmp(&v)) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        bin = bin.min(k - 1);
        counts[bin] += 1.0;
    }
    let mut nats = 0.0_f64;
    for (i, &c) in counts.iter().enumerate() {
        let p = c / n;
        let width = edges[i + 1] - edges[i];
        if p > 0.0 && width > 0.0 {
            nats -= x_ln_x(p) - p * width.ln();
        }
    }
    Ok((nats, k))
}
// #endregion 🔖Histogram

// #region 🔖Knn
fn kozachenko_leonenko_nats(x: &[f64], k: usize) -> Result<f64, EntropyError> {
    let n = x.len();
    if k == 0 || k >= n {
        return Err(EntropyError::InvalidConfig { field: "k", reason: "must satisfy 0 < k < n" });
    }
    let tree = KdTree::build(x, 1)?;
    let mut sum_log_eps = 0.0_f64;
    for i in 0..n {
        let neighbors = tree.k_nearest(&[x[i]], k, Metric::Chebyshev, Some(i));
        let eps = neighbors.last().map(|&(_, d)| d).unwrap_or(0.0);
        if eps <= 0.0 {
            return Err(EntropyError::DegenerateInput { what: "duplicate points cause zero k-th neighbor distance" });
        }
        sum_log_eps += (2.0 * eps).ln();
    }
    Ok(digamma(n as f64) - digamma(k as f64) + sum_log_eps / n as f64)
}
// #endregion 🔖Knn

// #region 🔖Spacing
fn sorted_with_clamped(x: &[f64], i: i64) -> f64 {
    let n = x.len() as i64;
    let clamped = i.clamp(1, n);
    x[(clamped - 1) as usize]
}

fn vasicek_nats(x: &[f64], m: usize) -> Result<f64, EntropyError> {
    let mut sorted = x.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let n = sorted.len();
    let nf = n as f64;
    let mut sum = 0.0_f64;
    for i in 1..=n as i64 {
        let upper = sorted_with_clamped(&sorted, i + m as i64);
        let lower = sorted_with_clamped(&sorted, i - m as i64);
        let spacing = upper - lower;
        if spacing <= 0.0 {
            return Err(EntropyError::DegenerateInput { what: "zero spacing between order statistics; consider jittering ties" });
        }
        sum += (nf / (2.0 * m as f64) * spacing).ln();
    }
    Ok(sum / nf)
}

fn correa_nats(x: &[f64], m: usize) -> Result<f64, EntropyError> {
    let mut sorted = x.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let n = sorted.len() as i64;
    let mut sum = 0.0_f64;
    for i in 1..=n {
        let lo = i - m as i64;
        let hi = i + m as i64;
        let window: Vec<f64> = (lo..=hi).map(|j| sorted_with_clamped(&sorted, j)).collect();
        let mean = window.iter().sum::<f64>() / window.len() as f64;
        let mut num = 0.0_f64;
        let mut denom = 0.0_f64;
        for (offset, &xj) in (lo..=hi).zip(window.iter()) {
            num += (xj - mean) * (offset - i) as f64;
            denom += (xj - mean).powi(2);
        }
        let ratio = num / (n as f64 * denom.max(1e-300));
        if ratio <= 0.0 {
            return Err(EntropyError::DegenerateInput { what: "non-positive Correa ratio; consider jittering ties" });
        }
        sum += ratio.ln();
    }
    Ok(-sum / n as f64)
}
// #endregion 🔖Spacing

// #region 🔖Gaussian
fn gaussian_mle_nats(x: &[f64]) -> Result<f64, EntropyError> {
    let (_, sd) = mean_and_sd(x);
    if sd <= 0.0 {
        return Err(EntropyError::DegenerateInput { what: "constant series has zero variance" });
    }
    Ok(0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E * sd * sd).ln())
}
// #endregion 🔖Gaussian

// #region 🔖Dispatch
/// 📈 Which continuous (differential) entropy estimator [`entropy_continuous`] applies.
#[derive(Clone, PartialEq, Debug)]
pub enum ContinuousMethod {
    Histogram(BinsSpec),
    Kde(KdeConfig),
    Knn { k: usize },
    Vasicek { m: usize },
    Correa { m: usize },
    GaussianMle,
}

fn method_name(method: &ContinuousMethod) -> &'static str {
    match method {
        ContinuousMethod::Histogram(_) => "histogram",
        ContinuousMethod::Kde(_) => "kde_loo",
        ContinuousMethod::Knn { .. } => "kozachenko_leonenko",
        ContinuousMethod::Vasicek { .. } => "vasicek",
        ContinuousMethod::Correa { .. } => "correa",
        ContinuousMethod::GaussianMle => "gaussian_mle",
    }
}

/// 📈 Estimates the differential entropy of the distribution underlying `x` (raw continuous
/// samples) using the given `method`.
pub fn entropy_continuous(x: &[f64], method: ContinuousMethod, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    validate_series(x, "continuous input")?;
    let n = x.len();
    if n < 2 {
        return Err(EntropyError::InsufficientData { what: "continuous entropy", needed: 2, actual: n });
    }

    let mut diagnostics = Vec::new();
    let nats = match &method {
        ContinuousMethod::Histogram(bins) => {
            let (nats, k) = histogram_entropy_nats(x, bins)?;
            diagnostics.push(("bins", k as f64));
            nats
        }
        ContinuousMethod::Kde(cfg) => {
            let density = KdeDensity::fit(x, *cfg)?;
            let est = density.entropy(LogBase::Nats)?;
            diagnostics.push(("bandwidth", density.h));
            est.value
        }
        ContinuousMethod::Knn { k } => {
            diagnostics.push(("k", *k as f64));
            kozachenko_leonenko_nats(x, *k)?
        }
        ContinuousMethod::Vasicek { m } => {
            let m = if *m == 0 { default_spacing_m(n) } else { *m };
            diagnostics.push(("m", m as f64));
            vasicek_nats(x, m)?
        }
        ContinuousMethod::Correa { m } => {
            let m = if *m == 0 { default_spacing_m(n) } else { *m };
            diagnostics.push(("m", m as f64));
            correa_nats(x, m)?
        }
        ContinuousMethod::GaussianMle => gaussian_mle_nats(x)?,
    };

    let mut warnings = Vec::new();
    if n < 30 {
        warnings.push(Warning::SmallSample { n, recommended: 30 });
    }

    Ok(Estimate {
        value: base.from_nats(nats),
        base,
        method: method_name(&method),
        n,
        n_effective: n as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics,
    })
}
// #endregion 🔖Dispatch

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn box_muller_gaussian(n: usize, seed: u64) -> Vec<f64> {
        let mut rng = crate::numeric::Xorshift64::new(seed);
        (0..n).map(|_| rng.next_gaussian()).collect()
    }

    #[test]
    fn gaussian_mle_matches_closed_form() {
        let x = box_muller_gaussian(5000, 1);
        let est = entropy_continuous(&x, ContinuousMethod::GaussianMle, LogBase::Nats).unwrap();
        let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
        assert!((est.value - expected).abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn knn_entropy_matches_gaussian_closed_form() {
        let x = box_muller_gaussian(3000, 2);
        let est = entropy_continuous(&x, ContinuousMethod::Knn { k: 5 }, LogBase::Nats).unwrap();
        let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
        assert!((est.value - expected).abs() < 0.1, "got {}", est.value);
    }

    #[test]
    fn kde_entropy_matches_gaussian_closed_form() {
        let x = box_muller_gaussian(2000, 3);
        let cfg = KdeConfig { kernel: Kernel::Gaussian, bandwidth: Bandwidth::Silverman };
        let est = entropy_continuous(&x, ContinuousMethod::Kde(cfg), LogBase::Nats).unwrap();
        let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
        assert!((est.value - expected).abs() < 0.1, "got {}", est.value);
    }

    #[test]
    fn vasicek_entropy_matches_gaussian_closed_form() {
        let x = box_muller_gaussian(5000, 4);
        let est = entropy_continuous(&x, ContinuousMethod::Vasicek { m: 0 }, LogBase::Nats).unwrap();
        let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
        assert!((est.value - expected).abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn correa_entropy_matches_gaussian_closed_form() {
        let x = box_muller_gaussian(3000, 5);
        let est = entropy_continuous(&x, ContinuousMethod::Correa { m: 0 }, LogBase::Nats).unwrap();
        let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
        assert!((est.value - expected).abs() < 0.1, "got {}", est.value);
    }

    #[test]
    fn uniform_entropy_near_zero() {
        let mut rng = crate::numeric::Xorshift64::new(6);
        let x: Vec<f64> = (0..5000).map(|_| rng.next_f64()).collect();
        let est = entropy_continuous(&x, ContinuousMethod::Vasicek { m: 0 }, LogBase::Nats).unwrap();
        assert!(est.value.abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn histogram_entropy_reasonable_for_uniform() {
        let mut rng = crate::numeric::Xorshift64::new(7);
        let x: Vec<f64> = (0..5000).map(|_| rng.next_f64()).collect();
        let est = entropy_continuous(&x, ContinuousMethod::Histogram(BinsSpec::Sturges), LogBase::Nats).unwrap();
        assert!(est.value.abs() < 0.2, "got {}", est.value);
    }

    #[test]
    fn rejects_constant_series() {
        let x = vec![1.0; 100];
        assert!(entropy_continuous(&x, ContinuousMethod::GaussianMle, LogBase::Nats).is_err());
        assert!(entropy_continuous(&x, ContinuousMethod::Knn { k: 3 }, LogBase::Nats).is_err());
    }

    #[test]
    fn rejects_too_few_samples() {
        let x = vec![1.0];
        assert!(matches!(
            entropy_continuous(&x, ContinuousMethod::GaussianMle, LogBase::Nats),
            Err(EntropyError::InsufficientData { .. })
        ));
    }

    #[test]
    fn kde_loo_differs_from_naive_resubstitution_direction() {
        // 🔐 LOO removes the self-term's downward bias, so LOO entropy should exceed naive
        // resubstitution (which double-counts each point against itself).
        let x = box_muller_gaussian(500, 8);
        let cfg = KdeConfig { kernel: Kernel::Gaussian, bandwidth: Bandwidth::Silverman };
        let density = KdeDensity::fit(&x, cfg).unwrap();
        let loo = density.entropy(LogBase::Nats).unwrap().value;
        let resub_nats = -x.iter().map(|&xi| density.pdf(xi).max(1e-300).ln()).sum::<f64>() / x.len() as f64;
        assert!(loo > resub_nats);
    }

    mod quick {
        use super::*;

        #[test]
        fn exponential_entropy_matches_closed_form() {
            // 🔐 differential entropy of Exp(1) is 1 nat.
            let mut rng = crate::numeric::Xorshift64::new(9);
            let x: Vec<f64> = (0..5000).map(|_| -rng.next_f64().max(1e-12).ln()).collect();
            let est = entropy_continuous(&x, ContinuousMethod::Vasicek { m: 0 }, LogBase::Nats).unwrap();
            assert!((est.value - 1.0).abs() < 0.05, "got {}", est.value);
        }

        #[test]
        fn all_continuous_estimators_agree_within_tolerance_on_gaussian() {
            let x = box_muller_gaussian(4000, 42);
            let expected = 0.5 * (2.0 * core::f64::consts::PI * core::f64::consts::E).ln();
            let methods = vec![
                ContinuousMethod::GaussianMle,
                ContinuousMethod::Knn { k: 5 },
                ContinuousMethod::Vasicek { m: 0 },
                ContinuousMethod::Kde(KdeConfig::default()),
            ];
            for method in methods {
                let est = entropy_continuous(&x, method.clone(), LogBase::Nats).unwrap();
                assert!((est.value - expected).abs() < 0.15, "{:?} -> {}", method, est.value);
            }
        }
    }
}
// #endregion 🔖Tests
