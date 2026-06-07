//! 🌳 Directed acyclic port graph: rectangle IO nodes on infinite canvas.

use serde::{Deserialize, Serialize};

pub use infinite_cavas as cavas;
pub use mathematical_graph_port_directed::{
    self as graph, apply_edge_handle_snap_to_fixture_v1_json, apply_redraw_layout_to_fixture_v1_json, BoardEngine, BoardEvent, Camera, DirectedPortGraphEngine, Edge, EdgeId, GraphExtension, Handle, HandleId, InteractionMode, Node, NodeId, RenderSnapshot, Selection,
};

/// 🌳 DAG board engine alias.
pub type DagBoardEngine = DirectedPortGraphEngine;

// #region 🔖IoNode
/// 📦 Rectangle node with named inputs on the left and outputs on the right.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IoNodeSpec {
    pub id: String,
    pub name: String,
    pub inputs: Vec<IoPortSpec>,
    pub outputs: Vec<IoPortSpec>,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "default_node_width")]
    pub width: f64,
    #[serde(default = "default_node_height")]
    pub height: f64,
}

fn default_node_width() -> f64 {
    160.0
}

fn default_node_height() -> f64 {
    56.0
}

/// 🪝 Named horizontal port on an IO node edge.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IoPortSpec {
    pub id: String,
    pub label: String,
}

/// 📐 Places input handles on the left and output handles on the right of a rectangle node.
pub fn io_node_handle_angles(input_index: usize, input_count: usize, output_index: usize, output_count: usize) -> (f64, f64) {
    let input_angle = port_angle_on_side(input_index, input_count.max(1), true);
    let output_angle = port_angle_on_side(output_index, output_count.max(1), false);
    (input_angle, output_angle)
}

fn port_angle_on_side(index: usize, count: usize, left: bool) -> f64 {
    let t = (index as f64 + 0.5) / count as f64;
    let y = (t - 0.5) * 0.8;
    if left {
        std::f64::consts::PI + y * std::f64::consts::FRAC_PI_2 * 0.9
    } else {
        y * std::f64::consts::FRAC_PI_2 * 0.9
    }
}
// #endregion 🔖IoNode

// #region 🔖Acyclicity
use std::collections::{HashMap, HashSet};

/// 🚫 Returns true when adding `source -> target` would create a cycle.
pub fn would_create_cycle(existing: &[(String, String)], source: &str, target: &str) -> bool {
    if source == target {
        return true;
    }
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for (u, v) in existing {
        adj.entry(u.clone()).or_default().push(v.clone());
    }
    adj.entry(source.to_string()).or_default().push(target.to_string());
    has_path(&adj, target, source)
}

fn has_path(adj: &HashMap<String, Vec<String>>, from: &str, to: &str) -> bool {
    let mut seen = HashSet::new();
    let mut stack = vec![from.to_string()];
    while let Some(n) = stack.pop() {
        if n == to {
            return true;
        }
        if !seen.insert(n.clone()) {
            continue;
        }
        if let Some(next) = adj.get(&n) {
            for m in next {
                stack.push(m.clone());
            }
        }
    }
    false
}
// #endregion 🔖Acyclicity

// #region 🔖Layout
use mathematical_core::tree_layout::buchheim_positions;
use serde_json::Value;

/// 🌲 Layered DAG layout options for fixture JSON.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagLayoutOptions {
    #[serde(default = "default_layer_spacing")]
    pub layer_spacing: f64,
    #[serde(default = "default_sibling_gap")]
    pub sibling_gap: f64,
    #[serde(default)]
    pub center_x: Option<f64>,
    #[serde(default)]
    pub center_y: Option<f64>,
}

fn default_layer_spacing() -> f64 {
    120.0
}

fn default_sibling_gap() -> f64 {
    40.0
}

impl Default for DagLayoutOptions {
    fn default() -> Self {
        Self { layer_spacing: default_layer_spacing(), sibling_gap: default_sibling_gap(), center_x: None, center_y: None }
    }
}

/// 🌳 Writes node centers from a layered DAG layout into `dag.fixture/v1`.
pub fn apply_dag_layout_to_fixture_v1_value(fixture: &mut Value, opts: &DagLayoutOptions) -> Result<(), String> {
    let Some(root) = fixture.as_object_mut() else {
        return Err("fixture root must be object".into());
    };
    if root.get("schema").and_then(|v| v.as_str()) != Some("dag.fixture/v1") {
        return Err("schema must be dag.fixture/v1".into());
    }
    let edges_json = root.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    let Some(nodes) = root.get_mut("nodes").and_then(|v| v.as_array_mut()) else {
        return Err("nodes array missing".into());
    };
    if nodes.is_empty() {
        return Ok(());
    }
    let mut handle_to_node: HashMap<String, String> = HashMap::new();
    let mut node_ids: HashSet<String> = HashSet::new();
    for node in nodes.iter() {
        let Some(obj) = node.as_object() else {
            continue;
        };
        let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
            continue;
        };
        node_ids.insert(nid.to_string());
        if let Some(handles) = obj.get("handles").and_then(|v| v.as_array()) {
            for h in handles {
                if let Some(hid) = h.get("id").and_then(|v| v.as_str()) {
                    handle_to_node.insert(hid.to_string(), nid.to_string());
                }
            }
        }
    }
    let mut directed: Vec<(String, String)> = Vec::new();
    for e in &edges_json {
        let Some(eo) = e.as_object() else {
            continue;
        };
        let src = eo.get("source").and_then(|v| v.as_str()).or_else(|| eo.get("sourceHandle").and_then(|v| v.as_str()));
        let tgt = eo.get("target").and_then(|v| v.as_str()).or_else(|| eo.get("targetHandle").and_then(|v| v.as_str()));
        let (Some(src_h), Some(tgt_h)) = (src, tgt) else {
            continue;
        };
        let u = handle_to_node.get(src_h).cloned().unwrap_or_else(|| src_h.to_string());
        let v = handle_to_node.get(tgt_h).cloned().unwrap_or_else(|| tgt_h.to_string());
        if u != v && node_ids.contains(&u) && node_ids.contains(&v) {
            directed.push((u, v));
        }
    }
    let mut incoming: HashMap<String, u32> = HashMap::new();
    for id in &node_ids {
        incoming.insert(id.clone(), 0);
    }
    for (_, v) in &directed {
        *incoming.entry(v.clone()).or_insert(0) += 1;
    }
    let roots: Vec<String> = node_ids.iter().filter(|id| incoming.get(*id).copied().unwrap_or(0) == 0).cloned().collect();
    let roots = if roots.is_empty() { node_ids.iter().cloned().collect() } else { roots };
    let mut depth: HashMap<String, i32> = HashMap::new();
    for r in &roots {
        depth.insert(r.clone(), 0);
    }
    for _ in 0..directed.len().saturating_add(node_ids.len()).saturating_add(4) {
        let mut changed = false;
        for (u, v) in &directed {
            let Some(&du) = depth.get(u) else {
                continue;
            };
            let nd = du + 1;
            if depth.get(v).copied().unwrap_or(-1) < nd {
                depth.insert(v.clone(), nd);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let pos = buchheim_positions(&roots, &directed, &depth);
    let mut minx = f64::INFINITY;
    let mut maxx = f64::NEG_INFINITY;
    let mut miny = f64::INFINITY;
    let mut maxy = f64::NEG_INFINITY;
    for (_, (x, y)) in &pos {
        minx = minx.min(*x);
        maxx = maxx.max(*x);
        miny = miny.min(*y);
        maxy = maxy.max(*y);
    }
    let cx = (minx + maxx) * 0.5;
    let cy = (miny + maxy) * 0.5;
    let gx = opts.center_x.unwrap_or(0.0);
    let gy = opts.center_y.unwrap_or(0.0);
    let dx = gx - cx * opts.sibling_gap;
    let dy = gy - cy * opts.layer_spacing;
    for node in nodes.iter_mut() {
        let Some(obj) = node.as_object_mut() else {
            continue;
        };
        let Some(nid) = obj.get("id").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some((bx, by)) = pos.get(nid) else {
            continue;
        };
        obj.insert("x".into(), serde_json::json!(bx * opts.sibling_gap + dx));
        obj.insert("y".into(), serde_json::json!(by * opts.layer_spacing + dy));
    }
    Ok(())
}
// #endregion 🔖Layout

// #region 🔖GraphExtension
/// 🧩 DAG-specific graph extension marker.
pub struct DagExtension;

impl cavas::CanvasExtension for DagExtension {
    fn extension_id(&self) -> &str {
        "dag"
    }
}

impl GraphExtension for DagExtension {}
// #endregion 🔖GraphExtension

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn io_node_handle_angles_left_right() {
        let (in_a, out_a) = io_node_handle_angles(0, 2, 0, 1);
        assert!(in_a > std::f64::consts::FRAC_PI_2);
        assert!(out_a.abs() < std::f64::consts::FRAC_PI_2);
    }

    #[test]
    fn cycle_detection_blocks_back_edge() {
        let edges = vec![("a".into(), "b".into()), ("b".into(), "c".into())];
        assert!(would_create_cycle(&edges, "c", "a"));
        assert!(!would_create_cycle(&edges, "a", "c"));
    }

    #[test]
    fn dag_layout_moves_nodes() {
        let mut fixture: Value = serde_json::json!({
            "schema": "dag.fixture/v1",
            "nodes": [
                {"id": "a", "x": 0, "y": 0, "handles": []},
                {"id": "b", "x": 0, "y": 0, "handles": []}
            ],
            "edges": [{"id": "e1", "source": "a", "target": "b"}]
        });
        apply_dag_layout_to_fixture_v1_value(&mut fixture, &DagLayoutOptions::default()).unwrap();
        let a_y = fixture["nodes"][0]["y"].as_f64().unwrap();
        let b_y = fixture["nodes"][1]["y"].as_f64().unwrap();
        assert!((b_y - a_y).abs() > 1.0);
    }
}
// #endregion 🔖Tests
