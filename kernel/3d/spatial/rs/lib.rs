//! 🌳 Generic bounding-volume-hierarchy spatial index over AABB-bounded items.
//!
//! Kernel-agnostic: works with any leaf payload (`kernel_3d_brepkit` instantiates
//! `Bvh<FaceId>`/`Bvh<EdgeId>` internally) using only [`kernel_3d_engine`]'s `Vec3`/`Aabb`
//! types, so this crate never depends on brepkit and stays reusable by other 3D kernels.

use kernel_3d_engine::{Aabb, Vec3};

// #region 🔖AabbHelpers
fn aabb_union(a: &Aabb, b: &Aabb) -> Aabb {
    Aabb {
        min: [a.min[0].min(b.min[0]), a.min[1].min(b.min[1]), a.min[2].min(b.min[2])],
        max: [a.max[0].max(b.max[0]), a.max[1].max(b.max[1]), a.max[2].max(b.max[2])],
    }
}

fn aabb_center(a: &Aabb) -> Vec3 {
    [(a.min[0] + a.max[0]) * 0.5, (a.min[1] + a.max[1]) * 0.5, (a.min[2] + a.max[2]) * 0.5]
}

fn aabb_longest_axis(a: &Aabb) -> usize {
    let extents = [a.max[0] - a.min[0], a.max[1] - a.min[1], a.max[2] - a.min[2]];
    if extents[0] >= extents[1] && extents[0] >= extents[2] {
        0
    } else if extents[1] >= extents[2] {
        1
    } else {
        2
    }
}

/// 📏 Squared distance from `point` to the closest point on `aabb` (0 if inside).
fn aabb_point_distance_sq(aabb: &Aabb, point: Vec3) -> f64 {
    let mut d = 0.0;
    for axis in 0..3 {
        let v = point[axis];
        if v < aabb.min[axis] {
            d += (aabb.min[axis] - v).powi(2);
        } else if v > aabb.max[axis] {
            d += (v - aabb.max[axis]).powi(2);
        }
    }
    d
}

fn aabb_overlaps(a: &Aabb, b: &Aabb) -> bool {
    a.min[0] <= b.max[0] && a.max[0] >= b.min[0] && a.min[1] <= b.max[1] && a.max[1] >= b.min[1] && a.min[2] <= b.max[2] && a.max[2] >= b.min[2]
}

/// 🎯 Ray-AABB slab intersection test (`origin + t * dir`, `t >= 0`).
fn aabb_ray_hits(aabb: &Aabb, origin: Vec3, dir: Vec3) -> bool {
    let mut tmin = f64::NEG_INFINITY;
    let mut tmax = f64::INFINITY;
    for axis in 0..3 {
        if dir[axis].abs() < 1e-12 {
            if origin[axis] < aabb.min[axis] || origin[axis] > aabb.max[axis] {
                return false;
            }
            continue;
        }
        let inv = 1.0 / dir[axis];
        let mut t0 = (aabb.min[axis] - origin[axis]) * inv;
        let mut t1 = (aabb.max[axis] - origin[axis]) * inv;
        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }
        tmin = tmin.max(t0);
        tmax = tmax.min(t1);
        if tmin > tmax {
            return false;
        }
    }
    tmax >= 0.0
}
// #endregion 🔖AabbHelpers

// #region 🔖Bvh
enum Node<T> {
    Leaf { aabb: Aabb, item: T },
    Branch { aabb: Aabb, left: Box<Node<T>>, right: Box<Node<T>> },
}

impl<T> Node<T> {
    fn aabb(&self) -> &Aabb {
        match self {
            Node::Leaf { aabb, .. } => aabb,
            Node::Branch { aabb, .. } => aabb,
        }
    }
}

/// 🌳 Bounding-volume hierarchy over `(Aabb, T)` items, built once via median split.
pub struct Bvh<T> {
    root: Option<Node<T>>,
}

impl<T> Bvh<T> {
    /// 🏗️ Builds a BVH from AABB-bounded items via recursive median split on the longest axis.
    pub fn build(items: Vec<(Aabb, T)>) -> Self {
        Self { root: Self::build_node(items) }
    }

    fn build_node(mut items: Vec<(Aabb, T)>) -> Option<Node<T>> {
        if items.is_empty() {
            return None;
        }
        if items.len() == 1 {
            let (aabb, item) = items.into_iter().next().expect("checked len == 1");
            return Some(Node::Leaf { aabb, item });
        }
        let bounds = items.iter().map(|(aabb, _)| aabb.clone()).reduce(|a, b| aabb_union(&a, &b)).expect("checked non-empty");
        let axis = aabb_longest_axis(&bounds);
        items.sort_by(|(a, _), (b, _)| aabb_center(a)[axis].partial_cmp(&aabb_center(b)[axis]).unwrap_or(std::cmp::Ordering::Equal));
        let mid = items.len() / 2;
        let right_items = items.split_off(mid);
        let left = Self::build_node(items).expect("checked non-empty left partition");
        let right = Self::build_node(right_items).expect("checked non-empty right partition");
        Some(Node::Branch { aabb: bounds, left: Box::new(left), right: Box::new(right) })
    }

    /// 🔍 Returns the item whose leaf AABB is nearest to `point` (by AABB distance, not exact
    /// item-surface distance) — callers refine among close candidates via the exact kernel
    /// query; this only narrows the candidate set.
    pub fn query_point_nearest(&self, point: Vec3) -> Option<&T> {
        let mut best: Option<(&T, f64)> = None;
        Self::visit_nearest(self.root.as_ref(), point, &mut best);
        best.map(|(item, _)| item)
    }

    fn visit_nearest<'a>(node: Option<&'a Node<T>>, point: Vec3, best: &mut Option<(&'a T, f64)>) {
        let Some(node) = node else { return };
        let bound_dist = aabb_point_distance_sq(node.aabb(), point);
        if let Some((_, best_dist)) = best {
            if bound_dist > *best_dist {
                return;
            }
        }
        match node {
            Node::Leaf { item, .. } => {
                let is_better = match best {
                    None => true,
                    Some((_, best_dist)) => bound_dist < *best_dist,
                };
                if is_better {
                    *best = Some((item, bound_dist));
                }
            }
            Node::Branch { left, right, .. } => {
                Self::visit_nearest(Some(left), point, best);
                Self::visit_nearest(Some(right), point, best);
            }
        }
    }

    /// 🎯 Returns all items whose leaf AABB is crossed by the ray `origin + t * dir` (`t >= 0`).
    pub fn query_ray(&self, origin: Vec3, dir: Vec3) -> Vec<&T> {
        let mut hits = Vec::new();
        Self::visit_ray(self.root.as_ref(), origin, dir, &mut hits);
        hits
    }

    fn visit_ray<'a>(node: Option<&'a Node<T>>, origin: Vec3, dir: Vec3, hits: &mut Vec<&'a T>) {
        let Some(node) = node else { return };
        if !aabb_ray_hits(node.aabb(), origin, dir) {
            return;
        }
        match node {
            Node::Leaf { item, .. } => hits.push(item),
            Node::Branch { left, right, .. } => {
                Self::visit_ray(Some(left), origin, dir, hits);
                Self::visit_ray(Some(right), origin, dir, hits);
            }
        }
    }

    /// 📦 Returns all items whose leaf AABB overlaps `query`.
    pub fn query_aabb_overlap(&self, query: &Aabb) -> Vec<&T> {
        let mut hits = Vec::new();
        Self::visit_overlap(self.root.as_ref(), query, &mut hits);
        hits
    }

    fn visit_overlap<'a>(node: Option<&'a Node<T>>, query: &Aabb, hits: &mut Vec<&'a T>) {
        let Some(node) = node else { return };
        if !aabb_overlaps(node.aabb(), query) {
            return;
        }
        match node {
            Node::Leaf { item, .. } => hits.push(item),
            Node::Branch { left, right, .. } => {
                Self::visit_overlap(Some(left), query, hits);
                Self::visit_overlap(Some(right), query, hits);
            }
        }
    }
}
// #endregion 🔖Bvh

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn aabb(min: Vec3, max: Vec3) -> Aabb {
        Aabb { min, max }
    }

    #[test]
    fn empty_bvh_returns_no_matches() {
        let bvh: Bvh<u32> = Bvh::build(Vec::new());
        assert_eq!(bvh.query_point_nearest([0.0, 0.0, 0.0]), None);
        assert!(bvh.query_ray([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]).is_empty());
        assert!(bvh.query_aabb_overlap(&aabb([0.0, 0.0, 0.0], [1.0, 1.0, 1.0])).is_empty());
    }

    #[test]
    fn nearest_point_finds_closest_leaf() {
        let items = vec![
            (aabb([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]), "near"),
            (aabb([10.0, 10.0, 10.0], [11.0, 11.0, 11.0]), "far"),
        ];
        let bvh = Bvh::build(items);
        assert_eq!(bvh.query_point_nearest([0.5, 0.5, 0.5]), Some(&"near"));
        assert_eq!(bvh.query_point_nearest([10.5, 10.5, 10.5]), Some(&"far"));
    }

    #[test]
    fn ray_hits_only_crossed_leaves() {
        let items = vec![
            (aabb([0.0, -1.0, -1.0], [1.0, 1.0, 1.0]), "hit"),
            (aabb([0.0, 10.0, 10.0], [1.0, 11.0, 11.0]), "miss"),
        ];
        let bvh = Bvh::build(items);
        let hits = bvh.query_ray([-5.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        assert_eq!(hits, vec![&"hit"]);
    }

    #[test]
    fn aabb_overlap_finds_intersecting_leaves() {
        let items = vec![
            (aabb([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]), "overlap"),
            (aabb([5.0, 5.0, 5.0], [6.0, 6.0, 6.0]), "disjoint"),
        ];
        let bvh = Bvh::build(items);
        let mut hits = bvh.query_aabb_overlap(&aabb([0.5, 0.5, 0.5], [2.0, 2.0, 2.0]));
        hits.sort();
        assert_eq!(hits, vec![&"overlap"]);
    }

    #[test]
    fn many_leaves_build_and_query_correctly() {
        let items: Vec<(Aabb, usize)> = (0..200).map(|i| (aabb([i as f64, 0.0, 0.0], [i as f64 + 0.5, 0.5, 0.5]), i)).collect();
        let bvh = Bvh::build(items);
        assert_eq!(bvh.query_point_nearest([100.2, 0.2, 0.2]), Some(&100));
    }
}
// #endregion 🔖Tests
