//! 🌲 A minimal k-d tree over row-major `f64` point sets: k-nearest-neighbor queries and
//! radius/range counts, the shared infrastructure behind Kozachenko-Leonenko differential
//! entropy, KSG mutual information, and kNN transfer entropy. Includes a brute-force `O(n)`
//! reference implementation used as the correctness oracle in tests.

use crate::{EntropyError, Metric};

// #region 🔖Distance
fn distance(a: &[f64], b: &[f64], metric: Metric) -> f64 {
    match metric {
        Metric::Euclidean => a.iter().zip(b).map(|(&x, &y)| (x - y) * (x - y)).sum::<f64>().sqrt(),
        Metric::Manhattan => a.iter().zip(b).map(|(&x, &y)| (x - y).abs()).sum(),
        Metric::Chebyshev => a.iter().zip(b).map(|(&x, &y)| (x - y).abs()).fold(0.0_f64, f64::max),
    }
}
// #endregion 🔖Distance

// #region 🔖KdTree
struct Node {
    idx: usize,
    split_dim: usize,
    left: Option<usize>,
    right: Option<usize>,
}

/// 🌲 A k-d tree over `n` points in `dim` dimensions (row-major, `points.len() == n * dim`).
pub struct KdTree {
    points: Vec<f64>,
    dim: usize,
    n: usize,
    nodes: Vec<Node>,
    root: Option<usize>,
}

fn build_recursive(points: &[f64], dim: usize, indices: &mut [usize], depth: usize, nodes: &mut Vec<Node>) -> Option<usize> {
    if indices.is_empty() {
        return None;
    }
    let split_dim = depth % dim;
    indices.sort_by(|&a, &b| points[a * dim + split_dim].total_cmp(&points[b * dim + split_dim]));
    let mid = indices.len() / 2;
    let idx = indices[mid];
    let node_pos = nodes.len();
    nodes.push(Node { idx, split_dim, left: None, right: None });
    let left = build_recursive(points, dim, &mut indices[..mid], depth + 1, nodes);
    let right = build_recursive(points, dim, &mut indices[mid + 1..], depth + 1, nodes);
    nodes[node_pos].left = left;
    nodes[node_pos].right = right;
    Some(node_pos)
}

fn insert_bounded(best: &mut Vec<(f64, usize)>, item: (f64, usize), k: usize) {
    if best.len() < k {
        best.push(item);
        return;
    }
    let mut max_i = 0;
    for i in 1..best.len() {
        if best[i].0 > best[max_i].0 {
            max_i = i;
        }
    }
    if item.0 < best[max_i].0 {
        best[max_i] = item;
    }
}

fn worst_distance(best: &[(f64, usize)], k: usize) -> f64 {
    if best.len() < k {
        f64::INFINITY
    } else {
        best.iter().map(|x| x.0).fold(0.0_f64, f64::max)
    }
}

impl KdTree {
    /// 🌲 Builds a tree over `points` (row-major `n x dim`). `O(n (log n)^2)` due to per-level
    /// sort-based median selection.
    pub fn build(points: &[f64], dim: usize) -> Result<Self, EntropyError> {
        if dim == 0 {
            return Err(EntropyError::InvalidConfig { field: "dim", reason: "must be at least 1" });
        }
        if !points.len().is_multiple_of(dim) {
            return Err(EntropyError::ShapeMismatch { what: "points", expected: dim, actual: points.len() % dim });
        }
        let n = points.len() / dim;
        if n == 0 {
            return Err(EntropyError::EmptyInput { what: "points" });
        }
        let mut indices: Vec<usize> = (0..n).collect();
        let mut nodes = Vec::with_capacity(n);
        let root = build_recursive(points, dim, &mut indices, 0, &mut nodes);
        Ok(Self { points: points.to_vec(), dim, n, nodes, root })
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn point(&self, idx: usize) -> &[f64] {
        &self.points[idx * self.dim..(idx + 1) * self.dim]
    }

    /// 🌲 The `k` nearest neighbors to `query` by `metric`, excluding stored index `exclude` if
    /// given (used for leave-one-out self-queries). Returns `(index, distance)` sorted ascending.
    pub fn k_nearest(&self, query: &[f64], k: usize, metric: Metric, exclude: Option<usize>) -> Vec<(usize, f64)> {
        let mut best: Vec<(f64, usize)> = Vec::with_capacity(k);
        self.search_knn(self.root, query, k, metric, exclude, &mut best);
        best.sort_by(|a, b| a.0.total_cmp(&b.0));
        best.into_iter().map(|(d, i)| (i, d)).collect()
    }

    fn search_knn(&self, node: Option<usize>, query: &[f64], k: usize, metric: Metric, exclude: Option<usize>, best: &mut Vec<(f64, usize)>) {
        let Some(pos) = node else { return };
        let node_ref = &self.nodes[pos];
        let point_idx = node_ref.idx;
        if Some(point_idx) != exclude {
            let d = distance(self.point(point_idx), query, metric);
            insert_bounded(best, (d, point_idx), k);
        }
        let split_dim = node_ref.split_dim;
        let diff = query[split_dim] - self.point(point_idx)[split_dim];
        let (near, far) = if diff < 0.0 { (node_ref.left, node_ref.right) } else { (node_ref.right, node_ref.left) };
        self.search_knn(near, query, k, metric, exclude, best);
        if diff.abs() < worst_distance(best, k) {
            self.search_knn(far, query, k, metric, exclude, best);
        }
    }

    /// 🌲 Strict count of points with `distance(query, point) < radius` (the KSG/Kozachenko-
    /// Leonenko convention: a closed radius equal to the k-th neighbor distance always excludes
    /// that neighbor itself, avoiding a `log(0)` in the digamma-based estimators).
    pub fn count_within_radius(&self, query: &[f64], radius: f64, metric: Metric, exclude: Option<usize>) -> usize {
        let mut count = 0usize;
        self.count_recursive(self.root, query, radius, metric, exclude, &mut count);
        count
    }

    fn count_recursive(&self, node: Option<usize>, query: &[f64], radius: f64, metric: Metric, exclude: Option<usize>, count: &mut usize) {
        let Some(pos) = node else { return };
        let node_ref = &self.nodes[pos];
        let point_idx = node_ref.idx;
        if Some(point_idx) != exclude && distance(self.point(point_idx), query, metric) < radius {
            *count += 1;
        }
        let split_dim = node_ref.split_dim;
        let diff = query[split_dim] - self.point(point_idx)[split_dim];
        self.count_recursive(node_ref.left, query, radius, metric, exclude, count);
        self.count_recursive(node_ref.right, query, radius, metric, exclude, count);
        let _ = diff; // 🌲 both children can contain in-radius points at any split; no pruning
                       // is safe here beyond what the recursion above already limits to O(n) worst case.
    }
}
// #endregion 🔖KdTree

// #region 🔖BruteForce
/// 🌲 `O(n)` reference k-nearest-neighbor search, used as the correctness oracle for [`KdTree`]
/// in tests and as a drop-in for callers with `n` small enough that tree overhead does not pay
/// off.
pub fn brute_force_knn(points: &[f64], dim: usize, query: &[f64], k: usize, metric: Metric, exclude: Option<usize>) -> Result<Vec<(usize, f64)>, EntropyError> {
    if dim == 0 || !points.len().is_multiple_of(dim) {
        return Err(EntropyError::InvalidConfig { field: "dim", reason: "must evenly divide points length" });
    }
    let n = points.len() / dim;
    let mut all: Vec<(usize, f64)> = (0..n)
        .filter(|&i| Some(i) != exclude)
        .map(|i| (i, distance(&points[i * dim..(i + 1) * dim], query, metric)))
        .collect();
    all.sort_by(|a, b| a.1.total_cmp(&b.1));
    all.truncate(k);
    Ok(all)
}
// #endregion 🔖BruteForce

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn random_points(n: usize, dim: usize, seed: u64) -> Vec<f64> {
        let mut rng = crate::numeric::Xorshift64::new(seed);
        (0..n * dim).map(|_| rng.next_f64() * 10.0 - 5.0).collect()
    }

    #[test]
    fn build_rejects_empty_and_bad_shape() {
        assert!(KdTree::build(&[], 2).is_err());
        assert!(KdTree::build(&[1.0, 2.0, 3.0], 2).is_err());
        assert!(KdTree::build(&[1.0, 2.0], 0).is_err());
    }

    #[test]
    fn kd_tree_matches_brute_force_knn_euclidean() {
        let points = random_points(200, 3, 11);
        let tree = KdTree::build(&points, 3).unwrap();
        for q in 0..20 {
            let query = &points[q * 3..(q + 1) * 3];
            let tree_result = tree.k_nearest(query, 5, Metric::Euclidean, Some(q));
            let brute_result = brute_force_knn(&points, 3, query, 5, Metric::Euclidean, Some(q)).unwrap();
            let tree_dists: Vec<f64> = tree_result.iter().map(|x| x.1).collect();
            let brute_dists: Vec<f64> = brute_result.iter().map(|x| x.1).collect();
            for (a, b) in tree_dists.iter().zip(brute_dists.iter()) {
                assert!((a - b).abs() < 1e-9, "query {q}: tree={tree_dists:?} brute={brute_dists:?}");
            }
        }
    }

    #[test]
    fn kd_tree_matches_brute_force_for_chebyshev_and_manhattan() {
        let points = random_points(150, 2, 22);
        let tree = KdTree::build(&points, 2).unwrap();
        for metric in [Metric::Chebyshev, Metric::Manhattan] {
            for q in 0..10 {
                let query = &points[q * 2..(q + 1) * 2];
                let tree_result = tree.k_nearest(query, 4, metric, Some(q));
                let brute_result = brute_force_knn(&points, 2, query, 4, metric, Some(q)).unwrap();
                let tree_dists: Vec<f64> = tree_result.iter().map(|x| x.1).collect();
                let brute_dists: Vec<f64> = brute_result.iter().map(|x| x.1).collect();
                assert_eq!(tree_dists.len(), brute_dists.len());
                for (a, b) in tree_dists.iter().zip(brute_dists.iter()) {
                    assert!((a - b).abs() < 1e-9);
                }
            }
        }
    }

    #[test]
    fn radius_count_matches_brute_force() {
        let points = random_points(100, 2, 33);
        let tree = KdTree::build(&points, 2).unwrap();
        for q in 0..10 {
            let query = &points[q * 2..(q + 1) * 2];
            let radius = 1.5;
            let tree_count = tree.count_within_radius(query, radius, Metric::Chebyshev, Some(q));
            let brute_count = (0..100)
                .filter(|&i| i != q)
                .filter(|&i| distance(&points[i * 2..(i + 1) * 2], query, Metric::Chebyshev) < radius)
                .count();
            assert_eq!(tree_count, brute_count, "query {q}");
        }
    }

    #[test]
    fn k_nearest_excludes_self_when_requested() {
        let points = vec![0.0, 0.0, 1.0, 0.0, 2.0, 0.0];
        let tree = KdTree::build(&points, 2).unwrap();
        let result = tree.k_nearest(&[0.0, 0.0], 2, Metric::Euclidean, Some(0));
        assert!(!result.iter().any(|&(i, _)| i == 0));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn k_nearest_on_single_point_tree() {
        let points = vec![5.0, 5.0];
        let tree = KdTree::build(&points, 2).unwrap();
        let result = tree.k_nearest(&[0.0, 0.0], 1, Metric::Euclidean, None);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, 0);
    }

    mod quick {
        use super::*;

        #[test]
        fn kd_tree_matches_brute_force_on_larger_random_set() {
            let points = random_points(1000, 4, 999);
            let tree = KdTree::build(&points, 4).unwrap();
            for q in (0..1000).step_by(97) {
                let query = &points[q * 4..(q + 1) * 4];
                let tree_result = tree.k_nearest(query, 8, Metric::Euclidean, Some(q));
                let brute_result = brute_force_knn(&points, 4, query, 8, Metric::Euclidean, Some(q)).unwrap();
                for (a, b) in tree_result.iter().zip(brute_result.iter()) {
                    assert!((a.1 - b.1).abs() < 1e-9);
                }
            }
        }
    }
}
// #endregion 🔖Tests
