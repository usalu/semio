//! 🔢 Graph topology markers and geometry-free layout math.

// #region 🔖Ids
/// 🧩 Stable node identifier.
pub type NodeId = u64;
/// 🪝 Stable handle identifier.
pub type HandleId = u64;
/// 🪢 Stable edge identifier.
pub type EdgeId = u64;
// #endregion 🔖Ids

// #region 🔖Edge
/// 🪢 Edge with typed endpoints (node id or handle id).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edge<E> {
    pub id: EdgeId,
    pub source: E,
    pub target: E,
}

impl<E: Copy + Ord> Edge<E> {
    /// 📐 Normalize endpoints for undirected storage.
    pub fn normalize_undirected(source: E, target: E) -> (E, E) {
        if source <= target {
            (source, target)
        } else {
            (target, source)
        }
    }
}
// #endregion 🔖Edge

// #region 🔖Directedness
/// ↔️ Compile-time directed vs undirected graph axis.
pub trait Directedness {
    const DIRECTED: bool;
}

/// ➡️ Directed edges keep source→target order.
#[derive(Clone, Copy, Debug, Default)]
pub struct Directed;

impl Directedness for Directed {
    const DIRECTED: bool = true;
}

/// ↔️ Undirected edges store ordered endpoint pair.
#[derive(Clone, Copy, Debug, Default)]
pub struct Undirected;

impl Directedness for Undirected {
    const DIRECTED: bool = false;
}

/// 📐 Apply directedness when storing edge endpoints.
#[inline]
pub fn orient_endpoints<E: Copy + Ord, D: Directedness>(source: E, target: E) -> (E, E) {
    if D::DIRECTED {
        (source, target)
    } else {
        Edge::<E>::normalize_undirected(source, target)
    }
}
// #endregion 🔖Directedness

// #region 🔖PortModel
/// 🔌 Compile-time normal (node) vs ported (handle) graph axis.
pub trait PortModel {
    type Endpoint: Copy + Ord + std::fmt::Debug;
    const HAS_PORTS: bool;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64;
    fn try_handle_endpoint(handle_id: HandleId) -> Option<Self::Endpoint>;
    fn endpoint_as_handle(endpoint: Self::Endpoint) -> Option<HandleId>;
}

/// 🟠 Node-to-node edges without handles.
#[derive(Clone, Copy, Debug, Default)]
pub struct Normal;

impl PortModel for Normal {
    type Endpoint = NodeId;
    const HAS_PORTS: bool = false;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64 {
        endpoint
    }
    fn try_handle_endpoint(_: HandleId) -> Option<Self::Endpoint> {
        None
    }
    fn endpoint_as_handle(_: Self::Endpoint) -> Option<HandleId> {
        None
    }
}

/// 🪝 Handle-to-handle edges on nodes.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ported;

impl PortModel for Ported {
    type Endpoint = HandleId;
    const HAS_PORTS: bool = true;
    fn endpoint_as_u64(endpoint: Self::Endpoint) -> u64 {
        endpoint
    }
    fn try_handle_endpoint(handle_id: HandleId) -> Option<Self::Endpoint> {
        Some(handle_id)
    }
    fn endpoint_as_handle(endpoint: Self::Endpoint) -> Option<HandleId> {
        Some(endpoint)
    }
}
// #endregion 🔖PortModel

// #region 🕸️ForceLayout
pub mod force_layout {
    use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

    // #region 🔖Vec2
    #[derive(Clone, Copy, Debug)]
    pub struct Vec2 {
        pub x: f64,
        pub y: f64,
    }

    impl Vec2 {
        pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
        #[inline]
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }
        #[inline]
        pub fn norm(self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }
    }

    impl Add for Vec2 {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Self::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    impl AddAssign for Vec2 {
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    impl Sub for Vec2 {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            Self::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    impl SubAssign for Vec2 {
        fn sub_assign(&mut self, rhs: Self) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }
    }

    impl Mul<f64> for Vec2 {
        type Output = Self;
        fn mul(self, s: f64) -> Self {
            Self::new(self.x * s, self.y * s)
        }
    }

    impl Mul<Vec2> for f64 {
        type Output = Vec2;
        fn mul(self, v: Vec2) -> Vec2 {
            v * self
        }
    }

    impl MulAssign<f64> for Vec2 {
        fn mul_assign(&mut self, s: f64) {
            self.x *= s;
            self.y *= s;
        }
    }

    impl Div<f64> for Vec2 {
        type Output = Self;
        fn div(self, s: f64) -> Self {
            Self::new(self.x / s, self.y / s)
        }
    }
    // #endregion 🔖Vec2

    /// ⚙️ Force-directed layout parameters (geometry-free).
    #[derive(Clone, Debug)]
    pub struct ForceLayoutOptions {
        pub iterations: u32,
        pub ideal_edge_length: f64,
        pub repulsion_strength: f64,
        pub spring_strength: f64,
        pub gravity: f64,
        pub center_x: f64,
        pub center_y: f64,
        pub time_step: f64,
        pub velocity_damping: f64,
        pub max_speed: f64,
        pub random_seed: u64,
        pub barnes_hut_theta: f64,
        pub pairwise_repulsion_max_bodies: u32,
    }

    impl Default for ForceLayoutOptions {
        fn default() -> Self {
            Self {
                iterations: 420,
                ideal_edge_length: 140.0,
                repulsion_strength: 6500.0,
                spring_strength: 0.028,
                gravity: 0.018,
                center_x: 0.0,
                center_y: 0.0,
                time_step: 0.85,
                velocity_damping: 0.88,
                max_speed: 48.0,
                random_seed: 0x5eedfaced0,
                barnes_hut_theta: 0.78,
                pairwise_repulsion_max_bodies: 56,
            }
        }
    }

    fn split_mix64(mut z: u64) -> u64 {
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    fn rand_unit_interval(seed: &mut u64) -> f64 {
        *seed = split_mix64(*seed);
        (*seed as f64) / (u64::MAX as f64)
    }

    #[inline]
    fn pairwise_repulsion_on_i_from_j(i: usize, j: usize, positions: &[Vec2], radii: &[f64], cool: f64, k_rep: f64) -> Vec2 {
        let delta = positions[j] - positions[i];
        let dist = delta.norm().max(1e-4);
        let rep = k_rep * cool * (radii[i] * radii[j]).max(1.0) / (dist * dist);
        (delta / dist) * (-rep)
    }

    fn add_pairwise_repulsion(forces: &mut [Vec2], positions: &[Vec2], radii: &[f64], n: usize, cool: f64, k_rep: f64) {
        for i in 0..n {
            for j in (i + 1)..n {
                let f = pairwise_repulsion_on_i_from_j(i, j, positions, radii, cool, k_rep);
                forces[i] += f;
                forces[j] -= f;
            }
        }
    }

    /// 🕸️ Run force-directed layout on abstract 2d positions.
    pub fn run_force_layout(positions: &mut [Vec2], radii: &[f64], edge_pairs: &[(usize, usize)], pin: &[Option<Vec2>], opts: &ForceLayoutOptions) {
        let n = positions.len();
        if n == 0 {
            return;
        }
        let mut velocities = vec![Vec2::ZERO; n];
        let gx = opts.center_x;
        let gy = opts.center_y;
        let k = opts.ideal_edge_length.max(1e-6);
        let iters = opts.iterations.max(1);
        for iter in 0..iters {
            let cool = (1.0 - iter as f64 / iters as f64).max(0.08);
            let mut forces = vec![Vec2::ZERO; n];
            let _theta = opts.barnes_hut_theta;
            let _pair_cap = opts.pairwise_repulsion_max_bodies;
            add_pairwise_repulsion(&mut forces, positions, radii, n, cool, opts.repulsion_strength);
            for &(i, j) in edge_pairs {
                let delta = positions[j] - positions[i];
                let dist = delta.norm().max(1e-4);
                let dir = delta / dist;
                let displacement = dist - k;
                let f = dir * (opts.spring_strength * cool * displacement);
                forces[i] += f;
                forces[j] -= f;
            }
            if opts.gravity > 0.0 {
                let g = opts.gravity * cool;
                for i in 0..n {
                    let to_c = Vec2::new(gx - positions[i].x, gy - positions[i].y);
                    forces[i] += to_c * g;
                }
            }
            for i in 0..n {
                if pin[i].is_some() {
                    forces[i] = Vec2::ZERO;
                }
            }
            let dt = opts.time_step * cool.sqrt();
            for i in 0..n {
                let mut v = (velocities[i] + forces[i] * dt) * opts.velocity_damping;
                let spd = v.norm();
                if spd > opts.max_speed {
                    v *= opts.max_speed / spd;
                }
                velocities[i] = v;
                if pin[i].is_none() {
                    positions[i] += v * dt;
                } else if let Some(p) = pin[i] {
                    positions[i] = p;
                    velocities[i] = Vec2::ZERO;
                }
            }
        }
    }

    /// 🎲 Scatter missing positions around anchor with deterministic jitter.
    pub fn seed_positions(positions: &mut [Vec2], pin: &[Option<Vec2>], anchor: Vec2, seed: u64) {
        let mut rng = seed;
        for i in 0..positions.len() {
            if pin[i].is_some() {
                continue;
            }
            if positions[i].norm() < 1e-9 {
                let t = i as f64;
                let ang = t * 2.39996322972865332;
                let r = 10.0 + t.sqrt() * 22.0;
                let jx = (rand_unit_interval(&mut rng) - 0.5) * 6.0;
                let jy = (rand_unit_interval(&mut rng) - 0.5) * 6.0;
                positions[i] = anchor + Vec2::new(r * ang.cos() + jx, r * ang.sin() + jy);
            }
        }
    }
}
// #endregion 🕸️ForceLayout

// #region 🌳TreeLayout
pub mod tree_layout {
    use std::collections::{HashMap, HashSet};

    /// 🌲 Buchheim tidy-tree on string-labeled directed edges.
    pub fn buchheim_positions(roots: &[String], directed: &[(String, String)], depth: &HashMap<String, i32>) -> HashMap<String, (f64, f64)> {
        let roots_set: HashSet<String> = roots.iter().cloned().collect();
        let mut incoming: HashMap<String, Vec<String>> = HashMap::new();
        for (u, v) in directed {
            incoming.entry(v.clone()).or_default().push(u.clone());
        }
        for v in incoming.values_mut() {
            v.sort();
            v.dedup();
        }
        let mut chosen_parent: HashMap<String, String> = HashMap::new();
        let mut all_ids: HashSet<String> = HashSet::new();
        for (u, v) in directed {
            all_ids.insert(u.clone());
            all_ids.insert(v.clone());
        }
        for r in roots {
            all_ids.insert(r.clone());
        }
        for id in &all_ids {
            if roots_set.contains(id) {
                continue;
            }
            let ps = incoming.get(id).cloned().unwrap_or_default();
            if ps.is_empty() {
                continue;
            }
            let best = ps
                .iter()
                .min_by_key(|p| {
                    let dp = depth.get(*p).copied().unwrap_or(0);
                    (dp, (*p).clone())
                })
                .expect("non-empty ps")
                .clone();
            chosen_parent.insert(id.clone(), best);
        }
        let mut ordered_ids: Vec<String> = all_ids.into_iter().collect();
        ordered_ids.sort();
        if ordered_ids.is_empty() {
            return HashMap::new();
        }
        let id_to_idx: HashMap<String, usize> = ordered_ids.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect();
        let super_idx = ordered_ids.len();
        let mut nodes: Vec<BuchheimNode> = ordered_ids
            .iter()
            .map(|id| BuchheimNode {
                ancestor: 0,
                change: 0.0,
                children: vec![],
                id: id.clone(),
                mod_: 0.0,
                number: 0,
                parent: None,
                shift: 0.0,
                synthetic: false,
                thread: None,
                x: -1.0,
                y: 0.0,
            })
            .collect();
        nodes.push(BuchheimNode {
            ancestor: super_idx,
            change: 0.0,
            children: vec![],
            id: "__tree_super__".into(),
            mod_: 0.0,
            number: 0,
            parent: None,
            shift: 0.0,
            synthetic: true,
            thread: None,
            x: -1.0,
            y: 0.0,
        });
        for (i, oid) in ordered_ids.iter().enumerate() {
            let pidx = if roots_set.contains(oid) {
                super_idx
            } else {
                match chosen_parent.get(oid) {
                    Some(p) => *id_to_idx.get(p).unwrap_or(&super_idx),
                    None => super_idx,
                }
            };
            nodes[i].parent = Some(pidx);
        }
        for p in 0..=super_idx {
            nodes[p].children.clear();
        }
        for i in 0..super_idx {
            let pi = nodes[i].parent.unwrap();
            nodes[pi].children.push(i);
        }
        for p in 0..=super_idx {
            let mut ch = nodes[p].children.clone();
            ch.sort_by_key(|&c| nodes[c].id.clone());
            nodes[p].children = ch;
        }
        for p in 0..=super_idx {
            if nodes[p].children.is_empty() {
                continue;
            }
            let ch = nodes[p].children.clone();
            for (k, &c) in ch.iter().enumerate() {
                nodes[c].number = (k + 1) as i32;
                nodes[c].ancestor = c;
            }
        }
        buchheim_first_walk(&mut nodes, super_idx, 1.0);
        let min_x = buchheim_second_walk(&mut nodes, super_idx, 0.0, 0, f64::INFINITY);
        if min_x.is_finite() && min_x < 0.0 {
            buchheim_third_walk(&mut nodes, super_idx, -min_x);
        }
        let mut out = HashMap::new();
        for (i, n) in nodes.iter().enumerate() {
            if i == super_idx || n.synthetic {
                continue;
            }
            out.insert(n.id.clone(), (n.x, n.y));
        }
        out
    }

    #[derive(Debug)]
    struct BuchheimNode {
        id: String,
        parent: Option<usize>,
        children: Vec<usize>,
        x: f64,
        y: f64,
        mod_: f64,
        thread: Option<usize>,
        ancestor: usize,
        change: f64,
        shift: f64,
        number: i32,
        synthetic: bool,
    }

    fn buchheim_left_brother(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
        let p = nodes[i].parent?;
        let ch = &nodes[p].children;
        let pos = ch.iter().position(|&c| c == i)?;
        if pos == 0 {
            return None;
        }
        Some(ch[pos - 1])
    }

    fn buchheim_leftmost_sibling(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
        let p = nodes[i].parent?;
        let ch = &nodes[p].children;
        if ch.first() == Some(&i) {
            return None;
        }
        ch.first().copied()
    }

    fn buchheim_next_right(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
        if let Some(t) = nodes[i].thread {
            return Some(t);
        }
        nodes[i].children.last().copied()
    }

    fn buchheim_next_left(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
        if let Some(t) = nodes[i].thread {
            return Some(t);
        }
        nodes[i].children.first().copied()
    }

    fn buchheim_move_subtree(nodes: &mut [BuchheimNode], wl: usize, wr: usize, shift: f64) {
        let subtrees = (nodes[wr].number - nodes[wl].number) as f64;
        if subtrees <= 0.0 {
            return;
        }
        nodes[wr].change -= shift / subtrees;
        nodes[wr].shift += shift;
        nodes[wl].change += shift / subtrees;
        nodes[wr].x += shift;
        nodes[wr].mod_ += shift;
    }

    fn buchheim_execute_shifts(nodes: &mut [BuchheimNode], v: usize) {
        let mut shift = 0.0f64;
        let mut change = 0.0f64;
        for &w in nodes[v].children.iter().rev() {
            nodes[w].x += shift;
            nodes[w].mod_ += shift;
            change += nodes[w].change;
            shift += nodes[w].shift + change;
        }
    }

    fn buchheim_apportion(nodes: &mut [BuchheimNode], v: usize, default_ancestor: usize, distance: f64) -> usize {
        let mut default_ancestor = default_ancestor;
        let w = match buchheim_left_brother(nodes, v) {
            Some(w) => w,
            None => return default_ancestor,
        };
        let mut vir = v;
        let mut vil = w;
        let mut vol = buchheim_leftmost_sibling(nodes, v).unwrap_or(v);
        let mut vor = v;
        let mut sir = nodes[v].mod_;
        let mut sor = nodes[v].mod_;
        let mut sil = nodes[vil].mod_;
        let mut sol = nodes[vol].mod_;
        loop {
            let vil_r = buchheim_next_right(nodes, vil);
            let vir_l = buchheim_next_left(nodes, vir);
            if vil_r.is_none() || vir_l.is_none() {
                break;
            }
            vil = vil_r.unwrap();
            vir = vir_l.unwrap();
            let vol_l = buchheim_next_left(nodes, vol);
            let vor_r = buchheim_next_right(nodes, vor);
            if vol_l.is_none() || vor_r.is_none() {
                break;
            }
            vol = vol_l.unwrap();
            vor = vor_r.unwrap();
            nodes[vor].ancestor = v;
            let shift = (nodes[vil].x + sil) - (nodes[vir].x + sir) + distance;
            if shift > 0.0 {
                let a = default_ancestor;
                buchheim_move_subtree(nodes, a, v, shift);
                sir += shift;
                sor += shift;
            }
            sil += nodes[vil].mod_;
            sir += nodes[vir].mod_;
            sol += nodes[vol].mod_;
            sor += nodes[vor].mod_;
        }
        default_ancestor
    }

    fn buchheim_first_walk(nodes: &mut [BuchheimNode], v: usize, distance: f64) -> usize {
        if nodes[v].children.is_empty() {
            if let Some(lb) = buchheim_left_brother(nodes, v) {
                nodes[v].x = nodes[lb].x + distance;
            } else {
                nodes[v].x = 0.0;
            }
            return v;
        }
        let mut default_ancestor = nodes[v].children[0];
        for &w in nodes[v].children.clone().iter() {
            buchheim_first_walk(nodes, w, distance);
            default_ancestor = buchheim_apportion(nodes, w, default_ancestor, distance);
        }
        buchheim_execute_shifts(nodes, v);
        let c0 = nodes[v].children[0];
        let c1 = *nodes[v].children.last().unwrap();
        let mid = (nodes[c0].x + nodes[c1].x) * 0.5;
        if let Some(w) = buchheim_left_brother(nodes, v) {
            nodes[v].x = nodes[w].x + distance;
            nodes[v].mod_ = nodes[v].x - mid;
        } else {
            nodes[v].x = mid;
        }
        v
    }

    fn buchheim_second_walk(nodes: &mut [BuchheimNode], v: usize, m: f64, depth: i32, min_x: f64) -> f64 {
        nodes[v].x += m;
        nodes[v].y = depth as f64;
        let mut min_x = min_x.min(nodes[v].x);
        for &w in nodes[v].children.clone().iter() {
            min_x = buchheim_second_walk(nodes, w, m + nodes[v].mod_, depth + 1, min_x);
        }
        min_x
    }

    fn buchheim_third_walk(nodes: &mut [BuchheimNode], v: usize, n: f64) {
        nodes[v].x += n;
        for &c in nodes[v].children.clone().iter() {
            buchheim_third_walk(nodes, c, n);
        }
    }
}
// #endregion 🌳TreeLayout

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::force_layout::{run_force_layout, ForceLayoutOptions, Vec2};
    use super::tree_layout::buchheim_positions;
    use super::{orient_endpoints, Directed, NodeId, Normal, PortModel, Ported, Undirected};

    #[test]
    fn undirected_orient_endpoints() {
        let (a, b) = orient_endpoints::<NodeId, Undirected>(3, 1);
        assert_eq!(a, 1);
        assert_eq!(b, 3);
    }

    #[test]
    fn directed_keeps_order() {
        let (a, b) = orient_endpoints::<NodeId, Directed>(3, 1);
        assert_eq!(a, 3);
        assert_eq!(b, 1);
    }

    #[test]
    fn normal_has_no_ports() {
        assert!(!Normal::HAS_PORTS);
        assert!(Ported::HAS_PORTS);
    }

    #[test]
    fn force_layout_moves_nodes() {
        let mut positions = vec![Vec2::new(0.0, 0.0), Vec2::new(100.0, 0.0)];
        let radii = vec![32.0, 32.0];
        let edges = vec![(0, 1)];
        let pin = vec![None, None];
        let opts = ForceLayoutOptions {
            iterations: 120,
            ideal_edge_length: 80.0,
            ..Default::default()
        };
        run_force_layout(&mut positions, &radii, &edges, &pin, &opts);
        let dist = (positions[1] - positions[0]).norm();
        assert!(dist.is_finite() && dist > 1.0);
        assert!((dist - 100.0).abs() > 0.01);
    }

    #[test]
    fn buchheim_tree_two_nodes() {
        let roots = vec!["a".into()];
        let directed = vec![("a".into(), "b".into())];
        let mut depth = std::collections::HashMap::new();
        depth.insert("a".into(), 0);
        depth.insert("b".into(), 1);
        let pos = buchheim_positions(&roots, &directed, &depth);
        assert!(pos.contains_key("a"));
        assert!(pos.contains_key("b"));
    }
}
// #endregion 🔖Tests
