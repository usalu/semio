//! 🌳 Directed acyclic port graph: rectangle IO nodes on infinite canvas.

use std::cell::Cell;

use serde::{Deserialize, Serialize};

pub use infinite_cavas as cavas;
pub use mathematical_graph_port_directed::{
    self as graph, compute_edge_bezier_points, handle_exterior_cap_fill_path, handle_exterior_cap_stroke_path,
    handle_outward_at_node_rim,
    DirectedPortGraphEngine, Edge, EdgeId,
    GraphExtension, Handle, HandleId, HandleRole, InteractionMode, Node, NodeId, RenderSnapshot, Selection, VelloThemePalette,
};
use graph::{handle_position, world_box_from_points, BoardEvent, WorldBox};

/// 🌳 DAG board engine alias.
pub type DagBoardEngine = DirectedPortGraphEngine;

// #region 🔖IoNode
const EMPTY_PORTS: &[IoPortSpec] = &[];

/// @emoji 🔤 Converts spaced or dashed labels into PascalCase display text.
pub fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
            }
        })
        .collect()
}

/// @emoji 🏷️ Normalizes node display name and abbreviation to PascalCase.
pub fn normalize_node_display(name: &str, abbreviation: &str) -> (String, String) {
    (to_pascal_case(name), to_pascal_case(abbreviation))
}

fn default_node_width() -> f64 {
    72.0
}

fn default_node_height() -> f64 {
    DAG_CHANNEL_ROW_HEIGHT
}

/// 📏 Fixed height of one input or output channel row on computation nodes.
pub const DAG_CHANNEL_ROW_HEIGHT: f64 = ui_styling::metrics::dag::CHANNEL_ROW_HEIGHT;

/// 📛 Reserved title row above computation IO channels.
const DAG_COMPUTATION_HEADER_ROWS: usize = 0;

const DAG_NODE_EDGE_INSET: f64 = ui_styling::metrics::dag::NODE_EDGE_INSET;
const DAG_NODE_COLUMN_GAP: f64 = ui_styling::metrics::dag::NODE_COLUMN_GAP;
const DAG_IO_COLUMN_MIN: f64 = ui_styling::metrics::dag::IO_COLUMN_MIN;
const DAG_IO_COLUMN_MAX: f64 = ui_styling::metrics::dag::IO_COLUMN_MAX;
const DAG_IO_WIDGET_HEIGHT: f64 = ui_styling::metrics::dag::IO_WIDGET_HEIGHT;
const DAG_SLIDER_KNOB_SCREEN_PX: f64 = ui_styling::metrics::dag::SLIDER_KNOB_SCREEN_PX;
const DAG_LABEL_SCREEN_PX: f64 = ui_styling::metrics::label::DAG_DEFAULT_PX;
const DAG_LABEL_COMPACT_SCREEN_PX: f64 = ui_styling::metrics::label::DAG_COMPACT_PX;

enum ComputationChannelRowSide {
    Input,
    Output,
}

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

fn port_label_text_width(label: &str, px: f64) -> f64 {
    let trimmed = label.trim();
    if trimmed.is_empty() || px < 4.0 {
        return 0.0;
    }
    let pad = px * 0.28;
    trimmed.len() as f64 * px * 0.62 + pad * 2.0
}

fn io_port_column_width(ports: &[IoPortSpec], px: f64) -> f64 {
    if ports.is_empty() {
        return 0.0;
    }
    let text_w = ports.iter().map(|port| port_label_text_width(port.display_code(), px)).fold(0.0, f64::max);
    text_w.clamp(DAG_IO_COLUMN_MIN, DAG_IO_COLUMN_MAX)
}

/// 📐 Computation node width from IO label columns and the horizontal title above the body.
pub fn computation_node_width(name: &str, inputs: &[IoPortSpec], outputs: &[IoPortSpec]) -> f64 {
    use cavas::text::label_extent;
    let port_px = DAG_LABEL_COMPACT_SCREEN_PX;
    let left_w = io_port_column_width(inputs, port_px);
    let right_w = io_port_column_width(outputs, port_px);
    let io_w = match (inputs.is_empty(), outputs.is_empty()) {
        (true, true) => 0.0,
        (true, false) => right_w,
        (false, true) => left_w,
        (false, false) => left_w + DAG_NODE_COLUMN_GAP + right_w,
    };
    let (name_w, _) = label_extent(name, DAG_LABEL_SCREEN_PX);
    let content = io_w.max(name_w);
    (content + DAG_NODE_EDGE_INSET * 2.0).max(24.0)
}

/// 📐 IO widget width from vertically rotated title metrics.
pub fn io_widget_width(name: &str) -> f64 {
    use cavas::text::label_extent;
    let name_px = DAG_LABEL_SCREEN_PX * ui_styling::metrics::label::DAG_LABEL_SCALE_MULT;
    let (_, label_h) = label_extent(name, name_px);
    (label_h + DAG_NODE_EDGE_INSET * 2.0 + 2.0).max(24.0)
}

/// 📐 IO widget height from vertically rotated title metrics plus a control band.
pub fn io_widget_height(name: &str) -> f64 {
    use cavas::text::label_extent;
    let name_px = DAG_LABEL_SCREEN_PX * ui_styling::metrics::label::DAG_LABEL_SCALE_MULT;
    let (label_w, _) = label_extent(name, name_px);
    (label_w + DAG_IO_WIDGET_HEIGHT + DAG_NODE_EDGE_INSET * 2.0).max(40.0)
}

/// 📐 Slider track width aligned with computation nodes (both IO columns, name and value sit outside).
pub fn slider_widget_width(name: &str, output: &IoPortSpec) -> f64 {
    let input = IoPortSpec { id: "in".into(), label: "in".into() , ..Default::default() };
    computation_node_width(name, std::slice::from_ref(&input), std::slice::from_ref(output))
}

/// 📐 Slider track height — one computation channel row.
pub fn slider_widget_height() -> f64 {
    DAG_CHANNEL_ROW_HEIGHT
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

fn input_port_row_hit_bounds(node: &DagNodeSpec, port_index: usize) -> Option<(f64, f64, f64, f64)> {
    let inputs = node.inputs();
    if port_index >= inputs.len() {
        return None;
    }
    let (x0, x1) = if uses_computation_layout(&node.kind) {
        computation_channel_row_divider_x_span(node, ComputationChannelRowSide::Input)
    } else {
        let hw = node.width * 0.5;
        (node.x - hw, node.x)
    };
    let count = inputs.len().max(1);
    let y_center = port_center_y(node, port_index, count);
    let half = if uses_computation_layout(&node.kind) {
        DAG_CHANNEL_ROW_HEIGHT * 0.5
    } else {
        node.height / count as f64 * 0.5
    };
    Some((x0, y_center - half, x1, y_center + half))
}

fn output_port_row_hit_bounds(node: &DagNodeSpec, port_index: usize) -> Option<(f64, f64, f64, f64)> {
    let outputs = node.outputs();
    if port_index >= outputs.len() {
        return None;
    }
    let (x0, x1) = if uses_computation_layout(&node.kind) {
        computation_channel_row_divider_x_span(node, ComputationChannelRowSide::Output)
    } else {
        let hw = node.width * 0.5;
        (node.x, node.x + hw)
    };
    let count = outputs.len().max(1);
    let y_center = port_center_y(node, port_index, count);
    let half = if uses_computation_layout(&node.kind) {
        DAG_CHANNEL_ROW_HEIGHT * 0.5
    } else {
        node.height / count as f64 * 0.5
    };
    Some((x0, y_center - half, x1, y_center + half))
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
    if uses_computation_layout(&node.kind) {
        computation_port_center_y(node, port_index)
    } else {
        proportional_port_center_y(node, port_index, count)
    }
}

/// 🪝 Named horizontal port on a DAG node edge.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IoPortSpec {
    pub id: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub code: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub abbreviation: String,
    #[serde(rename = "fullName", default, skip_serializing_if = "String::is_empty")]
    pub full_name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
}

impl IoPortSpec {
    pub fn named(code: impl Into<String>, abbreviation: impl Into<String>, id: impl Into<String>, full_name: impl Into<String>) -> Self {
        let id = id.into();
        let abbreviation = abbreviation.into();
        Self {
            code: code.into(),
            abbreviation: abbreviation.clone(),
            label: abbreviation,
            id,
            full_name: full_name.into(),
            ..Default::default()
        }
    }

    pub fn simple(id: impl Into<String>, label: impl Into<String>) -> Self {
        let id = id.into();
        let label = label.into();
        let code = if id.len() <= 2 {
            id.to_uppercase()
        } else {
            id.chars().take(2).collect::<String>().to_uppercase()
        };
        let abbreviation = if label.len() <= 3 {
            label.clone()
        } else {
            label.chars().take(3).collect()
        };
        Self {
            id: id.clone(),
            label: label.clone(),
            code,
            abbreviation: abbreviation.clone(),
            full_name: label,
            ..Default::default()
        }
    }

    pub fn display_code(&self) -> &str {
        if !self.code.is_empty() {
            return self.code.as_str();
        }
        self.label.as_str()
    }
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
// #endregion 🔖Media

// #region 🔖PreviewContent
const DAG_PREVIEW_PAD: f64 = 4.0;
const DAG_PREVIEW_ROW_HEIGHT: f64 = 14.0;
const DAG_PREVIEW_TREE_INDENT: f64 = 12.0;
const DAG_PREVIEW_TOGGLE_WIDTH: f64 = 10.0;
const DAG_PREVIEW_MAX_IMAGE: f64 = 200.0;
const DAG_PREVIEW_MIN_SIZE: f64 = 20.0;

/// 👁️ Typed preview payload rendered inside a preview node.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "variant")]
pub enum DagPreviewContent {
    Empty,
    Scalar { text: String },
    Image { src: String },
    Tree { json: serde_json::Value },
}

impl Default for DagPreviewContent {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Clone, Debug)]
struct PreviewTreeRow {
    path: String,
    depth: usize,
    label: String,
    summary: String,
    has_children: bool,
    expanded: bool,
}

#[derive(Clone, Debug)]
struct PreviewTreeRowLayout {
    path: String,
    row_rect: (f64, f64, f64, f64),
}

enum WidgetPointerKind {
    SliderDrag,
    SelectClick,
    PreviewToggle(String),
    ClusterExplode,
}

fn preview_scalar_text_width(text: &str) -> f64 {
    let px = DAG_LABEL_SCREEN_PX * ui_styling::metrics::label::DAG_LABEL_SCALE_MULT;
    port_label_text_width(text, px).max(text.len() as f64 * px * 0.55)
}

fn preview_media_natural_size(src: &str) -> (f64, f64) {
    use cavas::icon_codec::{board_resolve_icon_kind, BoardResolvedIcon};
    match board_resolve_icon_kind(src, |_| None) {
        BoardResolvedIcon::RasterRgba8 { w, h, .. } => (f64::from(w), f64::from(h)),
        BoardResolvedIcon::SvgPlain(s) | BoardResolvedIcon::SvgThemed(s) => {
            if let Ok(tree) = cavas::usvg::Tree::from_str(&s, cavas::svg_icon_vello09::usvg_options_icons()) {
                let (_, _, bw, bh) = cavas::svg_icon_vello09::svg_icon_content_bounds(&tree);
                if bw > 0.0 && bh > 0.0 && bw.is_finite() && bh.is_finite() {
                    return (bw, bh);
                }
            }
            (64.0, 48.0)
        }
        BoardResolvedIcon::None => (64.0, 48.0),
    }
}

fn clamp_preview_image_size(w: f64, h: f64) -> (f64, f64) {
    let max = DAG_PREVIEW_MAX_IMAGE;
    if w <= max && h <= max {
        return (w.max(1.0), h.max(1.0));
    }
    let scale = (max / w).min(max / h);
    ((w * scale).max(1.0), (h * scale).max(1.0))
}

fn preview_tree_collapsed_summary(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Object(map) => format!("{{{} keys}}", map.len()),
        serde_json::Value::Array(arr) => format!("[{} items]", arr.len()),
        serde_json::Value::String(s) => format!("\"{s}\""),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => "null".into(),
    }
}

fn preview_tree_scalar_display(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => format!("\"{s}\""),
        v => v.to_string(),
    }
}

fn preview_tree_rows(json: &serde_json::Value, expanded: &BTreeSet<String>, path: &str, depth: usize) -> Vec<PreviewTreeRow> {
    let mut rows = Vec::new();
    match json {
        serde_json::Value::Object(map) => {
            for (key, val) in map {
                let row_path = if path.is_empty() { key.clone() } else { format!("{path}.{key}") };
                let has_children = matches!(val, serde_json::Value::Object(_) | serde_json::Value::Array(_));
                let is_expanded = expanded.contains(&row_path);
                let summary = if has_children {
                    preview_tree_collapsed_summary(val)
                } else {
                    preview_tree_scalar_display(val)
                };
                rows.push(PreviewTreeRow {
                    path: row_path.clone(),
                    depth,
                    label: key.clone(),
                    summary,
                    has_children,
                    expanded: is_expanded,
                });
                if has_children && is_expanded {
                    rows.extend(preview_tree_rows(val, expanded, &row_path, depth + 1));
                }
            }
        }
        serde_json::Value::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                let key = format!("[{i}]");
                let row_path = if path.is_empty() { key.clone() } else { format!("{path}{key}") };
                let has_children = matches!(val, serde_json::Value::Object(_) | serde_json::Value::Array(_));
                let is_expanded = expanded.contains(&row_path);
                let summary = if has_children {
                    preview_tree_collapsed_summary(val)
                } else {
                    preview_tree_scalar_display(val)
                };
                rows.push(PreviewTreeRow {
                    path: row_path.clone(),
                    depth,
                    label: key,
                    summary,
                    has_children,
                    expanded: is_expanded,
                });
                if has_children && is_expanded {
                    rows.extend(preview_tree_rows(val, expanded, &row_path, depth + 1));
                }
            }
        }
        _ => {}
    }
    rows
}

/// 📐 Measures preview content size in world units.
pub fn measure_preview_content(content: &DagPreviewContent, expanded: &BTreeSet<String>) -> (f64, f64) {
    match content {
        DagPreviewContent::Empty => (DAG_PREVIEW_MIN_SIZE, DAG_PREVIEW_MIN_SIZE),
        DagPreviewContent::Scalar { text } => {
            let tw = preview_scalar_text_width(text);
            (tw.max(DAG_PREVIEW_MIN_SIZE), DAG_PREVIEW_ROW_HEIGHT.max(DAG_PREVIEW_MIN_SIZE))
        }
        DagPreviewContent::Image { src } => clamp_preview_image_size(preview_media_natural_size(src).0, preview_media_natural_size(src).1),
        DagPreviewContent::Tree { json } => {
            let rows = preview_tree_rows(json, expanded, "", 0);
            if rows.is_empty() {
                return (DAG_PREVIEW_MIN_SIZE, DAG_PREVIEW_MIN_SIZE);
            }
            let max_w = rows
                .iter()
                .map(|row| {
                    let indent = row.depth as f64 * DAG_PREVIEW_TREE_INDENT;
                    let toggle = if row.has_children { DAG_PREVIEW_TOGGLE_WIDTH } else { 0.0 };
                    let line = if row.has_children && !row.expanded {
                        format!("{}: {}", row.label, row.summary)
                    } else if !row.has_children {
                        format!("{}: {}", row.label, row.summary)
                    } else {
                        row.label.clone()
                    };
                    indent + toggle + port_label_text_width(&line, DAG_LABEL_COMPACT_SCREEN_PX)
                })
                .fold(DAG_PREVIEW_MIN_SIZE, f64::max);
            (max_w, rows.len() as f64 * DAG_PREVIEW_ROW_HEIGHT)
        }
    }
}

fn preview_content_node_size(content: &DagPreviewContent, expanded: &BTreeSet<String>) -> (f64, f64) {
    let (cw, ch) = measure_preview_content(content, expanded);
    (cw + DAG_PREVIEW_PAD * 2.0, ch + DAG_PREVIEW_PAD * 2.0)
}

fn preview_image_node_size(src: &str) -> (f64, f64) {
    let (cw, ch) = clamp_preview_image_size(preview_media_natural_size(src).0, preview_media_natural_size(src).1);
    (cw + DAG_PREVIEW_PAD * 2.0, ch + DAG_PREVIEW_PAD * 2.0)
}

/// 📐 Image input node size from media source.
pub fn image_widget_size(src: &str) -> (f64, f64) {
    preview_image_node_size(src)
}

/// 📐 Preview node size from typed content and fold state.
pub fn preview_widget_size(content: &DagPreviewContent, expanded: &BTreeSet<String>) -> (f64, f64) {
    preview_content_node_size(content, expanded)
}

fn preview_tree_row_layouts(node: &DagNodeSpec, json: &serde_json::Value, expanded: &BTreeSet<String>) -> Vec<PreviewTreeRowLayout> {
    let (x0, y0, x1, _y1) = preview_content_bounds(node);
    preview_tree_rows(json, expanded, "", 0)
        .into_iter()
        .enumerate()
        .map(|(index, row)| {
            let row_y0 = y0 + index as f64 * DAG_PREVIEW_ROW_HEIGHT;
            let row_y1 = row_y0 + DAG_PREVIEW_ROW_HEIGHT;
            let row_rect = if row.has_children {
                (x0, row_y0, x0 + (x1 - x0).max(1.0), row_y1)
            } else {
                (0.0, 0.0, 0.0, 0.0)
            };
            PreviewTreeRowLayout { path: row.path, row_rect }
        })
        .collect()
}
// #endregion 🔖PreviewContent

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
    Image {
        #[serde(default)]
        src: String,
        output: IoPortSpec,
    },
    Preview {
        #[serde(default)]
        content: DagPreviewContent,
        #[serde(default)]
        expanded: BTreeSet<String>,
        input: IoPortSpec,
    },
    Action {
        label: String,
        input: IoPortSpec,
    },
    Cluster {
        inputs: Vec<IoPortSpec>,
        outputs: Vec<IoPortSpec>,
    },
}

fn uses_computation_layout(kind: &DagNodeKind) -> bool {
    matches!(kind, DagNodeKind::Computation { .. } | DagNodeKind::Cluster { .. })
}

pub const DAG_CLUSTER_EXPLODE_HIT_SIZE: f64 = 14.0;

/// 💥 World-space hit rect for the cluster explode affordance.
pub fn cluster_explode_hit_rect(node: &DagNodeSpec) -> Option<(f64, f64, f64, f64)> {
    if !matches!(node.kind, DagNodeKind::Cluster { .. }) {
        return None;
    }
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    let size = DAG_CLUSTER_EXPLODE_HIT_SIZE;
    let x1 = node.x + hw - DAG_NODE_EDGE_INSET;
    let y0 = node.y - hh + DAG_NODE_EDGE_INSET;
    Some((x1 - size, y0, x1, y0 + size))
}

/// 💥 Whether a world point hits the cluster explode affordance.
pub fn cluster_explode_hit(node: &DagNodeSpec, world_x: f64, world_y: f64) -> bool {
    cluster_explode_hit_rect(node)
        .is_some_and(|(x0, y0, x1, y1)| point_in_rect(world_x, world_y, x0, y0, x1, y1))
}

/// 📦 DAG node with shared layout fields and a tagged kind.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagNodeSpec {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub abbreviation: String,
    #[serde(default)]
    pub icon: String,
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
        abbreviation: String,
        icon: String,
        inputs: Vec<IoPortSpec>,
        outputs: Vec<IoPortSpec>,
        variadic_inputs: bool,
        variadic_outputs: bool,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Self {
        let (name, abbreviation) = normalize_node_display(&name, &abbreviation);
        Self {
            id,
            name,
            abbreviation,
            icon,
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

    /// 🧩 Builds a cluster node with contract IO ports.
    pub fn cluster(
        id: String,
        name: String,
        abbreviation: String,
        icon: String,
        inputs: Vec<IoPortSpec>,
        outputs: Vec<IoPortSpec>,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Self {
        let (name, abbreviation) = normalize_node_display(&name, &abbreviation);
        Self {
            id,
            name,
            abbreviation,
            icon,
            x,
            y,
            width,
            height,
            kind: DagNodeKind::Cluster { inputs, outputs },
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
            DagNodeKind::Computation { inputs, .. } | DagNodeKind::Cluster { inputs, .. } => inputs,
            DagNodeKind::Screen { input, .. } | DagNodeKind::Preview { input, .. } | DagNodeKind::Action { input, .. } => {
                std::slice::from_ref(input)
            }
            _ => EMPTY_PORTS,
        }
    }

    /// ➡ Effective output ports for the node kind.
    pub fn outputs(&self) -> &[IoPortSpec] {
        match &self.kind {
            DagNodeKind::Computation { outputs, .. } | DagNodeKind::Cluster { outputs, .. } => outputs,
            DagNodeKind::Slider { output, .. }
            | DagNodeKind::Select { output, .. }
            | DagNodeKind::Note { output, .. }
            | DagNodeKind::Image { output, .. } => std::slice::from_ref(output),
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
    let pad = DAG_NODE_EDGE_INSET;
    let track_y = node.y;
    let hit_h = 8.0;
    (
        node.x - hw + pad,
        track_y - hit_h * 0.5,
        node.x + hw - pad,
        track_y + hit_h * 0.5,
    )
}

fn slider_value_world_center(node: &DagNodeSpec, value_text: &str, paint_px: f64, zoom: f64) -> (f64, f64) {
    use cavas::text::label_extent;
    let hw = node.width * 0.5;
    let (value_w, _) = label_extent(value_text, paint_px);
    let z = zoom.max(0.05);
    let screen_gap = DAG_LABEL_SCREEN_PX * ui_styling::metrics::label::DAG_LABEL_GAP_RATIO;
    let world_offset = (screen_gap + value_w * 0.5) / z;
    (node.x - hw - world_offset, node.y)
}

fn select_control_bounds(node: &DagNodeSpec) -> (f64, f64, f64, f64) {
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    (node.x - hw + DAG_NODE_EDGE_INSET, node.y + hh * 0.12, node.x + hw - 10.0, node.y + hh - DAG_NODE_EDGE_INSET)
}

/// 📐 Note node size from its text payload.
pub fn note_widget_size(text: &str) -> (f64, f64) {
    let display = if text.is_empty() { "…" } else { text };
    let tw = preview_scalar_text_width(display);
    (
        tw.max(DAG_PREVIEW_MIN_SIZE) + DAG_PREVIEW_PAD * 2.0,
        DAG_PREVIEW_ROW_HEIGHT.max(DAG_PREVIEW_MIN_SIZE) + DAG_PREVIEW_PAD * 2.0,
    )
}

fn preview_content_bounds(node: &DagNodeSpec) -> (f64, f64, f64, f64) {
    let hw = node.width * 0.5;
    let hh = node.height * 0.5;
    let pad = DAG_PREVIEW_PAD;
    (node.x - hw + pad, node.y - hh + pad, node.x + hw - pad, node.y + hh - pad)
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
    let label_w = port_label_text_width(label, px);
    node.x + node.width * 0.5 - DAG_NODE_EDGE_INSET - label_w
}

fn computation_column_divider_x(node: &DagNodeSpec) -> Option<f64> {
    let inputs = node.inputs();
    let outputs = node.outputs();
    if inputs.is_empty() || outputs.is_empty() {
        return None;
    }
    let hw = node.width * 0.5;
    let port_px = DAG_LABEL_COMPACT_SCREEN_PX;
    let left_w = io_port_column_width(inputs, port_px);
    let right_w = io_port_column_width(outputs, port_px);
    let left_end = node.x - hw + DAG_NODE_EDGE_INSET + left_w;
    let right_start = node.x + hw - DAG_NODE_EDGE_INSET - right_w;
    Some(if right_start <= left_end {
        node.x
    } else {
        (left_end + right_start) * 0.5
    })
}

fn computation_input_column_x_bounds(node: &DagNodeSpec) -> Option<(f64, f64)> {
    let inputs = node.inputs();
    if inputs.is_empty() && !node.variadic_inputs() {
        return None;
    }
    let hw = node.width * 0.5;
    let port_px = DAG_LABEL_COMPACT_SCREEN_PX;
    let left = node.x - hw + DAG_NODE_EDGE_INSET;
    let right = left + io_port_column_width(inputs, port_px);
    Some((left, right))
}

fn computation_output_column_x_bounds(node: &DagNodeSpec) -> Option<(f64, f64)> {
    let outputs = node.outputs();
    let variadic_outputs = matches!(&node.kind, DagNodeKind::Computation { variadic_outputs: true, .. });
    if outputs.is_empty() && !variadic_outputs {
        return None;
    }
    let hw = node.width * 0.5;
    let port_px = DAG_LABEL_COMPACT_SCREEN_PX;
    let right = node.x + hw - DAG_NODE_EDGE_INSET;
    let left = right - io_port_column_width(outputs, port_px);
    Some((left, right))
}

fn computation_io_side_row_counts(node: &DagNodeSpec) -> (usize, usize) {
    match &node.kind {
        DagNodeKind::Computation {
            inputs,
            outputs,
            variadic_inputs,
            variadic_outputs,
        } => {
            let input_rows = inputs.len() + usize::from(*variadic_inputs);
            let output_rows = outputs.len() + usize::from(*variadic_outputs);
            (input_rows, output_rows)
        }
        DagNodeKind::Cluster { inputs, outputs } => (inputs.len(), outputs.len()),
        _ => (0, 0),
    }
}

fn channel_row_divider_y(node_y: f64, node_height: f64, after_row_index: usize) -> f64 {
    let hh = node_height * 0.5;
    node_y - hh + after_row_index as f64 * DAG_CHANNEL_ROW_HEIGHT
}

fn computation_channel_row_divider_x_span(node: &DagNodeSpec, side: ComputationChannelRowSide) -> (f64, f64) {
    let hw = node.width * 0.5;
    let left_edge = node.x - hw;
    let right_edge = node.x + hw;
    match (computation_column_divider_x(node), side) {
        (Some(divider_x), ComputationChannelRowSide::Input) => (left_edge, divider_x),
        (Some(divider_x), ComputationChannelRowSide::Output) => (divider_x, right_edge),
        (None, ComputationChannelRowSide::Input) | (None, ComputationChannelRowSide::Output) => (left_edge, right_edge),
    }
}

fn computation_io_side_row_divider_indices(side_rows: usize, grid_rows: usize) -> std::ops::Range<usize> {
    if side_rows == 0 || grid_rows <= 1 {
        return 0..0;
    }
    1..side_rows + usize::from(side_rows < grid_rows)
}

fn computation_name_world_center(node: &DagNodeSpec, label: &str, paint_px: f64, zoom: f64) -> (f64, f64) {
    use cavas::text::label_extent;
    let hh = node.height * 0.5;
    let (_, label_h) = label_extent(label, paint_px);
    let z = zoom.max(0.05);
    let screen_gap = DAG_LABEL_SCREEN_PX * ui_styling::metrics::label::DAG_LABEL_GAP_COMPACT_RATIO;
    let world_offset = (screen_gap + label_h * 0.5) / z;
    (node.x, node.y - hh - world_offset)
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
    match &node.kind {
        DagNodeKind::Computation {
            inputs,
            outputs,
            variadic_inputs,
            variadic_outputs,
        } => computation_io_row_count(inputs.len(), outputs.len(), *variadic_inputs, *variadic_outputs) + DAG_COMPUTATION_HEADER_ROWS,
        DagNodeKind::Cluster { inputs, outputs } => {
            computation_io_row_count(inputs.len(), outputs.len(), false, false) + DAG_COMPUTATION_HEADER_ROWS
        }
        _ => 0,
    }
}

pub fn fit_node_size(node: &mut DagNodeSpec) {
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
        DagNodeKind::Cluster { inputs, outputs } => {
            node.width = computation_node_width(&node.name, inputs, outputs);
            node.height = computation_node_height(inputs.len(), outputs.len(), false, false);
        }
        DagNodeKind::Slider { output, .. } => {
            node.width = slider_widget_width(&node.name, output);
            node.height = slider_widget_height();
        }
        DagNodeKind::Note { text, .. } => {
            let (w, h) = note_widget_size(text);
            node.width = w;
            node.height = h;
        }
        DagNodeKind::Select { .. } | DagNodeKind::Action { .. } => {
            node.width = io_widget_width(&node.name);
            node.height = io_widget_height(&node.name);
        }
        DagNodeKind::Preview { content, expanded, .. } => {
            let (w, h) = preview_content_node_size(content, expanded);
            node.width = w;
            node.height = h;
        }
        DagNodeKind::Image { src, .. } => {
            let (w, h) = preview_image_node_size(src);
            node.width = w;
            node.height = h;
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
use std::collections::{BTreeSet, HashMap, HashSet};

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
    ui_styling::metrics::board::LAYOUT_LAYER_SPACING
}

fn default_sibling_gap() -> f64 {
    ui_styling::metrics::board::LAYOUT_SIBLING_GAP
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
        description: "Node icons only.",
        max_zoom: 0.35,
    },
    Lod {
        id: "compact",
        name: "Compact",
        description: "Horizontal abbreviations.",
        max_zoom: 0.55,
    },
    Lod {
        id: "normal",
        name: "Normal",
        description: "Vertical names with sections; input rows accept wire drops.",
        max_zoom: 1.25,
    },
    Lod {
        id: "detail",
        name: "Detail",
        description: "Abbreviations, port labels, and control text.",
        max_zoom: 2.5,
    },
    Lod {
        id: "micro",
        name: "Micro",
        description: "Names and maximum node fidelity.",
        max_zoom: f64::INFINITY,
    },
];

const DAG_LOD_SCALE: LodScale = LodScale { lods: DAG_LODS };

/// 📶 Uniform delay before each DAG LOD band activates (requires slightly more zoom for detail tiers).
const DAG_LOD_ZOOM_SHIFT: f64 = ui_styling::metrics::dag::LOD_ZOOM_SHIFT;

const DAG_LOD_BAND_FLOOR_ZOOM: &[f64] = ui_styling::metrics::dag::LOD_BAND_FLOOR_ZOOM;

fn dag_lod_resolve_zoom(zoom: f64) -> f64 {
    (zoom - DAG_LOD_ZOOM_SHIFT).max(0.05)
}

fn dag_lod_index(zoom: f64) -> usize {
    DAG_LOD_SCALE.resolve_index(dag_lod_resolve_zoom(zoom))
}

fn dag_lod_band_floor_zoom(lod_index: usize) -> f64 {
    let floor = cavas::lod::band_floor_zoom(DAG_LOD_BAND_FLOOR_ZOOM, lod_index, 0.05);
    if lod_index == 0 {
        floor
    } else {
        (floor + DAG_LOD_ZOOM_SHIFT).max(0.05)
    }
}

/// 🔵 Port dot world radius; screen size grows with camera zoom like node geometry.
const DAG_HANDLE_WORLD_RADIUS: f64 = ui_styling::radii::DAG_HANDLE_WORLD;

const DAG_NODE_STROKE_SCREEN_PX: f64 = ui_styling::strokes::DAG_NODE;
const DAG_NODE_STROKE_SELECTED_SCREEN_PX: f64 = ui_styling::strokes::DAG_NODE_SELECTED;
const DAG_NODE_STROKE_HOVERED_SCREEN_PX: f64 = ui_styling::strokes::DAG_NODE_HOVERED;
const DAG_EDGE_STROKE_SCREEN_PX: f64 = ui_styling::strokes::DAG_EDGE;
const DAG_EDGE_STROKE_MINIMAP_SCREEN_PX: f64 = ui_styling::strokes::DAG_EDGE_MINIMAP;
const DAG_CHROME_STROKE_SCREEN_PX: f64 = ui_styling::strokes::DAG_CHROME;
const DAG_BOUNDED_DRAG_HIT_PAD_PX: f64 = ui_styling::metrics::board::BOUNDED_DRAG_HIT_PAD_PX;

fn dag_world_stroke(screen_px: f64, zoom: f64) -> f64 {
    (screen_px / zoom.max(0.05)).max(1e-3)
}

fn dag_label_layout_px() -> f64 {
    DAG_LABEL_SCREEN_PX
}

fn dag_label_paint_px(zoom: f64, lod_index: usize) -> f64 {
    cavas::lod::lod_band_label_screen_px(DAG_LABEL_SCREEN_PX, zoom, dag_lod_band_floor_zoom(lod_index))
}

fn dag_label_compact_paint_px(zoom: f64, lod_index: usize) -> f64 {
    cavas::lod::lod_band_label_screen_px(DAG_LABEL_COMPACT_SCREEN_PX, zoom, dag_lod_band_floor_zoom(lod_index))
}

fn dag_node_body_fill(theme: &VelloThemePalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> cavas::vello::peniko::Color {
    if dimmed {
        vello_color_with_alpha(theme.node_fill_disabled, ui_styling::opacities::DISABLED_FILL_ALPHA)
    } else if selected {
        theme.node_fill_selected
    } else if highlighted {
        theme.node_fill_selection_exit
    } else if hovered {
        theme.node_fill_hovered
    } else {
        theme.node_fill
    }
}

pub(crate) fn dag_node_body_stroke(theme: &VelloThemePalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> cavas::vello::peniko::Color {
    if dimmed {
        vello_color_with_alpha(theme.node_stroke, ui_styling::opacities::DIM_STROKE_ALPHA)
    } else if selected {
        theme.node_stroke_selected
    } else if highlighted {
        theme.node_stroke_selection_exit
    } else if hovered {
        theme.node_stroke_hovered
    } else {
        theme.node_stroke
    }
}

pub(crate) fn dag_node_label_fill(theme: &VelloThemePalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> cavas::vello::peniko::Color {
    if dimmed {
        vello_color_with_alpha(theme.label_fill, ui_styling::opacities::DIM_LABEL_ALPHA)
    } else if selected {
        theme.label_fill_hovered
    } else if highlighted {
        theme.node_stroke_selection_exit
    } else if hovered {
        theme.label_fill_hovered
    } else {
        theme.label_fill
    }
}

/// @emoji 🧱 Internal column/row chrome inside a node body; selection/hover matches label emphasis.
pub(crate) fn dag_node_internal_chrome_stroke(
    body_stroke: cavas::vello::peniko::Color,
    label_fill: cavas::vello::peniko::Color,
    emphasized: bool,
) -> cavas::vello::peniko::Color {
    if emphasized {
        label_fill
    } else {
        body_stroke
    }
}

pub(crate) fn dag_handle_body_fill(theme: &VelloThemePalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> cavas::vello::peniko::Color {
    if dimmed {
        vello_color_with_alpha(theme.handle_fill_disabled, ui_styling::opacities::DISABLED_FILL_ALPHA)
    } else if selected {
        theme.handle_fill_selected
    } else if highlighted {
        theme.handle_fill_selection_exit
    } else if hovered {
        theme.handle_fill_hovered
    } else {
        theme.handle_fill
    }
}

pub(crate) fn dag_handle_body_stroke(theme: &VelloThemePalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> cavas::vello::peniko::Color {
    if dimmed {
        vello_color_with_alpha(theme.handle_stroke_disabled, ui_styling::opacities::DISABLED_STROKE_ALPHA)
    } else if selected {
        theme.handle_stroke_selected
    } else if highlighted {
        theme.handle_stroke_selection_exit
    } else if hovered {
        theme.handle_stroke_hovered
    } else {
        theme.handle_stroke
    }
}

pub(crate) fn dag_edge_body_stroke(theme: &VelloThemePalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> cavas::vello::peniko::Color {
    if dimmed {
        vello_color_with_alpha(theme.edge_stroke_disabled, ui_styling::opacities::DISABLED_STROKE_ALPHA)
    } else if selected {
        theme.edge_stroke_selected
    } else if highlighted {
        theme.edge_stroke_selection_exit
    } else if hovered {
        theme.edge_stroke_hovered
    } else {
        theme.edge_stroke
    }
}

fn dag_node_stroke_screen_px(dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> f64 {
    if dimmed {
        1.0
    } else if selected {
        DAG_NODE_STROKE_SELECTED_SCREEN_PX
    } else if highlighted || hovered {
        DAG_NODE_STROKE_HOVERED_SCREEN_PX
    } else {
        DAG_NODE_STROKE_SCREEN_PX
    }
}

/// @emoji 🎨 Node body fill when painted; `None` means stroke/text only (puzzle 2d overview+).
pub(crate) fn dag_node_paint_fill(
    lod: DagDrawLod,
    theme: &VelloThemePalette,
    dimmed: bool,
    selected: bool,
    highlighted: bool,
    hovered: bool,
) -> Option<cavas::vello::peniko::Color> {
    if lod == DagDrawLod::Minimap {
        return Some(dag_node_body_stroke(theme, dimmed, selected, highlighted, hovered));
    }
    let chrome = dimmed || selected || highlighted || hovered;
    if chrome {
        Some(dag_node_body_fill(theme, dimmed, selected, highlighted, hovered))
    } else {
        None
    }
}

/// 🏷️ Node label content shown at a draw LOD tier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagNodeLabel {
    None,
    Abbreviation,
    Name,
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

    pub fn from_id(id: &str) -> Option<Self> {
        match id.trim() {
            "minimap" => Some(Self::Minimap),
            "overview" => Some(Self::Overview),
            "compact" => Some(Self::Compact),
            "normal" => Some(Self::Normal),
            "detail" => Some(Self::Detail),
            "micro" => Some(Self::Micro),
            _ => None,
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

    pub fn node_icon_visible(self) -> bool {
        matches!(self, Self::Overview)
    }

    pub fn node_label(self) -> DagNodeLabel {
        match self {
            Self::Minimap | Self::Overview => DagNodeLabel::None,
            Self::Compact | Self::Detail => DagNodeLabel::Abbreviation,
            Self::Normal | Self::Micro => DagNodeLabel::Name,
        }
    }

    pub fn node_label_is_horizontal(self) -> bool {
        matches!(self, Self::Compact)
    }

    pub fn shows_computation_layout(self) -> bool {
        matches!(self, Self::Normal | Self::Detail | Self::Micro)
    }

    pub fn shows_port_labels(self) -> bool {
        matches!(self, Self::Detail | Self::Micro)
    }

    pub fn shows_handles(self) -> bool {
        matches!(self, Self::Detail | Self::Micro)
    }

    pub fn uses_input_row_connection_hitbox(self) -> bool {
        self == Self::Normal
    }

    pub fn uses_channel_row_pick(self) -> bool {
        matches!(self, Self::Detail | Self::Micro)
    }

    pub fn allows_connection_hit_picking(self) -> bool {
        self.uses_input_row_connection_hitbox() || self.shows_handles()
    }

    pub fn shows_controls(self) -> bool {
        matches!(self, Self::Normal | Self::Detail | Self::Micro)
    }

    pub fn shows_detail_text(self) -> bool {
        matches!(self, Self::Detail | Self::Micro)
    }

    pub fn edge_stroke_screen_px(self) -> f64 {
        match self {
            Self::Minimap => DAG_EDGE_STROKE_MINIMAP_SCREEN_PX,
            _ => DAG_EDGE_STROKE_SCREEN_PX,
        }
    }
}

/// 📶 Resolves the DAG draw LOD for a camera zoom factor.
pub fn dag_draw_lod(zoom: f64) -> DagDrawLod {
    DagDrawLod::from_scale_index(dag_lod_index(zoom))
}

fn lod_max_zoom_json(max_zoom: f64) -> serde_json::Value {
    if max_zoom.is_finite() {
        serde_json::json!(max_zoom)
    } else {
        serde_json::json!(f64::MAX)
    }
}

/// 📶 JSON LOD table for React window chrome (`id`, `name`, `description`, `maxZoom`).
pub fn dag_lod_scale_json() -> String {
    let rows: Vec<serde_json::Value> = DAG_LODS
        .iter()
        .map(|lod| {
            let max_zoom = if lod.max_zoom.is_finite() {
                lod.max_zoom + DAG_LOD_ZOOM_SHIFT
            } else {
                lod.max_zoom
            };
            serde_json::json!({
                "id": lod.id,
                "name": lod.name,
                "description": lod.description,
                "maxZoom": lod_max_zoom_json(max_zoom),
            })
        })
        .collect();
    serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
}
// #endregion 🔖Lod

fn dag_debug_log(msg: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&msg.into());
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("{msg}");
}

// #region 🔖ChannelRef
/// 🔌 Resolved fixture channel from a port handle hover or selection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagChannelRef {
    pub widget_id: String,
    pub port: String,
    pub direction: String,
}

impl DagChannelRef {
    pub fn is_input(&self) -> bool {
        self.direction == "in"
    }

    pub fn is_output(&self) -> bool {
        self.direction == "out"
    }
}
// #endregion 🔖ChannelRef

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
    automatic_lod: bool,
    forced_draw_lod: Option<DagDrawLod>,
    icon_paint_cache: graph::IconPaintCache,
    ghost_node: Option<DagNodeSpec>,
    pending_cluster_explode: Option<String>,
    computing_active: Option<NodeId>,
    computing_stale: HashSet<NodeId>,
    computing_active_anim_phase: Cell<f64>,
    computing_stale_anim_phase: Cell<f64>,
}

fn vello_color_with_alpha(color: cavas::vello::peniko::Color, alpha: u8) -> cavas::vello::peniko::Color {
    use cavas::vello::peniko::Color;
    let rgba = color.to_rgba8();
    Color::from_rgba8(rgba.r, rgba.g, rgba.b, alpha)
}

#[derive(Clone, Copy)]
struct DagNodePaintChrome {
    is_dimmed: bool,
    is_selected: bool,
    is_highlighted: bool,
    is_hovered: bool,
    is_computing: bool,
    is_stale: bool,
    body_fill_alpha: u8,
    ghost_tint: bool,
}

impl DagNodePaintChrome {
    fn ghost_preview() -> Self {
        Self {
            is_dimmed: false,
            is_selected: false,
            is_highlighted: false,
            is_hovered: false,
            is_computing: false,
            is_stale: false,
            body_fill_alpha: 255,
            ghost_tint: true,
        }
    }

    fn tint_highlighted(self) -> bool {
        self.is_highlighted || self.ghost_tint
    }

    fn has_interaction_chrome(self) -> bool {
        self.is_dimmed || self.is_selected || self.is_highlighted || self.is_hovered
    }
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
            automatic_lod: true,
            forced_draw_lod: None,
            icon_paint_cache: graph::IconPaintCache::new(),
            ghost_node: None,
            pending_cluster_explode: None,
            computing_active: None,
            computing_stale: HashSet::new(),
            computing_active_anim_phase: Cell::new(0.0),
            computing_stale_anim_phase: Cell::new(0.0),
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
        matches!(self.engine.hover, Some(hover) if hover == node_id)
    }

    fn resolve_minimap_hover_node_id(&self, world_x: f64, world_y: f64) -> Option<NodeId> {
        for idx in (0..self.fixture.nodes.len()).rev() {
            let node = &self.fixture.nodes[idx];
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            if world_x >= node.x - hw && world_x <= node.x + hw && world_y >= node.y - hh && world_y <= node.y + hh {
                return self.engine_node_id_for_index(idx);
            }
        }
        None
    }

    fn sync_minimap_pointer_hover(&mut self, world_x: f64, world_y: f64) {
        if !matches!(self.draw_lod_for_frame(), DagDrawLod::Minimap) {
            return;
        }
        if !matches!(self.engine.interaction, InteractionMode::Idle) {
            return;
        }
        self.engine.hover = self.resolve_minimap_hover_node_id(world_x, world_y);
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

    fn is_preselect_active(&self) -> bool {
        matches!(
            self.engine.interaction,
            InteractionMode::SelectionPending { .. } | InteractionMode::AreaSelect { .. }
        ) || !self.engine.preselect.node_ids.is_empty()
            || !self.engine.preselect.handle_ids.is_empty()
            || !self.engine.preselect.edge_ids.is_empty()
    }

    fn node_interaction_chrome(&self, node_id: NodeId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (
                self.is_node_preselected(node_id),
                self.is_node_preselect_removed(node_id),
                false,
            );
        }
        (self.is_node_selected(node_id), false, self.is_node_hovered(node_id))
    }

    fn handle_interaction_chrome(&self, handle_id: HandleId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (
                self.engine.preselect.handle_ids.contains(&handle_id),
                self.engine.preselect_removed.handle_ids.contains(&handle_id),
                false,
            );
        }
        (
            self.engine.selection.handle_ids.contains(&handle_id),
            false,
            self.engine.hover == Some(handle_id),
        )
    }

    fn edge_interaction_chrome(&self, edge_id: EdgeId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (
                self.engine.preselect.edge_ids.contains(&edge_id),
                self.engine.preselect_removed.edge_ids.contains(&edge_id),
                false,
            );
        }
        (
            self.engine.selection.edge_ids.contains(&edge_id),
            false,
            self.engine.hover == Some(edge_id),
        )
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

    /// ✅ Whether the engine has any committed node, edge, or handle selection.
    pub fn has_selection(&self) -> bool {
        !self.engine.selection.node_ids.is_empty()
            || !self.engine.selection.edge_ids.is_empty()
            || !self.engine.selection.handle_ids.is_empty()
    }

    /// 🖱️ Hovered fixture widget id for node body hover, or parent widget when a channel handle is hovered at detail LOD.
    pub fn hovered_node_id(&self) -> Option<String> {
        let hover = self.engine.hover?;
        if self.node_id_map.contains_key(&hover) {
            return self.widget_id_for_node_id(hover);
        }
        if self.draw_lod_for_frame().uses_channel_row_pick() {
            if let Some(handle) = self.engine.handles.get(&hover) {
                return self.widget_id_for_node_id(handle.node_id);
            }
        }
        None
    }

    fn decode_channel_ref(&self, target: u64) -> Option<DagChannelRef> {
        if self.node_id_map.contains_key(&target) {
            return None;
        }
        let key = self.handle_key_map.get(&target)?;
        let (node_id, port_id) = key.split_once(':')?;
        let node = self.fixture.nodes.iter().find(|entry| entry.id == node_id)?;
        let direction = if node.inputs().iter().any(|port| port.id == port_id) {
            "in"
        } else if node.outputs().iter().any(|port| port.id == port_id) {
            "out"
        } else {
            return None;
        };
        Some(DagChannelRef {
            widget_id: node_id.to_string(),
            port: port_id.to_string(),
            direction: direction.to_string(),
        })
    }

    /// 🔌 Hovered fixture channel when the pointer is over a port row or handle.
    pub fn hovered_channel(&self) -> Option<DagChannelRef> {
        let hover = self.engine.hover?;
        self.decode_channel_ref(hover)
    }

    /// 🔌 Selected fixture channels from handle picks in the current selection snapshot.
    pub fn selected_channels(&self) -> Vec<DagChannelRef> {
        self.engine
            .selection
            .handle_ids
            .iter()
            .filter_map(|&handle_id| self.decode_channel_ref(handle_id))
            .collect()
    }

    /// 🔌 Selected fixture channels as JSON.
    pub fn selected_channels_json(&self) -> String {
        serde_json::to_string(&self.selected_channels()).unwrap_or_else(|_| "[]".into())
    }

    /// 🔌 Hovered fixture channel as JSON, or `null`.
    pub fn hovered_channel_json(&self) -> String {
        match self.hovered_channel() {
            Some(channel) => serde_json::to_string(&channel).unwrap_or_else(|_| "null".into()),
            None => "null".into(),
        }
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

    // #region 🔖SelectionAlign
    fn dag_node_world_bounds(node: &DagNodeSpec) -> WorldBox {
        let hw = node.width * 0.5;
        let hh = node.height * 0.5;
        WorldBox {
            min_x: node.x - hw,
            min_y: node.y - hh,
            max_x: node.x + hw,
            max_y: node.y + hh,
        }
    }

    fn selected_fixture_nodes(&self) -> Vec<(usize, DagNodeSpec)> {
        let ids = self.selected_node_ids();
        ids.into_iter()
            .filter_map(|id| {
                self.fixture
                    .nodes
                    .iter()
                    .enumerate()
                    .find(|(_, node)| node.id == id)
                    .map(|(idx, node)| (idx, node.clone()))
            })
            .collect()
    }

    fn sync_fixture_node_center_to_engine(&mut self, idx: usize) {
        let node = &self.fixture.nodes[idx];
        let Some(nid) = self.node_id_for_widget_id(&node.id) else {
            return;
        };
        if let Some(engine_node) = self.engine.nodes.get_mut(&nid) {
            engine_node.center.x = node.x;
            engine_node.center.y = node.y;
        }
    }

    /// 📦 Screen-space union bounds of the current node selection for DOM chrome overlays.
    pub fn selection_union_bounds_screen_json(&self) -> String {
        let selected = self.selected_fixture_nodes();
        if selected.is_empty() {
            return "null".into();
        }
        use cavas::camera::{world_to_screen, Camera as CavasCamera, Viewport};
        use cavas::vello::kurbo::Point;
        let pad_world = 4.0 / self.fixture.camera.zoom.max(0.05);
        let mut corners = Vec::new();
        for (_, node) in &selected {
            let hw = node.width * 0.5 + pad_world;
            let hh = node.height * 0.5 + pad_world;
            corners.push(Point::new(node.x - hw, node.y - hh));
            corners.push(Point::new(node.x + hw, node.y + hh));
        }
        let Some(bounds) = world_box_from_points(&corners) else {
            return "null".into();
        };
        let cam = CavasCamera {
            x: self.fixture.camera.x,
            y: self.fixture.camera.y,
            zoom: self.fixture.camera.zoom,
        };
        let viewport = Viewport {
            width: self.width.max(1),
            height: self.height.max(1),
            dpr: self.dpr.max(1.0),
        };
        let tl = world_to_screen(&cam, &viewport, Point::new(bounds.min_x, bounds.min_y));
        let br = world_to_screen(&cam, &viewport, Point::new(bounds.max_x, bounds.max_y));
        serde_json::json!({
            "x": tl.x,
            "y": tl.y,
            "width": (br.x - tl.x).max(1.0),
            "height": (br.y - tl.y).max(1.0),
        })
        .to_string()
    }

    /// 📐 Aligns or distributes the current multi-node selection.
    pub fn align_selection(&mut self, mode: &str) -> Result<(), String> {
        use cavas::vello::kurbo::Point;
        let mut selected = self.selected_fixture_nodes();
        if selected.is_empty() {
            return Ok(());
        }
        match mode {
            "alignLeft" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let min_left = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).min_x).fold(f64::INFINITY, f64::min);
                for (_, node) in &mut selected {
                    node.x = min_left + node.width * 0.5;
                }
            }
            "alignRight" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let max_right = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).max_x).fold(f64::NEG_INFINITY, f64::max);
                for (_, node) in &mut selected {
                    node.x = max_right - node.width * 0.5;
                }
            }
            "alignTop" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let min_top = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).min_y).fold(f64::INFINITY, f64::min);
                for (_, node) in &mut selected {
                    node.y = min_top + node.height * 0.5;
                }
            }
            "alignBottom" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let max_bottom = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).max_y).fold(f64::NEG_INFINITY, f64::max);
                for (_, node) in &mut selected {
                    node.y = max_bottom - node.height * 0.5;
                }
            }
            "alignHorizontal" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let mut corners = Vec::new();
                for (_, node) in &selected {
                    let b = Self::dag_node_world_bounds(node);
                    corners.push(Point::new(b.min_x, b.min_y));
                    corners.push(Point::new(b.max_x, b.max_y));
                }
                let Some(union) = world_box_from_points(&corners) else {
                    return Ok(());
                };
                let center_x = (union.min_x + union.max_x) * 0.5;
                for (_, node) in &mut selected {
                    node.x = center_x;
                }
            }
            "alignVertical" => {
                if selected.len() < 2 {
                    return Ok(());
                }
                let mut corners = Vec::new();
                for (_, node) in &selected {
                    let b = Self::dag_node_world_bounds(node);
                    corners.push(Point::new(b.min_x, b.min_y));
                    corners.push(Point::new(b.max_x, b.max_y));
                }
                let Some(union) = world_box_from_points(&corners) else {
                    return Ok(());
                };
                let center_y = (union.min_y + union.max_y) * 0.5;
                for (_, node) in &mut selected {
                    node.y = center_y;
                }
            }
            "distributeHorizontal" => {
                if selected.len() < 3 {
                    return Ok(());
                }
                selected.sort_by(|a, b| a.1.x.partial_cmp(&b.1.x).unwrap_or(std::cmp::Ordering::Equal));
                let left = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).min_x).fold(f64::INFINITY, f64::min);
                let right = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).max_x).fold(f64::NEG_INFINITY, f64::max);
                let total_width: f64 = selected.iter().map(|(_, node)| node.width).sum();
                let gap = (right - left - total_width) / (selected.len() as f64 - 1.0);
                let mut cursor = left;
                for (_, node) in &mut selected {
                    node.x = cursor + node.width * 0.5;
                    cursor += node.width + gap;
                }
            }
            "distributeVertical" => {
                if selected.len() < 3 {
                    return Ok(());
                }
                selected.sort_by(|a, b| a.1.y.partial_cmp(&b.1.y).unwrap_or(std::cmp::Ordering::Equal));
                let top = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).min_y).fold(f64::INFINITY, f64::min);
                let bottom = selected.iter().map(|(_, node)| Self::dag_node_world_bounds(node).max_y).fold(f64::NEG_INFINITY, f64::max);
                let total_height: f64 = selected.iter().map(|(_, node)| node.height).sum();
                let gap = (bottom - top - total_height) / (selected.len() as f64 - 1.0);
                let mut cursor = top;
                for (_, node) in &mut selected {
                    node.y = cursor + node.height * 0.5;
                    cursor += node.height + gap;
                }
            }
            other => return Err(format!("unknown align mode: {other}")),
        }
        for (idx, node) in selected {
            self.fixture.nodes[idx].x = node.x;
            self.fixture.nodes[idx].y = node.y;
            self.sync_fixture_node_center_to_engine(idx);
        }
        Ok(())
    }
    // #endregion 🔖SelectionAlign

    /// 📍 Sets a fixture widget position in both the fixture and engine snapshots.
    pub fn set_widget_position(&mut self, widget_id: &str, x: f64, y: f64) -> Result<(), String> {
        let idx = self
            .fixture
            .nodes
            .iter()
            .position(|node| node.id == widget_id)
            .ok_or_else(|| format!("unknown widget: {widget_id}"))?;
        self.fixture.nodes[idx].x = x;
        self.fixture.nodes[idx].y = y;
        let Some(nid) = self.engine_node_id_for_index(idx) else {
            return Ok(());
        };
        if let Some(node) = self.engine.nodes.get_mut(&nid) {
            node.center.x = x;
            node.center.y = y;
        }
        Ok(())
    }

    /// 🗑️ Deletes the current selection from the fixture.
    pub fn delete_selected(&mut self) {
        let widget_ids = self.selected_node_ids();
        self.engine.delete_selection();
        self.fixture.nodes.retain(|node| !widget_ids.contains(&node.id));
        self.sync_edges_from_engine();
        self.rebuild_engine_with_layout(false);
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

    /// 🔌 Sets hover to a fixture channel handle, falling back to node hover below channel LOD.
    pub fn set_hover_channel(&mut self, widget_id: Option<&str>, port_id: Option<&str>) {
        if widget_id.is_none() {
            self.set_hover(None);
            return;
        }
        let widget_id = widget_id.expect("widget id");
        if let Some(port_id) = port_id {
            if self.draw_lod_for_frame().uses_channel_row_pick() {
                if let Some(hid) = self.handle_id_for_port(widget_id, port_id) {
                    if self.engine.hover != Some(hid) {
                        self.engine.hover = Some(hid);
                    }
                    return;
                }
            }
        }
        self.set_hover(Some(widget_id));
    }

    /// 🔌 Replaces channel handle selection from fixture channel JSON, falling back to node selection below channel LOD.
    pub fn set_selected_channels_json(&mut self, json: &str) {
        let channels: Vec<DagChannelRef> = serde_json::from_str(json).unwrap_or_default();
        if self.draw_lod_for_frame().uses_channel_row_pick() {
            let mut selection = Selection::default();
            for channel in channels {
                if let Some(hid) = self.handle_id_for_port(&channel.widget_id, &channel.port) {
                    selection.handle_ids.insert(hid);
                }
            }
            self.engine.selection = selection;
            self.engine.preselect = Selection::default();
            self.engine.preselect_removed = Selection::default();
            return;
        }
        let widget_ids: Vec<String> = channels.into_iter().map(|channel| channel.widget_id).collect();
        self.set_selection(&widget_ids);
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

    /// ⚙️ Marks one actively computing widget and downstream widgets as stale.
    pub fn set_computing_progress(&mut self, active_widget_id: Option<&str>, stale_widget_ids: &[String]) {
        self.computing_active = active_widget_id.and_then(|widget_id| self.node_id_for_widget_id(widget_id));
        self.computing_stale.clear();
        for widget_id in stale_widget_ids {
            if let Some(nid) = self.node_id_for_widget_id(widget_id) {
                if self.computing_active != Some(nid) {
                    self.computing_stale.insert(nid);
                }
            }
        }
    }

    /// ✅ Clears evaluating chrome from all nodes.
    pub fn clear_computing(&mut self) {
        self.computing_active = None;
        self.computing_stale.clear();
    }

    fn tick_computing_animation(&self) {
        if self.computing_active.is_some() {
            let next = (self.computing_active_anim_phase.get() + 0.02) % 1.0;
            self.computing_active_anim_phase.set(next);
        }
        if !self.computing_stale.is_empty() {
            let next = (self.computing_stale_anim_phase.get() + 0.008) % 1.0;
            self.computing_stale_anim_phase.set(next);
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

    /// 👻 Sets or clears the placement preview node painted with the normal LOD path.
    pub fn set_ghost_node(&mut self, node: Option<DagNodeSpec>) {
        self.ghost_node = node;
    }

    pub fn ghost_node(&self) -> Option<&DagNodeSpec> {
        self.ghost_node.as_ref()
    }

    pub fn automatic_lod(&self) -> bool {
        self.automatic_lod
    }

    pub fn forced_draw_lod_label(&self) -> Option<&'static str> {
        self.forced_draw_lod.map(|lod| lod.label())
    }

    /// 🔍 Pins draw LOD while the wheel gesture is active so chrome does not flicker across bands.
    pub fn set_wheel_zoom_active(&mut self, active: bool) {
        if active && !self.wheel_zoom_active {
            self.wheel_zoom_render_lod = Some(self.resolved_draw_lod());
        } else if !active {
            self.wheel_zoom_render_lod = None;
        }
        self.wheel_zoom_active = active;
    }

    fn resolved_draw_lod(&self) -> DagDrawLod {
        if !self.automatic_lod {
            if let Some(lod) = self.forced_draw_lod {
                return lod;
            }
        }
        dag_draw_lod(self.fixture.camera.zoom)
    }

    fn draw_lod_for_frame(&self) -> DagDrawLod {
        if !self.automatic_lod {
            if let Some(lod) = self.forced_draw_lod {
                return lod;
            }
        }
        if self.wheel_zoom_active {
            if let Some(pinned) = self.wheel_zoom_render_lod {
                return pinned;
            }
        }
        dag_draw_lod(self.fixture.camera.zoom)
    }

    /// 📶 Active draw LOD tier label (`minimap`, `overview`, …).
    pub fn draw_lod_label(&self) -> &'static str {
        self.draw_lod_for_frame().label()
    }

    /// 📶 When true (default), camera zoom selects draw LOD; when false, optional `forced_draw_lod` pins the tier.
    pub fn set_automatic_lod(&mut self, enabled: bool) {
        self.automatic_lod = enabled;
        if enabled {
            self.forced_draw_lod = None;
        }
    }

    /// 🔗 World-space proximity radius for channel auto-connect; `0` disables snapping.
    pub fn set_proximity_distance(&mut self, world: f64) {
        self.engine.proximity_distance_world = world.max(0.0);
    }

    /// 📶 Pins WASM draw LOD when {@link DagHost::set_automatic_lod} is false; pass an empty label to follow zoom bands.
    pub fn set_forced_draw_lod_label(&mut self, label: &str) {
        let trimmed = label.trim();
        if trimmed.is_empty() {
            self.forced_draw_lod = None;
            return;
        }
        self.forced_draw_lod = DagDrawLod::from_id(trimmed);
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
                if let Some(handle) = self.engine.handles.get_mut(&hid) {
                    handle.radius = DAG_HANDLE_WORLD_RADIUS;
                }
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
                if let Some(handle) = self.engine.handles.get_mut(&hid) {
                    handle.radius = DAG_HANDLE_WORLD_RADIUS;
                }
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

    fn handle_id_for_port(&self, node_id: &str, port_id: &str) -> Option<HandleId> {
        let key = format!("{node_id}:{port_id}");
        self.handle_key_map.iter().find(|(_, k)| k.as_str() == key).map(|(&hid, _)| hid)
    }

    fn port_row_handle_hit(&self, world_x: f64, world_y: f64, inputs: bool, outputs: bool) -> Option<HandleId> {
        for node in self.fixture.nodes.iter().rev() {
            if inputs {
                for (port_idx, port) in node.inputs().iter().enumerate() {
                    let Some((x0, y0, x1, y1)) = input_port_row_hit_bounds(node, port_idx) else {
                        continue;
                    };
                    if !point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        continue;
                    }
                    if let Some(hid) = self.handle_id_for_port(&node.id, &port.id) {
                        return Some(hid);
                    }
                }
            }
            if outputs {
                for (port_idx, port) in node.outputs().iter().enumerate() {
                    let Some((x0, y0, x1, y1)) = output_port_row_hit_bounds(node, port_idx) else {
                        continue;
                    };
                    if !point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        continue;
                    }
                    if let Some(hid) = self.handle_id_for_port(&node.id, &port.id) {
                        return Some(hid);
                    }
                }
            }
        }
        None
    }

    fn channel_row_handle_hit(&self, world_x: f64, world_y: f64) -> Option<HandleId> {
        if !self.draw_lod_for_frame().uses_channel_row_pick() {
            return None;
        }
        self.port_row_handle_hit(world_x, world_y, true, true)
    }

    fn handle_anchor_hit(&self, world_x: f64, world_y: f64) -> Option<HandleId> {
        if !self.draw_lod_for_frame().allows_connection_hit_picking() {
            return None;
        }
        use cavas::vello::kurbo::Point;
        let p = Point::new(world_x, world_y);
        for (&hid, handle) in self.engine.handles.iter().rev() {
            let Some(node) = self.engine.nodes.get(&handle.node_id) else {
                continue;
            };
            let pos = handle_position(node, handle);
            let dx = p.x - pos.x;
            let dy = p.y - pos.y;
            let tol = handle.radius + 6.0;
            if dx * dx + dy * dy <= tol * tol {
                return Some(hid);
            }
        }
        None
    }

    fn fixture_draggable_node_hit(&self, world_x: f64, world_y: f64) -> Option<NodeId> {
        for idx in (0..self.fixture.nodes.len()).rev() {
            let node = &self.fixture.nodes[idx];
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            if world_x < node.x - hw
                || world_x > node.x + hw
                || world_y < node.y - hh
                || world_y > node.y + hh
            {
                continue;
            }
            let Some(nid) = self.engine_node_id_for_index(idx) else {
                continue;
            };
            if self.engine.nodes.get(&nid).is_some_and(|n| n.draggable) {
                return Some(nid);
            }
        }
        None
    }

    fn connection_hit_world(&self, world_x: f64, world_y: f64) -> (f64, f64) {
        let Some(hid) = self.handle_anchor_hit(world_x, world_y) else {
            return (world_x, world_y);
        };
        let Some(handle) = self.engine.handles.get(&hid) else {
            return (world_x, world_y);
        };
        let Some(node) = self.engine.nodes.get(&handle.node_id) else {
            return (world_x, world_y);
        };
        let pos = handle_position(node, handle);
        (pos.x, pos.y)
    }

    fn world_hits_handle(&self, world_x: f64, world_y: f64) -> bool {
        self.handle_anchor_hit(world_x, world_y).is_some()
    }

    fn sync_channel_row_pointer_hover(&mut self, world_x: f64, world_y: f64) {
        if !self.draw_lod_for_frame().uses_channel_row_pick() {
            return;
        }
        if !matches!(self.engine.interaction, InteractionMode::Idle) {
            return;
        }
        if self.handle_anchor_hit(world_x, world_y).is_some() {
            return;
        }
        if let Some(hid) = self.channel_row_handle_hit(world_x, world_y) {
            self.engine.hover = Some(hid);
        }
    }

    fn try_node_rectangle_pointer_down(&mut self, world_x: f64, world_y: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool) -> bool {
        if button != 0 || alt {
            return false;
        }
        let Some(node_id) = self.fixture_draggable_node_hit(world_x, world_y) else {
            return false;
        };
        use cavas::vello::kurbo::Point;
        use graph::pick_merge_mode_for_modifiers;

        let point = Point::new(world_x, world_y);
        self.engine.pointer_down_on_draggable_node_at(node_id, point, shift, ctrl_or_meta);
        if self.draw_lod_for_frame().uses_channel_row_pick() {
            if let Some(hid) = self.channel_row_handle_hit(world_x, world_y) {
                let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.engine.selection_options.mode.as_str());
                self.engine.select_handle_with_mode(hid, merge_mode.as_str());
                self.engine.hover = Some(hid);
            }
        }
        true
    }

    fn widget_hit_at(&self, world_x: f64, world_y: f64) -> Option<(usize, WidgetPointerKind)> {
        for idx in (0..self.fixture.nodes.len()).rev() {
            let node = &self.fixture.nodes[idx];
            match &node.kind {
                DagNodeKind::Slider { .. } => {
                    let (x0, y0, x1, y1) = slider_track_bounds(node);
                    if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        return Some((idx, WidgetPointerKind::SliderDrag));
                    }
                }
                DagNodeKind::Select { .. } => {
                    let (x0, y0, x1, y1) = select_control_bounds(node);
                    if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        return Some((idx, WidgetPointerKind::SelectClick));
                    }
                }
                DagNodeKind::Preview { content: DagPreviewContent::Tree { json }, expanded, .. } => {
                    for row in preview_tree_row_layouts(node, json, expanded) {
                        let (x0, y0, x1, y1) = row.row_rect;
                        if x1 > x0 && point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                            return Some((idx, WidgetPointerKind::PreviewToggle(row.path)));
                        }
                    }
                }
                DagNodeKind::Cluster { .. } => {
                    if cluster_explode_hit(node, world_x, world_y) {
                        return Some((idx, WidgetPointerKind::ClusterExplode));
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn sync_fixture_node_size_to_engine(&mut self, idx: usize) {
        let node = &self.fixture.nodes[idx];
        let Some(nid) = self.engine_node_id_for_index(idx) else {
            return;
        };
        let Some(engine_node) = self.engine.nodes.get_mut(&nid) else {
            return;
        };
        engine_node.width = node.width;
        engine_node.height = node.height;
    }

    fn toggle_preview_tree_path(node: &mut DagNodeSpec, path: &str) {
        let DagNodeKind::Preview { content, expanded, .. } = &mut node.kind else {
            return;
        };
        if !matches!(content, DagPreviewContent::Tree { .. }) {
            return;
        }
        if expanded.contains(path) {
            expanded.remove(path);
        } else {
            expanded.insert(path.to_string());
        }
        fit_node_size(node);
    }

    /// 📐 Recomputes preview and image node sizes after content changes.
    pub fn fit_preview_sizes(&mut self) {
        for idx in 0..self.fixture.nodes.len() {
            let kind = &self.fixture.nodes[idx].kind;
            if matches!(kind, DagNodeKind::Preview { .. } | DagNodeKind::Image { .. }) {
                fit_node_size(&mut self.fixture.nodes[idx]);
                self.sync_fixture_node_size_to_engine(idx);
            }
        }
    }

    /// 📐 Recomputes note node sizes after text changes.
    pub fn fit_note_sizes(&mut self) {
        for idx in 0..self.fixture.nodes.len() {
            if matches!(self.fixture.nodes[idx].kind, DagNodeKind::Note { .. }) {
                fit_node_size(&mut self.fixture.nodes[idx]);
                self.sync_fixture_node_size_to_engine(idx);
            }
        }
    }

    fn try_widget_pointer_down(&mut self, world_x: f64, world_y: f64) -> bool {
        if !self.draw_lod_for_frame().shows_controls() {
            return false;
        }
        let Some((idx, kind)) = self.widget_hit_at(world_x, world_y) else {
            return false;
        };
        match kind {
            WidgetPointerKind::SliderDrag => {
                let node_id = self.fixture.nodes[idx].id.clone();
                self.widget_drag = Some(idx);
                if let Some(value) = set_slider_value_from_x(&mut self.fixture.nodes[idx], world_x) {
                    dag_debug_log(&format!("[DEBUG] dag slider value id={node_id} value={value:.3}"));
                }
            }
            WidgetPointerKind::SelectClick => {
                let node_id = self.fixture.nodes[idx].id.clone();
                if let Some(label) = advance_select_option(&mut self.fixture.nodes[idx]) {
                    dag_debug_log(&format!("[DEBUG] dag select option id={node_id} label={label}"));
                }
            }
            WidgetPointerKind::PreviewToggle(path) => {
                Self::toggle_preview_tree_path(&mut self.fixture.nodes[idx], &path);
                self.sync_fixture_node_size_to_engine(idx);
            }
            WidgetPointerKind::ClusterExplode => {
                self.pending_cluster_explode = Some(self.fixture.nodes[idx].id.clone());
            }
        }
        true
    }

    /// 💥 Takes a pending cluster explode request from the last widget hit.
    pub fn take_pending_cluster_explode(&mut self) -> Option<String> {
        self.pending_cluster_explode.take()
    }

    fn sync_connection_hit_picking_for_lod(&mut self) {
        let allow = self.draw_lod_for_frame().allows_connection_hit_picking();
        self.engine.handle_pointer_picking = allow;
        if !allow && matches!(self.engine.interaction, InteractionMode::DrawEdge { .. }) {
            self.engine.interaction = InteractionMode::Idle;
        }
    }

    /// @emoji 🧭 Minimap LOD: pointer-down inside the selection AABB moves the group without a discrete hit.
    fn lod_uses_bounded_drag(&self) -> bool {
        matches!(self.draw_lod_for_frame(), DagDrawLod::Minimap)
    }

    pub fn pointer_down(&mut self, x: f64, y: f64, extend: bool) {
        self.pointer_down_screen(x, y, 0, extend, false, false);
    }

    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.sync_connection_hit_picking_for_lod();
        self.last_screen_x = sx;
        self.last_screen_y = sy;
        let world = self.screen_to_world_point(sx, sy);
        if let Some(hit) = self.port_insert_hit(world.x, world.y, self.fixture.camera.zoom) {
            self.pending_port_insert = Some(hit);
            return;
        }
        let (hit_x, hit_y) = self.connection_hit_world(world.x, world.y);
        if self.world_hits_handle(hit_x, hit_y) {
            self.engine.pointer_down_screen(sx, sy, hit_x, hit_y, button, shift, ctrl_or_meta, alt);
            self.process_engine_events();
            self.sync_camera_from_engine();
            return;
        }
        if self.try_widget_pointer_down(world.x, world.y) {
            return;
        }
        if self.try_node_rectangle_pointer_down(world.x, world.y, button, shift, ctrl_or_meta, alt) {
            self.process_engine_events();
            self.sync_camera_from_engine();
            return;
        }
        let merge_from_modifiers = ctrl_or_meta || shift;
        if button == 0
            && !merge_from_modifiers
            && self.lod_uses_bounded_drag()
        {
            let pad = DAG_BOUNDED_DRAG_HIT_PAD_PX / self.fixture.camera.zoom.max(1e-9);
            if self.engine.try_begin_selection_union_drag_at(world, pad) {
                self.process_engine_events();
                self.sync_camera_from_engine();
                return;
            }
        }
        self.engine.pointer_down_screen(sx, sy, hit_x, hit_y, button, shift, ctrl_or_meta, alt);
        self.process_engine_events();
        self.sync_camera_from_engine();
    }

    pub fn widget_drag_active(&self) -> bool {
        self.widget_drag.is_some()
    }

    pub fn pointer_move(&mut self, x: f64, y: f64) {
        self.pointer_move_screen(x, y, false, false, false);
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.sync_connection_hit_picking_for_lod();
        self.last_screen_x = sx;
        self.last_screen_y = sy;
        let world = self.screen_to_world_point(sx, sy);
        if let Some(idx) = self.widget_drag {
            if let Some(value) = set_slider_value_from_x(&mut self.fixture.nodes[idx], world.x) {
                dag_debug_log(&format!("[DEBUG] dag slider value id={} value={value:.3}", self.fixture.nodes[idx].id));
            }
            return;
        }
        let (hit_x, hit_y) = self.connection_hit_world(world.x, world.y);
        self.engine.pointer_move_screen(sx, sy, hit_x, hit_y, shift, ctrl_or_meta, alt);
        self.sync_channel_row_pointer_hover(world.x, world.y);
        self.sync_minimap_pointer_hover(world.x, world.y);
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
        self.sync_connection_hit_picking_for_lod();
        self.last_screen_x = sx;
        self.last_screen_y = sy;
        if self.widget_drag.take().is_some() {
            return;
        }
        let world = self.screen_to_world_point(sx, sy);
        let (hit_x, hit_y) = self.connection_hit_world(world.x, world.y);
        self.engine.pointer_up_screen(sx, sy, hit_x, hit_y, shift, ctrl_or_meta, alt);
        self.process_engine_events();
        self.sync_node_positions_from_engine();
        self.sync_camera_from_engine();
    }

    pub fn set_vello_theme_from_json(&mut self, json: &str) -> Result<(), String> {
        self.vello_theme.merge_from_json(json)?;
        self.icon_paint_cache.clear();
        Ok(())
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

    fn node_handle_centers(node: &DagNodeSpec) -> Vec<cavas::vello::kurbo::Point> {
        use cavas::vello::kurbo::Point;
        use graph::handle_position_on_rectangle;
        let center = Point::new(node.x, node.y);
        let mut centers = Vec::new();
        for port_idx in 0..node.inputs().len() {
            let angle = io_node_rect_port_angle_for_node(node, port_idx, true);
            centers.push(handle_position_on_rectangle(center, node.width, node.height, angle));
        }
        for port_idx in 0..node.outputs().len() {
            let angle = io_node_rect_port_angle_for_node(node, port_idx, false);
            centers.push(handle_position_on_rectangle(center, node.width, node.height, angle));
        }
        centers
    }

    fn paint_node_handles_for_spec(
        &self,
        scene: &mut cavas::vello::Scene,
        aff: &cavas::vello::kurbo::Affine,
        cam: &cavas::camera::Camera,
        node: &DagNodeSpec,
        chrome: &DagNodePaintChrome,
    ) {
        use cavas::vello::kurbo::{Circle, Point};
        use cavas::vello::peniko::Fill;
        use cavas::vello::kurbo::Stroke;
        use graph::{handle_exterior_cap_fill_path, handle_exterior_cap_stroke_path, handle_outward_at_node_rim, NodeShape};

        let theme = &self.vello_theme;
        let handle_stroke_px = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX, cam.zoom);
        let tint = chrome.tint_highlighted();
        let fill = dag_handle_body_fill(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
        let stroke_c = dag_handle_body_stroke(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
        let handle_chrome = chrome.has_interaction_chrome();
        let center = Point::new(node.x, node.y);
        for handle_center in Self::node_handle_centers(node) {
            let outward = handle_outward_at_node_rim(handle_center, center, NodeShape::Rectangle, 0.0, node.width, node.height);
            if let Some(out) = outward {
                if handle_chrome {
                    scene.fill(
                        Fill::NonZero,
                        *aff,
                        fill,
                        None,
                        &handle_exterior_cap_fill_path(handle_center, out, DAG_HANDLE_WORLD_RADIUS),
                    );
                }
                scene.stroke(
                    &Stroke::new(handle_stroke_px),
                    *aff,
                    stroke_c,
                    None,
                    &handle_exterior_cap_stroke_path(handle_center, out, DAG_HANDLE_WORLD_RADIUS),
                );
            } else {
                let circle = Circle::new(handle_center, DAG_HANDLE_WORLD_RADIUS);
                if handle_chrome {
                    scene.fill(Fill::NonZero, *aff, fill, None, &circle);
                }
                scene.stroke(&Stroke::new(handle_stroke_px), *aff, stroke_c, None, &circle);
            }
        }
    }

    pub fn label_overlay_rows_for_node_spec(&self, node: &DagNodeSpec, ghost: bool) -> Vec<serde_json::Value> {
        let lod = self.draw_lod_for_frame();
        let zoom = self.fixture.camera.zoom;
        let lod_index = dag_lod_index(zoom);
        Self::label_overlay_rows_for_node(node, lod, zoom, lod_index, ghost)
    }

    fn label_overlay_rows_for_node(
        node: &DagNodeSpec,
        lod: DagDrawLod,
        zoom: f64,
        lod_index: usize,
        ghost: bool,
    ) -> Vec<serde_json::Value> {
        let paint_px = dag_label_paint_px(zoom, lod_index);
        let mut labels = Vec::new();
        if let Some(text) = Self::node_label_text(node, lod).map(str::to_string) {
            let (layout, x, y) = if lod.node_label_is_horizontal() {
                ("horizontal", node.x, node.y)
            } else if uses_computation_layout(&node.kind) && lod.shows_computation_layout() {
                let (lx, ly) = computation_name_world_center(node, &text, paint_px, zoom);
                ("horizontal", lx, ly)
            } else if matches!(node.kind, DagNodeKind::Slider { .. }) && lod.shows_controls() {
                let (lx, ly) = computation_name_world_center(node, &text, paint_px, zoom);
                ("horizontal", lx, ly)
            } else {
                let (lx, ly) = io_widget_label_center(node);
                ("vertical", lx, ly)
            };
            labels.push(serde_json::json!({
                "id": node.id,
                "text": text,
                "layout": layout,
                "x": x,
                "y": y,
                "nodeW": node.width,
                "nodeH": node.height,
                "fontScreenPx": paint_px,
                "ghost": ghost,
            }));
        }
        if lod.shows_port_labels() && !matches!(node.kind, DagNodeKind::Preview { .. } | DagNodeKind::Note { .. }) {
            for mut row in Self::port_label_overlay_rows(node, zoom, lod_index) {
                if let Some(obj) = row.as_object_mut() {
                    obj.insert("ghost".into(), serde_json::Value::Bool(ghost));
                }
                labels.push(row);
            }
        }
        labels
    }

    fn port_label_overlay_rows(node: &DagNodeSpec, zoom: f64, lod_index: usize) -> Vec<serde_json::Value> {
        use cavas::text::label_extent;
        let hw = node.width * 0.5;
        let handle_inset = 8.0 / zoom.max(0.05);
        let inputs = node.inputs();
        let outputs = node.outputs();
        let computation = uses_computation_layout(&node.kind);
        let port_layout_px = if computation {
            dag_label_compact_paint_px(zoom, lod_index)
        } else {
            dag_label_paint_px(zoom, lod_index)
        };
        let mut rows = Vec::new();
        let input_column_w = if computation {
            io_port_column_width(&inputs, port_layout_px)
        } else {
            (hw - handle_inset).max(8.0)
        };
        let output_column_w = if computation {
            io_port_column_width(&outputs, port_layout_px)
        } else {
            (hw - handle_inset).max(8.0)
        };
        for (i, port) in inputs.iter().enumerate() {
            let label = port.display_code().trim();
            if label.is_empty() {
                continue;
            }
            let world_y = port_center_y(node, i, inputs.len());
            let world_x = if computation {
                computation_input_label_x(node)
            } else {
                node.x - hw + handle_inset
            };
            rows.push(serde_json::json!({
                "id": node.id,
                "text": label,
                "layout": "horizontal",
                "align": "left",
                "x": world_x,
                "y": world_y,
                "nodeW": input_column_w,
                "nodeH": DAG_CHANNEL_ROW_HEIGHT,
                "fontScreenPx": port_layout_px,
            }));
        }
        for (i, port) in outputs.iter().enumerate() {
            let label = port.display_code().trim();
            if label.is_empty() {
                continue;
            }
            let world_y = port_center_y(node, i, outputs.len());
            let (world_x, column_w) = if computation {
                let left = computation_output_label_x(node, label, port_layout_px);
                (left + port_label_text_width(label, port_layout_px), output_column_w)
            } else {
                let (label_w, _) = label_extent(label, port_layout_px);
                (node.x + hw - handle_inset, label_w / zoom.max(0.05))
            };
            rows.push(serde_json::json!({
                "id": node.id,
                "text": label,
                "layout": "horizontal",
                "align": "right",
                "x": world_x,
                "y": world_y,
                "nodeW": column_w,
                "nodeH": DAG_CHANNEL_ROW_HEIGHT,
                "fontScreenPx": port_layout_px,
            }));
        }
        rows
    }

    fn is_editable_input_port(port: &IoPortSpec) -> bool {
        port.connected == Some(false)
            && matches!(port.value_type.as_deref(), Some("number" | "integer" | "text" | "boolean"))
    }

    fn param_overlay_rows_for_node(node: &DagNodeSpec) -> Vec<serde_json::Value> {
        let inputs = match &node.kind {
            DagNodeKind::Computation { inputs, .. } | DagNodeKind::Cluster { inputs, .. } => inputs,
            _ => return Vec::new(),
        };
        let mut rows = Vec::new();
        let hw = node.width * 0.5;
        let editor_w = (hw - DAG_NODE_EDGE_INSET).max(24.0);
        for (index, port) in inputs.iter().enumerate() {
            if !Self::is_editable_input_port(port) {
                continue;
            }
            let world_y = computation_port_center_y(node, index);
            let world_x = node.x - hw + DAG_NODE_EDGE_INSET + editor_w * 0.5;
            rows.push(serde_json::json!({
                "nodeId": node.id,
                "portId": port.id,
                "type": port.value_type,
                "value": port.value,
                "default": port.default,
                "x": world_x,
                "y": world_y,
                "w": editor_w,
                "h": DAG_CHANNEL_ROW_HEIGHT,
            }));
        }
        rows
    }

    /// 🎛️ Inline default editor anchors for unconnected primitive neuron inputs.
    pub fn param_overlay_paint_state_json(&self) -> Result<String, String> {
        let cam = &self.fixture.camera;
        let mut editors = Vec::new();
        for (idx, fixture_node) in self.fixture.nodes.iter().enumerate() {
            let node = self.node_spec_for_paint(idx, fixture_node);
            editors.extend(Self::param_overlay_rows_for_node(node.as_ref()));
        }
        if let Some(ghost) = self.ghost_node.as_ref() {
            editors.extend(Self::param_overlay_rows_for_node(ghost));
        }
        serde_json::to_string(&serde_json::json!({
            "camera": { "x": cam.x, "y": cam.y, "zoom": cam.zoom },
            "width": self.width,
            "height": self.height,
            "editors": editors,
        }))
        .map_err(|err| err.to_string())
    }

    /// 🏷️ Camera, draw LOD, and node label anchors for the JS canvas text overlay (must match the last GPU frame).
    pub fn label_overlay_paint_state_json(&self) -> Result<String, String> {
        let lod = self.draw_lod_for_frame();
        let cam = &self.fixture.camera;
        let lod_index = dag_lod_index(cam.zoom);
        let mut labels = Vec::new();
        for (idx, fixture_node) in self.fixture.nodes.iter().enumerate() {
            let node = self.node_spec_for_paint(idx, fixture_node);
            labels.extend(Self::label_overlay_rows_for_node(node.as_ref(), lod, cam.zoom, lod_index, false));
        }
        if let Some(ghost) = self.ghost_node.as_ref() {
            labels.extend(Self::label_overlay_rows_for_node(ghost, lod, cam.zoom, lod_index, true));
        }
        serde_json::to_string(&serde_json::json!({
            "camera": { "x": cam.x, "y": cam.y, "zoom": cam.zoom },
            "lod": lod.label(),
            "width": self.width,
            "height": self.height,
            "labels": labels,
        }))
        .map_err(|e| e.to_string())
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
        lod: DagDrawLod,
        layout_px: f64,
        paint_px: f64,
        lod_index: usize,
        label_fill: cavas::vello::peniko::Color,
        label_halo: cavas::vello::peniko::Color,
    ) {
        if Self::port_labels_delegated_to_js_overlay(lod) {
            return;
        }
        use cavas::camera::world_to_screen;
        use cavas::text::{append_label, label_extent};
        use cavas::vello::kurbo::Point;
        let hw = node.width * 0.5;
        let handle_inset = 8.0 / cam.zoom.max(0.05);
        let inputs = node.inputs();
        let outputs = node.outputs();
        let computation = uses_computation_layout(&node.kind);
        let port_layout_px = if computation { DAG_LABEL_COMPACT_SCREEN_PX } else { layout_px };
        let port_paint_px = if computation {
            dag_label_compact_paint_px(cam.zoom, lod_index)
        } else {
            paint_px
        };
        for (i, port) in inputs.iter().enumerate() {
            let world_y = port_center_y(node, i, inputs.len());
            let world_x = if computation {
                computation_input_label_x(node)
            } else {
                node.x - hw + handle_inset
            };
            append_label(
                scene,
                port.display_code(),
                world_to_screen(cam, viewport, Point::new(world_x, world_y)),
                port_paint_px,
                label_fill,
                label_halo,
            );
        }
        for (i, port) in outputs.iter().enumerate() {
            let world_y = port_center_y(node, i, outputs.len());
            let world_x = if computation {
                computation_output_label_x(node, port.display_code(), port_layout_px)
            } else {
                let (label_w, _) = label_extent(port.display_code(), layout_px);
                node.x + hw - handle_inset - label_w / cam.zoom.max(0.05)
            };
            append_label(
                scene,
                port.display_code(),
                world_to_screen(cam, viewport, Point::new(world_x, world_y)),
                port_paint_px,
                label_fill,
                label_halo,
            );
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
        let (w, h) = label_extent(trimmed, px);
        let mut label_scene = cavas::vello::Scene::new();
        append_label(&mut label_scene, trimmed, Point::new(0.0, 0.0), px, label_fill, label_halo);
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

    fn paint_computation_column_divider(
        scene: &mut cavas::vello::Scene,
        aff: cavas::vello::kurbo::Affine,
        node: &DagNodeSpec,
        chrome_stroke: f64,
        stroke: cavas::vello::peniko::Color,
    ) {
        use cavas::vello::kurbo::{Line, Point, Stroke};
        let Some(divider_x) = computation_column_divider_x(node) else {
            return;
        };
        let hh = node.height * 0.5;
        let top = node.y - hh;
        let bottom = node.y + hh;
        let stroke_style = Stroke::new(chrome_stroke);
        scene.stroke(
            &stroke_style,
            aff,
            stroke,
            None,
            &Line::new(Point::new(divider_x, top), Point::new(divider_x, bottom)),
        );
    }

    fn paint_computation_channel_row_highlights(
        &self,
        scene: &mut cavas::vello::Scene,
        aff: &cavas::vello::kurbo::Affine,
        node: &DagNodeSpec,
        theme: &VelloThemePalette,
        is_dimmed: bool,
    ) {
        use cavas::vello::kurbo::Rect;
        use cavas::vello::peniko::Fill;
        let mut paint_bounds = |(x0, y0, x1, y1): (f64, f64, f64, f64), selected: bool, highlighted: bool, hovered: bool| {
            if !selected && !highlighted && !hovered {
                return;
            }
            let fill = dag_handle_body_fill(theme, is_dimmed, selected, highlighted, hovered);
            scene.fill(
                Fill::NonZero,
                *aff,
                fill,
                None,
                &Rect::new(x0, y0, x1, y1),
            );
        };
        for (port_idx, port) in node.inputs().iter().enumerate() {
            let Some(bounds) = input_port_row_hit_bounds(node, port_idx) else {
                continue;
            };
            let Some(hid) = self.handle_id_for_port(&node.id, &port.id) else {
                continue;
            };
            let (selected, highlighted, hovered) = self.handle_interaction_chrome(hid);
            paint_bounds(bounds, selected, highlighted, hovered);
        }
        for (port_idx, port) in node.outputs().iter().enumerate() {
            let Some(bounds) = output_port_row_hit_bounds(node, port_idx) else {
                continue;
            };
            let Some(hid) = self.handle_id_for_port(&node.id, &port.id) else {
                continue;
            };
            let (selected, highlighted, hovered) = self.handle_interaction_chrome(hid);
            paint_bounds(bounds, selected, highlighted, hovered);
        }
    }

    fn computation_channel_row_divider_stroke(
        &self,
        node: &DagNodeSpec,
        port_id: &str,
        body_stroke: cavas::vello::peniko::Color,
        label_fill: cavas::vello::peniko::Color,
        default_stroke: cavas::vello::peniko::Color,
    ) -> cavas::vello::peniko::Color {
        let Some(hid) = self.handle_id_for_port(&node.id, port_id) else {
            return default_stroke;
        };
        let (selected, _, hovered) = self.handle_interaction_chrome(hid);
        if selected || hovered {
            dag_node_internal_chrome_stroke(body_stroke, label_fill, true)
        } else {
            default_stroke
        }
    }

    fn paint_computation_channel_row_dividers(
        &self,
        scene: &mut cavas::vello::Scene,
        aff: cavas::vello::kurbo::Affine,
        node: &DagNodeSpec,
        chrome_stroke: f64,
        stroke: cavas::vello::peniko::Color,
        body_stroke: cavas::vello::peniko::Color,
        label_fill: cavas::vello::peniko::Color,
        channel_row_pick: bool,
    ) {
        use cavas::vello::kurbo::{Line, Point, Stroke};
        let grid_rows = computation_channel_row_count(node);
        if grid_rows <= 1 {
            return;
        }
        let (input_rows, output_rows) = computation_io_side_row_counts(node);
        let stroke_style = Stroke::new(chrome_stroke);
        let row_stroke = |port_id: &str| {
            if channel_row_pick {
                self.computation_channel_row_divider_stroke(node, port_id, body_stroke, label_fill, stroke)
            } else {
                stroke
            }
        };
        if computation_input_column_x_bounds(node).is_some() {
            let (left, right) = computation_channel_row_divider_x_span(node, ComputationChannelRowSide::Input);
            let inputs = node.inputs();
            for row in computation_io_side_row_divider_indices(input_rows, grid_rows) {
                let y = channel_row_divider_y(node.y, node.height, row);
                let port_id = inputs.get(row.saturating_sub(1)).map(|port| port.id.as_str()).unwrap_or("");
                scene.stroke(
                    &stroke_style,
                    aff,
                    row_stroke(port_id),
                    None,
                    &Line::new(Point::new(left, y), Point::new(right, y)),
                );
            }
        }
        if computation_output_column_x_bounds(node).is_some() {
            let (left, right) = computation_channel_row_divider_x_span(node, ComputationChannelRowSide::Output);
            let outputs = node.outputs();
            for row in computation_io_side_row_divider_indices(output_rows, grid_rows) {
                let y = channel_row_divider_y(node.y, node.height, row);
                let port_id = outputs.get(row.saturating_sub(1)).map(|port| port.id.as_str()).unwrap_or("");
                scene.stroke(
                    &stroke_style,
                    aff,
                    row_stroke(port_id),
                    None,
                    &Line::new(Point::new(left, y), Point::new(right, y)),
                );
            }
        }
    }

    fn paint_preview_image_content(
        &self,
        scene: &mut cavas::vello::Scene,
        cam: &cavas::camera::Camera,
        viewport: &cavas::camera::Viewport,
        node: &DagNodeSpec,
        src: &str,
        label_fill: cavas::vello::peniko::Color,
        bg: cavas::vello::peniko::Color,
    ) {
        use cavas::camera::world_to_screen;
        use cavas::text::append_label;
        use cavas::vello::kurbo::Point;
        if src.is_empty() {
            let (x0, y0, x1, y1) = preview_content_bounds(node);
            let pos = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
            append_label(scene, "Image…", pos, DAG_LABEL_COMPACT_SCREEN_PX, label_fill, bg);
            return;
        }
        let (x0, y0, x1, y1) = preview_content_bounds(node);
        let center = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
        let w = (x1 - x0).max(1.0);
        let h = (y1 - y0).max(1.0);
        self.icon_paint_cache.append_icon_at_screen_rect(scene, src, center, w, h, label_fill, bg, true);
    }

    fn paint_preview_content(
        &self,
        scene: &mut cavas::vello::Scene,
        cam: &cavas::camera::Camera,
        viewport: &cavas::camera::Viewport,
        node: &DagNodeSpec,
        content: &DagPreviewContent,
        expanded: &BTreeSet<String>,
        paint_px: f64,
        label_fill: cavas::vello::peniko::Color,
        label_halo: cavas::vello::peniko::Color,
        bg: cavas::vello::peniko::Color,
    ) {
        use cavas::camera::world_to_screen;
        use cavas::text::append_label;
        use cavas::vello::kurbo::Point;
        match content {
            DagPreviewContent::Empty => {
                let (x0, y0, x1, y1) = preview_content_bounds(node);
                let pos = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
                append_label(scene, "—", pos, paint_px, label_fill, label_halo);
            }
            DagPreviewContent::Scalar { text } => {
                let (x0, y0, x1, y1) = preview_content_bounds(node);
                let display = if text.is_empty() { "—" } else { text.as_str() };
                let pos = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
                append_label(scene, display, pos, paint_px * 1.05, label_fill, label_halo);
            }
            DagPreviewContent::Image { src } => {
                self.paint_preview_image_content(scene, cam, viewport, node, src, label_fill, bg);
            }
            DagPreviewContent::Tree { json } => {
                let (x0, y0, _, _) = preview_content_bounds(node);
                let rows = preview_tree_rows(json, expanded, "", 0);
                for (index, row) in rows.iter().enumerate() {
                    let row_y = y0 + index as f64 * DAG_PREVIEW_ROW_HEIGHT + DAG_PREVIEW_ROW_HEIGHT * 0.5;
                    let indent = row.depth as f64 * DAG_PREVIEW_TREE_INDENT;
                    if row.has_children {
                        let glyph = if row.expanded { "▾" } else { "▸" };
                        let toggle_pos = world_to_screen(cam, viewport, Point::new(x0 + indent + DAG_PREVIEW_TOGGLE_WIDTH * 0.5, row_y));
                        append_label(scene, glyph, toggle_pos, paint_px * 0.9, label_fill, label_halo);
                    }
                    let text_x = x0 + indent + if row.has_children { DAG_PREVIEW_TOGGLE_WIDTH } else { 0.0 } + 2.0;
                    let line = if row.has_children && !row.expanded {
                        format!("{}: {}", row.label, row.summary)
                    } else if !row.has_children {
                        format!("{}: {}", row.label, row.summary)
                    } else {
                        row.label.clone()
                    };
                    let text_pos = world_to_screen(cam, viewport, Point::new(text_x, row_y));
                    append_label(scene, &line, text_pos, paint_px * 0.9, label_fill, label_halo);
                }
            }
        }
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

    fn node_label_text<'a>(node: &'a DagNodeSpec, lod: DagDrawLod) -> Option<&'a str> {
        if matches!(node.kind, DagNodeKind::Preview { .. } | DagNodeKind::Note { .. }) {
            return None;
        }
        let text = match lod.node_label() {
            DagNodeLabel::None => return None,
            DagNodeLabel::Abbreviation => node.abbreviation.trim(),
            DagNodeLabel::Name => node.name.trim(),
        };
        if text.is_empty() { None } else { Some(text) }
    }

    fn node_caption_delegated_to_js_overlay(node: &DagNodeSpec, lod: DagDrawLod) -> bool {
        Self::node_label_text(node, lod).is_some()
    }

    fn port_labels_delegated_to_js_overlay(lod: DagDrawLod) -> bool {
        lod.shows_port_labels()
    }

    fn should_paint_node_lod_icon(node: &DagNodeSpec, lod: DagDrawLod) -> bool {
        if !lod.node_icon_visible() {
            return false;
        }
        !uses_computation_layout(&node.kind) || !lod.shows_computation_layout()
    }

    fn paint_node_lod_icon(
        &self,
        scene: &mut cavas::vello::Scene,
        lod: DagDrawLod,
        center_screen: cavas::vello::kurbo::Point,
        node: &DagNodeSpec,
        zoom: f64,
        fg: cavas::vello::peniko::Color,
        bg: cavas::vello::peniko::Color,
    ) {
        if !Self::should_paint_node_lod_icon(node, lod) {
            return;
        }
        let icon = node.icon.trim();
        if icon.is_empty() {
            return;
        }
        let screen_w = node.width * zoom.max(0.05);
        let screen_h = node.height * zoom.max(0.05);
        self.icon_paint_cache.append_icon_at_screen_rect(scene, icon, center_screen, screen_w, screen_h, fg, bg, false);
    }

    fn paint_cluster_affordances(
        scene: &mut cavas::vello::Scene,
        cam: &cavas::camera::Camera,
        viewport: &cavas::camera::Viewport,
        node: &DagNodeSpec,
        paint_px: f64,
        label_fill: cavas::vello::peniko::Color,
        label_halo: cavas::vello::peniko::Color,
    ) {
        use cavas::camera::world_to_screen;
        use cavas::text::append_label;
        use cavas::vello::kurbo::Point;
        let (name_x, name_y) = computation_name_world_center(node, &node.name, paint_px, cam.zoom);
        let glyph_pos = world_to_screen(cam, viewport, Point::new(name_x - paint_px * 0.55, name_y));
        append_label(scene, "🧩", glyph_pos, paint_px * 0.85, label_fill, label_halo);
        if let Some((x0, y0, x1, y1)) = cluster_explode_hit_rect(node) {
            let cx = (x0 + x1) * 0.5;
            let cy = (y0 + y1) * 0.5;
            let explode_pos = world_to_screen(cam, viewport, Point::new(cx, cy));
            append_label(scene, "⤢", explode_pos, paint_px * 0.75, label_fill, label_halo);
        }
    }

    fn paint_computation_node_name(
        scene: &mut cavas::vello::Scene,
        cam: &cavas::camera::Camera,
        viewport: &cavas::camera::Viewport,
        node: &DagNodeSpec,
        label: &str,
        px: f64,
        label_fill: cavas::vello::peniko::Color,
        label_halo: cavas::vello::peniko::Color,
    ) {
        use cavas::camera::world_to_screen;
        use cavas::vello::kurbo::Point;
        let (label_x, label_y) = computation_name_world_center(node, label, px, cam.zoom);
        let anchor = world_to_screen(cam, viewport, Point::new(label_x, label_y));
        Self::paint_node_name_horizontal(scene, anchor, label, px, label_fill, label_halo);
    }

    fn paint_slider_name(
        scene: &mut cavas::vello::Scene,
        cam: &cavas::camera::Camera,
        viewport: &cavas::camera::Viewport,
        node: &DagNodeSpec,
        label: &str,
        px: f64,
        label_fill: cavas::vello::peniko::Color,
        label_halo: cavas::vello::peniko::Color,
    ) {
        Self::paint_computation_node_name(scene, cam, viewport, node, label, px, label_fill, label_halo);
    }

    fn paint_io_widget_name(
        scene: &mut cavas::vello::Scene,
        cam: &cavas::camera::Camera,
        viewport: &cavas::camera::Viewport,
        node: &DagNodeSpec,
        lod: DagDrawLod,
        label: &str,
        px: f64,
        label_fill: cavas::vello::peniko::Color,
        label_halo: cavas::vello::peniko::Color,
    ) {
        use cavas::camera::world_to_screen;
        use cavas::vello::kurbo::Point;
        if lod.node_label() == DagNodeLabel::None && !lod.shows_controls() {
            return;
        }
        let (label_x, label_y) = io_widget_label_center(node);
        let name_anchor = world_to_screen(cam, viewport, Point::new(label_x, label_y));
        Self::paint_node_name_vertical(scene, name_anchor, label, px, label_fill, label_halo);
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

    fn paint_computing_border_arc(
        &self,
        scene: &mut cavas::vello::Scene,
        aff: &cavas::vello::kurbo::Affine,
        rect: &cavas::vello::kurbo::Rect,
        cam_zoom: f64,
        color: cavas::vello::peniko::Color,
        start_t: f64,
        dashed: bool,
    ) {
        use cavas::vello::kurbo::{BezPath, Stroke};
        const SEGMENTS: usize = 40;
        const ARC_FRACTION: f64 = 0.24;
        let mut path = BezPath::new();
        for i in 0..=SEGMENTS {
            let local = i as f64 / SEGMENTS as f64;
            let t = (start_t + local * ARC_FRACTION).fract();
            let p = Self::rect_perimeter_point(rect, t);
            if i == 0 {
                path.move_to(p);
            } else {
                path.line_to(p);
            }
        }
        let stroke_px = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX * 1.75, cam_zoom);
        let mut stroke = Stroke::new(stroke_px);
        if dashed {
            stroke.dash_pattern = vec![stroke_px * 2.5, stroke_px * 2.0].into();
        }
        scene.stroke(&stroke, *aff, color, None, &path);
    }

    fn paint_computing_active_border(
        &self,
        scene: &mut cavas::vello::Scene,
        aff: &cavas::vello::kurbo::Affine,
        rect: &cavas::vello::kurbo::Rect,
        cam_zoom: f64,
        theme: &VelloThemePalette,
    ) {
        self.paint_computing_border_arc(
            scene,
            aff,
            rect,
            cam_zoom,
            theme.node_stroke_selected,
            self.computing_active_anim_phase.get(),
            false,
        );
    }

    fn paint_computing_stale_border(
        &self,
        scene: &mut cavas::vello::Scene,
        aff: &cavas::vello::kurbo::Affine,
        rect: &cavas::vello::kurbo::Rect,
        cam_zoom: f64,
        theme: &VelloThemePalette,
    ) {
        let highlight = vello_color_with_alpha(theme.node_stroke_selected, 220);
        self.paint_computing_border_arc(scene, aff, rect, cam_zoom, highlight, self.computing_stale_anim_phase.get(), true);
    }

    fn rect_perimeter_point(rect: &cavas::vello::kurbo::Rect, t: f64) -> cavas::vello::kurbo::Point {
        use cavas::vello::kurbo::Point;
        let t = t.fract();
        let w = rect.width();
        let h = rect.height();
        let perim = 2.0 * (w + h);
        let mut d = t * perim;
        if d <= w {
            return Point::new(rect.x0 + d, rect.y0);
        }
        d -= w;
        if d <= h {
            return Point::new(rect.x1, rect.y0 + d);
        }
        d -= h;
        if d <= w {
            return Point::new(rect.x1 - d, rect.y1);
        }
        d -= w;
        Point::new(rect.x0, rect.y1 - d)
    }

    fn paint_node_visual(
        &self,
        scene: &mut cavas::vello::Scene,
        aff: &cavas::vello::kurbo::Affine,
        cam: &cavas::camera::Camera,
        viewport: &cavas::camera::Viewport,
        lod: DagDrawLod,
        lod_index: usize,
        node: &DagNodeSpec,
        widget_drag_idx: Option<usize>,
        chrome: DagNodePaintChrome,
    ) {
        use cavas::camera::world_to_screen;
        use cavas::text::append_label;
        use cavas::vello::kurbo::{Circle, Line, Point, Rect, Stroke};
        use cavas::vello::peniko::Fill;

        let theme = &self.vello_theme;
        let label_halo = theme.label_halo;
        let hw = node.width * 0.5;
        let hh = node.height * 0.5;
        let rect = Rect::new(node.x - hw, node.y - hh, node.x + hw, node.y + hh);
        let tint = chrome.tint_highlighted();
        let fill = dag_node_paint_fill(lod, theme, chrome.is_dimmed, chrome.is_selected, chrome.is_highlighted, chrome.is_hovered)
            .map(|color| vello_color_with_alpha(color, chrome.body_fill_alpha));
        let stroke = dag_node_body_stroke(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
        let label_fill = dag_node_label_fill(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
        let internal_chrome_stroke =
            dag_node_internal_chrome_stroke(stroke, label_fill, chrome.is_hovered || chrome.is_selected || chrome.is_highlighted);
        let stroke_screen_px = dag_node_stroke_screen_px(chrome.is_dimmed, chrome.is_selected, chrome.is_highlighted, chrome.is_hovered);
        if let Some(fill) = fill {
            scene.fill(Fill::NonZero, *aff, fill, None, &rect);
        }
        if !chrome.is_selected {
            scene.stroke(&Stroke::new(dag_world_stroke(stroke_screen_px, cam.zoom)), *aff, stroke, None, &rect);
        }
        if chrome.is_computing {
            self.paint_computing_active_border(scene, aff, &rect, cam.zoom, theme);
        } else if chrome.is_stale {
            self.paint_computing_stale_border(scene, aff, &rect, cam.zoom, theme);
        }
        let layout_px = dag_label_layout_px();
        let paint_px = dag_label_paint_px(cam.zoom, lod_index);
        let chrome_stroke = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX, cam.zoom);
        let center_screen = world_to_screen(cam, viewport, Point::new(node.x, node.y));
        let node_fill = fill.unwrap_or(theme.node_fill);
        if !matches!(node.kind, DagNodeKind::Preview { .. } | DagNodeKind::Note { .. }) {
            self.paint_node_lod_icon(scene, lod, center_screen, node, cam.zoom, label_fill, node_fill);
        }
        let label_text = Self::node_label_text(node, lod);
        let caption_on_overlay = Self::node_caption_delegated_to_js_overlay(node, lod);
        if lod.node_label_is_horizontal() {
            if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                Self::paint_node_name_horizontal(scene, center_screen, label, paint_px, label_fill, label_halo);
            }
        } else {
            match &node.kind {
                DagNodeKind::Computation { .. } | DagNodeKind::Cluster { .. } => {
                    if lod.shows_computation_layout() {
                        let channel_row_pick = lod.uses_channel_row_pick();
                        if channel_row_pick {
                            self.paint_computation_channel_row_highlights(scene, aff, node, theme, chrome.is_dimmed);
                        }
                        Self::paint_computation_column_divider(scene, *aff, node, chrome_stroke, internal_chrome_stroke);
                        self.paint_computation_channel_row_dividers(
                            scene,
                            *aff,
                            node,
                            chrome_stroke,
                            internal_chrome_stroke,
                            stroke,
                            label_fill,
                            channel_row_pick,
                        );
                        if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                            Self::paint_computation_node_name(scene, cam, viewport, node, label, paint_px, label_fill, label_halo);
                        }
                        if matches!(node.kind, DagNodeKind::Cluster { .. }) {
                            Self::paint_cluster_affordances(scene, cam, viewport, node, paint_px, label_fill, label_halo);
                        }
                        if let DagNodeKind::Computation { variadic_inputs: true, .. } = &node.kind {
                            if cam.zoom >= DAG_VARIADIC_PLUS_ZOOM_THRESHOLD {
                                Self::paint_variadic_plus_controls(scene, cam, viewport, node, paint_px, label_fill, label_halo);
                            }
                        }
                    }
                }
                DagNodeKind::Slider { min, max, value, .. } => {
                    if let Some(label) = label_text.filter(|_| lod.shows_controls() && !caption_on_overlay) {
                        Self::paint_slider_name(scene, cam, viewport, node, label, paint_px, label_fill, label_halo);
                    }
                    if lod.shows_controls() {
                        let (x0, y0, x1, y1) = slider_track_bounds(node);
                        let track_y = (y0 + y1) * 0.5;
                        let track = Line::new(Point::new(x0, track_y), Point::new(x1, track_y));
                        let track_emphasized = chrome.is_hovered || chrome.is_selected || widget_drag_idx.is_some();
                        let track_stroke = if track_emphasized {
                            theme.label_fill_hovered
                        } else {
                            theme.edge_stroke
                        };
                        scene.stroke(&Stroke::new(chrome_stroke), *aff, track_stroke, None, &track);
                        let span = (max - min).max(1e-6);
                        let t = ((*value - *min) / span).clamp(0.0, 1.0);
                        let thumb_x = x0 + t * (x1 - x0);
                        let knob_dragging = widget_drag_idx.is_some();
                        let knob_fill = if knob_dragging {
                            theme.handle_fill_selected
                        } else {
                            label_fill
                        };
                        scene.fill(
                            Fill::NonZero,
                            *aff,
                            knob_fill,
                            None,
                            &Circle::new(Point::new(thumb_x, track_y), DAG_SLIDER_KNOB_SCREEN_PX / cam.zoom.max(0.05)),
                        );
                        let value_text = format!("{value:.1}");
                        let value_px = paint_px * 0.9;
                        let (vx, vy) = slider_value_world_center(node, &value_text, value_px, cam.zoom);
                        let value_pos = world_to_screen(cam, viewport, Point::new(vx, vy));
                        append_label(scene, &value_text, value_pos, value_px, label_fill, label_halo);
                    }
                }
                DagNodeKind::Select { options, selected, .. } => {
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                    }
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        Self::paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                    }
                    if lod.shows_controls() {
                        let (cx0, cy0, cx1, cy1) = select_control_bounds(node);
                        let control = Rect::new(cx0, cy0, cx1, cy1);
                        scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke, None, &control);
                        if lod.shows_detail_text() {
                            let option = options.get(*selected).map(String::as_str).unwrap_or("—");
                            let option_pos = world_to_screen(cam, viewport, Point::new((cx0 + cx1) * 0.5, (cy0 + cy1) * 0.5));
                            append_label(scene, option, option_pos, paint_px * 0.95, label_fill, label_halo);
                            let chevron = world_to_screen(cam, viewport, Point::new(cx1 - 6.0 / cam.zoom.max(0.05), (cy0 + cy1) * 0.5));
                            append_label(scene, "▾", chevron, paint_px, label_fill, label_halo);
                        }
                    }
                }
                DagNodeKind::Screen { media, .. } => {
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                    }
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        Self::paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                    }
                    if lod.shows_controls() {
                        let inset = 8.0 / cam.zoom.max(0.05);
                        let frame = Rect::new(node.x - hw + inset, node.y - hh + hh * 0.35, node.x + hw - inset, node.y + hh - inset);
                        scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke_selection_exit, None, &frame);
                    }
                    if lod.shows_detail_text() {
                        if let Some(media) = media {
                            let kind_label = match media.kind {
                                DagMediaKind::Image => "image",
                                DagMediaKind::Svg => "svg",
                                DagMediaKind::Pdf => "pdf",
                                DagMediaKind::Video => "video",
                            };
                            let hint = world_to_screen(cam, viewport, Point::new(node.x, node.y + hh * 0.1));
                            append_label(scene, kind_label, hint, paint_px * 0.85, vello_color_with_alpha(label_fill, ui_styling::opacities::KIND_HINT_ALPHA), label_halo);
                        }
                    }
                }
                DagNodeKind::Note { text, .. } => {
                    if lod.shows_detail_text() || lod.shows_controls() {
                        let (x0, y0, x1, y1) = preview_content_bounds(node);
                        let display = if text.is_empty() { "…" } else { text.as_str() };
                        let pos = world_to_screen(cam, viewport, Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
                        append_label(scene, display, pos, paint_px * 1.05, label_fill, label_halo);
                    }
                }
                DagNodeKind::Image { src, .. } => {
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                    }
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        Self::paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                    }
                    if lod.shows_controls() {
                        let (x0, y0, x1, y1) = preview_content_bounds(node);
                        let frame = Rect::new(x0, y0, x1, y1);
                        scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke, None, &frame);
                    }
                    if lod.shows_detail_text() || lod.shows_controls() {
                        self.paint_preview_image_content(scene, cam, viewport, node, src, label_fill, theme.raster_clear);
                    }
                }
                DagNodeKind::Preview { content, expanded, .. } => {
                    if lod.shows_detail_text() || lod.shows_controls() {
                        self.paint_preview_content(scene, cam, viewport, node, content, expanded, paint_px, label_fill, label_halo, theme.raster_clear);
                    }
                }
                DagNodeKind::Action { label, .. } => {
                    if lod.shows_controls() {
                        Self::paint_io_widget_channel_borders(scene, *aff, node, layout_px, chrome_stroke, internal_chrome_stroke);
                    }
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        Self::paint_io_widget_name(scene, cam, viewport, node, lod, label, paint_px, label_fill, label_halo);
                    }
                    if lod.shows_controls() {
                        let (x0, y0, x1, y1) = action_control_bounds(node);
                        let control = Rect::new(x0, y0, x1, y1);
                        scene.stroke(&Stroke::new(chrome_stroke), *aff, theme.edge_stroke, None, &control);
                        if lod.shows_detail_text() {
                            let pos = world_to_screen(cam, viewport, Point::new(node.x, node.y));
                            append_label(scene, label, pos, paint_px * 0.95, label_fill, label_halo);
                        }
                    }
                }
            }
        }
    }

    pub fn paint_scene(&self, scene: &mut cavas::vello::Scene, viewport_w: u32, viewport_h: u32, dpr: f64) {
        use cavas::camera::{camera_content_affine, Camera as CavasCamera, Viewport};
        use cavas::vello::kurbo::{Circle, Rect, Stroke};
        use cavas::vello::peniko::Fill;

        let theme = &self.vello_theme;
        self.tick_computing_animation();
        let cam = CavasCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: viewport_w.max(1), height: viewport_h.max(1), dpr: dpr.max(1.0) };
        let aff = camera_content_affine(&cam, &viewport);
        let lod = self.draw_lod_for_frame();
        let lod_index = dag_lod_index(cam.zoom);
        let lod_index_i8 = lod_index as i8;
        let prev_lod = self.last_logged_lod.get();
        if prev_lod != lod_index_i8 {
            self.last_logged_lod.set(lod_index_i8);
            dag_debug_log(&format!(
                "[DEBUG] dag draw lod={} zoom={:.3} icon={} label={:?}",
                lod.label(),
                cam.zoom,
                lod.node_icon_visible(),
                lod.node_label()
            ));
        }
        let snap = self.engine.render_snapshot();
        let edge_stroke = dag_world_stroke(lod.edge_stroke_screen_px(), cam.zoom);
        for (&eid, _edge) in &self.engine.edges {
            if let Some(curve) = self.engine.edge_curve(eid) {
                let (is_selected, is_highlighted, is_hovered) = self.edge_interaction_chrome(eid);
                let stroke_c = dag_edge_body_stroke(theme, false, is_selected, is_highlighted, is_hovered);
                scene.stroke(&Stroke::new(edge_stroke), aff, stroke_c, None, &curve);
            }
        }
        if let Some(preview) = snap.pending_edge {
            scene.stroke(&Stroke::new(edge_stroke), aff, dag_edge_body_stroke(theme, false, true, false, false), None, &preview);
        }
        if lod.shows_handles() {
            let handle_stroke_px = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX, cam.zoom);
            for (hid, center, _radius) in &snap.handles {
                let node_id = self.engine.handles.get(hid).map(|handle| handle.node_id);
                let is_dimmed = node_id.is_some_and(|nid| self.dimmed.contains(&nid));
                let (is_selected, is_highlighted, is_hovered) = self.handle_interaction_chrome(*hid);
                let fill = dag_handle_body_fill(theme, is_dimmed, is_selected, is_highlighted, is_hovered);
                let stroke_c = dag_handle_body_stroke(theme, is_dimmed, is_selected, is_highlighted, is_hovered);
                let chrome = is_dimmed || is_selected || is_highlighted || is_hovered;
                let outward = node_id.and_then(|nid| {
                    self.engine.nodes.get(&nid).and_then(|node| {
                        handle_outward_at_node_rim(*center, node.center, node.shape, node.radius, node.width, node.height)
                    })
                });
                if let Some(out) = outward {
                    if chrome {
                        scene.fill(Fill::NonZero, aff, fill, None, &handle_exterior_cap_fill_path(*center, out, DAG_HANDLE_WORLD_RADIUS));
                    }
                    scene.stroke(
                        &Stroke::new(handle_stroke_px),
                        aff,
                        stroke_c,
                        None,
                        &handle_exterior_cap_stroke_path(*center, out, DAG_HANDLE_WORLD_RADIUS),
                    );
                } else {
                    let circle = Circle::new(*center, DAG_HANDLE_WORLD_RADIUS);
                    if chrome {
                        scene.fill(Fill::NonZero, aff, fill, None, &circle);
                    }
                    scene.stroke(&Stroke::new(handle_stroke_px), aff, stroke_c, None, &circle);
                }
            }
            if let Some(ghost) = self.ghost_node.as_ref() {
                self.paint_node_handles_for_spec(scene, &aff, &cam, ghost, &DagNodePaintChrome::ghost_preview());
            }
        }
        let paint_minimap_node = |scene: &mut cavas::vello::Scene, idx: usize, fixture_node: &DagNodeSpec| {
            let node = self.node_spec_for_paint(idx, fixture_node);
            let node = node.as_ref();
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            let rect = Rect::new(node.x - hw, node.y - hh, node.x + hw, node.y + hh);
            let engine_nid = self.engine_node_id_for_index(idx);
            let is_dimmed = engine_nid.is_some_and(|nid| self.dimmed.contains(&nid));
            let (is_selected, is_highlighted, is_hovered) = engine_nid
                .map(|nid| self.node_interaction_chrome(nid))
                .unwrap_or((false, false, false));
            if let Some(fill) = dag_node_paint_fill(lod, theme, is_dimmed, is_selected, is_highlighted, is_hovered) {
                scene.fill(Fill::NonZero, aff, fill, None, &rect);
            }
        };
        if lod == DagDrawLod::Minimap {
            for (idx, fixture_node) in self.fixture.nodes.iter().enumerate() {
                let engine_nid = self.engine_node_id_for_index(idx);
                let chrome = engine_nid.is_some_and(|nid| {
                    let (selected, highlighted, hovered) = self.node_interaction_chrome(nid);
                    selected || highlighted || hovered
                });
                if !chrome {
                    paint_minimap_node(scene, idx, fixture_node);
                }
            }
            for (idx, fixture_node) in self.fixture.nodes.iter().enumerate() {
                let engine_nid = self.engine_node_id_for_index(idx);
                let chrome = engine_nid.is_some_and(|nid| {
                    let (selected, highlighted, hovered) = self.node_interaction_chrome(nid);
                    selected || highlighted || hovered
                });
                if chrome {
                    paint_minimap_node(scene, idx, fixture_node);
                }
            }
            return;
        }
        for (idx, fixture_node) in self.fixture.nodes.iter().enumerate() {
            let node = self.node_spec_for_paint(idx, fixture_node);
            let node = node.as_ref();
            let engine_nid = self.engine_node_id_for_index(idx);
            let is_dimmed = engine_nid.is_some_and(|nid| self.dimmed.contains(&nid));
            let (is_selected, is_highlighted, is_hovered) = engine_nid
                .map(|nid| self.node_interaction_chrome(nid))
                .unwrap_or((false, false, false));
            let is_computing = engine_nid.is_some_and(|nid| self.computing_active == Some(nid));
            let is_stale = engine_nid.is_some_and(|nid| self.computing_stale.contains(&nid));
            self.paint_node_visual(
                scene,
                &aff,
                &cam,
                &viewport,
                lod,
                lod_index,
                node,
                self.widget_drag.filter(|drag_idx| *drag_idx == idx),
                DagNodePaintChrome {
                    is_dimmed,
                    is_selected,
                    is_highlighted,
                    is_hovered,
                    is_computing,
                    is_stale,
                    body_fill_alpha: 255,
                    ghost_tint: false,
                },
            );
        }
        if let Some(ghost) = self.ghost_node.as_ref() {
            self.paint_node_visual(
                scene,
                &aff,
                &cam,
                &viewport,
                lod,
                lod_index,
                ghost,
                None,
                DagNodePaintChrome::ghost_preview(),
            );
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

        #[wasm_bindgen(js_name = labelOverlayPaintStateJson)]
        pub fn label_overlay_paint_state_json(&self) -> Result<String, JsValue> {
            self.state
                .borrow()
                .host
                .label_overlay_paint_state_json()
                .map_err(|e| JsValue::from_str(&e))
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

        #[wasm_bindgen(js_name = lodScaleJson)]
        pub fn lod_scale_json(&self) -> String {
            dag_lod_scale_json()
        }

        #[wasm_bindgen(js_name = setAutomaticLod)]
        pub fn set_automatic_lod(&self, enabled: bool) {
            self.state.borrow_mut().host.set_automatic_lod(enabled);
        }

        #[wasm_bindgen(js_name = setForcedDrawLodLabel)]
        pub fn set_forced_draw_lod_label(&self, label: &str) {
            self.state.borrow_mut().host.set_forced_draw_lod_label(label);
        }

        #[wasm_bindgen(js_name = drawLodLabel)]
        pub fn draw_lod_label(&self) -> String {
            self.state.borrow().host.draw_lod_label().to_string()
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
                    "A".into(),
                    "emoji:🔷".into(),
                    vec![],
                    vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }],
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
                    "B".into(),
                    "emoji:🔷".into(),
                    vec![IoPortSpec { id: "in".into(), label: "in".into() , ..Default::default() }],
                    vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }],
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
                "C".into(),
                "emoji:🔷".into(),
                vec![IoPortSpec { id: "in".into(), label: "in".into() , ..Default::default() }],
                vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }],
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
                abbreviation: "S".into(),
                icon: "emoji:🎚️".into(),
                x: 1.0,
                y: 2.0,
                width: 180.0,
                height: 80.0,
                kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 3.0, output: IoPortSpec { id: "out".into(), label: "value".into() , ..Default::default() } },
            },
            DagNodeSpec {
                id: "m".into(),
                name: "M".into(),
                abbreviation: "M".into(),
                icon: "emoji:📋".into(),
                x: 0.0,
                y: 0.0,
                width: 180.0,
                height: 80.0,
                kind: DagNodeKind::Select { options: vec!["A".into(), "B".into()], selected: 1, output: IoPortSpec { id: "out".into(), label: "mode".into() , ..Default::default() } },
            },
            DagNodeSpec {
                id: "p".into(),
                name: "P".into(),
                abbreviation: "P".into(),
                icon: "emoji:🖥️".into(),
                x: 0.0,
                y: 0.0,
                width: 200.0,
                height: 140.0,
                kind: DagNodeKind::Screen {
                    media: Some(DagMedia { kind: DagMediaKind::Svg, src: "data:image/svg+xml,test".into() }),
                    input: IoPortSpec { id: "in".into(), label: "result".into() , ..Default::default() },
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
    fn handle_hover_does_not_hover_parent_node() {
        let fixture = DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec::computation(
                "merge".into(),
                "Merge".into(),
                "M".into(),
                "emoji:🔀".into(),
                vec![IoPortSpec { id: "0".into(), label: "0".into() , ..Default::default() }],
                vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }],
                false,
                false,
                0.0,
                0.0,
                160.0,
                48.0,
            )],
            edges: vec![],
        };
        let mut host = DagHost::from_fixture(fixture);
        host.set_viewport(800, 600, 1.0);
        let (handle_center, node_id) = {
            let snap = host.engine.render_snapshot();
            let (hid, center, _) = snap.handles.first().expect("input handle");
            let node_id = host.engine.handles.get(hid).expect("handle").node_id;
            (*center, node_id)
        };
        let (sx, sy) = world_to_screen_px(&host, handle_center);
        host.pointer_move_screen(sx, sy, false, false, false);
        let hover = host.engine.hover.expect("handle hover");
        assert!(host.engine.handles.contains_key(&hover));
        assert!(host.hovered_node_id().is_none());
        assert!(!host.is_node_hovered(node_id));
        host.pointer_move_screen(0.0, 0.0, false, false, false);
        assert!(host.engine.hover.is_none());
    }

    #[test]
    fn idle_pointer_move_updates_hover() {
        use cavas::camera::{world_to_screen, Camera, Viewport};
        use cavas::vello::kurbo::Point;

        let mut host = DagHost::default_demo();
        host.set_viewport(800, 600, 1.0);
        let camera = Camera {
            x: host.fixture.camera.x,
            y: host.fixture.camera.y,
            zoom: host.fixture.camera.zoom,
        };
        let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
        let slider = world_to_screen(&camera, &viewport, Point::new(-400.0, -40.0));
        host.pointer_move_screen(slider.x, slider.y, false, false, false);
        assert_eq!(host.hovered_node_id().as_deref(), Some("slider"));
        host.pointer_move_screen(8.0, 8.0, false, false, false);
        assert!(host.hovered_node_id().is_none());
    }

    #[test]
    fn dag_node_spec_port_accessors_per_kind() {
        let slider = DagNodeSpec {
            id: "s".into(),
            name: "S".into(),
            abbreviation: "S".into(),
            icon: "emoji:🎚️".into(),
            x: 0.0,
            y: 0.0,
            width: 180.0,
            height: 80.0,
            kind: DagNodeKind::Slider { min: 0.0, max: 1.0, step: 0.1, value: 0.5, output: IoPortSpec { id: "out".into(), label: "value".into() , ..Default::default() } },
        };
        assert!(slider.inputs().is_empty());
        assert_eq!(slider.outputs().len(), 1);
        let screen = DagNodeSpec {
            id: "p".into(),
            name: "P".into(),
            abbreviation: "P".into(),
            icon: "emoji:🖥️".into(),
            x: 0.0,
            y: 0.0,
            width: 200.0,
            height: 140.0,
            kind: DagNodeKind::Screen { media: None, input: IoPortSpec { id: "in".into(), label: "in".into() , ..Default::default() } },
        };
        assert_eq!(screen.inputs().len(), 1);
        assert!(screen.outputs().is_empty());
    }

    #[test]
    fn dag_host_delete_selected_preserves_remaining_positions() {
        let mut host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                DagNodeSpec::computation("a".into(), "A".into(), "A".into(), "emoji:🔷".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }], false, false, 100.0, 200.0, 160.0, 56.0),
                DagNodeSpec::computation("b".into(), "B".into(), "B".into(), "emoji:🔷".into(), vec![IoPortSpec { id: "in".into(), label: "in".into() , ..Default::default() }], vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }], false, false, 400.0, 500.0, 160.0, 56.0),
                DagNodeSpec::computation("c".into(), "C".into(), "C".into(), "emoji:🔷".into(), vec![IoPortSpec { id: "in".into(), label: "in".into() , ..Default::default() }], vec![], false, false, 700.0, 300.0, 160.0, 56.0),
            ],
            edges: vec![
                DagFixtureEdgeV1 { id: "e1".into(), source: "a:out".into(), target: "b:in".into() },
                DagFixtureEdgeV1 { id: "e2".into(), source: "b:out".into(), target: "c:in".into() },
            ],
        });
        host.set_selection(&["b".to_string()]);
        host.delete_selected();
        let a = host.fixture.nodes.iter().find(|n| n.id == "a").expect("a");
        let c = host.fixture.nodes.iter().find(|n| n.id == "c").expect("c");
        assert!((a.x - 100.0).abs() < 0.01);
        assert!((a.y - 200.0).abs() < 0.01);
        assert!((c.x - 700.0).abs() < 0.01);
        assert!((c.y - 300.0).abs() < 0.01);
        assert!(host.fixture.nodes.iter().all(|n| n.id != "b"));
    }

    #[test]
    fn dag_host_delete_selected_removes_edge_only_selection() {
        let mut host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                DagNodeSpec::computation("a".into(), "A".into(), "A".into(), "emoji:🔷".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }], false, false, 100.0, 200.0, 160.0, 56.0),
                DagNodeSpec::computation("b".into(), "B".into(), "B".into(), "emoji:🔷".into(), vec![IoPortSpec { id: "in".into(), label: "in".into() , ..Default::default() }], vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }], false, false, 400.0, 500.0, 160.0, 56.0),
                DagNodeSpec::computation("c".into(), "C".into(), "C".into(), "emoji:🔷".into(), vec![IoPortSpec { id: "in".into(), label: "in".into() , ..Default::default() }], vec![], false, false, 700.0, 300.0, 160.0, 56.0),
            ],
            edges: vec![
                DagFixtureEdgeV1 { id: "e1".into(), source: "a:out".into(), target: "b:in".into() },
                DagFixtureEdgeV1 { id: "e2".into(), source: "b:out".into(), target: "c:in".into() },
            ],
        });
        let edge_id = *host.engine.edges.keys().next().expect("edge");
        host.engine.selection.edge_ids.insert(edge_id);
        assert!(host.has_selection());
        assert_eq!(host.fixture.edges.len(), 2);
        host.delete_selected();
        assert_eq!(host.fixture.edges.len(), 1);
        assert_eq!(host.fixture.nodes.len(), 3);
        assert!(!host.has_selection());
    }

    #[test]
    fn dag_host_reorganize_updates_engine_positions() {
        let mut host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                DagNodeSpec::computation("a".into(), "A".into(), "A".into(), "emoji:🔷".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }], false, false, 500.0, 500.0, 160.0, 56.0),
                DagNodeSpec::computation("b".into(), "B".into(), "B".into(), "emoji:🔷".into(), vec![IoPortSpec { id: "in".into(), label: "in".into() , ..Default::default() }], vec![], false, false, 500.0, 500.0, 160.0, 56.0),
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
        let output = IoPortSpec { id: "out".into(), label: "value".into() , ..Default::default() };
        let node = DagNodeSpec {
            id: "slider".into(),
            name: "Amount".into(),
            abbreviation: "Amount".into(),
            icon: "emoji:🎚️".into(),
            x: 100.0,
            y: 50.0,
            width: slider_widget_width("Amount", &output),
            height: slider_widget_height(),
            kind: DagNodeKind::Slider {
                min: 0.0,
                max: 10.0,
                step: 0.5,
                value: 2.0,
                output,
            },
        };
        let hw = node.width * 0.5;
        let hh = node.height * 0.5;
        let (left, top, right, bottom) = slider_track_bounds(&node);
        assert!(left >= node.x - hw);
        assert!(right <= node.x + hw);
        assert!((top + bottom) * 0.5 - node.y < 1e-6);
        assert!(top >= node.y - hh);
        assert!(bottom <= node.y + hh);
    }

    #[test]
    fn dag_host_slider_drag_mutates_value() {
        let output = IoPortSpec { id: "out".into(), label: "value".into() , ..Default::default() };
        let mut host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec {
                id: "slider".into(),
                name: "Amount".into(),
                abbreviation: "Amount".into(),
                icon: "emoji:🎚️".into(),
                x: 0.0,
                y: 0.0,
                width: slider_widget_width("Amount", &output),
                height: slider_widget_height(),
                kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 2.0, output },
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
    fn dag_host_slider_drag_ignored_when_controls_hidden() {
        let output = IoPortSpec { id: "out".into(), label: "value".into() , ..Default::default() };
        let mut host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec {
                id: "slider".into(),
                name: "Amount".into(),
                abbreviation: "Amount".into(),
                icon: "emoji:🎚️".into(),
                x: 0.0,
                y: 0.0,
                width: slider_widget_width("Amount", &output),
                height: slider_widget_height(),
                kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 2.0, output },
            }],
            edges: vec![],
        });
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("minimap");
        host.set_viewport(800, 600, 1.0);
        let (x0, y0, x1, y1) = slider_track_bounds(&host.fixture.nodes[0]);
        let mid_y = (y0 + y1) * 0.5;
        let (sx, sy) = world_to_screen_px(&host, cavas::vello::kurbo::Point::new((x0 + x1) * 0.5, mid_y));
        host.pointer_down(sx, sy, false);
        host.pointer_up(sx, sy);
        let DagNodeKind::Slider { value, .. } = host.fixture.nodes[0].kind else {
            panic!("expected slider");
        };
        assert!((value - 2.0).abs() < 1e-6, "minimap LOD should only move the node rectangle, not adjust the value");
    }

    #[test]
    fn dag_host_select_click_advances_option() {
        let mut host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec {
                id: "mode".into(),
                name: "Mode".into(),
                abbreviation: "Mode".into(),
                icon: "emoji:📋".into(),
                x: 0.0,
                y: 0.0,
                width: 180.0,
                height: 80.0,
                kind: DagNodeKind::Select { options: vec!["Add".into(), "Multiply".into()], selected: 0, output: IoPortSpec { id: "out".into(), label: "mode".into() , ..Default::default() } },
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
    fn dag_host_label_overlay_paint_state_json_includes_compact_labels() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("compact");
        let raw: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
        let labels = raw["labels"].as_array().expect("labels");
        assert!(!labels.is_empty());
        assert!(labels.iter().all(|row| row["layout"] == "horizontal"));
        assert!(labels[0]["text"].as_str().unwrap_or("").len() > 0);
    }

    #[test]
    fn dag_host_label_overlay_paint_state_json_includes_slider_name() {
        let mut host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 2.0 },
            nodes: vec![DagNodeSpec {
                id: "slider".into(),
                name: "Radius".into(),
                abbreviation: "Radius".into(),
                icon: "emoji:🎚️".into(),
                x: 0.0,
                y: 0.0,
                width: 120.0,
                height: 32.0,
                kind: DagNodeKind::Slider {
                    min: 0.0,
                    max: 10.0,
                    step: 0.5,
                    value: 3.0,
                    output: IoPortSpec { id: "out".into(), label: "value".into() , ..Default::default() },
                },
            }],
            edges: vec![],
        });
        host.set_viewport(1280, 800, 1.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("micro");
        let raw: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
        let labels = raw["labels"].as_array().expect("labels");
        assert!(labels.iter().any(|row| row["text"] == "Radius" && row["layout"] == "horizontal"));
    }

    #[test]
    fn label_overlay_port_rows_are_not_duplicated_in_json() {
        let mut host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 2.0 },
            nodes: vec![DagNodeSpec {
                id: "combine".into(),
                name: "Combine".into(),
                abbreviation: "Combine".into(),
                icon: "emoji:🔀".into(),
                x: 0.0,
                y: 0.0,
                width: 104.0,
                height: 28.0,
                kind: DagNodeKind::Computation {
                    inputs: vec![
                        IoPortSpec { id: "a".into(), label: "a".into() , ..Default::default() },
                        IoPortSpec { id: "b".into(), label: "b".into() , ..Default::default() },
                    ],
                    outputs: vec![IoPortSpec { id: "out".into(), label: "merged".into() , ..Default::default() }],
                    variadic_inputs: false,
                    variadic_outputs: false,
                },
            }],
            edges: vec![],
        });
        host.set_viewport(1280, 800, 1.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("micro");
        let raw: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
        let labels = raw["labels"].as_array().expect("labels");
        let port_rows: Vec<_> = labels
            .iter()
            .filter(|row| row["align"].as_str().is_some())
            .map(|row| (row["text"].as_str().unwrap_or(""), row["align"].as_str().unwrap_or("")))
            .collect();
        assert_eq!(port_rows.len(), 3);
        assert_eq!(port_rows.iter().filter(|(text, _)| *text == "a").count(), 1);
        assert_eq!(port_rows.iter().filter(|(text, _)| *text == "b").count(), 1);
        assert_eq!(port_rows.iter().filter(|(text, _)| *text == "merged").count(), 1);
    }

    #[test]
    fn dag_host_label_overlay_paint_state_json_includes_detail_port_labels() {
        let mut host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 2.0 },
            nodes: vec![DagNodeSpec {
                id: "box".into(),
                name: "brep.prim3d.box".into(),
                abbreviation: "box".into(),
                icon: "emoji:📦".into(),
                x: 0.0,
                y: 0.0,
                width: 96.0,
                height: 42.0,
                kind: DagNodeKind::Computation {
                    inputs: vec![
                        IoPortSpec { id: "width".into(), label: "width".into() , ..Default::default() },
                        IoPortSpec { id: "depth".into(), label: "depth".into() , ..Default::default() },
                    ],
                    outputs: vec![IoPortSpec { id: "out".into(), label: "geometry".into() , ..Default::default() }],
                    variadic_inputs: false,
                    variadic_outputs: false,
                },
            }],
            edges: vec![],
        });
        host.set_viewport(1280, 800, 1.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("detail");
        let raw: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
        let labels = raw["labels"].as_array().expect("labels");
        let port_labels: Vec<_> = labels
            .iter()
            .filter(|row| row["align"].as_str().is_some())
            .collect();
        assert_eq!(port_labels.len(), 3, "expected input and output channel labels");
        assert!(port_labels.iter().any(|row| row["text"] == "width" && row["align"] == "left"));
        assert!(port_labels.iter().any(|row| row["text"] == "geometry" && row["align"] == "right"));
    }

    #[test]
    fn dag_host_exports_screen_overlay_rect() {
        let host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec {
                id: "screen".into(),
                name: "Preview".into(),
                abbreviation: "Preview".into(),
                icon: "emoji:🖥️".into(),
                x: 100.0,
                y: 50.0,
                width: 200.0,
                height: 140.0,
                kind: DagNodeKind::Screen {
                    media: Some(DagMedia { kind: DagMediaKind::Svg, src: "data:image/svg+xml,test".into() }),
                    input: IoPortSpec { id: "in".into(), label: "result".into() , ..Default::default() },
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
    fn dag_host_area_select_previews_preselect_before_commit() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        let start_sx = 24.0;
        let start_sy = 24.0;
        let end_sx = 1100.0;
        let end_sy = 700.0;
        host.pointer_down_screen(start_sx, start_sy, 0, false, false, false);
        host.pointer_move_screen(end_sx, end_sy, false, false, false);
        assert!(
            matches!(host.engine.interaction, InteractionMode::AreaSelect { .. }),
            "expected area-select after marquee threshold"
        );
        let preselect = host.preselect_widget_ids();
        assert!(!preselect.is_empty(), "marquee drag should preview widget ids before commit");
        let preview_points: Vec<[f64; 2]> = serde_json::from_str(&host.selection_preview_points_json()).unwrap();
        assert!(preview_points.len() >= 2, "marquee overlay points should be published during drag");
        host.pointer_up_screen(end_sx, end_sy, false, false, false);
        assert!(!host.selected_node_ids().is_empty(), "marquee drag should commit selection on release");
        assert!(host.preselect_widget_ids().is_empty(), "preselect should clear after commit");
    }

    #[test]
    fn dag_host_align_selection_horizontal_and_vertical_center() {
        let mut host = DagHost::default_demo();
        host.set_viewport(800, 600, 1.0);
        host.set_selection(&["scale".into(), "combine".into(), "preview".into()]);
        host.align_selection("alignHorizontal").unwrap();
        let xs: Vec<f64> = host.selected_fixture_nodes().into_iter().map(|(_, node)| node.x).collect();
        assert!(xs.windows(2).all(|pair| (pair[0] - pair[1]).abs() < 1e-6));
        host.align_selection("alignVertical").unwrap();
        let ys: Vec<f64> = host.selected_fixture_nodes().into_iter().map(|(_, node)| node.y).collect();
        assert!(ys.windows(2).all(|pair| (pair[0] - pair[1]).abs() < 1e-6));
    }

    #[test]
    fn dag_host_align_selection_left_and_distribute_horizontal() {
        let mut host = DagHost::default_demo();
        host.set_viewport(800, 600, 1.0);
        host.set_selection(&["scale".into(), "combine".into(), "preview".into()]);
        host.align_selection("alignLeft").unwrap();
        let left_edges: Vec<f64> = host
            .selected_fixture_nodes()
            .into_iter()
            .map(|(_, node)| node.x - node.width * 0.5)
            .collect();
        assert!(left_edges.windows(2).all(|pair| (pair[0] - pair[1]).abs() < 1e-6));
        host.align_selection("distributeHorizontal").unwrap();
        let mut xs: Vec<f64> = host.selected_fixture_nodes().into_iter().map(|(_, node)| node.x).collect();
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert!(xs.windows(2).all(|pair| pair[1] > pair[0]));
    }

    #[test]
    fn dag_host_selection_union_bounds_screen_json_nonempty_for_selection() {
        let mut host = DagHost::default_demo();
        host.set_viewport(800, 600, 1.0);
        host.set_selection(&["scale".into(), "combine".into()]);
        let json = host.selection_union_bounds_screen_json();
        assert_ne!(json, "null");
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["width"].as_f64().unwrap_or(0.0) > 1.0);
        assert!(parsed["height"].as_f64().unwrap_or(0.0) > 1.0);
    }

    #[test]
    fn dag_host_minimap_bounded_drag_moves_selection_inside_union_bounds() {
        let mut host = DagHost::default_demo();
        host.set_viewport(800, 600, 1.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("minimap");
        host.set_camera(0.0, 0.0, 0.1);
        host.set_selection(&["scale".into(), "combine".into()]);
        let scale_before = host.fixture.nodes.iter().find(|n| n.id == "scale").expect("scale").clone();
        let combine_before = host.fixture.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
        let gap = cavas::vello::kurbo::Point::new(0.0, 0.0);
        use cavas::camera::{world_to_screen, Camera as CavasCamera, Viewport};
        let cam = CavasCamera {
            x: host.fixture.camera.x,
            y: host.fixture.camera.y,
            zoom: host.fixture.camera.zoom,
        };
        let viewport = Viewport {
            width: 800,
            height: 600,
            dpr: 1.0,
        };
        let start = world_to_screen(&cam, &viewport, gap);
        host.pointer_down_screen(start.x, start.y, 0, false, false, false);
        assert!(
            matches!(host.engine.interaction, InteractionMode::DragNodes { .. }),
            "expected bounded drag inside selection union at minimap LOD"
        );
        host.pointer_move_screen(start.x + 50.0, start.y + 30.0, false, false, false);
        host.pointer_up_screen(start.x + 50.0, start.y + 30.0, false, false, false);
        let zoom = host.fixture.camera.zoom;
        let dx = 50.0 / zoom;
        let dy = 30.0 / zoom;
        let scale_after = host.fixture.nodes.iter().find(|n| n.id == "scale").expect("scale");
        let combine_after = host.fixture.nodes.iter().find(|n| n.id == "combine").expect("combine");
        assert!((scale_after.x - (scale_before.x + dx)).abs() < 1e-3 && (scale_after.y - (scale_before.y + dy)).abs() < 1e-3);
        assert!((combine_after.x - (combine_before.x + dx)).abs() < 1e-3 && (combine_after.y - (combine_before.y + dy)).abs() < 1e-3);
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
    fn dag_host_node_drag_proximity_preview_and_connects() {
        let inputs = vec![IoPortSpec { id: "in".into(), label: "in".into() , ..Default::default() }];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }];
        let src_w = computation_node_width("Src", &[], &outputs);
        let tgt_w = computation_node_width("Tgt", &inputs, &outputs);
        let src_h = computation_node_height(0, 1, false, false);
        let tgt_h = computation_node_height(1, 1, false, false);
        let mut host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                DagNodeSpec::computation(
                    "src".into(),
                    "Src".into(),
                    "Src".into(),
                    "emoji:🔢".into(),
                    vec![],
                    outputs.clone(),
                    false,
                    false,
                    0.0,
                    0.0,
                    src_w,
                    src_h,
                ),
                DagNodeSpec::computation(
                    "tgt".into(),
                    "Tgt".into(),
                    "Tgt".into(),
                    "emoji:🔢".into(),
                    inputs,
                    outputs,
                    false,
                    false,
                    220.0,
                    0.0,
                    tgt_w,
                    tgt_h,
                ),
            ],
            edges: vec![],
        });
        host.set_viewport(1280, 800, 1.0);
        host.set_proximity_distance(120.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("normal");
        let src_center = cavas::vello::kurbo::Point::new(0.0, 0.0);
        let (sx, sy) = world_to_screen_px(&host, src_center);
        host.pointer_down_screen(sx, sy, 0, false, false, false);
        host.pointer_move_screen(sx + 200.0, sy, false, false, false);
        assert!(host.engine.render_snapshot().pending_edge.is_some(), "proximity drag should preview edge");
        host.pointer_up_screen(sx + 200.0, sy, false, false, false);
        assert!(
            host.fixture
                .edges
                .iter()
                .any(|edge| edge.source == "src:out" && edge.target == "tgt:in"),
            "proximity drag should commit edge"
        );
    }

    #[test]
    fn dag_host_node_drag_skips_wired_cut_inputs() {
        let inputs = vec![
            IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() },
            IoPortSpec { id: "b".into(), label: "b".into(), ..Default::default() },
        ];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
        let src_w = computation_node_width("Src", &[], &outputs);
        let cut_w = computation_node_width("Cut", &inputs, &outputs);
        let src_h = computation_node_height(0, 1, false, false);
        let cut_h = computation_node_height(2, 1, false, false);
        let mut host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                DagNodeSpec::computation(
                    "sphere".into(),
                    "Sphere".into(),
                    "Sphere".into(),
                    "emoji:🔵".into(),
                    vec![],
                    outputs.clone(),
                    false,
                    false,
                    0.0,
                    -60.0,
                    src_w,
                    src_h,
                ),
                DagNodeSpec::computation(
                    "torus".into(),
                    "Torus".into(),
                    "Torus".into(),
                    "emoji:🍩".into(),
                    vec![],
                    outputs.clone(),
                    false,
                    false,
                    0.0,
                    60.0,
                    src_w,
                    src_h,
                ),
                DagNodeSpec::computation(
                    "cut".into(),
                    "Cut".into(),
                    "Cut".into(),
                    "emoji:✂️".into(),
                    inputs,
                    outputs,
                    false,
                    false,
                    240.0,
                    0.0,
                    cut_w,
                    cut_h,
                ),
            ],
            edges: vec![
                DagFixtureEdgeV1 { id: "e1".into(), source: "sphere:out".into(), target: "cut:a".into() },
                DagFixtureEdgeV1 { id: "e2".into(), source: "torus:out".into(), target: "cut:b".into() },
            ],
        });
        assert_eq!(host.engine.edges.len(), 2, "fixture edges should load into engine");
        host.set_viewport(1280, 800, 1.0);
        host.set_proximity_distance(160.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("normal");
        let cut_center = cavas::vello::kurbo::Point::new(240.0, 0.0);
        let (sx, sy) = world_to_screen_px(&host, cut_center);
        host.pointer_down_screen(sx, sy, 0, false, false, false);
        host.pointer_move_screen(sx - 180.0, sy, false, false, false);
        assert!(
            host.engine.render_snapshot().pending_edge.is_none(),
            "dragging wired cut near sources must not preview proximity edges to occupied inputs"
        );
        host.pointer_up_screen(sx - 180.0, sy, false, false, false);
        assert_eq!(host.engine.edges.len(), 2);
        assert_eq!(host.fixture.edges.len(), 2);
    }

    #[test]
    fn dag_host_proximity_zero_disables_node_drag_connect() {
        let inputs = vec![IoPortSpec { id: "in".into(), label: "in".into() , ..Default::default() }];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }];
        let src_w = computation_node_width("Src", &[], &outputs);
        let tgt_w = computation_node_width("Tgt", &inputs, &outputs);
        let src_h = computation_node_height(0, 1, false, false);
        let tgt_h = computation_node_height(1, 1, false, false);
        let mut host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                DagNodeSpec::computation(
                    "src".into(),
                    "Src".into(),
                    "Src".into(),
                    "emoji:🔢".into(),
                    vec![],
                    outputs.clone(),
                    false,
                    false,
                    0.0,
                    0.0,
                    src_w,
                    src_h,
                ),
                DagNodeSpec::computation(
                    "tgt".into(),
                    "Tgt".into(),
                    "Tgt".into(),
                    "emoji:🔢".into(),
                    inputs,
                    outputs,
                    false,
                    false,
                    220.0,
                    0.0,
                    tgt_w,
                    tgt_h,
                ),
            ],
            edges: vec![],
        });
        host.set_viewport(1280, 800, 1.0);
        host.set_proximity_distance(0.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("normal");
        let (sx, sy) = world_to_screen_px(&host, cavas::vello::kurbo::Point::new(0.0, 0.0));
        host.pointer_down_screen(sx, sy, 0, false, false, false);
        host.pointer_move_screen(sx + 200.0, sy, false, false, false);
        assert!(host.engine.render_snapshot().pending_edge.is_none());
        host.pointer_up_screen(sx + 200.0, sy, false, false, false);
        assert!(host.fixture.edges.is_empty());
    }

    #[test]
    fn hidden_lod_connection_hit_picking_disabled() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        host.set_automatic_lod(false);
        let combine = host.fixture.nodes.iter().find(|n| n.id == "combine").expect("combine");
        let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
        let (x0, y0, x1, y1) = input_port_row_hit_bounds(combine, port_idx).expect("row bounds");
        let row_center = cavas::vello::kurbo::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let handle = handle_world(&host, "combine:b");
        for lod in ["minimap", "overview", "compact"] {
            host.set_forced_draw_lod_label(lod);
            assert!(!host.draw_lod_for_frame().allows_connection_hit_picking(), "{lod}");
            let (sx, sy) = world_to_screen_px(&host, row_center);
            host.pointer_down(sx, sy, false);
            assert!(
                !matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }),
                "{lod} input row should not start edge draw"
            );
            host.pointer_up(sx, sy);
            let (hsx, hsy) = world_to_screen_px(&host, handle);
            host.pointer_down(hsx, hsy, false);
            assert!(
                !matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }),
                "{lod} handle anchor should not start edge draw"
            );
            host.pointer_up(hsx, hsy);
        }
    }

    #[test]
    fn normal_lod_input_row_drags_node_handle_anchor_starts_edge_draw() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("normal");
        let combine = host.fixture.nodes.iter().find(|n| n.id == "combine").expect("combine");
        let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
        let (x0, y0, x1, y1) = input_port_row_hit_bounds(combine, port_idx).expect("row bounds");
        let row_center = cavas::vello::kurbo::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let handle = handle_world(&host, "combine:b");
        assert!(
            (row_center.x - handle.x).abs() > 4.0,
            "row center should sit away from the painted handle anchor"
        );
        let (sx, sy) = world_to_screen_px(&host, row_center);
        host.pointer_down(sx, sy, false);
        assert!(
            matches!(host.engine.interaction, InteractionMode::DragNode { .. }),
            "interior rectangle drag should move the node"
        );
        host.pointer_up(sx, sy);
        let (hsx, hsy) = world_to_screen_px(&host, handle);
        host.pointer_down(hsx, hsy, false);
        assert!(matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }));
    }

    #[test]
    fn set_hover_channel_targets_port_handle_at_detail_lod() {
        let mut host = DagHost::default_demo();
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("detail");
        host.set_hover_channel(Some("combine"), Some("b"));
        assert_eq!(
            host.hovered_channel(),
            Some(DagChannelRef {
                widget_id: "combine".into(),
                port: "b".into(),
                direction: "in".into(),
            })
        );
        host.set_hover_channel(None, None);
        assert!(host.hovered_channel().is_none());
    }

    #[test]
    fn set_hover_channel_falls_back_to_node_at_compact_lod() {
        let mut host = DagHost::default_demo();
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("compact");
        host.set_hover_channel(Some("combine"), Some("b"));
        assert_eq!(host.hovered_node_id().as_deref(), Some("combine"));
        assert!(host.hovered_channel().is_none());
    }

    #[test]
    fn hovered_channel_decodes_port_handle_at_detail_lod() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("detail");
        let combine = host.fixture.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
        let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
        let (x0, y0, x1, y1) = input_port_row_hit_bounds(&combine, port_idx).expect("row bounds");
        let row_center = cavas::vello::kurbo::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let (sx, sy) = world_to_screen_px(&host, row_center);
        host.pointer_move_screen(sx, sy, false, false, false);
        assert_eq!(
            host.hovered_channel(),
            Some(DagChannelRef {
                widget_id: "combine".into(),
                port: "b".into(),
                direction: "in".into(),
            })
        );
    }

    #[test]
    fn detail_lod_non_channel_body_hovers_and_selects_node() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("detail");
        let combine = host.fixture.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
        let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
        let (x0, y0, x1, y1) = input_port_row_hit_bounds(&combine, port_idx).expect("row bounds");
        let row_center = cavas::vello::kurbo::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let (sx, sy) = world_to_screen_px(&host, row_center);
        host.pointer_move_screen(sx, sy, false, false, false);
        assert!(host.hovered_node_id().as_deref() == Some("combine"));
        assert!(host.engine.hover.is_some());
        assert!(!host.engine.selection.node_ids.contains(
            &host.node_id_for_widget_id("combine").expect("combine node id")
        ));
        let divider_x = computation_column_divider_x(&combine).expect("divider");
        let (_, header_top, _, header_bottom) = channel_row_bounds(&combine, 0);
        let title_probe = cavas::vello::kurbo::Point::new(divider_x, (header_top + header_bottom) * 0.5);
        let (body_sx, body_sy) = world_to_screen_px(&host, title_probe);
        host.pointer_move_screen(body_sx, body_sy, false, false, false);
        assert!(host.hovered_node_id().as_deref() == Some("combine"));
        assert!(host.engine.hover.is_some());
        host.pointer_down(body_sx, body_sy, false);
        assert!(host.selected_node_ids().contains(&"combine".to_string()));
        host.pointer_up(body_sx, body_sy);
    }

    #[test]
    fn visible_handle_lod_row_center_does_not_start_edge_draw() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("detail");
        let scale = host.fixture.nodes.iter().find(|n| n.id == "scale").expect("scale");
        let port_idx = scale.outputs().iter().position(|p| p.id == "out").expect("port out");
        let (x0, y0, x1, y1) = output_port_row_hit_bounds(scale, port_idx).expect("row bounds");
        let row_center = cavas::vello::kurbo::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let handle = handle_world(&host, "scale:out");
        assert!((row_center.x - handle.x).abs() > 4.0, "row center should sit away from the painted handle anchor");
        let (sx, sy) = world_to_screen_px(&host, row_center);
        host.pointer_down(sx, sy, false);
        assert!(
            !matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }),
            "visible handles require anchor hit for wire draw"
        );
        let (hsx, hsy) = world_to_screen_px(&host, handle);
        host.pointer_down(hsx, hsy, false);
        assert!(matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }));
    }

    #[test]
    fn detail_lod_channel_row_drags_node_without_prior_selection() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("micro");
        let combine = host.fixture.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
        let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
        let (x0, y0, x1, y1) = input_port_row_hit_bounds(&combine, port_idx).expect("row bounds");
        let row_center = cavas::vello::kurbo::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let handle = handle_world(&host, "combine:b");
        assert!((row_center.x - handle.x).abs() > 4.0);
        let (sx, sy) = world_to_screen_px(&host, row_center);
        host.pointer_down(sx, sy, false);
        assert!(matches!(host.engine.interaction, InteractionMode::DragNode { .. }));
        assert!(host.selected_node_ids().contains(&"combine".to_string()));
    }

    #[test]
    fn detail_lod_title_row_drags_node() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("detail");
        let combine = host.fixture.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
        let port_idx = combine.inputs().iter().position(|p| p.id == "a").expect("port a");
        let (x0, y0, x1, y1) = input_port_row_hit_bounds(&combine, port_idx).expect("row bounds");
        let title_probe = cavas::vello::kurbo::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let (sx, sy) = world_to_screen_px(&host, title_probe);
        host.pointer_down(sx, sy, false);
        assert!(matches!(host.engine.interaction, InteractionMode::DragNode { .. }));
    }

    #[test]
    fn input_port_row_hit_bounds_span_input_channel() {
        let inputs = vec![
            IoPortSpec { id: "a".into(), label: "a".into() , ..Default::default() },
            IoPortSpec { id: "b".into(), label: "b".into() , ..Default::default() },
        ];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }];
        let width = computation_node_width("Node", &inputs, &outputs);
        let height = computation_node_height(2, 1, false, false);
        let node = DagNodeSpec::computation(
            "n".into(),
            "Node".into(),
            "Node".into(),
            "emoji:🔢".into(),
            inputs,
            outputs,
            false,
            false,
            0.0,
            0.0,
            width,
            height,
        );
        let hw = width * 0.5;
        let divider_x = computation_column_divider_x(&node).expect("divider");
        let (x0, _, x1, _) = input_port_row_hit_bounds(&node, 1).expect("row");
        assert!((x0 - (node.x - hw)).abs() < 1e-9);
        assert!((x1 - divider_x).abs() < 1e-9);
    }

    #[test]
    fn output_port_row_hit_bounds_span_output_channel() {
        let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into() , ..Default::default() }];
        let outputs = vec![
            IoPortSpec { id: "x".into(), label: "x".into() , ..Default::default() },
            IoPortSpec { id: "y".into(), label: "y".into() , ..Default::default() },
        ];
        let width = computation_node_width("Node", &inputs, &outputs);
        let height = computation_node_height(1, 2, false, false);
        let node = DagNodeSpec::computation(
            "n".into(),
            "Node".into(),
            "Node".into(),
            "emoji:🔢".into(),
            inputs,
            outputs,
            false,
            false,
            0.0,
            0.0,
            width,
            height,
        );
        let hw = width * 0.5;
        let divider_x = computation_column_divider_x(&node).expect("divider");
        let (x0, _, x1, _) = output_port_row_hit_bounds(&node, 1).expect("row");
        assert!((x0 - divider_x).abs() < 1e-9);
        assert!((x1 - (node.x + hw)).abs() < 1e-9);
    }

    #[test]
    fn variadic_plus_hit_maps_insert_index() {
        let inputs = vec![
            IoPortSpec { id: "0".into(), label: "0".into() , ..Default::default() },
            IoPortSpec { id: "1".into(), label: "1".into() , ..Default::default() },
        ];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }];
        let width = computation_node_width("dictionary.merge", &inputs, &outputs);
        let height = computation_node_height(2, 1, true, false);
        let host = DagHost::from_fixture_without_layout(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 2.0 },
            nodes: vec![DagNodeSpec::computation(
                "merge".into(),
                "Merge".into(),
                "Merge".into(),
                "emoji:🔀".into(),
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
    fn computation_node_width_uses_two_io_columns_without_name_strip() {
        let inputs = vec![
            IoPortSpec { id: "cornerA".into(), label: "cornerA".into() , ..Default::default() },
            IoPortSpec { id: "cornerB".into(), label: "cornerB".into() , ..Default::default() },
            IoPortSpec { id: "height".into(), label: "height".into() , ..Default::default() },
        ];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "geometry".into() , ..Default::default() }];
        let width = computation_node_width("Box", &inputs, &outputs);
        assert!(width > 70.0 && width < 82.0, "two IO columns should fit port labels, got {width}");
    }

    #[test]
    fn computation_io_columns_clamp_port_label_width() {
        let inputs_short = vec![IoPortSpec { id: "a".into(), label: "a".into() , ..Default::default() }];
        let inputs_long = vec![
            IoPortSpec { id: "cornerA".into(), label: "cornerA".into() , ..Default::default() },
            IoPortSpec { id: "cornerB".into(), label: "cornerB".into() , ..Default::default() },
        ];
        let outputs_short = vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }];
        let outputs_long = vec![IoPortSpec { id: "out".into(), label: "geometry".into() , ..Default::default() }];
        let short = computation_node_width("n", &inputs_short, &outputs_short);
        let long = computation_node_width("n", &inputs_long, &outputs_long);
        assert!(short >= 50.0, "IO columns should not collapse below minimum, got {short}");
        assert!(long <= 78.0, "long port labels should cap column width, got {long}");
        assert!(long >= short, "longer labels should not shrink columns");
    }

    #[test]
    fn computation_column_divider_splits_io_columns() {
        let inputs = vec![
            IoPortSpec { id: "cornerA".into(), label: "cornerA".into() , ..Default::default() },
            IoPortSpec { id: "cornerB".into(), label: "cornerB".into() , ..Default::default() },
        ];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "geometry".into() , ..Default::default() }];
        let width = computation_node_width("Box", &inputs, &outputs);
        let height = computation_node_height(2, 1, false, false);
        let node = DagNodeSpec::computation("box".into(), "Box".into(), "Box".into(), "emoji:📦".into(), inputs, outputs, false, false, 0.0, 0.0, width, height);
        let divider_x = computation_column_divider_x(&node).expect("divider");
        let hw = width * 0.5;
        assert!(divider_x > node.x - hw + 1.0);
        assert!(divider_x < node.x + hw - 1.0);
    }

    #[test]
    fn computation_name_sits_above_rectangle_centered() {
        let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into() , ..Default::default() }];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }];
        let width = computation_node_width("Box", &inputs, &outputs);
        let height = computation_node_height(1, 1, false, false);
        let node = DagNodeSpec::computation("box".into(), "Box".into(), "Box".into(), "emoji:📦".into(), inputs, outputs, false, false, 0.0, 0.0, width, height);
        let paint_px = dag_label_paint_px(1.0, 3);
        let (label_x, label_y) = computation_name_world_center(&node, "Box", paint_px, 1.0);
        assert!((label_x - node.x).abs() < 1e-6);
        let top = node.y - height * 0.5;
        assert!(label_y < top);
        let world_offset = top - label_y;
        let (_, label_h) = cavas::text::label_extent("Box", paint_px);
        let screen_offset = world_offset * 1.0;
        assert!((screen_offset - (DAG_LABEL_SCREEN_PX * ui_styling::metrics::label::DAG_LABEL_GAP_COMPACT_RATIO + label_h * 0.5)).abs() < 1e-6);
    }

    #[test]
    fn io_widget_size_fits_vertical_title() {
        let width = io_widget_width("Amount");
        let height = io_widget_height("Amount");
        assert!(width < height, "control nodes should be taller than wide for vertical titles");
        assert!(height >= 40.0);
    }

    #[test]
    fn slider_widget_size_matches_function_row_metrics() {
        let input = IoPortSpec { id: "in".into(), label: "in".into() , ..Default::default() };
        let output = IoPortSpec { id: "out".into(), label: "value".into() , ..Default::default() };
        let width = slider_widget_width("Amount", &output);
        let height = slider_widget_height();
        assert_eq!(width, computation_node_width("Amount", &[input], &[output]));
        assert_eq!(height, DAG_CHANNEL_ROW_HEIGHT);
        assert!(width > height, "slider track should be wider than tall");
    }

    #[test]
    fn computation_channel_row_count_matches_io_rows() {
        let node = DagNodeSpec::computation(
            "box".into(),
            "Box".into(),
            "Box".into(),
            "emoji:📦".into(),
            vec![
                IoPortSpec { id: "cornerA".into(), label: "cornerA".into() , ..Default::default() },
                IoPortSpec { id: "cornerB".into(), label: "cornerB".into() , ..Default::default() },
                IoPortSpec { id: "height".into(), label: "height".into() , ..Default::default() },
            ],
            vec![IoPortSpec { id: "out".into(), label: "geometry".into() , ..Default::default() }],
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
    fn computation_channel_row_dividers_stop_at_last_port_on_shorter_side() {
        let three_inputs = vec![
            IoPortSpec { id: "a".into(), label: "cornerA".into() , ..Default::default() },
            IoPortSpec { id: "b".into(), label: "cornerB".into() , ..Default::default() },
            IoPortSpec { id: "c".into(), label: "height".into() , ..Default::default() },
        ];
        let one_output = vec![IoPortSpec { id: "out".into(), label: "geometry".into() , ..Default::default() }];
        let three_outputs = vec![
            IoPortSpec { id: "outA".into(), label: "geometry".into() , ..Default::default() },
            IoPortSpec { id: "outB".into(), label: "mesh".into() , ..Default::default() },
            IoPortSpec { id: "outC".into(), label: "curve".into() , ..Default::default() },
        ];
        let one_input = vec![IoPortSpec { id: "a".into(), label: "cornerA".into() , ..Default::default() }];
        let more_inputs = DagNodeSpec::computation(
            "more-in".into(),
            "Box".into(),
            "Box".into(),
            "emoji:📦".into(),
            three_inputs.clone(),
            one_output.clone(),
            false,
            false,
            0.0,
            0.0,
            computation_node_width("Box", &three_inputs, &one_output),
            computation_node_height(3, 1, false, false),
        );
        let more_outputs = DagNodeSpec::computation(
            "more-out".into(),
            "Box".into(),
            "Box".into(),
            "emoji:📦".into(),
            one_input.clone(),
            three_outputs.clone(),
            false,
            false,
            0.0,
            0.0,
            computation_node_width("Box", &one_input, &three_outputs),
            computation_node_height(1, 3, false, false),
        );
        let grid = computation_channel_row_count(&more_inputs);
        assert_eq!(grid, 3);
        assert_eq!(computation_io_side_row_counts(&more_inputs), (3, 1));
        assert_eq!(computation_io_side_row_divider_indices(3, grid).collect::<Vec<_>>(), vec![1, 2]);
        assert_eq!(computation_io_side_row_divider_indices(1, grid).collect::<Vec<_>>(), vec![1]);
        assert_eq!(computation_io_side_row_counts(&more_outputs), (1, 3));
        assert_eq!(computation_io_side_row_divider_indices(1, grid).collect::<Vec<_>>(), vec![1]);
        assert_eq!(computation_io_side_row_divider_indices(3, grid).collect::<Vec<_>>(), vec![1, 2]);
    }

    #[test]
    fn computation_channel_row_dividers_align_with_row_bounds() {
        let inputs = vec![
            IoPortSpec { id: "a".into(), label: "cornerA".into() , ..Default::default() },
            IoPortSpec { id: "b".into(), label: "cornerB".into() , ..Default::default() },
            IoPortSpec { id: "c".into(), label: "height".into() , ..Default::default() },
        ];
        let outputs = vec![
            IoPortSpec { id: "outA".into(), label: "geometry".into() , ..Default::default() },
            IoPortSpec { id: "outB".into(), label: "mesh".into() , ..Default::default() },
        ];
        let width = computation_node_width("Box", &inputs, &outputs);
        let height = computation_node_height(3, 2, false, false);
        let node = DagNodeSpec::computation("box".into(), "Box".into(), "Box".into(), "emoji:📦".into(), inputs, outputs, false, false, 0.0, 0.0, width, height);
        let (input_rows, output_rows) = computation_io_side_row_counts(&node);
        assert_eq!(input_rows, 3);
        assert_eq!(output_rows, 2);
        let (input_left, input_right) = computation_input_column_x_bounds(&node).expect("input column");
        let (output_left, output_right) = computation_output_column_x_bounds(&node).expect("output column");
        assert!(input_left < input_right);
        assert!(output_left < output_right);
        assert!(input_right < output_left);
        let divider_x = computation_column_divider_x(&node).expect("divider");
        let (input_span_left, input_span_right) = computation_channel_row_divider_x_span(&node, ComputationChannelRowSide::Input);
        let (output_span_left, output_span_right) = computation_channel_row_divider_x_span(&node, ComputationChannelRowSide::Output);
        assert!((input_span_left - (node.x - width * 0.5)).abs() < 1e-6);
        assert!((input_span_right - divider_x).abs() < 1e-6);
        assert!((output_span_left - divider_x).abs() < 1e-6);
        assert!((output_span_right - (node.x + width * 0.5)).abs() < 1e-6);
        assert!(input_span_right > input_right);
        assert!(output_span_left < output_left);
        let divider_y = channel_row_divider_y(node.y, node.height, 1);
        let (_, _row0_top, _, row0_bottom) = channel_row_bounds(&node, 0);
        let (_, row1_top, _, _row1_bottom) = channel_row_bounds(&node, 1);
        assert!((divider_y - row0_bottom).abs() < 1e-6);
        assert!((divider_y - row1_top).abs() < 1e-6);
    }

    #[test]
    fn computation_node_size_fits_io_labels() {
        let inputs = vec![
            IoPortSpec { id: "width".into(), label: "width".into() , ..Default::default() },
            IoPortSpec { id: "depth".into(), label: "depth".into() , ..Default::default() },
            IoPortSpec { id: "height".into(), label: "height".into() , ..Default::default() },
        ];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "geometry".into() , ..Default::default() }];
        let width = computation_node_width("brep.prim3d.box", &inputs, &outputs);
        let height = computation_node_height(3, 1, false, false);
        assert!(height <= 42.0, "expected compact height, got {height}");
        assert!(height < 96.0, "expected shorter than legacy 4-row layout");
        assert!(width > 100.0 && width < 120.0, "expected balanced IO column width, got {width}");
    }

    #[test]
    fn io_node_rect_port_angles_on_edges() {
        use cavas::vello::kurbo::Point;
        use graph::handle_position_on_rectangle;
        let inputs = vec![
            IoPortSpec { id: "a".into(), label: "a".into() , ..Default::default() },
            IoPortSpec { id: "b".into(), label: "b".into() , ..Default::default() },
        ];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }];
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
    fn computation_port_handle_caps_bulge_outward() {
        use cavas::vello::kurbo::{Point, Shape};
        use graph::{handle_exterior_cap_fill_path, handle_outward_at_node_rim, handle_position_on_rectangle, NodeShape};
        let inputs = vec![
            IoPortSpec { id: "0".into(), label: "0".into() , ..Default::default() },
            IoPortSpec { id: "1".into(), label: "1".into() , ..Default::default() },
        ];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "dictionary".into() , ..Default::default() }];
        let width = computation_node_width("Merge", &inputs, &outputs);
        let height = computation_node_height(2, 1, true, false);
        let center = Point::new(100.0, 50.0);
        let left_angle = io_node_rect_port_angle(center.x, center.y, width, height, 0, 2, true);
        let right_angle = io_node_rect_port_angle(center.x, center.y, width, height, 0, 1, false);
        let left_pos = handle_position_on_rectangle(center, width, height, left_angle);
        let right_pos = handle_position_on_rectangle(center, width, height, right_angle);
        let left_out = handle_outward_at_node_rim(left_pos, center, NodeShape::Rectangle, 0.0, width, height).expect("left outward");
        let right_out = handle_outward_at_node_rim(right_pos, center, NodeShape::Rectangle, 0.0, width, height).expect("right outward");
        assert!((left_out.x + 1.0).abs() < 1e-9 && left_out.y.abs() < 1e-9);
        assert!((right_out.x - 1.0).abs() < 1e-9 && right_out.y.abs() < 1e-9);
        let left_cap = handle_exterior_cap_fill_path(left_pos, left_out, DAG_HANDLE_WORLD_RADIUS);
        let right_cap = handle_exterior_cap_fill_path(right_pos, right_out, DAG_HANDLE_WORLD_RADIUS);
        assert!(left_cap.bounding_box().x0 < left_pos.x - 1.0, "left input cap must bulge outside the west edge");
        assert!(right_cap.bounding_box().x1 > right_pos.x + 1.0, "right output cap must bulge outside the east edge");
    }

    #[test]
    fn dag_draw_lod_maps_zoom_to_puzzle2d_bands() {
        assert_eq!(dag_draw_lod(0.1), DagDrawLod::Minimap);
        assert_eq!(dag_draw_lod(0.45), DagDrawLod::Overview);
        assert_eq!(dag_draw_lod(0.65), DagDrawLod::Compact);
        assert_eq!(dag_draw_lod(1.2), DagDrawLod::Normal);
        assert_eq!(dag_draw_lod(2.5), DagDrawLod::Detail);
        assert_eq!(dag_draw_lod(3.0), DagDrawLod::Micro);
        assert_eq!(dag_draw_lod(5.0), DagDrawLod::Micro);
    }

    #[test]
    fn dag_draw_lod_progressive_disclosure_gates() {
        assert_eq!(DagDrawLod::Normal.node_label(), DagNodeLabel::Name);
        assert_eq!(DagDrawLod::Detail.node_label(), DagNodeLabel::Abbreviation);
        assert!(DagDrawLod::Normal.shows_computation_layout());
        assert!(!DagDrawLod::Compact.shows_computation_layout());
        assert!(!DagDrawLod::Normal.shows_handles());
        assert!(DagDrawLod::Detail.shows_handles());
        assert!(DagDrawLod::Normal.uses_input_row_connection_hitbox());
        assert!(!DagDrawLod::Detail.uses_input_row_connection_hitbox());
        assert!(DagDrawLod::Detail.uses_channel_row_pick());
        assert!(DagDrawLod::Micro.uses_channel_row_pick());
        assert!(!DagDrawLod::Normal.uses_channel_row_pick());
        assert!(!DagDrawLod::Minimap.allows_connection_hit_picking());
        assert!(!DagDrawLod::Overview.allows_connection_hit_picking());
        assert!(!DagDrawLod::Compact.allows_connection_hit_picking());
        assert!(DagDrawLod::Normal.allows_connection_hit_picking());
        assert!(DagDrawLod::Detail.allows_connection_hit_picking());
        assert!(DagDrawLod::Micro.allows_connection_hit_picking());
        assert!(DagDrawLod::Detail.shows_port_labels());
        assert!(DagDrawLod::Micro.shows_port_labels());
        assert_eq!(DagDrawLod::Minimap.edge_stroke_screen_px(), DAG_EDGE_STROKE_MINIMAP_SCREEN_PX);
        assert_eq!(DagDrawLod::Normal.edge_stroke_screen_px(), DAG_EDGE_STROKE_SCREEN_PX);
    }

    #[test]
    fn wheel_zoom_pins_draw_lod_until_gesture_ends() {
        let mut host = DagHost::default_demo();
        host.fixture.camera.zoom = 1.2;
        assert_eq!(host.draw_lod_for_frame(), DagDrawLod::Normal);
        host.set_wheel_zoom_active(true);
        host.fixture.camera.zoom = 0.45;
        assert_eq!(host.draw_lod_for_frame(), DagDrawLod::Normal);
        host.set_wheel_zoom_active(false);
        assert_eq!(host.draw_lod_for_frame(), DagDrawLod::Overview);
    }

    #[test]
    fn dag_handle_world_radius_is_zoom_invariant() {
        assert_eq!(DAG_HANDLE_WORLD_RADIUS, 5.0);
    }

    #[test]
    fn dag_label_paint_px_scales_with_zoom_inside_lod_band() {
        assert_eq!(dag_label_layout_px(), DAG_LABEL_SCREEN_PX);
        let normal = 3usize;
        let normal_floor = dag_lod_band_floor_zoom(normal);
        assert!((dag_label_paint_px(normal_floor, normal) - DAG_LABEL_SCREEN_PX).abs() < 1e-9);
        assert!((dag_label_paint_px(1.1, normal) - DAG_LABEL_SCREEN_PX * 1.1 / normal_floor).abs() < 1e-9);
        assert!((dag_label_paint_px(normal_floor, normal) - dag_label_paint_px(1.1, normal) * normal_floor / 1.1).abs() < 1e-9);
        assert!((dag_label_compact_paint_px(1.1, normal) - DAG_LABEL_COMPACT_SCREEN_PX * 1.1 / normal_floor).abs() < 1e-9);
    }

    #[test]
    fn dag_paint_scene_keeps_labels_when_lod_forced_at_low_zoom() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("compact");
        host.fixture.camera.zoom = 0.25;
        let mut scene = cavas::vello::Scene::new();
        host.paint_scene(&mut scene, 1280, 800, 1.0);
        assert!(
            scene.encoding().path_tags.len() > 12,
            "compact LOD at low zoom should still paint abbreviation labels"
        );
    }

    #[test]
    fn dag_label_colors_use_theme_label_fields() {
        use cavas::vello::peniko::Color;
        let theme = VelloThemePalette {
            label_fill: Color::from_rgba8(240, 241, 245, 255),
            label_halo: Color::from_rgba8(10, 12, 16, 180),
            node_stroke: Color::from_rgba8(90, 100, 110, 255),
            ..VelloThemePalette::default()
        };
        assert_ne!(theme.label_fill.to_rgba8(), theme.node_stroke.to_rgba8());
    }

    #[test]
    fn dag_node_paint_fill_matches_puzzle2d_lod_chrome() {
        use cavas::vello::peniko::Color;
        let theme = VelloThemePalette {
            node_fill: Color::from_rgba8(10, 20, 30, 255),
            node_stroke: Color::from_rgba8(200, 210, 220, 255),
            node_fill_hovered: Color::from_rgba8(40, 50, 60, 255),
            node_stroke_hovered: Color::from_rgba8(90, 100, 110, 255),
            node_fill_selected: Color::from_rgba8(70, 80, 90, 255),
            node_stroke_selected: Color::from_rgba8(120, 130, 140, 255),
            node_fill_selection_exit: Color::from_rgba8(196, 228, 213, 255),
            node_stroke_selection_exit: Color::from_rgba8(80, 140, 110, 255),
            ..VelloThemePalette::default()
        };
        assert_eq!(
            dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, false, false, false)
                .expect("minimap neutral")
                .to_rgba8(),
            theme.node_stroke.to_rgba8()
        );
        assert!(dag_node_paint_fill(DagDrawLod::Overview, &theme, false, false, false, false).is_none());
        assert!(dag_node_paint_fill(DagDrawLod::Normal, &theme, false, false, false, false).is_none());
        assert_eq!(
            dag_node_paint_fill(DagDrawLod::Normal, &theme, false, true, false, false)
                .expect("selected")
                .to_rgba8(),
            theme.node_fill_selected.to_rgba8()
        );
        assert_eq!(
            dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, false, false, true)
                .expect("minimap hovered")
                .to_rgba8(),
            theme.node_stroke_hovered.to_rgba8()
        );
        assert_eq!(
            dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, false, true, false)
                .expect("minimap highlighted")
                .to_rgba8(),
            theme.node_stroke_selection_exit.to_rgba8()
        );
        assert_eq!(
            dag_node_body_stroke(&theme, false, true, false, true).to_rgba8(),
            theme.node_stroke_selected.to_rgba8()
        );
        assert_eq!(
            dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, true, false, true)
                .expect("minimap selected")
                .to_rgba8(),
            theme.node_stroke_selected.to_rgba8()
        );
        assert_ne!(
            dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, false, false, true)
                .expect("minimap hovered")
                .to_rgba8(),
            theme.node_fill_hovered.to_rgba8()
        );
        assert_eq!(
            dag_node_body_stroke(&theme, false, false, false, true).to_rgba8(),
            theme.node_stroke_hovered.to_rgba8()
        );
        assert_eq!(
            dag_node_body_stroke(&theme, false, false, true, false).to_rgba8(),
            theme.node_stroke_selection_exit.to_rgba8()
        );
        assert_eq!(
            dag_node_label_fill(&theme, false, false, false, true).to_rgba8(),
            theme.label_fill_hovered.to_rgba8()
        );
        assert_eq!(
            dag_node_label_fill(&theme, false, false, true, false).to_rgba8(),
            theme.node_stroke_selection_exit.to_rgba8()
        );
        assert_eq!(
            dag_node_label_fill(&theme, false, false, false, false).to_rgba8(),
            theme.label_fill.to_rgba8()
        );
        assert_eq!(
            dag_node_label_fill(&theme, false, true, false, false).to_rgba8(),
            theme.label_fill_hovered.to_rgba8()
        );
        let body = dag_node_body_stroke(&theme, false, false, false, false);
        let label = dag_node_label_fill(&theme, false, false, false, true);
        assert_eq!(
            dag_node_internal_chrome_stroke(body, label, true).to_rgba8(),
            label.to_rgba8()
        );
        assert_eq!(
            dag_node_internal_chrome_stroke(body, label, false).to_rgba8(),
            body.to_rgba8()
        );
        let body_selected = dag_node_body_stroke(&theme, false, true, false, false);
        let label_selected = dag_node_label_fill(&theme, false, true, false, false);
        assert_eq!(
            dag_node_internal_chrome_stroke(body_selected, label_selected, true).to_rgba8(),
            label_selected.to_rgba8()
        );
        assert_eq!(
            dag_node_internal_chrome_stroke(body_selected, label_selected, false).to_rgba8(),
            body_selected.to_rgba8()
        );
    }

    #[test]
    fn dag_handle_and_edge_stroke_use_theme_defaults() {
        use cavas::vello::peniko::Color;
        let theme = VelloThemePalette {
            edge_stroke: Color::from_rgba8(100, 110, 120, 255),
            edge_stroke_hovered: Color::from_rgba8(10, 20, 30, 255),
            edge_stroke_selected: Color::from_rgba8(40, 50, 60, 255),
            handle_stroke: Color::from_rgba8(130, 140, 150, 255),
            handle_stroke_hovered: Color::from_rgba8(20, 30, 40, 255),
            handle_stroke_selected: Color::from_rgba8(50, 60, 70, 255),
            ..VelloThemePalette::default()
        };
        assert_eq!(dag_edge_body_stroke(&theme, false, false, false, false).to_rgba8(), theme.edge_stroke.to_rgba8());
        assert_eq!(dag_handle_body_stroke(&theme, false, false, false, false).to_rgba8(), theme.handle_stroke.to_rgba8());
    }

    #[test]
    fn manual_lod_pins_draw_tier_until_automatic_restored() {
        let mut host = DagHost::default_demo();
        host.fixture.camera.zoom = 1.0;
        assert_eq!(host.draw_lod_label(), "normal");
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("minimap");
        assert_eq!(host.draw_lod_for_frame(), DagDrawLod::Minimap);
        host.fixture.camera.zoom = 5.0;
        assert_eq!(host.draw_lod_for_frame(), DagDrawLod::Minimap);
        host.set_automatic_lod(true);
        assert_eq!(host.draw_lod_for_frame(), DagDrawLod::Micro);
    }

    #[test]
    fn note_widget_size_grows_with_text() {
        let short = note_widget_size("hi");
        let long = note_widget_size("some longer note text");
        assert!(long.0 > short.0);
        assert!(short.0 >= DAG_PREVIEW_MIN_SIZE + DAG_PREVIEW_PAD * 2.0);
    }

    #[test]
    fn fit_note_sizes_resizes_after_text_change() {
        let mut host = DagHost::from_fixture(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec {
                id: "note".into(),
                name: "Note".into(),
                abbreviation: "Note".into(),
                icon: "emoji:📝".into(),
                x: 0.0,
                y: 0.0,
                width: 80.0,
                height: 80.0,
                kind: DagNodeKind::Note { text: "hi".into(), output: IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() } },
            }],
            edges: vec![],
        });
        let short_h = host.fixture.nodes[0].height;
        let DagNodeKind::Note { text, .. } = &mut host.fixture.nodes[0].kind else {
            panic!("expected note");
        };
        *text = "a much longer note body".into();
        host.fit_note_sizes();
        assert!(host.fixture.nodes[0].height >= short_h);
        assert!(host.fixture.nodes[0].width > note_widget_size("hi").0);
    }

    #[test]
    fn note_label_overlay_skips_title_and_ports() {
        let mut host = DagHost::from_fixture(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec {
                id: "note".into(),
                name: "Note".into(),
                abbreviation: "Note".into(),
                icon: "emoji:📝".into(),
                x: 0.0,
                y: 0.0,
                width: note_widget_size("hello").0,
                height: note_widget_size("hello").1,
                kind: DagNodeKind::Note { text: "hello".into(), output: IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() } },
            }],
            edges: vec![],
        });
        host.set_viewport(800, 600, 1.0);
        let raw: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
        let labels = raw["labels"].as_array().expect("labels");
        assert!(labels.iter().all(|row| row["text"] != "Note" && row["text"] != "out"));
    }

    #[test]
    fn note_preview_action_port_accessors() {
        let note = DagNodeSpec {
            id: "note".into(),
            name: "Note".into(),
            abbreviation: "Note".into(),
            icon: "emoji:📝".into(),
            x: 0.0,
            y: 0.0,
            width: note_widget_size("hi").0,
            height: note_widget_size("hi").1,
            kind: DagNodeKind::Note { text: "hi".into(), output: IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() } },
        };
        assert!(note.inputs().is_empty());
        assert_eq!(note.outputs().len(), 1);
        let preview = DagNodeSpec {
            id: "preview".into(),
            name: "Preview".into(),
            abbreviation: "Preview".into(),
            icon: "emoji:👁️".into(),
            x: 0.0,
            y: 0.0,
            width: 120.0,
            height: 48.0,
            kind: DagNodeKind::Preview {
                content: DagPreviewContent::Scalar { text: "3".into() },
                expanded: BTreeSet::new(),
                input: IoPortSpec { id: "in".into(), label: "in".into() , ..Default::default() },
            },
        };
        assert_eq!(preview.inputs().len(), 1);
        assert!(preview.outputs().is_empty());
    }

    #[test]
    fn to_pascal_case_normalizes_spaced_labels() {
        assert_eq!(to_pascal_case("pass through"), "PassThrough");
        assert_eq!(to_pascal_case("Draw Rectangle"), "DrawRectangle");
    }

    #[test]
    fn dag_draw_lod_node_content_matrix() {
        assert!(!DagDrawLod::Minimap.node_icon_visible());
        assert_eq!(DagDrawLod::Minimap.node_label(), DagNodeLabel::None);
        assert!(DagDrawLod::Overview.node_icon_visible());
        assert_eq!(DagDrawLod::Overview.node_label(), DagNodeLabel::None);
        assert!(!DagDrawLod::Compact.node_icon_visible());
        assert_eq!(DagDrawLod::Compact.node_label(), DagNodeLabel::Abbreviation);
        assert!(!DagDrawLod::Normal.node_icon_visible());
        assert_eq!(DagDrawLod::Normal.node_label(), DagNodeLabel::Name);
        assert!(!DagDrawLod::Detail.node_icon_visible());
        assert_eq!(DagDrawLod::Detail.node_label(), DagNodeLabel::Abbreviation);
        assert!(!DagDrawLod::Micro.node_icon_visible());
        assert_eq!(DagDrawLod::Micro.node_label(), DagNodeLabel::Name);
        let computation = DagNodeSpec::computation(
            "add".into(),
            "Add".into(),
            "Add".into(),
            "emoji:➕".into(),
            vec![IoPortSpec { id: "a".into(), label: "a".into() , ..Default::default() }],
            vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }],
            false,
            false,
            0.0,
            0.0,
            120.0,
            56.0,
        );
        assert!(DagHost::should_paint_node_lod_icon(&computation, DagDrawLod::Overview));
        assert!(!DagHost::should_paint_node_lod_icon(&computation, DagDrawLod::Detail));
        assert!(!DagHost::should_paint_node_lod_icon(&computation, DagDrawLod::Micro));
    }

    #[test]
    fn dag_node_spec_round_trips_display_fields() {
        let node = DagNodeSpec::computation(
            "n".into(),
            "pass through".into(),
            "pass".into(),
            "emoji:➡️".into(),
            vec![],
            vec![IoPortSpec { id: "out".into(), label: "out".into() , ..Default::default() }],
            false,
            false,
            0.0,
            0.0,
            80.0,
            24.0,
        );
        let json = serde_json::to_string(&node).unwrap();
        let back: DagNodeSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(back.name, "PassThrough");
        assert_eq!(back.abbreviation, "Pass");
        assert_eq!(back.icon, "emoji:➡️");
    }

    #[test]
    fn preview_tree_toggle_expands_and_resizes() {
        let json = serde_json::json!({ "alpha": { "beta": 1 }, "gamma": "x" });
        let mut host = DagHost::from_fixture(DagFixtureV1 {
            schema: "dag.fixture/v1".into(),
            camera: DagCameraV1 { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec {
                id: "preview".into(),
                name: "Preview".into(),
                abbreviation: "Preview".into(),
                icon: "emoji:👁️".into(),
                x: 0.0,
                y: 0.0,
                width: 80.0,
                height: 80.0,
                kind: DagNodeKind::Preview {
                    content: DagPreviewContent::Tree { json },
                    expanded: BTreeSet::new(),
                    input: IoPortSpec { id: "in".into(), label: "in".into() , ..Default::default() },
                },
            }],
            edges: vec![],
        });
        host.set_viewport(800, 600, 1.0);
        let collapsed_h = host.fixture.nodes[0].height;
        let layouts = preview_tree_row_layouts(&host.fixture.nodes[0], &serde_json::json!({ "alpha": { "beta": 1 }, "gamma": "x" }), &BTreeSet::new());
        let row = layouts.iter().find(|entry| entry.path == "alpha").expect("alpha row");
        let (x0, y0, x1, y1) = row.row_rect;
        let world_x = x0 + (x1 - x0) * 0.75;
        let world_y = (y0 + y1) * 0.5;
        use cavas::camera::{world_to_screen, Camera as CavasCamera, Viewport};
        use cavas::vello::kurbo::Point;
        let cam = CavasCamera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
        let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
        let screen = world_to_screen(&cam, &viewport, Point::new(world_x, world_y));
        host.pointer_down(screen.x, screen.y, false);
        let expanded_h = host.fixture.nodes[0].height;
        assert!(expanded_h > collapsed_h);
        let DagNodeKind::Preview { expanded, .. } = &host.fixture.nodes[0].kind else {
            panic!("preview kind");
        };
        assert!(expanded.contains("alpha"));
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

    #[test]
    fn cluster_node_round_trips_serde() {
        let inputs = vec![IoPortSpec::simple("a", "a")];
        let outputs = vec![IoPortSpec::simple("out", "out")];
        let node = DagNodeSpec::cluster(
            "cluster".into(),
            "Cluster".into(),
            "Cluster".into(),
            "emoji:🧩".into(),
            inputs,
            outputs,
            10.0,
            20.0,
            120.0,
            80.0,
        );
        let json = serde_json::to_string(&node).unwrap();
        let back: DagNodeSpec = serde_json::from_str(&json).unwrap();
        assert!(matches!(back.kind, DagNodeKind::Cluster { .. }));
    }

    #[test]
    fn cluster_explode_hit_rect_detects_top_right_affordance() {
        let inputs = vec![IoPortSpec::simple("a", "a")];
        let outputs = vec![IoPortSpec::simple("out", "out")];
        let node = DagNodeSpec::cluster(
            "cluster".into(),
            "Cluster".into(),
            "Cluster".into(),
            "emoji:🧩".into(),
            inputs,
            outputs,
            0.0,
            0.0,
            120.0,
            80.0,
        );
        let (x0, y0, x1, y1) = cluster_explode_hit_rect(&node).expect("rect");
        assert!(cluster_explode_hit(&node, (x0 + x1) * 0.5, (y0 + y1) * 0.5));
        assert!(!cluster_explode_hit(&node, node.x - 50.0, node.y - 50.0));
    }
}
// #endregion 🔖Tests
