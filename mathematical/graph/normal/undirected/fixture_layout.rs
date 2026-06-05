//! ↔️ Normal undirected fixture layout: node-id edges, symmetric springs, no port handles.

use mathematical_core::force_layout::{self, ForceLayoutOptions as CoreForceLayoutOptions, Vec2};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

use infinite_cavas::board_json_visible_or_true;

// #region 🕸️ForceGraphLayout
/// ⚙️ Force-directed layout parameters for normal undirected node-id graphs.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForceGraphLayoutOptions {
    #[serde(default = "default_iterations")]
    pub iterations: u32,
    #[serde(default = "default_ideal_edge_length")]
    pub ideal_edge_length: f64,
    #[serde(default = "default_repulsion_strength")]
    pub repulsion_strength: f64,
    #[serde(default = "default_spring_strength")]
    pub spring_strength: f64,
    #[serde(default = "default_gravity")]
    pub gravity: f64,
    #[serde(default)]
    pub center_x: Option<f64>,
    #[serde(default)]
    pub center_y: Option<f64>,
    #[serde(default = "default_time_step")]
    pub time_step: f64,
    #[serde(default = "default_velocity_damping")]
    pub velocity_damping: f64,
    #[serde(default = "default_max_speed")]
    pub max_speed: f64,
    #[serde(default = "default_random_seed")]
    pub random_seed: u64,
    #[serde(default = "default_barnes_hut_theta")]
    pub barnes_hut_theta: f64,
    #[serde(default = "default_pairwise_repulsion_max_bodies")]
    pub pairwise_repulsion_max_bodies: u32,
    #[serde(default)]
    pub locked_node_ids: Vec<String>,
}

fn default_iterations() -> u32 {
    420
}
fn default_ideal_edge_length() -> f64 {
    140.0
}
fn default_repulsion_strength() -> f64 {
    6500.0
}
fn default_spring_strength() -> f64 {
    0.028
}
fn default_gravity() -> f64 {
    0.018
}
fn default_time_step() -> f64 {
    0.85
}
fn default_velocity_damping() -> f64 {
    0.88
}
fn default_max_speed() -> f64 {
    48.0
}
fn default_random_seed() -> u64 {
    0x5eedfaced0
}
fn default_barnes_hut_theta() -> f64 {
    0.78
}
fn default_pairwise_repulsion_max_bodies() -> u32 {
    56
}

impl Default for ForceGraphLayoutOptions {
    fn default() -> Self {
        Self {
            iterations: default_iterations(),
            ideal_edge_length: default_ideal_edge_length(),
            repulsion_strength: default_repulsion_strength(),
            spring_strength: default_spring_strength(),
            gravity: default_gravity(),
            center_x: None,
            center_y: None,
            time_step: default_time_step(),
            velocity_damping: default_velocity_damping(),
            max_speed: default_max_speed(),
            random_seed: default_random_seed(),
            barnes_hut_theta: default_barnes_hut_theta(),
            pairwise_repulsion_max_bodies: default_pairwise_repulsion_max_bodies(),
            locked_node_ids: Vec::new(),
        }
    }
}

fn fixture_schema_ok(schema: Option<&str>) -> bool {
    matches!(schema, Some("puzzle.2d.fixture/v1") | Some("reasoning.mindmap.fixture/v1"))
}

fn node_repulsion_radius(node: &Value) -> f64 {
    let Some(obj) = node.as_object() else {
        return 32.0;
    };
    if obj.get("shape").and_then(|v| v.as_str()) == Some("rectangle") {
        let w = obj.get("width").and_then(|v| v.as_f64()).unwrap_or(40.0);
        let h = obj.get("height").and_then(|v| v.as_f64()).unwrap_or(40.0);
        return ((w * w + h * h).sqrt() * 0.5).max(8.0);
    }
    obj.get("radius")
        .and_then(|v| v.as_f64())
        .filter(|r| r.is_finite() && *r > 0.0)
        .unwrap_or(32.0)
}

fn fixture_edge_node_ids(eo: &serde_json::Map<String, Value>) -> Option<(&str, &str)> {
    let source = eo.get("source").and_then(|v| v.as_str())?;
    let target = eo.get("target").and_then(|v| v.as_str())?;
    Some((source, target))
}

fn core_opts(opts: &ForceGraphLayoutOptions, gx: f64, gy: f64) -> CoreForceLayoutOptions {
    CoreForceLayoutOptions {
        iterations: opts.iterations,
        ideal_edge_length: opts.ideal_edge_length,
        repulsion_strength: opts.repulsion_strength,
        spring_strength: opts.spring_strength,
        gravity: opts.gravity,
        center_x: gx,
        center_y: gy,
        time_step: opts.time_step,
        velocity_damping: opts.velocity_damping,
        max_speed: opts.max_speed,
        random_seed: opts.random_seed,
        barnes_hut_theta: opts.barnes_hut_theta,
        pairwise_repulsion_max_bodies: opts.pairwise_repulsion_max_bodies,
    }
}

/// ↔️ Resolves edge endpoint strings to node ids (node-id graphs only).
pub fn resolve_node_id_endpoint(endpoint_id: &str, id_to_index: &HashMap<String, usize>) -> Option<String> {
    id_to_index.contains_key(endpoint_id).then(|| endpoint_id.to_string())
}

/// 🕸️ Runs undirected force layout on a mindmap or puzzle 2d fixture with node-id edges.
pub fn apply_force_graph_layout_to_fixture_v1_value(fixture: &mut Value, opts: &ForceGraphLayoutOptions) -> Result<(), String> {
    apply_force_graph_layout_to_fixture_v1_value_resolved(fixture, opts, resolve_node_id_endpoint)
}

/// 🕸️ Force layout with a custom endpoint→node-id resolver (ported graphs pass handle lookup here).
pub fn apply_force_graph_layout_to_fixture_v1_value_resolved(
    fixture: &mut Value,
    opts: &ForceGraphLayoutOptions,
    resolve_node_id: impl Fn(&str, &HashMap<String, usize>) -> Option<String>,
) -> Result<(), String> {
    let Some(root) = fixture.as_object_mut() else {
        return Err("fixture root must be object".into());
    };
    if !fixture_schema_ok(root.get("schema").and_then(|v| v.as_str())) {
        return Err("schema must be puzzle.2d.fixture/v1 or reasoning.mindmap.fixture/v1".into());
    }
    let edges = root.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    let Some(nodes) = root.get_mut("nodes").and_then(|v| v.as_array_mut()) else {
        return Err("nodes array missing".into());
    };
    if nodes.is_empty() {
        return Ok(());
    }
    let locked_ids: HashSet<String> = opts.locked_node_ids.iter().cloned().collect();
    let mut id_to_index: HashMap<String, usize> = HashMap::new();
    let mut visible_node_indices: Vec<usize> = Vec::new();
    let mut optional_xy: Vec<Option<(f64, f64)>> = Vec::new();
    let mut is_locked: Vec<bool> = Vec::new();
    let mut positions: Vec<Vec2> = Vec::new();
    let mut radii: Vec<f64> = Vec::new();
    for (raw_idx, node) in nodes.iter().enumerate() {
        let Some(obj) = node.as_object() else {
            return Err("node must be object".into());
        };
        if !board_json_visible_or_true(obj) {
            continue;
        }
        let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
            return Err("node id missing".into());
        };
        let x_opt = obj.get("x").and_then(|v| v.as_f64());
        let y_opt = obj.get("y").and_then(|v| v.as_f64());
        let xy = match (x_opt, y_opt) {
            (Some(x), Some(y)) if x.is_finite() && y.is_finite() => Some((x, y)),
            _ => None,
        };
        id_to_index.insert(nid.to_string(), positions.len());
        visible_node_indices.push(raw_idx);
        optional_xy.push(xy);
        is_locked.push(locked_ids.contains(nid));
        positions.push(Vec2::ZERO);
        radii.push(node_repulsion_radius(node));
    }
    let n = positions.len();
    if n == 0 {
        return Ok(());
    }
    let mut sum = Vec2::ZERO;
    let mut finite_ct: u32 = 0;
    for xy in &optional_xy {
        if let Some((x, y)) = xy {
            sum += Vec2::new(*x, *y);
            finite_ct += 1;
        }
    }
    let anchor = if finite_ct > 0 {
        sum / (finite_ct as f64)
    } else {
        Vec2::new(opts.center_x.unwrap_or(0.0), opts.center_y.unwrap_or(0.0))
    };
    for i in 0..n {
        positions[i] = if let Some((x, y)) = optional_xy[i] {
            Vec2::new(x, y)
        } else {
            Vec2::ZERO
        };
    }
    let pin: Vec<Option<Vec2>> = (0..n)
        .map(|i| if is_locked[i] { Some(positions[i]) } else { None })
        .collect();
    force_layout::seed_positions(&mut positions, &pin, anchor, opts.random_seed);
    let mut edge_pairs: Vec<(usize, usize)> = Vec::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    for e in &edges {
        let Some(eo) = e.as_object() else {
            continue;
        };
        if !board_json_visible_or_true(eo) {
            continue;
        }
        let Some((src, tgt)) = fixture_edge_node_ids(eo) else {
            continue;
        };
        let Some(a) = resolve_node_id(src, &id_to_index) else {
            continue;
        };
        let Some(b) = resolve_node_id(tgt, &id_to_index) else {
            continue;
        };
        if a == b {
            continue;
        }
        let Some(&ia) = id_to_index.get(&a) else {
            continue;
        };
        let Some(&ib) = id_to_index.get(&b) else {
            continue;
        };
        let lo = ia.min(ib);
        let hi = ia.max(ib);
        if seen.insert((lo, hi)) {
            edge_pairs.push((lo, hi));
        }
    }
    let mut cx = 0.0f64;
    let mut cy = 0.0f64;
    for p in &positions {
        cx += p.x;
        cy += p.y;
    }
    cx /= n as f64;
    cy /= n as f64;
    let gx = opts.center_x.unwrap_or(cx);
    let gy = opts.center_y.unwrap_or(cy);
    force_layout::run_force_layout(&mut positions, &radii, &edge_pairs, &pin, &core_opts(opts, gx, gy));
    for (idx, raw_idx) in visible_node_indices.into_iter().enumerate() {
        let Some(node) = nodes.get_mut(raw_idx) else {
            continue;
        };
        let Some(obj) = node.as_object_mut() else {
            continue;
        };
        obj.insert("x".into(), serde_json::json!(positions[idx].x));
        obj.insert("y".into(), serde_json::json!(positions[idx].y));
    }
    Ok(())
}

/// 🕸️ JSON entry for undirected force layout.
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
// #endregion 🕸️ForceGraphLayout

// #region 🔁RedrawLayout
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
}

/// ↔️ Redraw dispatcher for normal undirected graphs (`force-graph` only).
pub fn apply_redraw_layout_to_fixture_v1_json(fixture_json: &str, options_json: &str) -> Result<String, String> {
    let opts: RedrawFixtureOptions = serde_json::from_str(options_json).map_err(|e| e.to_string())?;
    if opts.redraw_handles_after {
        return Err("normal undirected redraw does not support redrawHandlesAfter".into());
    }
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
        other => return Err(format!("normal undirected redraw does not support mode: {other}")),
    }
    serde_json::to_string(&fixture).map_err(|e| e.to_string())
}
// #endregion 🔁RedrawLayout
