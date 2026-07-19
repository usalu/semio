//! ➡️ Transfer entropy and active information storage: how much a source series' past reduces
//! uncertainty about a target's future, beyond what the target's own past already explains.
//! Supports a quantile-binned discrete backend and a Frenzel-Pompe kNN (KSG-style) continuous
//! backend, both built from delay-embedded history vectors.

use crate::knn::KdTree;
use crate::numeric::digamma;
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Metric, Warning};

// #region 🔖Embedding
fn quantile_bin(x: &[f64], bins: usize) -> Vec<u32> {
    let mut sorted = x.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let edges: Vec<f64> = (1..bins).map(|i| sorted[(sorted.len() * i / bins).min(sorted.len() - 1)]).collect();
    x.iter().map(|&v| edges.partition_point(|&e| e <= v) as u32).collect()
}

/// ➡️ Delay-embedded history matrix (row-major, `n_rows x dim`) of `x[i-dim..i]` for `i` in
/// `start..x.len()`, most-recent sample last in each row.
fn history_matrix(x: &[f64], dim: usize, start: usize) -> Vec<f64> {
    let mut out = Vec::with_capacity((x.len() - start) * dim);
    for i in start..x.len() {
        for j in 0..dim {
            out.push(x[i - dim + j]);
        }
    }
    out
}
// #endregion 🔖Embedding

// #region 🔖KsgGeneralized
/// ➡️ Generalized KSG-1 mutual information for multivariate `X` (`x_dim` columns) and `Y`
/// (`y_dim` columns) packed as `[X | Y]` row-major joint points.
fn ksg1_generalized(joint: &[f64], total_dim: usize, x_dim: usize, k: usize) -> Result<f64, EntropyError> {
    let n = joint.len() / total_dim;
    if k == 0 || k >= n {
        return Err(EntropyError::InvalidConfig { field: "k", reason: "must satisfy 0 < k < n" });
    }
    let tree = KdTree::build(joint, total_dim)?;
    let row = |i: usize| -> &[f64] { &joint[i * total_dim..(i + 1) * total_dim] };
    let sub_dist = |a: &[f64], b: &[f64], lo: usize, hi: usize| -> f64 { (lo..hi).map(|d| (a[d] - b[d]).abs()).fold(0.0_f64, f64::max) };

    let mut sum = 0.0_f64;
    for i in 0..n {
        let neighbors = tree.k_nearest(row(i), k, Metric::Chebyshev, Some(i));
        let eps = neighbors.last().map(|&(_, d)| d).unwrap_or(0.0);
        let mut nx = 0usize;
        let mut ny = 0usize;
        for j in 0..n {
            if j == i {
                continue;
            }
            if sub_dist(row(i), row(j), 0, x_dim) < eps {
                nx += 1;
            }
            if sub_dist(row(i), row(j), x_dim, total_dim) < eps {
                ny += 1;
            }
        }
        sum += digamma(nx as f64 + 1.0) + digamma(ny as f64 + 1.0);
    }
    Ok(digamma(k as f64) - sum / n as f64 + digamma(n as f64))
}

/// ➡️ Generalized Frenzel-Pompe kNN conditional mutual information `I(X;Y|Z)` for multivariate
/// `X`/`Y`/`Z` packed as `[X | Y | Z]` row-major joint points.
fn ksg_cmi_generalized(joint: &[f64], total_dim: usize, x_dim: usize, y_dim: usize, k: usize) -> Result<f64, EntropyError> {
    let n = joint.len() / total_dim;
    if k == 0 || k >= n {
        return Err(EntropyError::InvalidConfig { field: "k", reason: "must satisfy 0 < k < n" });
    }
    let z_start = x_dim + y_dim;
    let tree = KdTree::build(joint, total_dim)?;
    let row = |i: usize| -> &[f64] { &joint[i * total_dim..(i + 1) * total_dim] };
    let sub_dist = |a: &[f64], b: &[f64], lo: usize, hi: usize| -> f64 { (lo..hi).map(|d| (a[d] - b[d]).abs()).fold(0.0_f64, f64::max) };

    let mut sum = 0.0_f64;
    for i in 0..n {
        let neighbors = tree.k_nearest(row(i), k, Metric::Chebyshev, Some(i));
        let eps = neighbors.last().map(|&(_, d)| d).unwrap_or(0.0);
        let mut n_xz = 0usize;
        let mut n_yz = 0usize;
        let mut n_z = 0usize;
        for j in 0..n {
            if j == i {
                continue;
            }
            let xz = sub_dist(row(i), row(j), 0, x_dim).max(sub_dist(row(i), row(j), z_start, total_dim));
            let yz = sub_dist(row(i), row(j), x_dim, z_start).max(sub_dist(row(i), row(j), z_start, total_dim));
            let z = sub_dist(row(i), row(j), z_start, total_dim);
            if xz < eps {
                n_xz += 1;
            }
            if yz < eps {
                n_yz += 1;
            }
            if z < eps {
                n_z += 1;
            }
        }
        sum += digamma(n_xz as f64 + 1.0) + digamma(n_yz as f64 + 1.0) - digamma(n_z as f64 + 1.0);
    }
    Ok(digamma(k as f64) - sum / n as f64)
}
// #endregion 🔖KsgGeneralized

// #region 🔖Config
/// ➡️ Which estimator [`transfer_entropy`] and [`active_information_storage`] use.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TeBackend {
    Discrete { bins: usize },
    Knn { k: usize },
}

/// ➡️ Configuration for [`transfer_entropy`]. History embeddings use a fixed delay `tau = 1`.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TransferConfig {
    pub k_history: usize,
    pub l_history: usize,
    pub backend: TeBackend,
}

impl TransferConfig {
    pub fn new(k_history: usize, l_history: usize, backend: TeBackend) -> Result<Self, EntropyError> {
        if k_history == 0 || l_history == 0 {
            return Err(EntropyError::InvalidConfig { field: "history", reason: "k_history and l_history must be at least 1" });
        }
        Ok(Self { k_history, l_history, backend })
    }
}
// #endregion 🔖Config

// #region 🔖TransferEntropy
/// ➡️ Transfer entropy `TE(source -> target) = I(target_future ; source_past | target_past)`.
pub fn transfer_entropy(source: &[f64], target: &[f64], cfg: TransferConfig) -> Result<Estimate, EntropyError> {
    if source.len() != target.len() {
        return Err(EntropyError::LengthMismatch { expected: source.len(), actual: target.len() });
    }
    let n = source.len();
    let start = cfg.k_history.max(cfg.l_history);
    if n <= start {
        return Err(EntropyError::InsufficientData { what: "transfer_entropy", needed: start + 1, actual: n });
    }
    let n_samples = n - start;

    let nats = match cfg.backend {
        TeBackend::Discrete { bins } => {
            if bins < 2 {
                return Err(EntropyError::InvalidConfig { field: "bins", reason: "must be at least 2" });
            }
            let source_symbols = quantile_bin(source, bins);
            let target_symbols = quantile_bin(target, bins);
            let mut a = Vec::with_capacity(n_samples); // source_past packed
            let mut b = Vec::with_capacity(n_samples); // target_past packed
            let mut c = Vec::with_capacity(n_samples); // target_future
            let a_size = (bins as u64).pow(cfg.l_history as u32).min(u32::MAX as u64) as usize;
            let b_size = (bins as u64).pow(cfg.k_history as u32).min(u32::MAX as u64) as usize;
            if (bins as u64).pow(cfg.l_history as u32) > u32::MAX as u64 || (bins as u64).pow(cfg.k_history as u32) > u32::MAX as u64 {
                return Err(EntropyError::InvalidConfig { field: "history", reason: "packed alphabet exceeds u32::MAX" });
            }
            for i in start..n {
                let a_sym = source_symbols[i - cfg.l_history..i].iter().fold(0u64, |acc, &s| acc * bins as u64 + s as u64) as u32;
                let b_sym = target_symbols[i - cfg.k_history..i].iter().fold(0u64, |acc, &s| acc * bins as u64 + s as u64) as u32;
                a.push(a_sym);
                b.push(b_sym);
                c.push(target_symbols[i]);
            }
            let _ = (a_size, b_size);
            crate::mutual::conditional_mutual_information(&a, &c, &b, LogBase::Nats)?.value
        }
        TeBackend::Knn { k } => {
            let a = history_matrix(source, cfg.l_history, start);
            let b = history_matrix(target, cfg.k_history, start);
            let c: Vec<f64> = (start..n).map(|i| target[i]).collect();
            let total_dim = cfg.l_history + cfg.k_history + 1;
            let mut joint = Vec::with_capacity(n_samples * total_dim);
            for i in 0..n_samples {
                joint.extend_from_slice(&a[i * cfg.l_history..(i + 1) * cfg.l_history]);
                joint.extend_from_slice(&b[i * cfg.k_history..(i + 1) * cfg.k_history]);
                joint.push(c[i]);
            }
            ksg_cmi_generalized(&joint, total_dim, cfg.l_history, 1, k)?
        }
    };
    let nats = crate::numeric::clamp_near_zero(nats, 1e-6);

    let mut warnings = Vec::new();
    if n_samples < 200 {
        warnings.push(Warning::SmallSample { n: n_samples, recommended: 200 });
    }

    Ok(Estimate {
        value: nats,
        base: LogBase::Nats,
        method: "transfer_entropy",
        n: n_samples,
        n_effective: n_samples as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("k_history", cfg.k_history as f64), ("l_history", cfg.l_history as f64)],
    })
}
// #endregion 🔖TransferEntropy

// #region 🔖ActiveInformationStorage
/// ➡️ Active information storage `AIS(Y) = I(Y_future ; Y_past)`, how predictable a series is
/// from its own `k_history`-length past.
pub fn active_information_storage(x: &[f64], k_history: usize, backend: TeBackend, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if k_history == 0 {
        return Err(EntropyError::InvalidConfig { field: "k_history", reason: "must be at least 1" });
    }
    let n = x.len();
    if n <= k_history {
        return Err(EntropyError::InsufficientData { what: "active_information_storage", needed: k_history + 1, actual: n });
    }
    let n_samples = n - k_history;

    let nats = match backend {
        TeBackend::Discrete { bins } => {
            if bins < 2 {
                return Err(EntropyError::InvalidConfig { field: "bins", reason: "must be at least 2" });
            }
            let symbols = quantile_bin(x, bins);
            if (bins as u64).pow(k_history as u32) > u32::MAX as u64 {
                return Err(EntropyError::InvalidConfig { field: "k_history", reason: "packed alphabet exceeds u32::MAX" });
            }
            let mut past = Vec::with_capacity(n_samples);
            let mut future = Vec::with_capacity(n_samples);
            for i in k_history..n {
                let sym = symbols[i - k_history..i].iter().fold(0u64, |acc, &s| acc * bins as u64 + s as u64) as u32;
                past.push(sym);
                future.push(symbols[i]);
            }
            crate::mutual::mutual_information(&past, &future, crate::estimators::DiscreteMethod::Plugin, LogBase::Nats)?.value
        }
        TeBackend::Knn { k } => {
            let past = history_matrix(x, k_history, k_history);
            let future: Vec<f64> = (k_history..n).map(|i| x[i]).collect();
            let total_dim = k_history + 1;
            let mut joint = Vec::with_capacity(n_samples * total_dim);
            for i in 0..n_samples {
                joint.extend_from_slice(&past[i * k_history..(i + 1) * k_history]);
                joint.push(future[i]);
            }
            ksg1_generalized(&joint, total_dim, k_history, k)?
        }
    };
    let clamped = crate::numeric::clamp_near_zero(nats, 1e-6);

    Ok(Estimate {
        value: base.from_nats(clamped),
        base,
        method: "active_information_storage",
        n: n_samples,
        n_effective: n_samples as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings: Vec::new(),
        diagnostics: vec![("k_history", k_history as f64)],
    })
}
// #endregion 🔖ActiveInformationStorage

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_config_rejects_zero_history() {
        assert!(TransferConfig::new(0, 1, TeBackend::Discrete { bins: 3 }).is_err());
    }

    #[test]
    fn te_of_independent_series_discrete_is_near_zero() {
        let mut rng = crate::numeric::Xorshift64::new(1);
        let n = 3000;
        let source: Vec<f64> = (0..n).map(|_| rng.next_f64()).collect();
        let target: Vec<f64> = (0..n).map(|_| rng.next_f64()).collect();
        let cfg = TransferConfig::new(1, 1, TeBackend::Discrete { bins: 3 }).unwrap();
        let est = transfer_entropy(&source, &target, cfg).unwrap();
        assert!(est.value.abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn te_detects_coupling_discrete() {
        // 🔐 target[i] = source[i-1] (with some noise mixed via binning): TE(source->target)
        // should be clearly larger than TE(target->source).
        let mut rng = crate::numeric::Xorshift64::new(2);
        let n = 4000;
        let source: Vec<f64> = (0..n).map(|_| rng.next_f64()).collect();
        let mut target = vec![0.0; n];
        for i in 1..n {
            target[i] = source[i - 1];
        }
        let cfg = TransferConfig::new(1, 1, TeBackend::Discrete { bins: 4 }).unwrap();
        let forward = transfer_entropy(&source, &target, cfg).unwrap();
        let backward = transfer_entropy(&target, &source, cfg).unwrap();
        assert!(forward.value > backward.value, "forward={} backward={}", forward.value, backward.value);
        assert!(forward.value > 0.1, "forward={}", forward.value);
    }

    #[test]
    fn ais_of_white_noise_is_near_zero() {
        let mut rng = crate::numeric::Xorshift64::new(3);
        let x: Vec<f64> = (0..2000).map(|_| rng.next_f64()).collect();
        let est = active_information_storage(&x, 1, TeBackend::Discrete { bins: 3 }, LogBase::Nats).unwrap();
        assert!(est.value.abs() < 0.05, "got {}", est.value);
    }

    #[test]
    fn ais_of_highly_predictable_series_is_positive() {
        let mut rng = crate::numeric::Xorshift64::new(4);
        let n = 2000;
        let mut x = vec![0.0; n];
        for i in 1..n {
            x[i] = 0.9 * x[i - 1] + 0.1 * rng.next_gaussian();
        }
        let cfg_backend = TeBackend::Discrete { bins: 4 };
        let est = active_information_storage(&x, 1, cfg_backend, LogBase::Nats).unwrap();
        assert!(est.value > 0.1, "got {}", est.value);
    }

    #[test]
    fn te_rejects_length_mismatch() {
        let cfg = TransferConfig::new(1, 1, TeBackend::Discrete { bins: 3 }).unwrap();
        assert!(matches!(transfer_entropy(&[1.0, 2.0], &[1.0], cfg), Err(EntropyError::LengthMismatch { .. })));
    }

    mod quick {
        use super::*;

        #[test]
        fn te_knn_detects_coupling_on_coupled_logistic_maps() {
            let mut rng = crate::numeric::Xorshift64::new(5);
            let n = 800;
            let mut x = vec![0.4 + 0.1 * rng.next_f64(); n];
            let mut y = vec![0.4 + 0.1 * rng.next_f64(); n];
            let r = 3.7;
            let coupling = 0.3;
            for i in 1..n {
                x[i] = r * x[i - 1] * (1.0 - x[i - 1]);
                let driven = (1.0 - coupling) * y[i - 1] + coupling * x[i - 1];
                y[i] = r * driven * (1.0 - driven);
            }
            let cfg = TransferConfig::new(1, 1, TeBackend::Knn { k: 4 }).unwrap();
            let forward = transfer_entropy(&x, &y, cfg).unwrap();
            let backward = transfer_entropy(&y, &x, cfg).unwrap();
            assert!(forward.value > backward.value, "forward={} backward={}", forward.value, backward.value);
        }
    }
}
// #endregion 🔖Tests
