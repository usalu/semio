//! 🌳 Directed acyclic port graph: rectangle IO nodes on infinite canvas.

use std::cell::Cell;

use serde::{Deserialize, Serialize};

pub use infinite_cavas as cavas;
pub use mathematical_graph_port_directed::{
    self as graph, compute_edge_bezier_points, DirectedPortGraphEngine, Edge, EdgeId, GraphExtension, Handle, HandleId, HandleRole, InteractionMode, Node, NodeId, RenderSnapshot, Selection,
    VelloThemePalette,
};
use graph::BoardEvent;

/// 🌳 DAG board engine alias.
pub type DagBoardEngine = DirectedPortGraphEngine;

// #region 🔖IoNode
const EMPTY_PORTS: &[IoPortSpec] = &[];

fn default_node_width() -> f64 {
    72.0
}

fn default_node_height() -> f64 {
    DAG_CHANNEL_ROW_HEIGHT
}

/// 📏 Fixed height of one input or output channel row on computation nodes.
pub const DAG_CHANNEL_ROW_HEIGHT: f64 = 14.0;

/// 📛 Reserved title row above computation IO channels.
const DAG_COMPUTATION_HEADER_ROWS: usize = 0;

const DAG_NODE_EDGE_INSET: f64 = 4.0;
const DAG_NODE_COLUMN_GAP: f64 = 3.0;
const DAG_IO_WIDGET_HEIGHT: f64 = 28.0;
const DAG_LABEL_SCREEN_PX: f64 = 11.0;
const DAG_LABEL_COMPACT_SCREEN_PX: f64 = 10.0;

/// 🔢 Row count for a computation node body from its IO and variadic flags.
pub fn computation_io_row_count(input_count: usize, output_count: usize, variadic_inputs: bool, variadic_outputs: bool) -> usize {
    let input_rows = input_count + usize::from(variadic_inputs);
    let output_rows = output_count + usize::from(variadic_outputs);
    input_rows.max(output_rows).max(1)
}

/// 📐 Computation node height from channel row count.
pub fn computation_node_height(input_count: usize, output_count: usize, variadic_inputs: bool, variadic_outputs: bool) -> f64 {
    (computation_io_row_count(input_count, output_count, variadic_inputs, variadic_outputs) + DAG_COMPUTATION_HEADER_ROWS) as f64
        * DAG_CHANNEL_ROW_HEIGHT
}

/// 📐 Computation node width from IO labels and the centered name.
pub fn computation_node_width(name: &str, inputs: &[IoPortSpec], outputs: &[IoPortSpec]) -> f64 {
    use cavas::text::label_extent;
    let px = DAG_LABEL_SCREEN_PX;
    let (name_w, _) = label_extent(name, px);
    let left_w = inputs.iter().map(|port| label_extent(&port.label, px).0).fold(0.0, f64::max);
    let right_w = outputs.iter().map(|port| label_extent(&port.label, px).0).fold(0.0, f64::max);
    let content = left_w + DAG_NODE_COLUMN_GAP + name_w + DAG_NODE_COLUMN_GAP + right_w;
    (content + DAG_NODE_EDGE_INSET * 2.0).max(40.0)
}

/// 📐 IO widget width from vertically rotated title metrics.
pub fn io_widget_width(name: &str) -> f64 {
    use cavas::text::label_extent;
    let name_px = DAG_LABEL_SCREEN_PX * 1.05;
    let (_, label_h) = label_extent(name, name_px);
    (label_h + DAG_NODE_EDGE_INSET * 2.0 + 6.0).max(32.0)
}

/// 📐 IO widget height from vertically rotated title metrics plus a control band.
pub fn io_widget_height(name: &str) -> f64 {
    use cavas::text::label_extent;
    let name_px = DAG_LABEL_SCREEN_PX * 1.05;
    let (label_w, _) = label_extent(name, name_px);
    (label_w + DAG_IO_WIDGET_HEIGHT + DAG_NODE_EDGE_INSET * 2.0).max(40.0)
}

fn io_widget_label_center(node: &DagNodeSpec) -> (f64, f64) {
    let hh = node.height * 0.5;
    (node.x, node.y - hh * 0.2)
}

fn channel_row_center_y(node_y: f64, node_height: f64, row_index: usize) -> f64 {
    let hh = node_height * 0.5;
    node_y - hh + (row_index as f64 + 0.5) * DAG_CHANNEL_ROW_HEIGHT
}

fn channel_row_bounds(node: &DagNodeSpec, row_index: usize) -> (f64, f64, f64, f64) {
    let hw = node.width * 0.5;
    let y_center = channel_row_center_y(node.y, node.height, row_index);
    let half = DAG_CHANNEL_ROW_HEIGHT * 0.5;
    (node.x - hw, y_center - half, node.x + hw, y_center + half)
}

fn computation_port_center_y(node: &DagNodeSpec, port_index: usize) -> f64 {
    channel_row_center_y(node.y, node.height, port_index + DAG_COMPUTATION_HEADER_ROWS)
}

fn proportional_port_center_y(node: &DagNodeSpec, port_index: usize, count: usize) -> f64 {
    let hh = node.height * 0.5;
    let t = (port_index as f64 + 0.5) / count.max(1) as f64;
    node.y - hh + t * node.height
}

fn port_center_y(node: &DagNodeSpec, port_index: usize, count: usize) -> f64 {
    if matches!(node.kind, DagNodeKind::Computation { .. }) {
        computation_port_center_y(node, port_index)
    } else {
        proportional_port_center_y(node, port_index, count)
    }
}

/// 🪝 Named horizontal port on a DAG node edge.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IoPortSpec {
    pub id: String,
    pub label: String,
}

/// 🖼 Screen media payload for output nodes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagMedia {
    pub kind: DagMediaKind,
    pub src: String,
}

/// 🎬 Screen media kind discriminator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DagMediaKind {
    Image,
    Svg,
    Pdf,
    Video,
}

/// 🧩 Tagged node kind: computation, slider, select, or screen.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DagNodeKind {
    Computation {
        inputs: Vec<IoPortSpec>,
        outputs: Vec<IoPortSpec>,
        #[serde(default)]
        variadic_inputs: bool,
        #[serde(default)]
        variadic_outputs: bool,
    },
    Slider {
        min: f64,
        max: f64,
        step: f64,
        value: f64,
        output: IoPortSpec,
    },
    Select {
        options: Vec<String>,
        #[serde(default)]
        selected: usize,
        output: IoPortSpec,
    },
    Screen {
        #[serde(default)]
        media: Option<DagMedia>,
        input: IoPortSpec,
    },
    Note {
        text: String,
        output: IoPortSpec,
    },
    Preview {
        text: String,
        input: IoPortSpec,
    },
    Action {
        label: String,
        input: IoPortSpec,
    },
}

/// 📦 DAG node with shared layout fields and a tagged kind.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagNodeSpec {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "default_node_width")]
    pub width: f64,
    #[serde(default = "default_node_height")]
    pub height: f64,
    #[serde(flatten)]
    pub kind: DagNodeKind,
}

impl DagNodeSpec {
    /// 🔧 Builds a computation node with explicit IO ports.
    pub fn computation(
        id: String,
        name: String,
        inputs: Vec<IoPortSpec>,
        outputs: Vec<IoPortSpec>,
        variadic_inputs: bool,
        variadic_outputs: bool,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Self {
        Self {
            id,
            name,
            x,
            y,
            width,
            height,
            kind: DagNodeKind::Computation {
                inputs,
                outputs,
                variadic_inputs,
                variadic_outputs,
            },
        }
    }

    /// ➕ Whether the node exposes variadic input insert controls.
    pub fn variadic_inputs(&self) -> bool {
        match &self.kind {
            DagNodeKind::Computation { variadic_inputs, .. } => *variadic_inputs,
            _ => false,
        }
    }

    /// ⬅ Effective input ports for the node kind.
    pub fn inputs(&self) -> &[IoPortSpec] {
        match &self.kind {
            DagNodeKind::Computation { inputs, .. } => inputs,
            DagNodeKind::Screen { input, .. } | DagNodeKind::Preview { input, .. } | DagNodeKind::Action { input, .. } => {
                std::slice::from_ref(input)
            }
            _ => EMPTY_PORTS,
        }
    }

    /// ➡ Effective output ports for the node kind.
    pub fn outputs(&self) -> &[IoPortSpec] {
        match &self.kind {
            DagNodeKind::Computation { outputs, .. } => outputs,
            DagNodeKind::Slider { output, .. }
            | DagNodeKind::Select { output, .. }
            | DagNodeKind::Note { output, .. } => std::slice::from_ref(output),
            _ => EMPTY_PORTS,
        }
    }
}

fn point_in_rect(px: f64, py: f64, x0: f64, y0: f64, x1: f64, y1: f64) -> bool {
    px >= x0.min(x1) && px <= x0.max(x1) && py >= y0.min(y1) && py <= y0.max(y1)
}

/// 📍 World-space center of the draggable slider track.
pub fn slider_track_center(node: &DagNodeSpec) -> Option<(f64, f64)> {
    match &node.kind {
        DagNodeKind::Slider { .. } => {
            let (x0, y0, x1, y1) = slider_track_bounds(node);
            Some(((x0 + x1) * 0.5, (y0 + y1) * 0.5))
        }
        _ => None,
    }
}

fn slider_track_bounds(node: &DagNodeSpec) -> (f64, f64, f64, f64) {
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    let pad = DAG_NODE_EDGE_INSET;
    let track_y = node.y + hh - 8.0;
    let track_h = 6.0;
    (
        node.x - hw + pad,
        track_y - track_h * 0.5,
        node.x + hw - pad - 12.0,
        track_y + track_h * 0.5,
    )
}

fn select_control_bounds(node: &DagNodeSpec) -> (f64, f64, f64, f64) {
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    (node.x - hw + DAG_NODE_EDGE_INSET, node.y + hh * 0.12, node.x + hw - 10.0, node.y + hh - DAG_NODE_EDGE_INSET)
}

fn note_content_bounds(node: &DagNodeSpec) -> (f64, f64, f64, f64) {
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    let pad = DAG_NODE_EDGE_INSET;
    let top = node.y + hh * 0.08;
    (node.x - hw + pad, top, node.x + hw - pad, node.y + hh - pad)
}

fn preview_content_bounds(node: &DagNodeSpec) -> (f64, f64, f64, f64) {
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    let pad = DAG_NODE_EDGE_INSET;
    let top = node.y + hh * 0.08;
    (node.x - hw + pad, top, node.x + hw - pad, node.y + hh - pad)
}

fn action_control_bounds(node: &DagNodeSpec) -> (f64, f64, f64, f64) {
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    let pad = DAG_NODE_EDGE_INSET;
    (node.x - hw + pad, node.y + hh * 0.08, node.x + hw - pad, node.y + hh - pad)
}

fn set_slider_value_from_x(node: &mut DagNodeSpec, world_x: f64) -> Option<f64> {
    let (left, _, right, _) = slider_track_bounds(node);
    let DagNodeKind::Slider { min, max, step, value, .. } = &mut node.kind else {
        return None;
    };
    let span = (right - left).max(1e-6);
    let t = ((world_x - left) / span).clamp(0.0, 1.0);
    let raw = *min + t * (*max - *min);
    let stepped = if *step > 0.0 { (raw / *step).round() * *step } else { raw };
    *value = stepped.clamp(*min, *max);
    Some(*value)
}

fn advance_select_option(node: &mut DagNodeSpec) -> Option<String> {
    let DagNodeKind::Select { options, selected, .. } = &mut node.kind else {
        return None;
    };
    if options.is_empty() {
        return None;
    }
    *selected = (*selected + 1) % options.len();
    Some(options[*selected].clone())
}

/// 📐 Places input handles on the left and output handles on the right of a rectangle node.
pub fn io_node_handle_angles(input_index: usize, input_count: usize, output_index: usize, output_count: usize) -> (f64, f64) {
    let input_angle = port_angle_on_side(input_index, input_count.max(1), true);
    let output_angle = port_angle_on_side(output_index, output_count.max(1), false);
    (input_angle, output_angle)
}

const DAG_VARIADIC_PLUS_ZOOM_THRESHOLD: f64 = 1.5;

fn computation_input_label_x(node: &DagNodeSpec) -> f64 {
    node.x - node.width * 0.5 + DAG_NODE_EDGE_INSET
}

fn computation_output_label_x(node: &DagNodeSpec, label: &str, px: f64) -> f64 {
    use cavas::text::label_extent;
    let (label_w, _) = label_extent(label, px);
    node.x + node.width * 0.5 - DAG_NODE_EDGE_INSET - label_w
}

fn computation_name_column_bounds(node: &DagNodeSpec, px: f64) -> (f64, f64) {
    use cavas::text::label_extent;
    let hw = node.width * 0.5;
    let inputs = node.inputs();
    let outputs = node.outputs();
    let left_w = inputs.iter().map(|port| label_extent(&port.label, px).0).fold(0.0, f64::max);
    let right_w = outputs.iter().map(|port| label_extent(&port.label, px).0).fold(0.0, f64::max);
    let (name_w, _) = label_extent(&node.name, px * 1.05);
    let name_left = if inputs.is_empty() && outputs.is_empty() {
        node.x - name_w * 0.5
    } else if inputs.is_empty() {
        node.x + hw - DAG_NODE_EDGE_INSET - right_w - DAG_NODE_COLUMN_GAP - name_w
    } else {
        node.x - hw + DAG_NODE_EDGE_INSET + left_w + DAG_NODE_COLUMN_GAP
    };
    (name_left, name_left + name_w)
}

fn io_widget_name_column_bounds(node: &DagNodeSpec, px: f64) -> (f64, f64, f64, f64) {
    use cavas::text::label_extent;
    let hh = node.height * 0.5;
    let name_px = px * 1.05;
    let (name_w, name_h) = label_extent(&node.name, name_px);
    let visual_w = name_h;
    let top = node.y - hh + DAG_NODE_EDGE_INSET;
    let bottom = node.y + hh * 0.12;
    let x0 = node.x - visual_w * 0.5;
    let x1 = node.x + visual_w * 0.5;
    (x0, top, x1, bottom)
}

fn computation_channel_row_count(node: &DagNodeSpec) -> usize {
    let DagNodeKind::Computation {
        inputs,
        outputs,
        variadic_inputs,
        variadic_outputs,
    } = &node.kind
    else {
        return 0;
    };
    computation_io_row_count(inputs.len(), outputs.len(), *variadic_inputs, *variadic_outputs) + DAG_COMPUTATION_HEADER_ROWS
}

fn fit_node_size(node: &mut DagNodeSpec) {
    match &node.kind {
        DagNodeKind::Computation {
            inputs,
            outputs,
            variadic_inputs,
            variadic_outputs,
        } => {
            node.width = computation_node_width(&node.name, inputs, outputs);
            node.height = computation_node_height(inputs.len(), outputs.len(), *variadic_inputs, *variadic_outputs);
        }
        DagNodeKind::Slider { .. } | DagNodeKind::Select { .. } | DagNodeKind::Note { .. } | DagNodeKind::Preview { .. } | DagNodeKind::Action { .. } => {
            node.width = io_widget_width(&node.name);
            node.height = io_widget_height(&node.name);
        }
        DagNodeKind::Screen { .. } => {}
    }
}

fn variadic_input_insert_positions(node: &DagNodeSpec) -> Vec<(usize, f64, f64)> {
    let inputs = node.inputs();
    if !node.variadic_inputs() {
        return vec![];
    }
    let row = inputs.len();
    let port_y = computation_port_center_y(node, row);
    let port_x = computation_input_label_x(node);
    vec![(inputs.len(), port_x, port_y)]
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

/// 📐 Rectangle-layout port angle (north-zero CCW) aligned with painted IO labels.
pub fn io_node_rect_port_angle(x: f64, y: f64, width: f64, height: f64, index: usize, count: usize, left: bool) -> f64 {
    use cavas::vello::kurbo::Point;
    use graph::rectangle_handle_angle_toward;
    let hw = width * 0.5;
    let row_count = computation_io_row_count(count, count, false, false);
    let channel_rows = row_count + DAG_COMPUTATION_HEADER_ROWS;
    let port_y = if channel_rows * (DAG_CHANNEL_ROW_HEIGHT as usize) == height.round() as usize {
        channel_row_center_y(y, height, index + DAG_COMPUTATION_HEADER_ROWS)
    } else {
        let hh = height * 0.5;
        let t = (index as f64 + 0.5) / count.max(1) as f64;
        y - hh + t * height
    };
    let port_x = if left { x - hw } else { x + hw };
    rectangle_handle_angle_toward(Point::new(x, y), width, height, Point::new(port_x, port_y))
}

fn io_node_rect_port_angle_for_node(node: &DagNodeSpec, port_index: usize, left: bool) -> f64 {
    use cavas::vello::kurbo::Point;
    use graph::rectangle_handle_angle_toward;
    let hw = node.width * 0.5;
    let count = if left { node.inputs().len() } else { node.outputs().len() };
    let port_y = port_center_y(node, port_index, count);
    let port_x = if left { node.x - hw } else { node.x + hw };
    rectangle_handle_angle_toward(Point::new(node.x, node.y), node.width, node.height, Point::new(port_x, port_y))
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

/// 🧭 Tree layout flow direction for layered DAG positions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DagLayoutOrientation {
    #[default]
    LeftRight,
    TopBottom,
}

/// 🌲 Layered DAG layout options for fixture JSON.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagLayoutOptions {
    #[serde(default = "default_layer_spacing")]
    pub layer_spacing: f64,
    #[serde(default = "default_sibling_gap")]
    pub sibling_gap: f64,
    #[serde(default)]
    pub orientation: DagLayoutOrientation,
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

fn resolve_layout_node_id(handle_to_node: &HashMap<String, String>, key: &str, node_ids: &HashSet<String>) -> String {
    if let Some(nid) = handle_to_node.get(key) {
        return nid.clone();
    }
    if node_ids.contains(key) {
        return key.to_string();
    }
    if let Some(base) = key.split(':').next() {
        if node_ids.contains(base) {
            return base.to_string();
        }
    }
    key.to_string()
}

impl Default for DagLayoutOptions {
    fn default() -> Self {
        Self {
            layer_spacing: default_layer_spacing(),
            sibling_gap: default_sibling_gap(),
            orientation: DagLayoutOrientation::default(),
            center_x: None,
            center_y: None,
        }
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
        let u = resolve_layout_node_id(&handle_to_node, src_h, &node_ids);
        let v = resolve_layout_node_id(&handle_to_node, tgt_h, &node_ids);
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
    let (dx, dy) = match opts.orientation {
        DagLayoutOrientation::LeftRight => (gx - cy * opts.layer_spacing, gy - cx * opts.sibling_gap),
        DagLayoutOrientation::TopBottom => (gx - cx * opts.sibling_gap, gy - cy * opts.layer_spacing),
    };
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
        let (nx, ny) = match opts.orientation {
            DagLayoutOrientation::LeftRight => (by * opts.layer_spacing + dx, bx * opts.sibling_gap + dy),
            DagLayoutOrientation::TopBottom => (bx * opts.sibling_gap + dx, by * opts.layer_spacing + dy),
        };
        obj.insert("x".into(), serde_json::json!(nx));
        obj.insert("y".into(), serde_json::json!(ny));
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

// #region 🔖Lod
use cavas::lod::{Lod, LodScale};

const DAG_LODS: &[Lod; 6] = &[
    Lod {
        id: "minimap",
        name: "Minimap",
        description: "Whole-graph silhouette; fill only.",
        max_zoom: 0.15,
    },
    Lod {
        id: "overview",
        name: "Overview",
        description: "Horizontal node names; no sections or handles.",
        max_zoom: 0.35,
    },
    Lod {
        id: "compact",
        name: "Compact",
        description: "Dense graph with horizontal node names.",
        max_zoom: 0.55,
    },
    Lod {
        id: "normal",
        name: "Normal",
        description: "Three sections, vertical names, and handle dots.",
        max_zoom: 1.25,
    },
    Lod {
        id: "detail",
        name: "Detail",
        description: "Port labels and control value text.",
        max_zoom: 2.5,
    },
    Lod {
        id: "micro",
        name: "Micro",
        description: "Maximum node fidelity.",
        max_zoom: f64::INFINITY,
    },
];

const DAG_LOD_SCALE: LodScale = LodScale { lods: DAG_LODS };

/// 🔵 Port dot radius in screen pixels (world radius divides by camera zoom when painting).
const DAG_HANDLE_SCREEN_RADIUS_PX: f64 = 5.0;

const DAG_NODE_STROKE_SCREEN_PX: f64 = 1.5;
const DAG_NODE_STROKE_SELECTED_SCREEN_PX: f64 = 2.25;
const DAG_NODE_STROKE_HOVERED_SCREEN_PX: f64 = 2.0;
const DAG_EDGE_STROKE_SCREEN_PX: f64 = 2.0;
const DAG_CHROME_STROKE_SCREEN_PX: f64 = 1.25;

fn dag_world_stroke(screen_px: f64, zoom: f64) -> f64 {
    (screen_px / zoom.max(0.05)).max(1e-3)
}

fn dag_label_screen_px(lod: DagDrawLod) -> f64 {
    match lod {
        DagDrawLod::Compact => DAG_LABEL_COMPACT_SCREEN_PX,
        _ => DAG_LABEL_SCREEN_PX,
    }
}

/// 📶 Camera-zoom draw tier for DAG node chrome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagDrawLod {
    Minimap,
    Overview,
    Compact,
    Normal,
    Detail,
    Micro,
}

impl DagDrawLod {
    pub fn label(self) -> &'static str {
        match self {
            Self::Minimap => "minimap",
            Self::Overview => "overview",
            Self::Compact => "compact",
            Self::Normal => "normal",
            Self::Detail => "detail",
            Self::Micro => "micro",
        }
    }

    fn from_scale_index(index: usize) -> Self {
        match index {
            0 => Self::Minimap,
            1 => Self::Overview,
            2 => Self::Compact,
            3 => Self::Normal,
            4 => Self::Detail,
            _ => Self::Micro,
        }
    }

    pub fn name_is_horizontal(self) -> bool {
        matches!(self, Self::Overview | Self::Compact)
    }

    pub fn shows_name(self) -> bool {
        matches!(self, Self::Detail | Self::Micro)
    }

    pub fn shows_computation_layout(self) -> bool {
        matches!(self, Self::Normal | Self::Detail | Self::Micro)
    }

    pub fn shows_port_labels(self) -> bool {
        matches!(self, Self::Micro)
    }

    pub fn shows_handles(self) -> bool {
        matches!(self, Self::Normal | Self::Detail | Self::Micro)
    }

    pub fn shows_controls(self) -> bool {
        matches!(self, Self::Normal | Self::Detail | Self::Micro)
    }

    pub fn shows_detail_text(self) -> bool {
        matches!(self, Self::Detail | Self::Micro)
    }
}

/// 📶 Resolves the DAG draw LOD for a camera zoom factor.
pub fn dag_draw_lod(zoom: f64) -> DagDrawLod {
    DagDrawLod::from_scale_index(DAG_LOD_SCALE.resolve_index(zoom))
}
// #endregion 🔖Lod

fn dag_debug_log(msg: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&msg.into());
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("{msg}");
}

// #region 🔖DagHost

/// 🌳 Retained DAG host: typed nodes, edges, engine, camera.
pub struct DagHost {
    pub fixture: DagFixtureV1,
    pub engine: DagBoardEngine,
    pub vello_theme: VelloThemePalette,
    width: u32,
    height: u32,
    dpr: f64,
    last_screen_x: f64,
    last_screen_y: f64,
    node_id_map: HashMap<NodeId, usize>,
    handle_key_map: HashMap<HandleId, String>,
    edge_id_map: HashMap<EdgeId, String>,
    widget_drag: Option<usize>,
    pending_port_insert: Option<(String, usize)>,
    last_logged_lod: Cell<i8>,
    dimmed: HashSet<NodeId>,
    wheel_zoom_active: bool,
    wheel_zoom_render_lod: Option<DagDrawLod>,
}

fn vello_color_with_alpha(color: cavas::vello::peniko::Color, alpha: u8) -> cavas::vello::peniko::Color {
    use cavas::vello::peniko::Color;
    let rgba = color.to_rgba8();
    Color::from_rgba8(rgba.r, rgba.g, rgba.b, alpha)
}

/// 📦 `dag.fixture/v1` document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagFixtureV1 {
    pub schema: String,
    pub camera: DagCameraV1,
    pub nodes: Vec<DagNodeSpec>,
    pub edges: Vec<DagFixtureEdgeV1>,
}

/// 📷 Fixture camera snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagCameraV1 {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

/// 🔗 Edge between port handles.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagFixtureEdgeV1 {
    pub id: String,
    pub source: String,
    pub target: String,
}

impl Default for DagFixtureV1 {
    fn default() -> Self {
        serde_json::from_str(include_str!("fixture/demo.dag.json")).unwrap_or_else(|_| Self {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![],
            edges: vec![],
        })
    }
}

impl DagHost {
    pub fn default_demo() -> Self {
        Self::from_fixture(DagFixtureV1::default())
    }

    pub fn from_fixture(fixture: DagFixtureV1) -> Self {
        Self::from_fixture_with_layout(fixture, false)
    }

    /// 🌳 Builds a host without running auto-layout (preserves node positions).
    pub fn from_fixture_without_layout(fixture: DagFixtureV1) -> Self {
        Self::from_fixture(fixture)
    }

    fn from_fixture_with_layout(fixture: DagFixtureV1, apply_layout: bool) -> Self {
        let mut host = Self {
            fixture,
            engine: DagBoardEngine::new(),
            vello_theme: VelloThemePalette::default(),
            width: 1,
            height: 1,
            dpr: 1.0,
            last_screen_x: 0.0,
            last_screen_y: 0.0,
            node_id_map: HashMap::new(),
            handle_key_map: HashMap::new(),
            edge_id_map: HashMap::new(),
            widget_drag: None,
            pending_port_insert: None,
            last_logged_lod: Cell::new(-1),
            dimmed: HashSet::new(),
            wheel_zoom_active: false,
            wheel_zoom_render_lod: None,
        };
        host.rebuild_engine_with_layout(apply_layout);
        host
    }

    fn engine_node_id_for_index(&self, idx: usize) -> Option<NodeId> {
        self.node_id_map.iter().find(|(_, &i)| i == idx).map(|(&nid, _)| nid)
    }

    fn node_id_for_widget_id(&self, widget_id: &str) -> Option<NodeId> {
        let idx = self.fixture.nodes.iter().position(|n| n.id == widget_id)?;
        self.engine_node_id_for_index(idx)
    }

    fn widget_id_for_node_id(&self, node_id: NodeId) -> Option<String> {
        let idx = *self.node_id_map.get(&node_id)?;
        self.fixture.nodes.get(idx).map(|n| n.id.clone())
    }

    fn is_node_hovered(&self, node_id: NodeId) -> bool {
        match self.engine.hover {
            Some(hover) if hover == node_id => true,
            Some(hover) => self.engine.handles.get(&hover).is_some_and(|handle| handle.node_id == node_id),
            None => false,
        }
    }

    fn is_node_selected(&self, node_id: NodeId) -> bool {
        self.engine.selection.node_ids.contains(&node_id)
    }

    fn is_node_preselected(&self, node_id: NodeId) -> bool {
        self.engine.preselect.node_ids.contains(&node_id)
    }

    fn is_node_preselect_removed(&self, node_id: NodeId) -> bool {
        self.engine.preselect_removed.node_ids.contains(&node_id)
    }

    fn sync_camera_from_engine(&mut self) {
        let cam = self.engine.camera;
        self.fixture.camera = DagCameraV1 { x: cam.x, y: cam.y, zoom: cam.zoom };
    }

    /// 🎯 Selected fixture node ids from the engine selection snapshot.
    pub fn selected_node_ids(&self) -> Vec<String> {
        self.engine
            .selection
            .node_ids
            .iter()
            .filter_map(|&nid| self.widget_id_for_node_id(nid))
            .collect()
    }

    /// 🖱️ Hovered fixture node id when the pointer is over a node or its port handle.
    pub fn hovered_node_id(&self) -> Option<String> {
        let hover = self.engine.hover?;
        if self.node_id_map.contains_key(&hover) {
            return self.widget_id_for_node_id(hover);
        }
        let node_id = self.engine.handles.get(&hover).map(|handle| handle.node_id)?;
        self.widget_id_for_node_id(node_id)
    }

    /// ✅ Replaces node selection from fixture widget ids.
    pub fn set_selection(&mut self, widget_ids: &[String]) {
        self.engine.selection = Selection::default();
        for widget_id in widget_ids {
            if let Some(nid) = self.node_id_for_widget_id(widget_id) {
                self.engine.selection.node_ids.insert(nid);
            }
        }
        self.engine.preselect = Selection::default();
        self.engine.preselect_removed = Selection::default();
    }

    /// 🎯 Configures rectangle/lasso area-select behavior.
    pub fn set_selection_options(&mut self, method: &str, mode: &str, select_nodes: bool, select_handles: bool, select_edges: bool) {
        self.engine.set_selection_options(method, mode, select_nodes, select_handles, select_edges);
    }

    /// 🧿 Screen-space marquee overlay points for the shared selection overlay.
    pub fn selection_preview_points_json(&self) -> String {
        let points: Vec<[f64; 2]> = self
            .engine
            .selection_preview_points()
            .iter()
            .map(|p| [p.x, p.y])
            .collect();
        serde_json::to_string(&points).unwrap_or_else(|_| "[]".into())
    }

    pub fn selection_preview_crossing(&self) -> bool {
        self.engine.selection_preview_crossing()
    }

    /// 👁️ Preselected widget ids during an in-flight marquee gesture.
    pub fn preselect_widget_ids(&self) -> Vec<String> {
        self.engine
            .preselect
            .node_ids
            .iter()
            .filter_map(|&nid| self.widget_id_for_node_id(nid))
            .collect()
    }

    /// 👁️ Widget ids highlighted as marquee-exit candidates.
    pub fn preselect_removed_widget_ids(&self) -> Vec<String> {
        self.engine
            .preselect_removed
            .node_ids
            .iter()
            .filter_map(|&nid| self.widget_id_for_node_id(nid))
            .collect()
    }

    /// ↩️ Cancels an in-flight area select and restores the pre-drag selection.
    pub fn cancel_area_select(&mut self) -> bool {
        self.engine.cancel_area_select()
    }

    /// 🗑️ Deletes the current node selection from the fixture.
    pub fn delete_selected(&mut self) {
        let widget_ids = self.selected_node_ids();
        self.engine.delete_selection();
        self.fixture.nodes.retain(|node| !widget_ids.contains(&node.id));
        self.fixture.edges.retain(|edge| {
            !widget_ids.iter().any(|id| edge.source.starts_with(id.as_str()) || edge.target.starts_with(id.as_str()))
        });
        self.rebuild_engine();
    }

    /// ⌨️ Selects every fixture node id.
    pub fn select_all_node_ids(&self) -> Vec<String> {
        self.fixture.nodes.iter().map(|node| node.id.clone()).collect()
    }

    pub fn select_all(&mut self) {
        self.engine.select_all();
    }

    /// 🖱️ Sets hover to a fixture widget id, or clears hover.
    pub fn set_hover(&mut self, widget_id: Option<&str>) {
        let next = widget_id.and_then(|id| self.node_id_for_widget_id(id));
        if self.engine.hover != next {
            self.engine.hover = next;
        }
    }

    /// 🌫️ Marks preview-off nodes as dimmed on the canvas.
    pub fn set_dimmed(&mut self, widget_ids: &[String]) {
        self.dimmed.clear();
        for widget_id in widget_ids {
            if let Some(nid) = self.node_id_for_widget_id(widget_id) {
                self.dimmed.insert(nid);
            }
        }
    }

    /// 📋 Preview-off fixture node ids currently dimmed on the canvas.
    pub fn dimmed_node_ids(&self) -> Vec<String> {
        self.dimmed.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect()
    }

    /// ➕ Returns and clears a pending variadic input insert request from the last pointer down.
    pub fn take_pending_port_insert(&mut self) -> Option<(String, usize)> {
        self.pending_port_insert.take()
    }

    /// 🎯 Hit-tests variadic `+` controls; returns node id and insert index.
    pub fn port_insert_hit(&self, world_x: f64, world_y: f64, zoom: f64) -> Option<(String, usize)> {
        if zoom < DAG_VARIADIC_PLUS_ZOOM_THRESHOLD {
            return None;
        }
        for node in self.fixture.nodes.iter().rev() {
            if !node.variadic_inputs() {
                continue;
            }
            let inputs = node.inputs();
            let hw = node.width * 0.5;
            let row = inputs.len() + DAG_COMPUTATION_HEADER_ROWS;
            let (x0, y0, hit_x1, y1) = {
                let (x0, y0, _x1, y1) = channel_row_bounds(node, row);
                (x0, y0, node.x - hw * 0.5, y1)
            };
            if point_in_rect(world_x, world_y, x0, y0, hit_x1, y1) {
                return Some((node.id.clone(), inputs.len()));
            }
        }
        None
    }

    pub fn set_viewport(&mut self, width: u32, height: u32, dpr: f64) {
        self.width = width.max(1);
        self.height = height.max(1);
        self.dpr = dpr.max(1.0);
    }

    /// 🔍 Pins draw LOD while the wheel gesture is active so chrome does not flicker across bands.
    pub fn set_wheel_zoom_active(&mut self, active: bool) {
        if active && !self.wheel_zoom_active {
            self.wheel_zoom_render_lod = Some(dag_draw_lod(self.fixture.camera.zoom));
        } else if !active {
            self.wheel_zoom_render_lod = None;
        }
        self.wheel_zoom_active = active;
    }

    fn draw_lod_for_frame(&self) -> DagDrawLod {
        if self.wheel_zoom_active {
            if let Some(pinned) = self.wheel_zoom_render_lod {
                return pinned;
            }
        }
        dag_draw_lod(self.fixture.camera.zoom)
    }

    pub fn load_fixture_json(json: &str) -> Result<Self, String> {
        let fixture: DagFixtureV1 = serde_json::from_str(json).map_err(|e| e.to_string())?;
        if fixture.schema != "dag.fixture/v1" {
            return Err("schema must be dag.fixture/v1".into());
        }
        Ok(Self::from_fixture(fixture))
    }

    pub fn fixture_json(&self) -> Result<String, String> {
        serde_json::to_string(&self.fixture).map_err(|e| e.to_string())
    }

    /// 🌳 Recomputes node positions from the current graph using layered tree layout.
    pub fn reorganize(&mut self, opts: &DagLayoutOptions) -> Result<(), String> {
        let mut fixture_value = serde_json::to_value(&self.fixture).map_err(|e| e.to_string())?;
        apply_dag_layout_to_fixture_v1_value(&mut fixture_value, opts)?;
        self.fixture = serde_json::from_value(fixture_value).map_err(|e| e.to_string())?;
        self.rebuild_engine_with_layout(false);
        Ok(())
    }

    fn rebuild_engine(&mut self) {
        self.rebuild_engine_with_layout(true);
    }

    fn rebuild_engine_with_layout(&mut self, apply_layout: bool) {
        self.engine = DagBoardEngine::new();
        self.engine.enforce_acyclic = true;
        self.node_id_map.clear();
        self.handle_key_map.clear();
        self.edge_id_map.clear();
        for node in &mut self.fixture.nodes {
            fit_node_size(node);
        }
        let (cx, cy, zoom) = (self.fixture.camera.x, self.fixture.camera.y, self.fixture.camera.zoom);
        self.engine.set_camera(cx, cy, zoom);
        if apply_layout {
            let mut fixture_value = serde_json::to_value(&self.fixture).unwrap_or_else(|_| serde_json::json!({}));
            let _ = apply_dag_layout_to_fixture_v1_value(&mut fixture_value, &DagLayoutOptions::default());
            if let Ok(updated) = serde_json::from_value::<DagFixtureV1>(fixture_value.clone()) {
                self.fixture = updated;
            }
        }
        let mut next_node: u64 = 1;
        let mut next_handle: u64 = 10;
        let mut handle_map: HashMap<String, u64> = HashMap::new();
        for (idx, node) in self.fixture.nodes.iter().enumerate() {
            let nid = next_node;
            next_node += 1;
            self.node_id_map.insert(nid, idx);
            self.engine.create_rect_node(nid, node.x, node.y, node.width, node.height, true);
            let inputs = node.inputs();
            let outputs = node.outputs();
            for (port_idx, port) in inputs.iter().enumerate() {
                let in_a = io_node_rect_port_angle_for_node(node, port_idx, true);
                let hid = next_handle;
                next_handle += 1;
                let key = format!("{}:{}", node.id, port.id);
                handle_map.insert(key.clone(), hid);
                self.handle_key_map.insert(hid, key);
                self.engine.create_handle(hid, nid, in_a);
                self.engine.set_handle_role(hid, HandleRole::Target);
            }
            for (port_idx, port) in outputs.iter().enumerate() {
                let out_a = io_node_rect_port_angle_for_node(node, port_idx, false);
                let hid = next_handle;
                next_handle += 1;
                let key = format!("{}:{}", node.id, port.id);
                handle_map.insert(key.clone(), hid);
                self.handle_key_map.insert(hid, key);
                self.engine.create_handle(hid, nid, out_a);
                self.engine.set_handle_role(hid, HandleRole::Source);
            }
        }
        let existing: Vec<(String, String)> = self
            .fixture
            .edges
            .iter()
            .filter_map(|e| {
                let src = e.source.split(':').next()?.to_string();
                let tgt = e.target.split(':').next()?.to_string();
                Some((src, tgt))
            })
            .collect();
        let mut eid: u64 = 100;
        for edge in &self.fixture.edges {
            if would_create_cycle(&existing, edge.source.split(':').next().unwrap_or(""), edge.target.split(':').next().unwrap_or("")) {
                continue;
            }
            let src = handle_map.get(&edge.source).copied();
            let tgt = handle_map.get(&edge.target).copied();
            if let (Some(s), Some(t)) = (src, tgt) {
                let id = Self::parse_fixture_edge_numeric_id(&edge.id).unwrap_or(eid);
                eid = eid.max(id).saturating_add(1);
                self.engine.create_edge(id, s, t);
                self.edge_id_map.insert(id, edge.id.clone());
            }
        }
        self.engine.set_next_edge_id(eid);
    }

    fn parse_fixture_edge_numeric_id(id: &str) -> Option<u64> {
        id.strip_prefix('e').and_then(|s| s.parse().ok())
    }

    fn screen_to_world_point(&self, sx: f64, sy: f64) -> cavas::vello::kurbo::Point {
        use cavas::camera::{screen_to_world, Camera as CavasCamera, Viewport};
        use cavas::vello::kurbo::Point;
        let cam = CavasCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: self.width, height: self.height, dpr: self.dpr };
        screen_to_world(&cam, &viewport, Point::new(sx, sy))
    }

    fn sync_node_positions_from_engine(&mut self) {
        for (&nid, &idx) in &self.node_id_map {
            if let Some(node) = self.engine.nodes.get(&nid) {
                self.fixture.nodes[idx].x = node.center.x;
                self.fixture.nodes[idx].y = node.center.y;
            }
        }
    }

    fn node_spec_for_paint<'a>(&'a self, idx: usize, node: &'a DagNodeSpec) -> std::borrow::Cow<'a, DagNodeSpec> {
        let Some(nid) = self.engine_node_id_for_index(idx) else {
            return std::borrow::Cow::Borrowed(node);
        };
        let Some(engine_node) = self.engine.nodes.get(&nid) else {
            return std::borrow::Cow::Borrowed(node);
        };
        if (engine_node.center.x - node.x).abs() < 1e-9 && (engine_node.center.y - node.y).abs() < 1e-9 {
            return std::borrow::Cow::Borrowed(node);
        }
        let mut synced = node.clone();
        synced.x = engine_node.center.x;
        synced.y = engine_node.center.y;
        std::borrow::Cow::Owned(synced)
    }

    fn sync_edges_from_engine(&mut self) {
        let mut edges = Vec::with_capacity(self.engine.edges.len());
        for (eid, edge) in &self.engine.edges {
            let Some(source) = self.handle_key_map.get(&edge.source).cloned() else {
                continue;
            };
            let Some(target) = self.handle_key_map.get(&edge.target).cloned() else {
                continue;
            };
            let id = self.edge_id_map.get(eid).cloned().unwrap_or_else(|| format!("e{eid}"));
            self.edge_id_map.insert(*eid, id.clone());
            edges.push(DagFixtureEdgeV1 { id, source, target });
        }
        self.fixture.edges = edges;
    }

    fn process_engine_events(&mut self) {
        let events = self.engine.drain_events();
        let mut moved = false;
        let mut wired = false;
        for event in events {
            match event {
                BoardEvent::NodeMoved { id, x, y } => {
                    moved = true;
                    dag_debug_log(&format!("[DEBUG] dag node moved id={id} x={x:.1} y={y:.1}"));
                }
                BoardEvent::EdgeConnected { id, source, target } => {
                    wired = true;
                    dag_debug_log(&format!("[DEBUG] dag edge connected id={id} source={source} target={target}"));
                }
                BoardEvent::EdgeRemoved { id } => {
                    wired = true;
                    dag_debug_log(&format!("[DEBUG] dag edge removed id={id}"));
                }
                BoardEvent::SelectionChanged { node_ids, .. } => {
                    let ids: Vec<String> = node_ids.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect();
                    dag_debug_log(&format!("[DEBUG] dag selection changed: {}", ids.join(", ")));
                }
                BoardEvent::PreselectChanged { node_ids, removed_node_ids, .. } => {
                    let ids: Vec<String> = node_ids.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect();
                    let removed: Vec<String> = removed_node_ids.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect();
                    dag_debug_log(&format!("[DEBUG] dag preselect ids=[{}] removed=[{}]", ids.join(", "), removed.join(", ")));
                }
                BoardEvent::HoverChanged { id } => {
                    let label = id.and_then(|nid| self.widget_id_for_node_id(nid).or_else(|| {
                        self.engine.handles.get(&nid).and_then(|handle| self.widget_id_for_node_id(handle.node_id))
                    }));
                    dag_debug_log(&format!("[DEBUG] dag hover changed: {}", label.as_deref().unwrap_or("—")));
                }
            }
        }
        if moved {
            self.sync_node_positions_from_engine();
        }
        if wired {
            self.sync_edges_from_engine();
        }
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.fixture.camera = DagCameraV1 { x, y, zoom };
        self.engine.set_camera(x, y, zoom);
    }

    fn world_hits_handle(&self, world_x: f64, world_y: f64) -> bool {
        use cavas::vello::kurbo::Point;
        let p = Point::new(world_x, world_y);
        let snap = self.engine.render_snapshot();
        for (_, center, radius) in &snap.handles {
            let dx = p.x - center.x;
            let dy = p.y - center.y;
            let tol = radius + 6.0;
            if dx * dx + dy * dy <= tol * tol {
                return true;
            }
        }
        false
    }

    fn widget_hit_at(&self, world_x: f64, world_y: f64) -> Option<(usize, bool)> {
        for idx in (0..self.fixture.nodes.len()).rev() {
            let node = &self.fixture.nodes[idx];
            match &node.kind {
                DagNodeKind::Slider { .. } => {
                    let (x0, y0, x1, y1) = slider_track_bounds(node);
                    if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        return Some((idx, true));
                    }
                }
                DagNodeKind::Select { .. } => {
                    let (x0, y0, x1, y1) = select_control_bounds(node);
                    if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        return Some((idx, false));
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn try_widget_pointer_down(&mut self, world_x: f64, world_y: f64) -> bool {
        let Some((idx, is_slider)) = self.widget_hit_at(world_x, world_y) else {
            return false;
        };
        let node_id = self.fixture.nodes[idx].id.clone();
        if is_slider {
            self.widget_drag = Some(idx);
            if let Some(value) = set_slider_value_from_x(&mut self.fixture.nodes[idx], world_x) {
                dag_debug_log(&format!("[DEBUG] dag slider value id={node_id} value={value:.3}"));
            }
            return true;
        }
        if let Some(label) = advance_select_option(&mut self.fixture.nodes[idx]) {
            dag_debug_log(&format!("[DEBUG] dag select option id={node_id} label={label}"));
        }
        true
    }

    pub fn pointer_down(&mut self, x: f64, y: f64, extend: bool) {
        self.pointer_down_screen(x, y, 0, extend, false, false);
    }

    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.last_screen_x = sx;
        self.last_screen_y = sy;
        let world = self.screen_to_world_point(sx, sy);
        if let Some(hit) = self.port_insert_hit(world.x, world.y, self.fixture.camera.zoom) {
            self.pending_port_insert = Some(hit);
            return;
        }
        if !self.world_hits_handle(world.x, world.y) && self.try_widget_pointer_down(world.x, world.y) {
            return;
        }
        self.engine.pointer_down_screen(sx, sy, world.x, world.y, button, shift, ctrl_or_meta, alt);
        self.process_engine_events();
        self.sync_camera_from_engine();
    }

    pub fn pointer_move(&mut self, x: f64, y: f64) {
        self.pointer_move_screen(x, y, false, false, false);
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.last_screen_x = sx;
        self.last_screen_y = sy;
        let world = self.screen_to_world_point(sx, sy);
        if let Some(idx) = self.widget_drag {
            if let Some(value) = set_slider_value_from_x(&mut self.fixture.nodes[idx], world.x) {
                dag_debug_log(&format!("[DEBUG] dag slider value id={} value={value:.3}", self.fixture.nodes[idx].id));
            }
            return;
        }
        self.engine.pointer_move_screen(sx, sy, world.x, world.y, shift, ctrl_or_meta, alt);
        if matches!(
            self.engine.interaction,
            InteractionMode::DragNode { .. } | InteractionMode::DragNodes { .. }
        ) {
            self.sync_node_positions_from_engine();
        }
        self.process_engine_events();
        self.sync_camera_from_engine();
    }

    pub fn pointer_up(&mut self, x: f64, y: f64) {
        self.pointer_up_screen(x, y, false, false, false);
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.last_screen_x = sx;
        self.last_screen_y = sy;
        if self.widget_drag.take().is_some() {
            return;
        }
        let world = self.screen_to_world_point(sx, sy);
        self.engine.pointer_up_screen(sx, sy, world.x, world.y, shift, ctrl_or_meta, alt);
        self.process_engine_events();
        self.sync_node_positions_from_engine();
        self.sync_camera_from_engine();
    }

    pub fn set_vello_theme_from_json(&mut self, json: &str) -> Result<(), String> {
        self.vello_theme.merge_from_json(json)
    }

    /// 🖼 Screen-node overlay rects in CSS pixel space for DOM media layers.
    pub fn node_overlays_json(&self) -> Result<String, String> {
        use cavas::camera::{world_to_screen, Camera as CavasCamera, Viewport};
        use cavas::vello::kurbo::Point;
        let cam = CavasCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: self.width.max(1), height: self.height.max(1), dpr: self.dpr.max(1.0) };
        let mut overlays = Vec::new();
        for node in &self.fixture.nodes {
            let DagNodeKind::Screen { media: Some(media), .. } = &node.kind else {
                continue;
            };
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            let inset = 8.0 / cam.zoom.max(0.05);
            let top = node.y - hh + hh * 0.35;
            let bottom = node.y + hh - inset;
            let left = node.x - hw + inset;
            let right = node.x + hw - inset;
            let tl = world_to_screen(&cam, &viewport, Point::new(left, top));
            let br = world_to_screen(&cam, &viewport, Point::new(right, bottom));
            let media_kind = match media.kind {
                DagMediaKind::Image => "image",
                DagMediaKind::Svg => "svg",
                DagMediaKind::Pdf => "pdf",
                DagMediaKind::Video => "video",
            };
            overlays.push(serde_json::json!({
                "id": node.id,
                "mediaKind": media_kind,
                "src": media.src,
                "rect": { "x": tl.x, "y": tl.y, "w": (br.x - tl.x).max(1.0), "h": (br.y - tl.y).max(1.0) }
            }));
        }
        serde_json::to_string(&overlays).map_err(|e| e.to_string())
    }

    fn paint_variadic_plus_controls(
        scene: &mut cavas::vello::Scene,
        cam: &cavas::camera::Camera,
        viewport: &cavas::camera::Viewport,
        node: &DagNodeSpec,
        px: f64,
        fill: cavas::vello::peniko::Color,
        halo: cavas::vello::peniko::Color,
    ) {
        use cavas::camera::world_to_screen;
        use cavas::text::append_label;
        for (_, px_world, py_world) in variadic_input_insert_positions(node) {
            let screen = world_to_screen(cam, viewport, cavas::vello::kurbo::Point::new(px_world, py_world));
            append_label(scene, "+", screen, px * 0.95, fill, halo);
        }
    }

    fn paint_port_labels(
        scene: &mut cavas::vello::Scene,
        cam: &cavas::camera::Camera,
        viewport: &cavas::camera::Viewport,
        node: &DagNodeSpec,
        px: f64,
        label_fill: cavas::vello::peniko::Color,
        label_halo: cavas::vello::peniko::Color,
    ) {
        use cavas::camera::world_to_screen;
        use cavas::text::{append_label, label_extent};
        use cavas::vello::kurbo::Point;
        let hw = node.width * 0.5;
        let handle_inset = 8.0 / cam.zoom.max(0.05);
        let inputs = node.inputs();
        let outputs = node.outputs();
        let computation = matches!(node.kind, DagNodeKind::Computation { .. });
        for (i, port) in inputs.iter().enumerate() {
            let world_y = port_center_y(node, i, inputs.len());
            let world_x = if computation {
                computation_input_label_x(node)
            } else {
                node.x - hw + handle_inset
            };
            append_label(scene, &port.label, world_to_screen(cam, viewport, Point::new(world_x, world_y)), px, label_fill, label_halo);
        }
        for (i, port) in outputs.iter().enumerate() {
            let world_y = port_center_y(node, i, outputs.len());
            let world_x = if computation {
                computation_output_label_x(node, &port.label, px)
            } else {
                let (label_w, _) = label_extent(&port.label, px);
                node.x + hw - handle_inset - label_w
            };
            append_label(scene, &port.label, world_to_screen(cam, viewport, Point::new(world_x, world_y)), px, label_fill, label_halo);
        }
    }

    fn paint_node_name_vertical(
        scene: &mut cavas::vello::Scene,
        center_screen: cavas::vello::kurbo::Point,
        name: &str,
        px: f64,
        label_fill: cavas::vello::peniko::Color,
        label_halo: cavas::vello::peniko::Color,
    ) {
        use cavas::text::{append_label, label_extent};
        use cavas::vello::kurbo::{Affine, Point};
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return;
        }
        let name_px = px * 1.05;
        let (w, h) = label_extent(trimmed, name_px);
        let mut label_scene = cavas::vello::Scene::new();
        append_label(&mut label_scene, trimmed, Point::new(0.0, 0.0), name_px, label_fill, label_halo);
        let rot = Affine::translate((center_screen.x, center_screen.y))
            * Affine::rotate(-std::f64::consts::FRAC_PI_2)
            * Affine::translate((-w * 0.5, -h * 0.5));
        scene.append(&label_scene, Some(rot));
    }

    fn paint_node_name(
        scene: &mut cavas::vello::Scene,
        center_screen: cavas::vello::kurbo::Point,
        node: &DagNodeSpec,
        px: f64,
        label_fill: cavas::vello::peniko::Color,
        label_halo: cavas::vello::peniko::Color,
    ) {
        if node.width >= node.height {
            Self::paint_node_name_horizontal(scene, center_screen, &node.name, px, label_fill, label_halo);
        } else {
            Self::paint_node_name_vertical(scene, center_screen, &node.name, px, label_fill, label_halo);
        }
    }

    fn paint_computation_channel_borders(
        scene: &mut cavas::vello::Scene,
        aff: cavas::vello::kurbo::Affine,
        node: &DagNodeSpec,
        px: f64,
        chrome_stroke: f64,
        stroke: cavas::vello::peniko::Color,
    ) {
        use cavas::vello::kurbo::{Line, Point, Stroke};
        let DagNodeKind::Computation {
            inputs,
            outputs,
            variadic_inputs,
            variadic_outputs,
        } = &node.kind
        else {
            return;
        };
        let hw = node.width * 0.5;
        let hh = node.height * 0.5;
        let top = node.y - hh;
        let bottom = node.y + hh;
        let left = node.x - hw;
        let right = node.x + hw;
        let stroke_style = Stroke::new(chrome_stroke);
        let row_count = computation_io_row_count(inputs.len(), outputs.len(), *variadic_inputs, *variadic_outputs) + DAG_COMPUTATION_HEADER_ROWS;
        for boundary in 1..row_count {
            let y = top + boundary as f64 * DAG_CHANNEL_ROW_HEIGHT;
            scene.stroke(&stroke_style, aff, stroke, None, &Line::new(Point::new(left, y), Point::new(right, y)));
        }
        let (name_left, name_right) = computation_name_column_bounds(node, px);
        scene.stroke(&stroke_style, aff, stroke, None, &Line::new(Point::new(name_left, top), Point::new(name_left, bottom)));
        scene.stroke(&stroke_style, aff, stroke, None, &Line::new(Point::new(name_right, top), Point::new(name_right, bottom)));
    }

    fn paint_io_widget_channel_borders(
        scene: &mut cavas::vello::Scene,
        aff: cavas::vello::kurbo::Affine,
        node: &DagNodeSpec,
        px: f64,
        chrome_stroke: f64,
        stroke: cavas::vello::peniko::Color,
    ) {
        use cavas::vello::kurbo::{Line, Point, Stroke};
        let (name_left, top, name_right, bottom) = io_widget_name_column_bounds(node, px);
        let stroke_style = Stroke::new(chrome_stroke);
        scene.stroke(&stroke_style, aff, stroke, None, &Line::new(Point::new(name_left, top), Point::new(name_left, bottom)));
        scene.stroke(&stroke_style, aff, stroke, None, &Line::new(Point::new(name_right, top), Point::new(name_right, bottom)));
    }

    fn paint_computation_node_name(
        scene: &mut cavas::vello::Scene,
        cam: &cavas::camera::Camera,
        viewport: &cavas::camera::Viewport,
        node: &DagNodeSpec,
        px: f64,
        label_fill: cavas::vello::peniko::Color,
        label_halo: cavas::vello::peniko::Color,
    ) {
        use cavas::camera::world_to_screen;
        use cavas::vello::kurbo::Point;
        let anchor = world_to_screen(cam, viewport, Point::new(node.x, node.y));
        Self::paint_node_name_vertical(scene, anchor, &node.name, px, label_fill, label_halo);
    }

    fn paint_io_widget_name(
        scene: &mut cavas::vello::Scene,
        cam: &cavas::camera::Camera,
        viewport: &cavas::camera::Viewport,
        node: &DagNodeSpec,
        lod: DagDrawLod,
        px: f64,
        label_fill: cavas::vello::peniko::Color,
        label_halo: cavas::vello::peniko::Color,
    ) {
        use cavas::camera::world_to_screen;
        use cavas::vello::kurbo::Point;
        if !(lod.shows_name() || lod.shows_controls()) {
            return;
        }
        let (label_x, label_y) = io_widget_label_center(node);
        let name_anchor = world_to_screen(cam, viewport, Point::new(label_x, label_y));
        Self::paint_node_name_vertical(scene, name_anchor, &node.name, px, label_fill, label_halo);
    }

    fn paint_node_name_horizontal(
        scene: &mut cavas::vello::Scene,
        center_screen: cavas::vello::kurbo::Point,
        name: &str,
        px: f64,
        label_fill: cavas::vello::peniko::Color,
        label_halo: cavas::vello::peniko::Color,
    ) {
        use cavas::text::{append_label, label_extent};
        use cavas::vello::kurbo::Point;
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return;
        }
        let (w, h) = label_extent(trimmed, px);
        append_label(
            scene,
            trimmed,
            Point::new(center_screen.x - w * 0.5, center_screen.y - h * 0.5),
            px,
            label_fill,
            label_halo,
        );
    }

    /// 👻 Paints a translucent highlighted ghost node preview (ignores LOD).
    pub fn paint_ghost_node(&self, scene: &mut cavas::vello::Scene, node: &DagNodeSpec, viewport_w: u32, viewport_h: u32, dpr: f64) {
        use cavas::camera::{camera_content_affine, world_to_screen, Camera as CavasCamera, Viewport};
        use cavas::vello::kurbo::{Point, Rect, Stroke};
        use cavas::vello::peniko::Fill;

        let theme = &self.vello_theme;
        let cam = CavasCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: viewport_w.max(1), height: viewport_h.max(1), dpr: dpr.max(1.0) };
        let aff = camera_content_affine(&cam, &viewport);
        let accent = theme.wire_stroke_highlighted;
        let ghost_fill = vello_color_with_alpha(accent, 48);
        let label_fill = theme.node_stroke;
        let label_halo = vello_color_with_alpha(theme.raster_clear, 200);
        let hw = node.width * 0.5;
        let hh = node.height * 0.5;
        let rect = Rect::new(node.x - hw, node.y - hh, node.x + hw, node.y + hh);
        scene.fill(Fill::NonZero, aff, ghost_fill, None, &rect);
        scene.stroke(&Stroke::new(dag_world_stroke(DAG_NODE_STROKE_HOVERED_SCREEN_PX, cam.zoom)), aff, accent, None, &rect);
        let px = DAG_LABEL_SCREEN_PX;
        let center_screen = world_to_screen(&cam, &viewport, Point::new(node.x, node.y));
        if matches!(node.kind, DagNodeKind::Computation { .. }) {
            let chrome_stroke = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX, cam.zoom);
            Self::paint_computation_channel_borders(scene, aff, node, px, chrome_stroke, theme.node_stroke);
            Self::paint_computation_node_name(scene, &cam, &viewport, node, px, label_fill, label_halo);
            Self::paint_port_labels(scene, &cam, &viewport, node, px, label_fill, label_halo);
        } else {
            Self::paint_node_name_horizontal(scene, center_screen, &node.name, px, label_fill, label_halo);
        }
    }

    pub fn paint_scene(&self, scene: &mut cavas::vello::Scene, viewport_w: u32, viewport_h: u32, dpr: f64) {
        use cavas::camera::{camera_content_affine, world_to_screen, Camera as CavasCamera, Viewport};
        use cavas::text::append_label;
        use cavas::vello::kurbo::{Circle, Line, Point, Rect, Stroke};
        use cavas::vello::peniko::Fill;

        let theme = &self.vello_theme;
        let cam = CavasCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: viewport_w.max(1), height: viewport_h.max(1), dpr: dpr.max(1.0) };
        let aff = camera_content_affine(&cam, &viewport);
        let lod = self.draw_lod_for_frame();
        let lod_index = DAG_LOD_SCALE.resolve_index(cam.zoom) as i8;
        let prev_lod = self.last_logged_lod.get();
        if prev_lod != lod_index {
            self.last_logged_lod.set(lod_index);
            dag_debug_log(&format!("[DEBUG] dag draw lod={} zoom={:.3}", lod.label(), cam.zoom));
        }
        let snap = self.engine.render_snapshot();
        let edge_stroke = dag_world_stroke(DAG_EDGE_STROKE_SCREEN_PX, cam.zoom);
        for curve in &snap.edges {
            scene.stroke(&Stroke::new(edge_stroke), aff, theme.edge_stroke, None, curve);
        }
        if let Some((a, b)) = snap.pending_edge {
            let preview = compute_edge_bezier_points(a, b, a, b);
            scene.stroke(&Stroke::new(edge_stroke), aff, theme.edge_stroke_selected, None, &preview);
        }
        let node_stroke = theme.node_stroke;
        let node_fill = theme.node_fill;
        let label_fill = theme.node_stroke;
        let label_halo = vello_color_with_alpha(theme.raster_clear, 200);
        let accent = theme.wire_stroke_highlighted;
        for (idx, fixture_node) in self.fixture.nodes.iter().enumerate() {
            let node = self.node_spec_for_paint(idx, fixture_node);
            let node = node.as_ref();
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            let rect = Rect::new(node.x - hw, node.y - hh, node.x + hw, node.y + hh);
            let engine_nid = self.engine_node_id_for_index(idx);
            let is_dimmed = engine_nid.is_some_and(|nid| self.dimmed.contains(&nid));
            let is_selected = engine_nid.is_some_and(|nid| self.is_node_selected(nid));
            let is_hovered = engine_nid.is_some_and(|nid| self.is_node_hovered(nid));
            let (fill, stroke, stroke_screen_px) = if is_dimmed {
                (
                    vello_color_with_alpha(theme.node_fill_disabled, 120),
                    vello_color_with_alpha(node_stroke, 110),
                    1.0,
                )
            } else if is_selected {
                (theme.node_fill_selected, theme.node_stroke_selected, DAG_NODE_STROKE_SELECTED_SCREEN_PX)
            } else if is_hovered {
                (theme.node_fill_hovered, node_stroke, DAG_NODE_STROKE_HOVERED_SCREEN_PX)
            } else {
                (node_fill, node_stroke, DAG_NODE_STROKE_SCREEN_PX)
            };
            scene.fill(Fill::NonZero, aff, fill, None, &rect);
            if lod == DagDrawLod::Minimap {
                continue;
            }
            scene.stroke(&Stroke::new(dag_world_stroke(stroke_screen_px, cam.zoom)), aff, stroke, None, &rect);
            let px = dag_label_screen_px(lod);
            let chrome_stroke = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX, cam.zoom);
            let center_screen = world_to_screen(&cam, &viewport, Point::new(node.x, node.y));
            if lod.name_is_horizontal() {
                Self::paint_node_name_horizontal(scene, center_screen, &node.name, px, label_fill, label_halo);
                continue;
            }
            match &node.kind {
                DagNodeKind::Computation { variadic_inputs, .. } => {
                    if lod.shows_computation_layout() {
                        Self::paint_computation_channel_borders(scene, aff, node, px, chrome_stroke, theme.node_stroke);
                        Self::paint_computation_node_name(scene, &cam, &viewport, node, px, label_fill, label_halo);
                        Self::paint_port_labels(scene, &cam, &viewport, node, px, label_fill, label_halo);
                        if *variadic_inputs && cam.zoom >= DAG_VARIADIC_PLUS_ZOOM_THRESHOLD {
                            Self::paint_variadic_plus_controls(scene, &cam, &viewport, node, px, accent, label_halo);
                        }
                    }
                }
                DagNodeKind::Slider { min, max, value, .. } => {
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, aff, node, px, chrome_stroke, node_stroke);
                    }
                    Self::paint_io_widget_name(scene, &cam, &viewport, node, lod, px, label_fill, label_halo);
                    if lod.shows_controls() {
                        let (x0, y0, x1, y1) = slider_track_bounds(node);
                        let track_y = (y0 + y1) * 0.5;
                        let track = Line::new(Point::new(x0, track_y), Point::new(x1, track_y));
                        scene.stroke(&Stroke::new(chrome_stroke), aff, theme.edge_stroke, None, &track);
                        let span = (max - min).max(1e-6);
                        let t = ((*value - *min) / span).clamp(0.0, 1.0);
                        let thumb_x = x0 + t * (x1 - x0);
                        scene.fill(Fill::NonZero, aff, accent, None, &Circle::new(Point::new(thumb_x, track_y), 5.0 / cam.zoom.max(0.05)));
                    }
                    if lod.shows_detail_text() {
                        let value_text = format!("{value:.1}");
                        let (_, _, x1, _) = slider_track_bounds(node);
                        let value_pos = world_to_screen(&cam, &viewport, Point::new(x1 + 8.0 / cam.zoom.max(0.05), node.y + hh - 14.0));
                        append_label(scene, &value_text, value_pos, px * 0.9, label_fill, label_halo);
                    }
                    if lod.shows_port_labels() {
                        Self::paint_port_labels(scene, &cam, &viewport, node, px, label_fill, label_halo);
                    }
                }
                DagNodeKind::Select { options, selected, .. } => {
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, aff, node, px, chrome_stroke, node_stroke);
                    }
                    Self::paint_io_widget_name(scene, &cam, &viewport, node, lod, px, label_fill, label_halo);
                    if lod.shows_controls() {
                        let (cx0, cy0, cx1, cy1) = select_control_bounds(node);
                        let control = Rect::new(cx0, cy0, cx1, cy1);
                        scene.stroke(&Stroke::new(chrome_stroke), aff, theme.edge_stroke, None, &control);
                        if lod.shows_detail_text() {
                            let option = options.get(*selected).map(String::as_str).unwrap_or("—");
                            let option_pos = world_to_screen(&cam, &viewport, Point::new((cx0 + cx1) * 0.5, (cy0 + cy1) * 0.5));
                            append_label(scene, option, option_pos, px * 0.95, label_fill, label_halo);
                            let chevron = world_to_screen(&cam, &viewport, Point::new(cx1 - 6.0 / cam.zoom.max(0.05), (cy0 + cy1) * 0.5));
                            append_label(scene, "▾", chevron, px, label_fill, label_halo);
                        }
                    }
                    if lod.shows_port_labels() {
                        Self::paint_port_labels(scene, &cam, &viewport, node, px, label_fill, label_halo);
                    }
                }
                DagNodeKind::Screen { media, .. } => {
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, aff, node, px, chrome_stroke, node_stroke);
                    }
                    Self::paint_io_widget_name(scene, &cam, &viewport, node, lod, px, label_fill, label_halo);
                    if lod.shows_controls() {
                        let inset = 8.0 / cam.zoom.max(0.05);
                        let frame = Rect::new(node.x - hw + inset, node.y - hh + hh * 0.35, node.x + hw - inset, node.y + hh - inset);
                        scene.stroke(&Stroke::new(chrome_stroke), aff, theme.edge_stroke_selection_exit, None, &frame);
                    }
                    if lod.shows_detail_text() {
                        if let Some(media) = media {
                            let kind_label = match media.kind {
                                DagMediaKind::Image => "image",
                                DagMediaKind::Svg => "svg",
                                DagMediaKind::Pdf => "pdf",
                                DagMediaKind::Video => "video",
                            };
                            let hint = world_to_screen(&cam, &viewport, Point::new(node.x, node.y + hh * 0.1));
                            append_label(scene, kind_label, hint, px * 0.85, vello_color_with_alpha(label_fill, 140), label_halo);
                        }
                    }
                    if lod.shows_port_labels() {
                        Self::paint_port_labels(scene, &cam, &viewport, node, px, label_fill, label_halo);
                    }
                }
                DagNodeKind::Note { text, .. } => {
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, aff, node, px, chrome_stroke, node_stroke);
                    }
                    Self::paint_io_widget_name(scene, &cam, &viewport, node, lod, px, label_fill, label_halo);
                    if lod.shows_controls() {
                        let (x0, y0, x1, y1) = note_content_bounds(node);
                        let frame = Rect::new(x0, y0, x1, y1);
                        scene.stroke(&Stroke::new(chrome_stroke), aff, theme.edge_stroke, None, &frame);
                    }
                    if lod.shows_detail_text() {
                        let display = if text.is_empty() { "Note…" } else { text.as_str() };
                        let pos = world_to_screen(&cam, &viewport, Point::new(node.x, node.y + hh * 0.12));
                        append_label(scene, display, pos, px * 0.95, label_fill, label_halo);
                    }
                    if lod.shows_port_labels() {
                        Self::paint_port_labels(scene, &cam, &viewport, node, px, label_fill, label_halo);
                    }
                }
                DagNodeKind::Preview { text, .. } => {
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, aff, node, px, chrome_stroke, node_stroke);
                    }
                    Self::paint_io_widget_name(scene, &cam, &viewport, node, lod, px, label_fill, label_halo);
                    if lod.shows_controls() {
                        let (x0, y0, x1, y1) = preview_content_bounds(node);
                        let frame = Rect::new(x0, y0, x1, y1);
                        scene.stroke(&Stroke::new(chrome_stroke), aff, theme.edge_stroke_selection_exit, None, &frame);
                    }
                    if lod.shows_detail_text() || lod.shows_controls() {
                        let display = if text.is_empty() { "—" } else { text.as_str() };
                        let pos = world_to_screen(&cam, &viewport, Point::new(node.x, node.y + hh * 0.12));
                        append_label(scene, display, pos, px * 1.05, label_fill, label_halo);
                    }
                    if lod.shows_port_labels() {
                        Self::paint_port_labels(scene, &cam, &viewport, node, px, label_fill, label_halo);
                    }
                }
                DagNodeKind::Action { label, .. } => {
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, aff, node, px, chrome_stroke, node_stroke);
                    }
                    Self::paint_io_widget_name(scene, &cam, &viewport, node, lod, px, label_fill, label_halo);
                    if lod.shows_controls() {
                        let (x0, y0, x1, y1) = action_control_bounds(node);
                        let control = Rect::new(x0, y0, x1, y1);
                        scene.stroke(&Stroke::new(chrome_stroke), aff, theme.edge_stroke, None, &control);
                        if lod.shows_detail_text() {
                            let pos = world_to_screen(&cam, &viewport, Point::new(node.x, node.y));
                            append_label(scene, label, pos, px * 0.95, label_fill, label_halo);
                        }
                    }
                    if lod.shows_port_labels() {
                        Self::paint_port_labels(scene, &cam, &viewport, node, px, label_fill, label_halo);
                    }
                }
            }
        }
        if lod.shows_handles() {
            let handle_world_r = DAG_HANDLE_SCREEN_RADIUS_PX / cam.zoom.max(0.05);
            for (hid, center, _radius) in &snap.handles {
                let fill = match self.engine.handles.get(hid).map(|h| h.role) {
                    Some(HandleRole::Source) => theme.wire_stroke_highlighted,
                    Some(HandleRole::Target) => theme.edge_stroke_selection_exit,
                    _ => theme.edge_stroke,
                };
                scene.fill(Fill::NonZero, aff, fill, None, &Circle::new(*center, handle_world_r));
            }
        }
    }
}
// #endregion 🔖DagHost

// #region 🔖WasmSession
#[cfg(target_arch = "wasm32")]
mod wasm_session {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::future_to_promise;
    use web_sys::HtmlCanvasElement;

    struct DagSessionInner {
        host: DagHost,
        gpu: cavas::gpu_session::CanvasGpuSession,
        width: u32,
        height: u32,
        dpr: f64,
    }

    #[wasm_bindgen]
    pub struct DagSession {
        state: Rc<RefCell<DagSessionInner>>,
    }

    #[wasm_bindgen]
    impl DagSession {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            Self { state: Rc::new(RefCell::new(DagSessionInner { host: DagHost::default_demo(), gpu: cavas::gpu_session::CanvasGpuSession::default(), width: 1, height: 1, dpr: 1.0 })) }
        }

        #[wasm_bindgen(js_name = loadFixtureJson)]
        pub fn load_fixture_json(&self, json: &str) -> Result<(), JsValue> {
            let host = DagHost::load_fixture_json(json).map_err(|e| JsValue::from_str(&e))?;
            self.state.borrow_mut().host = host;
            Ok(())
        }

        #[wasm_bindgen(js_name = fixtureJson)]
        pub fn fixture_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.fixture_json().map_err(|e| JsValue::from_str(&e))
        }

        #[wasm_bindgen(js_name = nodeOverlaysJson)]
        pub fn node_overlays_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.node_overlays_json().map_err(|e| JsValue::from_str(&e))
        }

        #[wasm_bindgen(js_name = attachCanvas)]
        pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> js_sys::Promise {
            let inner = self.state.clone();
            let lw = logical_w.max(1);
            let lh = logical_h.max(1);
            let dpr = dpr.max(1.0);
            let pw = ((lw as f64 * dpr).round() as u32).max(1);
            let ph = ((lh as f64 * dpr).round() as u32).max(1);
            future_to_promise(async move {
                let (render_ctx, renderer, surface) = cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph)
                    .await
                    .map_err(|err| JsValue::from_str(&err))?;
                let mut g = inner.borrow_mut();
                g.width = lw;
                g.height = lh;
                g.dpr = dpr;
                g.host.set_viewport(lw, lh, dpr);
                g.gpu.finish_attach(canvas, render_ctx, renderer, surface);
                Ok(JsValue::UNDEFINED)
            })
        }

        #[wasm_bindgen(js_name = gpuReady)]
        pub fn gpu_ready(&self) -> bool {
            self.state.borrow().gpu.gpu_ready()
        }

        #[wasm_bindgen(js_name = setSize)]
        pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
            let mut inner = self.state.borrow_mut();
            inner.width = width.max(1);
            inner.height = height.max(1);
            inner.dpr = dpr.max(1.0);
            let (w, h, d) = (inner.width, inner.height, inner.dpr);
            inner.host.set_viewport(w, h, d);
            let pw = ((w as f64 * d).round() as u32).max(1);
            let ph = ((h as f64 * d).round() as u32).max(1);
            inner.gpu.resize_surface(pw, ph);
        }

        #[wasm_bindgen(js_name = setCamera)]
        pub fn set_camera(&self, x: f64, y: f64, zoom: f64) {
            self.state.borrow_mut().host.set_camera(x, y, zoom);
        }

        #[wasm_bindgen(js_name = setWheelZoomActive)]
        pub fn set_wheel_zoom_active(&self, active: bool) {
            self.state.borrow_mut().host.set_wheel_zoom_active(active);
        }

        #[wasm_bindgen(js_name = pointerDown)]
        pub fn pointer_down(&self, x: f64, y: f64, extend: bool) {
            self.state.borrow_mut().host.pointer_down(x, y, extend);
        }

        #[wasm_bindgen(js_name = pointerMove)]
        pub fn pointer_move(&self, x: f64, y: f64) {
            self.state.borrow_mut().host.pointer_move(x, y);
        }

        #[wasm_bindgen(js_name = pointerUp)]
        pub fn pointer_up(&self, x: f64, y: f64) {
            self.state.borrow_mut().host.pointer_up(x, y);
        }

        #[wasm_bindgen(js_name = reorganize)]
        pub fn reorganize(&self, options_json: &str) -> Result<(), JsValue> {
            let opts = if options_json.trim().is_empty() {
                DagLayoutOptions::default()
            } else {
                serde_json::from_str(options_json).unwrap_or_default()
            };
            self.state.borrow_mut().host.reorganize(&opts).map_err(|e| JsValue::from_str(&e))
        }

        #[wasm_bindgen(js_name = setVelloThemeJson)]
        pub fn set_vello_theme_json(&mut self, json: &str) {
            let _ = self.state.borrow_mut().host.set_vello_theme_from_json(json);
        }

        #[wasm_bindgen(js_name = renderFrame)]
        pub fn render_frame(&self) -> Result<(), JsValue> {
            let mut inner = self.state.borrow_mut();
            let mut scene = cavas::vello::Scene::new();
            let clear = inner.host.vello_theme.raster_clear;
            inner.host.paint_scene(&mut scene, inner.width, inner.height, inner.dpr);
            let scene = cavas::render::scale_scene_for_device_pixel_ratio(scene, inner.dpr);
            inner.gpu.render_frame(&scene, clear)
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_session::DagSession;
// #endregion 🔖WasmSession

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
    fn dag_selection_hover_and_dimmed_map_widget_ids() {
        let fixture = DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                DagNodeSpec::computation(
                    "a".into(),
                    "A".into(),
                    vec![],
                    vec![IoPortSpec { id: "out".into(), label: "out".into() }],
                    false,
                    false,
                    0.0,
                    0.0,
                    160.0,
                    24.0,
                ),
                DagNodeSpec::computation(
                    "b".into(),
                    "B".into(),
                    vec![IoPortSpec { id: "in".into(), label: "in".into() }],
                    vec![IoPortSpec { id: "out".into(), label: "out".into() }],
                    false,
                    false,
                    200.0,
                    0.0,
                    160.0,
                    24.0,
                ),
            ],
            edges: vec![],
        };
        let mut host = DagHost::from_fixture(fixture);
        host.set_selection(&["a".to_string()]);
        assert_eq!(host.selected_node_ids(), vec!["a"]);
        host.set_hover(Some("b"));
        assert_eq!(host.hovered_node_id().as_deref(), Some("b"));
        host.set_dimmed(&["a".to_string()]);
        assert_eq!(host.dimmed_node_ids(), vec!["a"]);
    }

    #[test]
    fn cycle_detection_blocks_back_edge() {
        let edges = vec![("a".into(), "b".into()), ("b".into(), "c".into())];
        assert!(would_create_cycle(&edges, "c", "a"));
        assert!(!would_create_cycle(&edges, "a", "c"));
    }

    #[test]
    fn dag_layout_left_right_orders_depth_on_x() {
        let mut fixture: Value = serde_json::json!({
            "schema": "dag.fixture/v1",
            "nodes": [
                {"id": "a", "x": 0, "y": 0, "handles": []},
                {"id": "b", "x": 0, "y": 0, "handles": []}
            ],
            "edges": [{"id": "e1", "source": "a", "target": "b"}]
        });
        apply_dag_layout_to_fixture_v1_value(&mut fixture, &DagLayoutOptions::default()).unwrap();
        let a_x = fixture["nodes"][0]["x"].as_f64().unwrap();
        let b_x = fixture["nodes"][1]["x"].as_f64().unwrap();
        assert!(b_x > a_x + 1.0);
    }

    #[test]
    fn dag_layout_top_bottom_orders_depth_on_y() {
        let mut fixture: Value = serde_json::json!({
            "schema": "dag.fixture/v1",
            "nodes": [
                {"id": "a", "x": 0, "y": 0, "handles": []},
                {"id": "b", "x": 0, "y": 0, "handles": []}
            ],
            "edges": [{"id": "e1", "source": "a", "target": "b"}]
        });
        let opts = DagLayoutOptions { orientation: DagLayoutOrientation::TopBottom, ..DagLayoutOptions::default() };
        apply_dag_layout_to_fixture_v1_value(&mut fixture, &opts).unwrap();
        let a_y = fixture["nodes"][0]["y"].as_f64().unwrap();
        let b_y = fixture["nodes"][1]["y"].as_f64().unwrap();
        assert!(b_y > a_y + 1.0);
    }

    #[test]
    fn dag_layout_spacing_scales_coordinates() {
        let mut fixture: Value = serde_json::json!({
            "schema": "dag.fixture/v1",
            "nodes": [
                {"id": "a", "x": 0, "y": 0, "handles": []},
                {"id": "b", "x": 0, "y": 0, "handles": []}
            ],
            "edges": [{"id": "e1", "source": "a", "target": "b"}]
        });
        apply_dag_layout_to_fixture_v1_value(&mut fixture, &DagLayoutOptions::default()).unwrap();
        let default_gap = (fixture["nodes"][1]["x"].as_f64().unwrap() - fixture["nodes"][0]["x"].as_f64().unwrap()).abs();
        let mut wide: Value = fixture.clone();
        apply_dag_layout_to_fixture_v1_value(
            &mut wide,
            &DagLayoutOptions {
                layer_spacing: 240.0,
                sibling_gap: 80.0,
                ..DagLayoutOptions::default()
            },
        )
        .unwrap();
        let wide_gap = (wide["nodes"][1]["x"].as_f64().unwrap() - wide["nodes"][0]["x"].as_f64().unwrap()).abs();
        assert!(wide_gap > default_gap * 1.5);
    }

    #[test]
    fn dag_node_spec_serde_round_trip_kinds() {
        let nodes = vec![
            DagNodeSpec::computation(
                "c".into(),
                "C".into(),
                vec![IoPortSpec { id: "in".into(), label: "in".into() }],
                vec![IoPortSpec { id: "out".into(), label: "out".into() }],
                false,
                false,
                0.0,
                0.0,
                160.0,
                56.0,
            ),
            DagNodeSpec {
                id: "s".into(),
                name: "S".into(),
                x: 1.0,
                y: 2.0,
                width: 180.0,
                height: 80.0,
                kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 3.0, output: IoPortSpec { id: "out".into(), label: "value".into() } },
            },
            DagNodeSpec {
                id: "m".into(),
                name: "M".into(),
                x: 0.0,
                y: 0.0,
                width: 180.0,
                height: 80.0,
                kind: DagNodeKind::Select { options: vec!["A".into(), "B".into()], selected: 1, output: IoPortSpec { id: "out".into(), label: "mode".into() } },
            },
            DagNodeSpec {
                id: "p".into(),
                name: "P".into(),
                x: 0.0,
                y: 0.0,
                width: 200.0,
                height: 140.0,
                kind: DagNodeKind::Screen {
                    media: Some(DagMedia { kind: DagMediaKind::Svg, src: "data:image/svg+xml,test".into() }),
                    input: IoPortSpec { id: "in".into(), label: "result".into() },
                },
            },
        ];
        for node in nodes {
            let json = serde_json::to_string(&node).unwrap();
            let back: DagNodeSpec = serde_json::from_str(&json).unwrap();
            assert_eq!(node, back);
        }
    }

    #[test]
    fn dag_node_spec_port_accessors_per_kind() {
        let slider = DagNodeSpec {
            id: "s".into(),
            name: "S".into(),
            x: 0.0,
            y: 0.0,
            width: 180.0,
            height: 80.0,
            kind: DagNodeKind::Slider { min: 0.0, max: 1.0, step: 0.1, value: 0.5, output: IoPortSpec { id: "out".into(), label: "value".into() } },
        };
        assert!(slider.inputs().is_empty());
        assert_eq!(slider.outputs().len(), 1);
        let screen = DagNodeSpec {
            id: "p".into(),
            name: "P".into(),
            x: 0.0,
            y: 0.0,
            width: 200.0,
            height: 140.0,
            kind: DagNodeKind::Screen { media: None, input: IoPortSpec { id: "in".into(), label: "in".into() } },
        };
        assert_eq!(screen.inputs().len(), 1);
        assert!(screen.outputs().is_empty());
    }

    #[test]
    fn dag_host_reorganize_updates_engine_positions() {
        let mut host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                DagNodeSpec::computation("a".into(), "A".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into() }], false, false, 500.0, 500.0, 160.0, 56.0),
                DagNodeSpec::computation("b".into(), "B".into(), vec![IoPortSpec { id: "in".into(), label: "in".into() }], vec![], false, false, 500.0, 500.0, 160.0, 56.0),
            ],
            edges: vec![DagFixtureEdgeV1 { id: "e1".into(), source: "a:out".into(), target: "b:in".into() }],
        });
        host.reorganize(&DagLayoutOptions::default()).unwrap();
        let a = host.fixture.nodes.iter().find(|n| n.id == "a").expect("a");
        let b = host.fixture.nodes.iter().find(|n| n.id == "b").expect("b");
        assert!(b.x > a.x);
    }

    #[test]
    fn dag_host_loads_demo_fixture() {
        let host = DagHost::default_demo();
        assert_eq!(host.fixture.schema, "dag.fixture/v1");
        assert_eq!(host.fixture.nodes.len(), 5);
        assert_eq!(host.fixture.edges.len(), 4);
        assert!(!host.engine.render_snapshot().edges.is_empty());
    }

    #[test]
    fn slider_track_bounds_stay_inside_node_rect() {
        let node = DagNodeSpec {
            id: "slider".into(),
            name: "Amount".into(),
            x: 100.0,
            y: 50.0,
            width: 180.0,
            height: 48.0,
            kind: DagNodeKind::Slider {
                min: 0.0,
                max: 10.0,
                step: 0.5,
                value: 2.0,
                output: IoPortSpec { id: "out".into(), label: "value".into() },
            },
        };
        let hw = node.width * 0.5;
        let hh = node.height * 0.5;
        let (left, top, right, bottom) = slider_track_bounds(&node);
        assert!(left >= node.x - hw);
        assert!(right <= node.x + hw);
        assert!(top >= node.y - hh);
        assert!(bottom <= node.y + hh);
    }

    #[test]
    fn dag_host_slider_drag_mutates_value() {
        let mut host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec {
                id: "slider".into(),
                name: "Amount".into(),
                x: 0.0,
                y: 0.0,
                width: 180.0,
                height: 80.0,
                kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 2.0, output: IoPortSpec { id: "out".into(), label: "value".into() } },
            }],
            edges: vec![],
        });
        host.set_viewport(800, 600, 1.0);
        let (x0, y0, x1, y1) = slider_track_bounds(&host.fixture.nodes[0]);
        let mid_y = (y0 + y1) * 0.5;
        let (sx, sy) = world_to_screen_px(&host, cavas::vello::kurbo::Point::new((x0 + x1) * 0.5, mid_y));
        host.pointer_down(sx, sy, false);
        host.pointer_up(sx, sy);
        let DagNodeKind::Slider { value, .. } = host.fixture.nodes[0].kind else {
            panic!("expected slider");
        };
        assert!((value - 2.0).abs() > 0.1);
    }

    #[test]
    fn dag_host_select_click_advances_option() {
        let mut host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec {
                id: "mode".into(),
                name: "Mode".into(),
                x: 0.0,
                y: 0.0,
                width: 180.0,
                height: 80.0,
                kind: DagNodeKind::Select { options: vec!["Add".into(), "Multiply".into()], selected: 0, output: IoPortSpec { id: "out".into(), label: "mode".into() } },
            }],
            edges: vec![],
        });
        host.set_viewport(800, 600, 1.0);
        let (x0, y0, x1, y1) = select_control_bounds(&host.fixture.nodes[0]);
        let (sx, sy) = world_to_screen_px(&host, cavas::vello::kurbo::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
        host.pointer_down(sx, sy, false);
        let DagNodeKind::Select { selected, .. } = host.fixture.nodes[0].kind else {
            panic!("expected select");
        };
        assert_eq!(selected, 1);
    }

    #[test]
    fn dag_host_exports_screen_overlay_rect() {
        let host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec {
                id: "screen".into(),
                name: "Preview".into(),
                x: 100.0,
                y: 50.0,
                width: 200.0,
                height: 140.0,
                kind: DagNodeKind::Screen {
                    media: Some(DagMedia { kind: DagMediaKind::Svg, src: "data:image/svg+xml,test".into() }),
                    input: IoPortSpec { id: "in".into(), label: "result".into() },
                },
            }],
            edges: vec![],
        });
        let mut host = host;
        host.set_viewport(1280, 800, 1.0);
        let json = host.node_overlays_json().unwrap();
        let overlays: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(overlays.len(), 1);
        assert_eq!(overlays[0]["id"], "screen");
        assert_eq!(overlays[0]["mediaKind"], "svg");
        assert!(overlays[0]["rect"]["w"].as_f64().unwrap_or(0.0) > 10.0);
    }

    fn handle_world(host: &DagHost, port_key: &str) -> cavas::vello::kurbo::Point {
        let hid = host.handle_key_map.iter().find(|(_, key)| key.as_str() == port_key).map(|(id, _)| *id).expect("handle");
        host.engine
            .render_snapshot()
            .handles
            .iter()
            .find(|(id, _, _)| *id == hid)
            .map(|(_, p, _)| *p)
            .expect("handle pos")
    }

    fn world_to_screen_px(host: &DagHost, p: cavas::vello::kurbo::Point) -> (f64, f64) {
        (p.x + host.width as f64 * 0.5, p.y + host.height as f64 * 0.5)
    }

    #[test]
    fn dag_host_drags_node_in_world_space() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        let mut dragged = false;
        for (nid, node) in host.engine.nodes.clone() {
            let grab = cavas::vello::kurbo::Point::new(node.center.x - node.width * 0.4, node.center.y);
            let (sx, sy) = world_to_screen_px(&host, grab);
            host.pointer_down(sx, sy, false);
            if !matches!(host.engine.interaction, InteractionMode::DragNode { node_id, .. } if node_id == nid) {
                host.pointer_up(sx, sy);
                continue;
            }
            let before = host.engine.nodes.get(&nid).expect("node").center;
            host.pointer_move(sx + 40.0, sy + 30.0);
            let idx = *host.node_id_map.get(&nid).expect("fixture index");
            let fixture = &host.fixture.nodes[idx];
            let engine = host.engine.nodes.get(&nid).expect("node").center;
            assert!((fixture.x - engine.x).abs() < 1e-6, "fixture x should track engine during drag");
            assert!((fixture.y - engine.y).abs() < 1e-6, "fixture y should track engine during drag");
            host.pointer_up(sx + 40.0, sy + 30.0);
            let after = host.engine.nodes.get(&nid).expect("node").center;
            assert!((after.x - before.x).abs() > 1.0);
            assert!((after.y - before.y).abs() > 1.0);
            dragged = true;
            break;
        }
        assert!(dragged, "expected at least one draggable node hit via screen coordinates");
    }

    #[test]
    fn dag_host_reconnects_edge_endpoint() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        let in_w = handle_world(&host, "combine:b");
        let out_w = handle_world(&host, "scale:out");
        let (in_sx, in_sy) = world_to_screen_px(&host, in_w);
        let (out_sx, out_sy) = world_to_screen_px(&host, out_w);
        host.pointer_down(in_sx, in_sy, false);
        assert!(matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }));
        host.pointer_move(out_sx, out_sy);
        host.pointer_up(out_sx, out_sy);
        let e3 = host.fixture.edges.iter().find(|e| e.id == "e3").expect("e3");
        assert_eq!(e3.source, "scale:out");
    }

    #[test]
    fn variadic_plus_hit_maps_insert_index() {
        let inputs = vec![
            IoPortSpec { id: "0".into(), label: "0".into() },
            IoPortSpec { id: "1".into(), label: "1".into() },
        ];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into() }];
        let width = computation_node_width("dictionary.merge", &inputs, &outputs);
        let height = computation_node_height(2, 1, true, false);
        let host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 2.0 },
            nodes: vec![DagNodeSpec::computation(
                "merge".into(),
                "dictionary.merge".into(),
                inputs,
                outputs,
                true,
                false,
                0.0,
                0.0,
                width,
                height,
            )],
            edges: vec![],
        });
        let positions = variadic_input_insert_positions(&host.fixture.nodes[0]);
        assert_eq!(positions.len(), 1);
        let (_, px, py) = positions[0];
        let hit = host.port_insert_hit(px, py, 2.0).expect("hit");
        assert_eq!(hit.0, "merge");
        assert_eq!(hit.1, 2);
        assert!(host.port_insert_hit(px, py, 1.0).is_none());
    }

    #[test]
    fn computation_name_column_bounds_leave_visible_gutter() {
        let inputs = vec![
            IoPortSpec { id: "cornerA".into(), label: "cornerA".into() },
            IoPortSpec { id: "cornerB".into(), label: "cornerB".into() },
            IoPortSpec { id: "height".into(), label: "height".into() },
        ];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "geometry".into() }];
        let width = computation_node_width("brep.box", &inputs, &outputs);
        let height = computation_node_height(3, 1, false, false);
        let node = DagNodeSpec::computation("box".into(), "brep.box".into(), inputs, outputs, false, false, 0.0, 0.0, width, height);
        let (name_left, name_right) = computation_name_column_bounds(&node, DAG_LABEL_SCREEN_PX);
        assert!(name_left < name_right);
        let hw = width * 0.5;
        assert!(name_left > node.x - hw + 1.0);
        assert!(name_right < node.x + hw - 1.0);
    }

    #[test]
    fn io_widget_size_fits_vertical_title() {
        let width = io_widget_width("Amount");
        let height = io_widget_height("Amount");
        assert!(width < height, "control nodes should be taller than wide for vertical titles");
        assert!(height >= 40.0);
    }

    #[test]
    fn computation_channel_row_count_matches_io_rows() {
        let node = DagNodeSpec::computation(
            "box".into(),
            "brep.box".into(),
            vec![
                IoPortSpec { id: "cornerA".into(), label: "cornerA".into() },
                IoPortSpec { id: "cornerB".into(), label: "cornerB".into() },
                IoPortSpec { id: "height".into(), label: "height".into() },
            ],
            vec![IoPortSpec { id: "out".into(), label: "geometry".into() }],
            false,
            false,
            0.0,
            0.0,
            120.0,
            42.0,
        );
        assert_eq!(computation_channel_row_count(&node), 3);
    }

    #[test]
    fn computation_node_size_fits_io_labels() {
        let inputs = vec![
            IoPortSpec { id: "cornerA".into(), label: "cornerA".into() },
            IoPortSpec { id: "cornerB".into(), label: "cornerB".into() },
            IoPortSpec { id: "height".into(), label: "height".into() },
        ];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "geometry".into() }];
        let width = computation_node_width("brep.box", &inputs, &outputs);
        let height = computation_node_height(3, 1, false, false);
        assert!(height <= 42.0, "expected compact height, got {height}");
        assert!(height < 96.0, "expected shorter than legacy 4-row layout");
        assert!(width > 40.0, "expected content-fitted width, got {width}");
    }

    #[test]
    fn io_node_rect_port_angles_on_edges() {
        use cavas::vello::kurbo::Point;
        use graph::handle_position_on_rectangle;
        let inputs = vec![
            IoPortSpec { id: "a".into(), label: "a".into() },
            IoPortSpec { id: "b".into(), label: "b".into() },
        ];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into() }];
        let width = computation_node_width("node", &inputs, &outputs);
        let height = computation_node_height(2, 1, false, false);
        let hw = width * 0.5;
        let left = io_node_rect_port_angle(0.0, 0.0, width, height, 0, 2, true);
        let right = io_node_rect_port_angle(0.0, 0.0, width, height, 0, 1, false);
        let left_pos = handle_position_on_rectangle(Point::new(0.0, 0.0), width, height, left);
        let right_pos = handle_position_on_rectangle(Point::new(0.0, 0.0), width, height, right);
        assert!(left_pos.x < -hw + 1.0);
        assert!(right_pos.x > hw - 1.0);
        assert!(left_pos.y < 0.0);
        assert!(right_pos.y < 0.0);
    }

    #[test]
    fn dag_draw_lod_maps_zoom_to_puzzle2d_bands() {
        assert_eq!(dag_draw_lod(0.1), DagDrawLod::Minimap);
        assert_eq!(dag_draw_lod(0.3), DagDrawLod::Overview);
        assert_eq!(dag_draw_lod(0.5), DagDrawLod::Compact);
        assert_eq!(dag_draw_lod(1.0), DagDrawLod::Normal);
        assert_eq!(dag_draw_lod(2.0), DagDrawLod::Detail);
        assert_eq!(dag_draw_lod(5.0), DagDrawLod::Micro);
    }

    #[test]
    fn dag_draw_lod_progressive_disclosure_gates() {
        assert!(!DagDrawLod::Normal.shows_name());
        assert!(DagDrawLod::Detail.shows_name());
        assert!(DagDrawLod::Normal.shows_computation_layout());
        assert!(!DagDrawLod::Compact.shows_computation_layout());
        assert!(!DagDrawLod::Detail.shows_port_labels());
        assert!(DagDrawLod::Micro.shows_port_labels());
    }

    #[test]
    fn wheel_zoom_pins_draw_lod_until_gesture_ends() {
        let mut host = DagHost::default_demo();
        host.fixture.camera.zoom = 1.0;
        assert_eq!(host.draw_lod_for_frame(), DagDrawLod::Normal);
        host.set_wheel_zoom_active(true);
        host.fixture.camera.zoom = 0.3;
        assert_eq!(host.draw_lod_for_frame(), DagDrawLod::Normal);
        host.set_wheel_zoom_active(false);
        assert_eq!(host.draw_lod_for_frame(), DagDrawLod::Overview);
    }

    #[test]
    fn note_preview_action_port_accessors() {
        let note = DagNodeSpec {
            id: "note".into(),
            name: "Note".into(),
            x: 0.0,
            y: 0.0,
            width: 160.0,
            height: 48.0,
            kind: DagNodeKind::Note { text: "hi".into(), output: IoPortSpec { id: "out".into(), label: "out".into() } },
        };
        assert!(note.inputs().is_empty());
        assert_eq!(note.outputs().len(), 1);
        let preview = DagNodeSpec {
            id: "preview".into(),
            name: "Preview".into(),
            x: 0.0,
            y: 0.0,
            width: 120.0,
            height: 48.0,
            kind: DagNodeKind::Preview { text: "3".into(), input: IoPortSpec { id: "in".into(), label: "in".into() } },
        };
        assert_eq!(preview.inputs().len(), 1);
        assert!(preview.outputs().is_empty());
    }

    #[test]
    fn dag_paint_scene_smoke_at_overview_and_micro_zoom() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        let mut scene = cavas::vello::Scene::new();
        host.fixture.camera.zoom = 0.3;
        host.paint_scene(&mut scene, 1280, 800, 1.0);
        host.fixture.camera.zoom = 5.0;
        host.paint_scene(&mut scene, 1280, 800, 1.0);
    }
}
// #endregion 🔖Tests
