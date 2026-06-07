//! ➡️ Directed port graph base: engine aliases, scene descriptors, layouts, board types.

pub mod scene_json;
pub mod types;

pub use infinite_cavas as cavas;
pub use mathematical_graph_port::*;
pub use scene_json::{
    board_json_visible_option, board_json_visible_or_true, fixture_edge_handle_ids_from_object, normalize_board_descriptor_hidden_to_visible, EdgeDescJson, FixtureV1Json, SceneDescriptorJson,
    WireDescJson,
};
pub use types::*;
pub use mathematical_core::{Directed, Ported};
pub use mathematical_graph::{
    area_preselect_ids, merge_ids_into_selection, merge_pick_into_selection, normalize_selection_mode, pick_merge_mode_for_modifiers, selection_contains_edge_curve,
    selection_contains_handle_point, selection_contains_node_bounds, selection_drag_enclosing, selection_drag_shape, selection_screen_overlay_points, SELECTION_CLICK_MAX_DISTANCE_PX,
    SELECTION_LASSO_MIN_POINT_DISTANCE_PX, SELECTION_MARQUEE_DRAG_THRESHOLD_PX,
};

/// ➡️ Port graph engine with directed handle endpoints.
pub type DirectedPortGraphEngine = GraphEngine<Ported, Directed>;

/// ⚙️ Puzzle 2d board engine alias.
pub type BoardEngine = DirectedPortGraphEngine;

/// 🪢 Cubic edge connecting two handles (legacy field names).
#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    pub id: EdgeId,
    pub source_handle: HandleId,
    pub target_handle: HandleId,
}

// #region 🔖EdgeEndpointResolution
/// 🔗 Resolves a ported edge endpoint to a node id (handle lookup, then node id).
fn resolve_endpoint_node_id(endpoint_id: &str, handle_to_node: &std::collections::HashMap<String, String>) -> String {
    handle_to_node.get(endpoint_id).cloned().unwrap_or_else(|| endpoint_id.to_string())
}
// #endregion 🔖EdgeEndpointResolution

// #region 🕸️ForceGraphLayout
pub mod force_graph {
    use serde_json::Value;
    use std::collections::HashMap;

    use crate::board_json_visible_or_true;
    pub use mathematical_graph_normal_undirected::ForceGraphLayoutOptions;

    fn build_handle_to_node(nodes: &[Value]) -> HashMap<String, String> {
        let mut handle_to_node: HashMap<String, String> = HashMap::new();
        for node in nodes {
            let Some(obj) = node.as_object() else {
                continue;
            };
            if !board_json_visible_or_true(obj) {
                continue;
            };
            let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
                continue;
            };
            let Some(handles) = obj.get("handles").and_then(|v| v.as_array()) else {
                continue;
            };
            for h in handles {
                let Some(ho) = h.as_object() else {
                    continue;
                };
                if !board_json_visible_or_true(ho) {
                    continue;
                };
                if let Some(hid) = ho.get("id").and_then(|v| v.as_str()) {
                    handle_to_node.insert(hid.to_string(), nid.to_string());
                }
            }
        }
        handle_to_node
    }

    /// 🕸️ Ported force layout: resolves handle endpoints, then delegates to normal undirected physics.
    pub fn apply_force_graph_layout_to_fixture_v1_value(fixture: &mut Value, opts: &ForceGraphLayoutOptions) -> Result<(), String> {
        let nodes = fixture
            .as_object()
            .and_then(|root| root.get("nodes"))
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let handle_to_node = build_handle_to_node(&nodes);
        mathematical_graph_normal_undirected::apply_force_graph_layout_to_fixture_v1_value_resolved(
            fixture,
            opts,
            |endpoint, id_to_index| {
                let node_id = handle_to_node.get(endpoint).cloned().unwrap_or_else(|| endpoint.to_string());
                id_to_index.contains_key(&node_id).then_some(node_id)
            },
        )
    }

    /// 🕸️ JSON entry for ported force layout (handle endpoints resolved before undirected physics).
    pub fn apply_force_graph_layout_to_fixture_v1_json(fixture_json: &str, options_json: &str) -> Result<String, String> {
        let mut fixture: Value = serde_json::from_str(fixture_json).map_err(|e| e.to_string())?;
        let opts: ForceGraphLayoutOptions = if options_json.trim().is_empty() {
            ForceGraphLayoutOptions::default()
        } else {
            serde_json::from_str(options_json).map_err(|e| e.to_string())?
        };
        apply_force_graph_layout_to_fixture_v1_value(&mut fixture, &opts)?;
        serde_json::to_string(&fixture).map_err(|e| e.to_string())
    }
}
// #endregion 🕸️ForceGraphLayout

// #region 🌳HierarchicalTreeLayout
pub mod hierarchical_tree {
    use serde::Deserialize;
    use serde_json::Value;
    use std::collections::{HashMap, HashSet};

    use crate::board_json_visible_or_true;
    use crate::fixture_edge_handle_ids_from_object;

    /// 🌳 Buchheim tidy-tree knobs: rank gap, sibling breadth, growth-axis string, optional world anchor for the laid subtree.
    #[derive(Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct HierarchicalTreeLayoutOptions {
        #[serde(default = "default_layer_spacing")]
        pub layer_spacing: f64,
        #[serde(default = "default_sibling_gap")]
        pub sibling_gap: f64,
        #[serde(default = "default_direction")]
        pub direction: String,
        #[serde(default)]
        pub center_x: Option<f64>,
        #[serde(default)]
        pub center_y: Option<f64>,
        /// 📌 Node ids whose incoming fixture centers are kept; Buchheim still runs for placement of unlocked nodes.
        #[serde(default)]
        pub locked_node_ids: Vec<String>,
    }

    fn default_layer_spacing() -> f64 {
        120.0
    }
    fn default_sibling_gap() -> f64 {
        28.0
    }
    fn default_direction() -> String {
        "downwards".into()
    }

    impl Default for HierarchicalTreeLayoutOptions {
        fn default() -> Self {
            Self { layer_spacing: default_layer_spacing(), sibling_gap: default_sibling_gap(), direction: default_direction(), center_x: None, center_y: None, locked_node_ids: Vec::new() }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum TreeDirection {
        Downwards,
        Upwards,
        Right,
        Left,
    }

    impl TreeDirection {
        fn parse(s: &str) -> Result<Self, String> {
            match s.trim().to_ascii_lowercase().as_str() {
                "down" | "downwards" => Ok(Self::Downwards),
                "up" | "upwards" => Ok(Self::Upwards),
                "right" => Ok(Self::Right),
                "left" => Ok(Self::Left),
                _ => Err(format!("unknown hierarchical tree direction: {s}")),
            }
        }
    }

    fn half_extent(node: &Value) -> f64 {
        let Some(obj) = node.as_object() else {
            return 24.0;
        };
        if obj.get("shape").and_then(|v| v.as_str()) == Some("rectangle") {
            let w = obj.get("width").and_then(|v| v.as_f64()).unwrap_or(40.0);
            let h = obj.get("height").and_then(|v| v.as_f64()).unwrap_or(40.0);
            return (w.max(h) * 0.5).max(8.0);
        }
        obj.get("radius").and_then(|v| v.as_f64()).filter(|r| r.is_finite() && *r > 0.0).unwrap_or(24.0)
    }

    const TREE_SUPER_ID: &str = "__tree_super__";

    /** @emoji 🌲 Buchheim et al. (GD 2002) tidy tree: O(n) Reingold–Tilford with even sibling spacing (after pymag-trees listing 12). */
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

    fn buchheim_ancestor(nodes: &[BuchheimNode], vil: usize, v: usize, default_ancestor: usize) -> usize {
        let par = nodes[v].parent.expect("buchheim ancestor needs parent");
        let pa = nodes[vil].ancestor;
        if nodes[par].children.iter().any(|&c| c == pa) {
            pa
        } else {
            default_ancestor
        }
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
        let mut vor = v;
        let mut vil = w;
        let mut vol = match buchheim_leftmost_sibling(nodes, v) {
            Some(s) => s,
            None => return default_ancestor,
        };
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
                let a = buchheim_ancestor(nodes, vil, v, default_ancestor);
                buchheim_move_subtree(nodes, a, v, shift);
                sir += shift;
                sor += shift;
            }
            sil += nodes[vil].mod_;
            sir += nodes[vir].mod_;
            sol += nodes[vol].mod_;
            sor += nodes[vor].mod_;
        }
        if let Some(vil_r) = buchheim_next_right(nodes, vil) {
            if buchheim_next_right(nodes, vor).is_none() {
                nodes[vor].thread = Some(vil_r);
                nodes[vor].mod_ += sil - sor;
            }
        } else if buchheim_next_left(nodes, vir).is_some() && buchheim_next_left(nodes, vol).is_none() {
            if let Some(vir_l) = buchheim_next_left(nodes, vir) {
                nodes[vol].thread = Some(vir_l);
                nodes[vol].mod_ += sir - sol;
            }
            default_ancestor = v;
        }
        default_ancestor
    }

    fn buchheim_first_walk(nodes: &mut [BuchheimNode], v: usize, distance: f64) -> usize {
        if nodes[v].children.is_empty() {
            if buchheim_leftmost_sibling(nodes, v).is_some() {
                let lb = buchheim_left_brother(nodes, v).expect("leaf with leftmost sibling has left brother");
                nodes[v].x = nodes[lb].x + distance;
            } else {
                nodes[v].x = 0.0;
            }
            return v;
        }
        let mut default_ancestor = nodes[v].children[0];
        for &w in &nodes[v].children.clone() {
            buchheim_first_walk(nodes, w, distance);
            default_ancestor = buchheim_apportion(nodes, w, default_ancestor, distance);
        }
        buchheim_execute_shifts(nodes, v);
        let c0 = nodes[v].children[0];
        let c1 = *nodes[v].children.last().expect("internal node has children");
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
        for &w in &nodes[v].children.clone() {
            min_x = buchheim_second_walk(nodes, w, m + nodes[v].mod_, depth + 1, min_x);
        }
        min_x
    }

    fn buchheim_third_walk(nodes: &mut [BuchheimNode], v: usize, n: f64) {
        nodes[v].x += n;
        for &c in &nodes[v].children.clone() {
            buchheim_third_walk(nodes, c, n);
        }
    }

    fn run_buchheim_layout(id_to_node: &HashMap<String, Value>, roots: &[String], directed: &[(String, String)], depth: &HashMap<String, i32>) -> Result<HashMap<String, (f64, f64)>, String> {
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
        for id in id_to_node.keys() {
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
        let mut ordered_ids: Vec<String> = id_to_node.keys().cloned().collect();
        ordered_ids.sort();
        let id_to_idx: HashMap<String, usize> = ordered_ids.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect();
        let super_idx = ordered_ids.len();
        let mut nodes: Vec<BuchheimNode> =
            ordered_ids.iter().map(|id| BuchheimNode { ancestor: 0, change: 0.0, children: vec![], id: id.clone(), mod_: 0.0, number: 0, parent: None, shift: 0.0, synthetic: false, thread: None, x: -1.0, y: 0.0 }).collect();
        nodes.push(BuchheimNode { ancestor: super_idx, change: 0.0, children: vec![], id: TREE_SUPER_ID.to_string(), mod_: 0.0, number: 0, parent: None, shift: 0.0, synthetic: true, thread: None, x: -1.0, y: 0.0 });
        for (i, oid) in ordered_ids.iter().enumerate() {
            let pidx = if roots_set.contains(oid) {
                super_idx
            } else {
                match chosen_parent.get(oid) {
                    Some(p) => *id_to_idx.get(p).ok_or_else(|| format!("missing parent index for {p}"))?,
                    None => super_idx,
                }
            };
            nodes[i].parent = Some(pidx);
        }
        for p in 0..=super_idx {
            nodes[p].children.clear();
        }
        for i in 0..super_idx {
            let pi = nodes[i].parent.ok_or_else(|| "tree node missing parent".to_string())?;
            nodes[pi].children.push(i);
        }
        for p in 0..=super_idx {
            let mut ch: Vec<usize> = nodes[p].children.clone();
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
        let dist = 1.0f64;
        buchheim_first_walk(&mut nodes, super_idx, dist);
        let min_x = buchheim_second_walk(&mut nodes, super_idx, 0.0, 0, f64::INFINITY);
        if min_x.is_finite() && min_x < 0.0 {
            buchheim_third_walk(&mut nodes, super_idx, -min_x);
        }
        let mut out: HashMap<String, (f64, f64)> = HashMap::new();
        for (i, n) in nodes.iter().enumerate() {
            if i == super_idx || n.synthetic {
                continue;
            }
            out.insert(n.id.clone(), (n.x, n.y));
        }
        Ok(out)
    }

    /// 🌳 Writes node centers: Buchheim tidy-tree on a spanning forest (min-depth parent tie-break id), synthetic multi-root; super-root not serialized.
    pub fn apply_hierarchical_tree_layout_to_fixture_v1_value(fixture: &mut Value, opts: &HierarchicalTreeLayoutOptions) -> Result<(), String> {
        let dir = TreeDirection::parse(&opts.direction)?;
        let Some(root) = fixture.as_object_mut() else {
            return Err("fixture root must be object".into());
        };
        if root.get("schema").and_then(|v| v.as_str()) != Some("puzzle.2d.fixture/v1") {
            return Err("schema must be puzzle.2d.fixture/v1".into());
        }
        let edges_json = root.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let Some(nodes) = root.get_mut("nodes").and_then(|v| v.as_array_mut()) else {
            return Err("nodes array missing".into());
        };
        if nodes.is_empty() {
            return Ok(());
        }
        let mut handle_to_node: HashMap<String, String> = HashMap::new();
        let mut id_to_node: HashMap<String, Value> = HashMap::new();
        for node in nodes.iter() {
            let Some(obj) = node.as_object() else {
                continue;
            };
            if !board_json_visible_or_true(obj) {
                continue;
            }
            let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
                continue;
            };
            id_to_node.insert(nid.to_string(), node.clone());
            let Some(handles) = obj.get("handles").and_then(|v| v.as_array()) else {
                continue;
            };
            for h in handles {
                let Some(ho) = h.as_object() else {
                    continue;
                };
                if !board_json_visible_or_true(ho) {
                    continue;
                }
                if let Some(hid) = ho.get("id").and_then(|v| v.as_str()) {
                    handle_to_node.insert(hid.to_string(), nid.to_string());
                }
            }
        }
        if id_to_node.is_empty() {
            return Ok(());
        }
        let mut directed: Vec<(String, String)> = Vec::new();
        let mut seen_dir: HashSet<(String, String)> = HashSet::new();
        for e in &edges_json {
            let Some(eo) = e.as_object() else {
                continue;
            };
            if !board_json_visible_or_true(eo) {
                continue;
            }
            let Some((src_h, tgt_h)) = fixture_edge_handle_ids_from_object(eo) else {
                continue;
            };
            let source_node_id = crate::resolve_endpoint_node_id(src_h, &handle_to_node);
            let target_node_id = crate::resolve_endpoint_node_id(tgt_h, &handle_to_node);
            if source_node_id == target_node_id {
                continue;
            }
            if !id_to_node.contains_key(&source_node_id) || !id_to_node.contains_key(&target_node_id) {
                continue;
            }
            if seen_dir.insert((source_node_id.clone(), target_node_id.clone())) {
                directed.push((source_node_id, target_node_id));
            }
        }
        let mut incoming_edge_count_by_node: HashMap<String, u32> = HashMap::new();
        for id in id_to_node.keys() {
            incoming_edge_count_by_node.insert(id.clone(), 0);
        }
        for (_source_nid, target_nid) in &directed {
            *incoming_edge_count_by_node.entry(target_nid.clone()).or_insert(0) += 1;
        }
        let mut roots: Vec<String> = Vec::new();
        for node in nodes.iter() {
            let Some(obj) = node.as_object() else {
                continue;
            };
            if !board_json_visible_or_true(obj) {
                continue;
            }
            if obj.get("root").and_then(|v| v.as_bool()) == Some(true) {
                if let Some(nid) = obj.get("id").and_then(|v| v.as_str()) {
                    roots.push(nid.to_string());
                }
            }
        }
        roots.sort();
        roots.dedup();
        if roots.is_empty() {
            for (id, &d) in &incoming_edge_count_by_node {
                if d == 0 {
                    roots.push(id.clone());
                }
            }
            roots.sort();
        }
        if roots.is_empty() {
            roots = id_to_node.keys().cloned().collect();
            roots.sort();
        }
        let mut depth: HashMap<String, i32> = HashMap::new();
        for r in &roots {
            depth.insert(r.clone(), 0);
        }
        let cap = directed.len().saturating_mul(3).saturating_add(nodes.len()).saturating_add(8);
        for _ in 0..cap {
            let mut changed = false;
            for (source_nid, target_nid) in &directed {
                let Some(&dp) = depth.get(source_nid) else {
                    continue;
                };
                let nd = dp + 1;
                let cur = *depth.get(target_nid).unwrap_or(&-1);
                if nd > cur {
                    depth.insert(target_nid.clone(), nd);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let max_depth = depth.values().copied().max().unwrap_or(0);
        for id in id_to_node.keys() {
            depth.entry(id.clone()).or_insert(max_depth + 1);
        }
        let raw = run_buchheim_layout(&id_to_node, &roots, &directed, &depth)?;
        let mean_half: f64 = id_to_node.values().map(|nv| half_extent(nv)).sum::<f64>() / id_to_node.len().max(1) as f64;
        let along_scale = (opts.sibling_gap + 2.0 * mean_half).max(8.0);
        let mut pos: HashMap<String, (f64, f64)> = HashMap::new();
        for (id, (bx, by)) in raw {
            let along = bx * along_scale;
            let orth = by * opts.layer_spacing;
            let (lx, ly) = match dir {
                TreeDirection::Downwards => (along, orth),
                TreeDirection::Upwards => (along, -orth),
                TreeDirection::Right => (orth, along),
                TreeDirection::Left => (-orth, along),
            };
            pos.insert(id, (lx, ly));
        }
        let mut minx = f64::INFINITY;
        let mut maxx = f64::NEG_INFINITY;
        let mut miny = f64::INFINITY;
        let mut maxy = f64::NEG_INFINITY;
        for (id, (x, y)) in &pos {
            let h = half_extent(id_to_node.get(id).unwrap());
            minx = minx.min(x - h);
            maxx = maxx.max(x + h);
            miny = miny.min(y - h);
            maxy = maxy.max(y + h);
        }
        if !minx.is_finite() {
            minx = 0.0;
            maxx = 1.0;
            miny = 0.0;
            maxy = 1.0;
        }
        let cx = (minx + maxx) * 0.5;
        let cy = (miny + maxy) * 0.5;
        let gx = opts.center_x.unwrap_or(0.0);
        let gy = opts.center_y.unwrap_or(0.0);
        let dx = gx - cx;
        let dy = gy - cy;
        let locked_set: HashSet<String> = opts.locked_node_ids.iter().cloned().collect();
        let mut pinned_world: HashMap<String, (f64, f64)> = HashMap::new();
        if !locked_set.is_empty() {
            for node in nodes.iter() {
                let Some(obj) = node.as_object() else {
                    continue;
                };
                if !board_json_visible_or_true(obj) {
                    continue;
                }
                let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
                    continue;
                };
                if !locked_set.contains(nid) {
                    continue;
                }
                if !id_to_node.contains_key(nid) {
                    continue;
                }
                let px = obj.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let py = obj.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                pinned_world.insert(nid.to_string(), (px, py));
            }
        }
        for (id, (x, y)) in pos {
            let (fx, fy) = if let Some(&(px, py)) = pinned_world.get(&id) { (px, py) } else { (x + dx, y + dy) };
            let idx = nodes.iter().position(|n| n.get("id").and_then(|v| v.as_str()) == Some(id.as_str())).ok_or_else(|| format!("node index {id}"))?;
            let Some(obj) = nodes[idx].as_object_mut() else {
                continue;
            };
            obj.insert("x".into(), serde_json::json!(fx));
            obj.insert("y".into(), serde_json::json!(fy));
        }
        Ok(())
    }
}
// #endregion 🌳HierarchicalTreeLayout

// #region 🔁RedrawLayout
pub mod redraw_layout {
    use crate::cavas::vello::kurbo::Point;
    use serde::Deserialize;
    use serde_json::Value;
    use std::collections::HashMap;

    use crate::force_graph::{apply_force_graph_layout_to_fixture_v1_value, ForceGraphLayoutOptions};
    use crate::hierarchical_tree::{apply_hierarchical_tree_layout_to_fixture_v1_value, HierarchicalTreeLayoutOptions};
    use crate::{circle_handle_angle_toward, distance_between, rectangle_handle_angle_toward};
    use crate::board_json_visible_or_true;
    use crate::fixture_edge_handle_ids_from_object;

    #[derive(Debug, Clone, Copy)]
    enum NodeShapeSnap {
        Circle { cx: f64, cy: f64 },
        Rect { cx: f64, cy: f64, w: f64, h: f64 },
    }

    impl NodeShapeSnap {
        fn center(self) -> Point {
            match self {
                NodeShapeSnap::Circle { cx, cy, .. } | NodeShapeSnap::Rect { cx, cy, .. } => Point::new(cx, cy),
            }
        }

        fn handle_angle_toward(self, toward: Point) -> Option<f64> {
            let c = self.center();
            if distance_between(c, toward) <= 1e-9 {
                return None;
            }
            Some(match self {
                NodeShapeSnap::Circle { cx, cy, .. } => circle_handle_angle_toward(Point::new(cx, cy), toward),
                NodeShapeSnap::Rect { cx, cy, w, h } => rectangle_handle_angle_toward(Point::new(cx, cy), w, h, toward),
            })
        }
    }

    fn parse_node_shape_snap(node: &serde_json::Map<String, Value>) -> Option<NodeShapeSnap> {
        let cx = node.get("x").and_then(|v| v.as_f64())?;
        let cy = node.get("y").and_then(|v| v.as_f64())?;
        if node.get("shape").and_then(|v| v.as_str()) == Some("rectangle") {
            let w = node.get("width").and_then(|v| v.as_f64())?;
            let h = node.get("height").and_then(|v| v.as_f64())?;
            Some(NodeShapeSnap::Rect { cx, cy, w, h })
        } else {
            node.get("radius").and_then(|v| v.as_f64())?;
            Some(NodeShapeSnap::Circle { cx, cy })
        }
    }

    /// 🔗 Sets each edge endpoint handle `angle` so the chord follows node centers; last edge wins on shared handles.
    pub fn apply_edge_handle_snap_to_fixture_v1_value(fixture: &mut Value) -> Result<(), String> {
        let Some(root) = fixture.as_object_mut() else {
            return Err("fixture root must be object".into());
        };
        if root.get("schema").and_then(|v| v.as_str()) != Some("puzzle.2d.fixture/v1") {
            return Err("schema must be puzzle.2d.fixture/v1".into());
        }
        let edges_json = root.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let Some(nodes) = root.get_mut("nodes").and_then(|v| v.as_array_mut()) else {
            return Err("nodes array missing".into());
        };
        let mut shapes: Vec<Option<NodeShapeSnap>> = Vec::with_capacity(nodes.len());
        let mut handle_loc: HashMap<String, (usize, usize)> = HashMap::new();
        for (ni, node_val) in nodes.iter().enumerate() {
            let Some(no) = node_val.as_object() else {
                shapes.push(None);
                continue;
            };
            if !board_json_visible_or_true(no) {
                shapes.push(None);
                continue;
            }
            shapes.push(parse_node_shape_snap(no));
            let Some(hs) = no.get("handles").and_then(|v| v.as_array()) else {
                continue;
            };
            for (hi, h) in hs.iter().enumerate() {
                let Some(ho) = h.as_object() else {
                    continue;
                };
                if !board_json_visible_or_true(ho) {
                    continue;
                }
                if let Some(hid) = ho.get("id").and_then(|v| v.as_str()) {
                    handle_loc.insert(hid.to_string(), (ni, hi));
                }
            }
        }
        let mut angle_by_loc: HashMap<(usize, usize), f64> = HashMap::new();
        for e in &edges_json {
            let Some(eo) = e.as_object() else {
                continue;
            };
            if !board_json_visible_or_true(eo) {
                continue;
            }
            let Some((src_h, tgt_h)) = fixture_edge_handle_ids_from_object(eo) else {
                continue;
            };
            let Some(&(ni_a, hi_a)) = handle_loc.get(src_h) else {
                continue;
            };
            let Some(&(ni_b, hi_b)) = handle_loc.get(tgt_h) else {
                continue;
            };
            let Some(sa) = shapes.get(ni_a).copied().flatten() else {
                continue;
            };
            let Some(sb) = shapes.get(ni_b).copied().flatten() else {
                continue;
            };
            if let Some(ang_a) = sa.handle_angle_toward(sb.center()) {
                angle_by_loc.insert((ni_a, hi_a), ang_a);
            }
            if let Some(ang_b) = sb.handle_angle_toward(sa.center()) {
                angle_by_loc.insert((ni_b, hi_b), ang_b);
            }
        }
        for ((ni, hi), ang) in angle_by_loc {
            let Some(node_val) = nodes.get_mut(ni) else {
                continue;
            };
            let Some(no) = node_val.as_object_mut() else {
                continue;
            };
            let Some(hs) = no.get_mut("handles").and_then(|v| v.as_array_mut()) else {
                continue;
            };
            let Some(h) = hs.get_mut(hi) else {
                continue;
            };
            let Some(ho) = h.as_object_mut() else {
                continue;
            };
            ho.insert("angle".into(), serde_json::json!(ang));
        }
        Ok(())
    }

    pub fn apply_edge_handle_snap_to_fixture_v1_json(fixture_json: &str) -> Result<String, String> {
        let mut fixture: Value = serde_json::from_str(fixture_json).map_err(|e| e.to_string())?;
        apply_edge_handle_snap_to_fixture_v1_value(&mut fixture)?;
        serde_json::to_string(&fixture).map_err(|e| e.to_string())
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct RedrawFixtureOptions {
        mode: String,
        #[serde(default)]
        center_x: Option<f64>,
        #[serde(default)]
        center_y: Option<f64>,
        #[serde(default)]
        random_seed: Option<u64>,
        #[serde(default)]
        redraw_handles_after: bool,
        #[serde(default)]
        locked_node_ids: Vec<String>,
        #[serde(default)]
        force_graph: Option<ForceGraphLayoutOptions>,
        #[serde(default)]
        hierarchical_tree: Option<HierarchicalTreeLayoutOptions>,
    }

    pub fn apply_redraw_layout_to_fixture_v1_json(fixture_json: &str, options_json: &str) -> Result<String, String> {
        let opts: RedrawFixtureOptions = serde_json::from_str(options_json).map_err(|e| e.to_string())?;
        let mut fixture: Value = serde_json::from_str(fixture_json).map_err(|e| e.to_string())?;
        match opts.mode.as_str() {
            "force-graph" => {
                let mut fo = opts.force_graph.clone().unwrap_or_default();
                if opts.center_x.is_some() {
                    fo.center_x = opts.center_x;
                }
                if opts.center_y.is_some() {
                    fo.center_y = opts.center_y;
                }
                if let Some(s) = opts.random_seed {
                    fo.random_seed = s;
                }
                for id in &opts.locked_node_ids {
                    if !fo.locked_node_ids.contains(id) {
                        fo.locked_node_ids.push(id.clone());
                    }
                }
                apply_force_graph_layout_to_fixture_v1_value(&mut fixture, &fo)?;
            }
            "hierarchical-tree" => {
                let mut hierarchical_opts = opts.hierarchical_tree.clone().unwrap_or_default();
                if opts.center_x.is_some() {
                    hierarchical_opts.center_x = opts.center_x;
                }
                if opts.center_y.is_some() {
                    hierarchical_opts.center_y = opts.center_y;
                }
                for id in &opts.locked_node_ids {
                    if !hierarchical_opts.locked_node_ids.contains(id) {
                        hierarchical_opts.locked_node_ids.push(id.clone());
                    }
                }
                apply_hierarchical_tree_layout_to_fixture_v1_value(&mut fixture, &hierarchical_opts)?;
            }
            other => return Err(format!("unknown redraw mode: {other}")),
        }
        if opts.redraw_handles_after {
            apply_edge_handle_snap_to_fixture_v1_value(&mut fixture)?;
        }
        serde_json::to_string(&fixture).map_err(|e| e.to_string())
    }
}
// #endregion 🔁RedrawLayout

// #region 🔖GraphExtension
/// 🧩 Extension hook for domain-specific graph behavior.
pub trait GraphExtension: cavas::CanvasExtension {}

pub use force_graph::{apply_force_graph_layout_to_fixture_v1_json, apply_force_graph_layout_to_fixture_v1_value, ForceGraphLayoutOptions};
pub use redraw_layout::{apply_edge_handle_snap_to_fixture_v1_json, apply_redraw_layout_to_fixture_v1_json};
// #endregion 🔖GraphExtension

// #region 🔖Tests
#[cfg(test)]
mod quadrant_tests {
    use super::*;

    #[test]
    fn board_engine_alias_works() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(11, 1, 3.14);
        engine.create_edge(100, 10, 11);
        assert_eq!(engine.render_snapshot().edges.len(), 1);
    }
}
// #endregion 🔖Tests
