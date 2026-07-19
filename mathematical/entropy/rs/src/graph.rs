//! 🕸️ Graph entropy over plain edge lists (no graph-library dependency): degree-distribution
//! entropy and random-walk entropy rate.

use crate::numeric::x_ln_x;
use crate::{ConfidenceInterval, EntropyError, Estimate, LogBase, Warning};

// #region 🔖Degree
/// 🕸️ Shannon entropy of the (out-)degree distribution of a graph given as an edge list.
/// `directed` selects out-degree counting; undirected counts each edge toward both endpoints.
pub fn degree_distribution_entropy(edges: &[(u32, u32)], n_nodes: usize, directed: bool, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if n_nodes == 0 {
        return Err(EntropyError::EmptyInput { what: "n_nodes" });
    }
    let mut degree = vec![0u64; n_nodes];
    for &(a, b) in edges {
        let (a, b) = (a as usize, b as usize);
        if a >= n_nodes || b >= n_nodes {
            return Err(EntropyError::ShapeMismatch { what: "edge endpoint", expected: n_nodes, actual: a.max(b) + 1 });
        }
        degree[a] += 1;
        if !directed {
            degree[b] += 1;
        }
    }
    let max_degree = degree.iter().copied().max().unwrap_or(0) as usize;
    let mut hist = vec![0.0_f64; max_degree + 1];
    for &d in &degree {
        hist[d as usize] += 1.0;
    }
    let n = n_nodes as f64;
    let nats = -hist.iter().map(|&c| x_ln_x(c / n)).sum::<f64>();

    let mut warnings = Vec::new();
    if n_nodes < 30 {
        warnings.push(Warning::SmallSample { n: n_nodes, recommended: 30 });
    }

    Ok(Estimate {
        value: base.from_nats(nats),
        base,
        method: "degree_distribution_entropy",
        n: n_nodes,
        n_effective: n_nodes as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings,
        diagnostics: vec![("max_degree", max_degree as f64), ("edges", edges.len() as f64)],
    })
}
// #endregion 🔖Degree

// #region 🔖RandomWalk
/// 🕸️ Random-walk entropy rate `sum_i pi_i * H(row_i)`: the average per-step uncertainty of a
/// simple (optionally weighted) random walk on the graph, weighted by the walk's stationary node
/// distribution (via power iteration).
pub fn random_walk_entropy_rate(edges: &[(u32, u32)], n_nodes: usize, weights: Option<&[f64]>, base: LogBase) -> Result<Estimate, EntropyError> {
    base.validate()?;
    if n_nodes == 0 {
        return Err(EntropyError::EmptyInput { what: "n_nodes" });
    }
    if let Some(w) = weights {
        if w.len() != edges.len() {
            return Err(EntropyError::LengthMismatch { expected: edges.len(), actual: w.len() });
        }
    }
    let mut adjacency = vec![0.0_f64; n_nodes * n_nodes];
    for (i, &(a, b)) in edges.iter().enumerate() {
        let (a, b) = (a as usize, b as usize);
        if a >= n_nodes || b >= n_nodes {
            return Err(EntropyError::ShapeMismatch { what: "edge endpoint", expected: n_nodes, actual: a.max(b) + 1 });
        }
        let w = weights.map(|ws| ws[i]).unwrap_or(1.0);
        if w < 0.0 || !w.is_finite() {
            return Err(EntropyError::InvalidProbability { index: i, value: w });
        }
        adjacency[a * n_nodes + b] += w;
        adjacency[b * n_nodes + a] += w;
    }

    let mut transition = vec![0.0_f64; n_nodes * n_nodes];
    let mut row_entropy_nats = vec![0.0_f64; n_nodes];
    for i in 0..n_nodes {
        let row_sum: f64 = adjacency[i * n_nodes..(i + 1) * n_nodes].iter().sum();
        if row_sum > 0.0 {
            for j in 0..n_nodes {
                transition[i * n_nodes + j] = adjacency[i * n_nodes + j] / row_sum;
            }
        } else {
            transition[i * n_nodes + i] = 1.0; // 🕸️ isolated node: absorbing self-loop
        }
        row_entropy_nats[i] = -transition[i * n_nodes..(i + 1) * n_nodes].iter().map(|&p| x_ln_x(p)).sum::<f64>();
    }

    let mut pi = vec![1.0 / n_nodes as f64; n_nodes];
    let mut converged = false;
    for _ in 0..10_000 {
        let mut next = vec![0.0_f64; n_nodes];
        for i in 0..n_nodes {
            if pi[i] <= 0.0 {
                continue;
            }
            for j in 0..n_nodes {
                next[j] += pi[i] * transition[i * n_nodes + j];
            }
        }
        let delta: f64 = pi.iter().zip(next.iter()).map(|(&a, &b)| (a - b).abs()).sum();
        pi = next;
        if delta < 1e-12 {
            converged = true;
            break;
        }
    }
    if !converged {
        return Err(EntropyError::NotConverged { what: "random walk stationary distribution", iterations: 10_000 });
    }

    let nats = pi.iter().zip(row_entropy_nats.iter()).map(|(&p, &h)| p * h).sum::<f64>();

    Ok(Estimate {
        value: base.from_nats(nats),
        base,
        method: "random_walk_entropy_rate",
        n: n_nodes,
        n_effective: n_nodes as f64,
        std_error: None,
        ci: None::<ConfidenceInterval>,
        warnings: Vec::new(),
        diagnostics: vec![("edges", edges.len() as f64)],
    })
}
// #endregion 🔖RandomWalk

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degree_entropy_of_regular_graph_is_zero() {
        // 🔐 a 4-cycle: every node has degree 2.
        let edges = [(0, 1), (1, 2), (2, 3), (3, 0)];
        let est = degree_distribution_entropy(&edges, 4, false, LogBase::Bits).unwrap();
        assert!(est.value.abs() < 1e-9);
    }

    #[test]
    fn degree_entropy_rejects_out_of_range_endpoint() {
        let edges = [(0, 5)];
        assert!(matches!(
            degree_distribution_entropy(&edges, 3, false, LogBase::Bits),
            Err(EntropyError::ShapeMismatch { .. })
        ));
    }

    #[test]
    fn random_walk_entropy_rate_of_complete_graph_matches_uniform_row_entropy() {
        // 🔐 K4: every node connects to every other node; each row is uniform over 3 neighbors.
        let edges = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
        let est = random_walk_entropy_rate(&edges, 4, None, LogBase::Bits).unwrap();
        let expected = 3.0_f64.log2();
        assert!((est.value - expected).abs() < 1e-6, "got {}", est.value);
    }

    #[test]
    fn random_walk_entropy_rate_of_cycle_matches_binary_entropy() {
        // 🔐 4-cycle: every row is [0.5, 0.5] over its two neighbors -> 1 bit.
        let edges = [(0, 1), (1, 2), (2, 3), (3, 0)];
        let est = random_walk_entropy_rate(&edges, 4, None, LogBase::Bits).unwrap();
        assert!((est.value - 1.0).abs() < 1e-6, "got {}", est.value);
    }

    #[test]
    fn random_walk_handles_isolated_node() {
        let edges = [(0, 1)];
        let est = random_walk_entropy_rate(&edges, 3, None, LogBase::Bits).unwrap();
        assert!(est.value.is_finite());
    }

    #[test]
    fn random_walk_rejects_negative_weight() {
        let edges = [(0, 1)];
        assert!(random_walk_entropy_rate(&edges, 2, Some(&[-1.0]), LogBase::Bits).is_err());
    }
}
// #endregion 🔖Tests
