//! 🌳 Directed acyclic port graph: rectangle IO nodes on infinite canvas.

use std::cell::Cell;
use std::collections::{BTreeSet, HashMap, HashSet};

use serde::{Deserialize, Serialize};

use mathematical_graph_manifest::{flow_dag::flow_dag_manifest, ManifestValidator, PropertyBag};
#[cfg(test)]
use mathematical_graph_manifest::PropertyValue;

use graph::{handle_position, world_box_from_points, BoardEvent, WorldBox};
pub use infinite_board_port_directed::{
    self as graph, compute_edge_bezier_points, compute_edge_sharp_sz_path, handle_exterior_cap_fill_path, handle_exterior_cap_peak, handle_exterior_cap_stroke_path, handle_exterior_cap_triangle_fill_path, handle_exterior_cap_triangle_peak,
    handle_exterior_cap_triangle_stroke_path, handle_outward_at_node_rim, CanvasPalette, DirectedPortGraphEngine, Edge, EdgeId, GraphExtension, Handle, HandleId, HandleRole, InteractionMode, Node, NodeId, RenderSnapshot, Selection,
};
pub use infinite_cavas as cavas;

/// 🌳 DAG board engine alias.
pub type DagBoardEngine = DirectedPortGraphEngine;

//#region ⚠️ Errors
/// 🚨 Crate-local error for DAG fixture parsing, layout, and host-state mutation.
#[derive(Debug, thiserror::Error)]
pub enum DagError {
    #[error("fixture root must be object")]
    FixtureRootNotObject,
    #[error("schema must be dag.fixture")]
    SchemaMismatch,
    #[error("nodes array missing")]
    NodesMissing,
    #[error("{0}")]
    InvalidNodeKind(String),
    #[error("unknown align mode: {0}")]
    UnknownAlignMode(String),
    #[error("unknown widget: {0}")]
    UnknownWidget(String),
    #[error("{0}")]
    CanvasTheme(String),
    #[error("gridFactor must be finite and in (0, 1e6]")]
    GridFactorOutOfRange,
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
//#endregion ⚠️ Errors

// #region 🔖PortSide
/// ↔️ Which side of a computation node a variadic insert targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagPortSide {
    Input,
    Output,
}
// #endregion 🔖PortSide

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
const DAG_IO_COLUMN_WIDTH: f64 = ui_styling::metrics::dag::IO_COLUMN_WIDTH;
const DAG_COMPONENT_WIDTH: f64 = DAG_IO_COLUMN_WIDTH * 2.0;
const DAG_DISTRIBUTE_MIN_GAP: f64 = ui_styling::metrics::board::LAYOUT_SIBLING_GAP;
const DAG_IO_WIDGET_HEIGHT: f64 = ui_styling::metrics::dag::IO_WIDGET_HEIGHT;
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
    (computation_io_row_count(input_count, output_count, variadic_inputs, variadic_outputs) + DAG_COMPUTATION_HEADER_ROWS) as f64 * DAG_CHANNEL_ROW_HEIGHT
}

fn port_label_text_width(label: &str, px: f64) -> f64 {
    let trimmed = label.trim();
    if trimmed.is_empty() || px < 4.0 {
        return 0.0;
    }
    let pad = px * 0.28;
    trimmed.len() as f64 * px * 0.62 + pad * 2.0
}

fn io_port_column_width(ports: &[IoPortSpec], _px: f64) -> f64 {
    if ports.is_empty() {
        0.0
    } else {
        DAG_IO_COLUMN_WIDTH
    }
}

/// 📐 Uniform computation component width shared by every flow component node.
pub fn computation_node_width(_name: &str, _inputs: &[IoPortSpec], _outputs: &[IoPortSpec]) -> f64 {
    DAG_COMPONENT_WIDTH
}

/// 📐 IO widget width aligned with all flow components.
pub fn io_widget_width(_name: &str) -> f64 {
    DAG_COMPONENT_WIDTH
}

/// 📐 IO widget height from vertically rotated title metrics plus a control band.
pub fn io_widget_height(name: &str) -> f64 {
    use cavas::text::label_extent;
    let name_px = DAG_LABEL_SCREEN_PX * ui_styling::metrics::label::DAG_LABEL_SCALE_MULT;
    let (label_w, _) = label_extent(name, name_px);
    (label_w + DAG_IO_WIDGET_HEIGHT + DAG_NODE_EDGE_INSET * 2.0).max(40.0)
}

/// 📐 Slider track width aligned with computation components.
pub fn slider_widget_width(_name: &str, _output: &IoPortSpec) -> f64 {
    DAG_COMPONENT_WIDTH
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
    let half = if uses_computation_layout(&node.kind) { DAG_CHANNEL_ROW_HEIGHT * 0.5 } else { node.height / count as f64 * 0.5 };
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
    let half = if uses_computation_layout(&node.kind) { DAG_CHANNEL_ROW_HEIGHT * 0.5 } else { node.height / count as f64 * 0.5 };
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

/// 🔌 Visual shape of a port handle cap.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum PortShape {
    #[default]
    Semicircle,
    Triangle,
}

/// 📐 Edge routing style between port handles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum EdgeRouteStyle {
    #[default]
    Bezier,
    SharpSz,
}

/// 🪝 Named horizontal port on a DAG node edge.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
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
    #[dsl(key = "type")]
    pub value_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    #[serde(rename = "resourceKind", skip_serializing_if = "Option::is_none")]
    pub resource_kind: Option<String>,
    #[serde(default = "default_port_cardinality")]
    pub cardinality: String,
    #[serde(default)]
    pub shape: PortShape,
    #[serde(default = "default_port_visible")]
    pub visible: bool,
}

fn default_port_visible() -> bool {
    true
}

fn default_port_cardinality() -> String {
    "!".into()
}

impl Default for IoPortSpec {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            code: String::new(),
            abbreviation: String::new(),
            full_name: String::new(),
            value_type: None,
            default: None,
            value: None,
            connected: None,
            resource_kind: None,
            cardinality: default_port_cardinality(),
            shape: PortShape::default(),
            visible: true,
        }
    }
}

impl IoPortSpec {
    pub fn named(code: impl Into<String>, abbreviation: impl Into<String>, id: impl Into<String>, full_name: impl Into<String>) -> Self {
        let id = id.into();
        let abbreviation = abbreviation.into();
        Self { code: code.into(), abbreviation: abbreviation.clone(), label: abbreviation, id, full_name: full_name.into(), cardinality: default_port_cardinality(), shape: PortShape::default(), ..Default::default() }
    }

    pub fn simple(id: impl Into<String>, label: impl Into<String>) -> Self {
        let id = id.into();
        let label = label.into();
        let code = if id.len() <= 2 { id.to_uppercase() } else { id.chars().take(2).collect::<String>().to_uppercase() };
        let abbreviation = if label.len() <= 3 { label.clone() } else { label.chars().take(3).collect() };
        Self { id: id.clone(), label: label.clone(), code, abbreviation: abbreviation.clone(), full_name: label, ..Default::default() }
    }

    pub fn display_code(&self) -> &str {
        if !self.code.is_empty() {
            return self.code.as_str();
        }
        self.label.as_str()
    }

    pub fn label_with_cardinality(&self, lod: DagDrawLod) -> String {
        let label = self.display_label(lod).trim();
        if label.is_empty() {
            return self.cardinality.clone();
        }
        format!("{} {}", self.cardinality, label)
    }
}

/// 🖼 Screen media payload for output nodes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DagMedia {
    pub kind: DagMediaKind,
    pub src: String,
}

/// 🎬 Screen media kind discriminator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(rename_all = "camelCase", tag = "variant")]
pub enum DagPreviewContent {
    #[default]
    Empty,
    Scalar {
        text: String,
    },
    Image {
        src: String,
    },
    Tree {
        json: serde_json::Value,
    },
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
    ExportClick,
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
            if let Ok((_, _, bw, bh)) = cavas::svg_icon::svg_icon_content_bounds_from_str(&s) {
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

fn truncate_label_to_fit_width(text: &str, max_width: f64, px: f64) -> String {
    use cavas::text::label_extent;
    let trimmed = text.trim();
    if trimmed.is_empty() || max_width <= 0.0 || px < 4.0 {
        return text.to_string();
    }
    let (full_w, _) = label_extent(trimmed, px);
    if full_w <= max_width {
        return text.to_string();
    }
    let ellipsis = "…";
    let mut best = ellipsis.to_string();
    for (byte_idx, ch) in trimmed.char_indices() {
        let end = byte_idx + ch.len_utf8();
        let candidate = format!("{}{ellipsis}", &trimmed[..end]);
        let (w, _) = label_extent(&candidate, px);
        if w <= max_width {
            best = candidate;
        } else {
            break;
        }
    }
    best
}

fn note_text_origin_x(node: &DagNodeSpec) -> f64 {
    let (x0, _, _, _) = preview_content_bounds(node);
    x0 + DAG_PREVIEW_PAD
}

fn hit_byte_in_note_line(line: &str, world_x: f64, line_origin_x: f64, font_px: f64) -> usize {
    use cavas::text::label_byte_world_x;
    if line.is_empty() {
        return 0;
    }
    let mut boundaries = vec![0usize];
    for (index, _) in line.char_indices() {
        if index > 0 {
            boundaries.push(index);
        }
    }
    if boundaries.last().copied() != Some(line.len()) {
        boundaries.push(line.len());
    }
    for pair in boundaries.windows(2) {
        let start = pair[0];
        let end = pair[1];
        let x0 = label_byte_world_x(line, start, line_origin_x, font_px);
        let x1 = label_byte_world_x(line, end, line_origin_x, font_px);
        if world_x < (x0 + x1) * 0.5 {
            return start;
        }
    }
    line.len()
}

fn preview_tree_rows(json: &serde_json::Value, expanded: &BTreeSet<String>, path: &str, depth: usize) -> Vec<PreviewTreeRow> {
    let mut rows = Vec::new();
    match json {
        serde_json::Value::Object(map) => {
            for (key, val) in map {
                let row_path = if path.is_empty() { key.clone() } else { format!("{path}.{key}") };
                let has_children = matches!(val, serde_json::Value::Object(_) | serde_json::Value::Array(_));
                let is_expanded = expanded.contains(&row_path);
                let summary = if has_children { preview_tree_collapsed_summary(val) } else { preview_tree_scalar_display(val) };
                rows.push(PreviewTreeRow { path: row_path.clone(), depth, label: key.clone(), summary, has_children, expanded: is_expanded });
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
                let summary = if has_children { preview_tree_collapsed_summary(val) } else { preview_tree_scalar_display(val) };
                rows.push(PreviewTreeRow { path: row_path.clone(), depth, label: key, summary, has_children, expanded: is_expanded });
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
                    let line = if row.has_children && row.expanded { row.label.clone() } else { format!("{}: {}", row.label, row.summary) };
                    indent + toggle + port_label_text_width(&line, DAG_LABEL_COMPACT_SCREEN_PX)
                })
                .fold(DAG_PREVIEW_MIN_SIZE, f64::max);
            (max_w, rows.len() as f64 * DAG_PREVIEW_ROW_HEIGHT)
        }
    }
}

fn preview_content_node_size(content: &DagPreviewContent, expanded: &BTreeSet<String>) -> (f64, f64) {
    let (_, ch) = measure_preview_content(content, expanded);
    (DAG_COMPONENT_WIDTH, ch + DAG_PREVIEW_PAD * 2.0)
}

fn preview_image_node_size(src: &str) -> (f64, f64) {
    let inner_w = (DAG_COMPONENT_WIDTH - DAG_PREVIEW_PAD * 2.0).max(1.0);
    let (nw, nh) = preview_media_natural_size(src);
    let (cw, ch) = clamp_preview_image_size(nw, nh);
    let aspect = if cw > 0.0 { ch / cw } else { 1.0 };
    let inner_h = (inner_w * aspect).max(DAG_PREVIEW_MIN_SIZE);
    (DAG_COMPONENT_WIDTH, inner_h + DAG_PREVIEW_PAD * 2.0)
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
            let row_rect = if row.has_children { (x0, row_y0, x0 + (x1 - x0).max(1.0), row_y1) } else { (0.0, 0.0, 0.0, 0.0) };
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
    Export {
        label: String,
        format: String,
        input: IoPortSpec,
    },
    Cluster {
        inputs: Vec<IoPortSpec>,
        outputs: Vec<IoPortSpec>,
    },
    AppInstance {
        #[serde(rename = "instanceId")]
        instance_id: String,
        #[serde(rename = "programId")]
        program_id: String,
        #[serde(rename = "appId")]
        app_id: String,
        #[serde(default)]
        icon: String,
        inputs: Vec<IoPortSpec>,
        outputs: Vec<IoPortSpec>,
    },
}

/// 🏷️ Serialized `kind` tag for a {@link DagNodeKind} variant.
pub fn dag_node_kind_tag(kind: &DagNodeKind) -> &'static str {
    match kind {
        DagNodeKind::Computation { .. } => "computation",
        DagNodeKind::Slider { .. } => "slider",
        DagNodeKind::Select { .. } => "select",
        DagNodeKind::Screen { .. } => "screen",
        DagNodeKind::Note { .. } => "note",
        DagNodeKind::Image { .. } => "image",
        DagNodeKind::Preview { .. } => "preview",
        DagNodeKind::Action { .. } => "action",
        DagNodeKind::Export { .. } => "export",
        DagNodeKind::Cluster { .. } => "cluster",
        DagNodeKind::AppInstance { .. } => "appInstance",
    }
}

fn validate_dag_fixture_node_kinds(nodes: &[DagNodeSpec]) -> Result<(), DagError> {
    let manifest = flow_dag_manifest();
    let validator = ManifestValidator::new(&manifest);
    for node in nodes {
        validator.validate_node_kind(dag_node_kind_tag(&node.kind)).map_err(|error| DagError::InvalidNodeKind(format!("{}: {}", error.path, error.message)))?;
    }
    Ok(())
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
    cluster_explode_hit_rect(node).is_some_and(|(x0, y0, x1, y1)| point_in_rect(world_x, world_y, x0, y0, x1, y1))
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_kind: Option<String>,
    #[serde(default)]
    pub properties: PropertyBag,
    #[serde(flatten)]
    pub kind: DagNodeKind,
}

impl Default for DagNodeSpec {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            abbreviation: String::new(),
            icon: String::new(),
            x: 0.0,
            y: 0.0,
            width: default_node_width(),
            height: default_node_height(),
            operator_kind: None,
            properties: PropertyBag::new(),
            kind: DagNodeKind::Computation { inputs: vec![], outputs: vec![], variadic_inputs: false, variadic_outputs: false },
        }
    }
}

impl DagNodeSpec {
    /// 🔧 Builds a computation node with explicit IO ports.
    #[allow(clippy::too_many_arguments, reason = "positional constructor called by external crates (framework/surface/node-graph/rs, sequence/core/rs, flow/core/rs); bundling into a params struct is a breaking API change out of this crate's scope")]
    pub fn computation(id: String, name: String, abbreviation: String, icon: String, inputs: Vec<IoPortSpec>, outputs: Vec<IoPortSpec>, variadic_inputs: bool, variadic_outputs: bool, x: f64, y: f64, width: f64, height: f64) -> Self {
        let (name, abbreviation) = normalize_node_display(&name, &abbreviation);
        Self { id, name, abbreviation, icon, x, y, width, height, operator_kind: None, properties: PropertyBag::new(), kind: DagNodeKind::Computation { inputs, outputs, variadic_inputs, variadic_outputs } }
    }

    /// 🧩 Builds a cluster node with contract IO ports.
    #[allow(clippy::too_many_arguments, reason = "positional constructor called by external crates (framework/surface/node-graph/rs, sequence/core/rs, flow/core/rs); bundling into a params struct is a breaking API change out of this crate's scope")]
    pub fn cluster(id: String, name: String, abbreviation: String, icon: String, inputs: Vec<IoPortSpec>, outputs: Vec<IoPortSpec>, x: f64, y: f64, width: f64, height: f64) -> Self {
        let (name, abbreviation) = normalize_node_display(&name, &abbreviation);
        Self { id, name, abbreviation, icon, x, y, width, height, operator_kind: None, properties: PropertyBag::new(), kind: DagNodeKind::Cluster { inputs, outputs } }
    }

    /// ➕ Whether the node exposes variadic input insert controls.
    pub fn variadic_inputs(&self) -> bool {
        match &self.kind {
            DagNodeKind::Computation { variadic_inputs, .. } => *variadic_inputs,
            _ => false,
        }
    }

    /// ➕ Whether the node exposes variadic output insert controls.
    pub fn variadic_outputs(&self) -> bool {
        match &self.kind {
            DagNodeKind::Computation { variadic_outputs, .. } => *variadic_outputs,
            _ => false,
        }
    }

    /// ⬅ Effective input ports for the node kind.
    pub fn inputs(&self) -> &[IoPortSpec] {
        match &self.kind {
            DagNodeKind::Computation { inputs, .. } | DagNodeKind::Cluster { inputs, .. } | DagNodeKind::AppInstance { inputs, .. } => inputs,
            DagNodeKind::Screen { input, .. } | DagNodeKind::Preview { input, .. } | DagNodeKind::Action { input, .. } | DagNodeKind::Export { input, .. } => std::slice::from_ref(input),
            _ => EMPTY_PORTS,
        }
    }

    /// ➡ Effective output ports for the node kind.
    pub fn outputs(&self) -> &[IoPortSpec] {
        match &self.kind {
            DagNodeKind::Computation { outputs, .. } | DagNodeKind::Cluster { outputs, .. } | DagNodeKind::AppInstance { outputs, .. } => outputs,
            DagNodeKind::Slider { output, .. } | DagNodeKind::Select { output, .. } | DagNodeKind::Note { output, .. } | DagNodeKind::Image { output, .. } => std::slice::from_ref(output),
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
    (node.x - hw + pad, track_y - hit_h * 0.5, node.x + hw - pad, track_y + hit_h * 0.5)
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
pub fn note_widget_size(_text: &str) -> (f64, f64) {
    (DAG_COMPONENT_WIDTH, DAG_CHANNEL_ROW_HEIGHT)
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
    Some(if right_start <= left_end { node.x } else { (left_end + right_start) * 0.5 })
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
        DagNodeKind::Computation { inputs, outputs, variadic_inputs, variadic_outputs } => {
            let input_rows = inputs.len() + usize::from(*variadic_inputs);
            let output_rows = outputs.len() + usize::from(*variadic_outputs);
            (input_rows, output_rows)
        }
        DagNodeKind::Cluster { inputs, outputs } => (inputs.len(), outputs.len()),
        DagNodeKind::AppInstance { inputs, outputs, .. } => (inputs.len(), outputs.len()),
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
    let (_name_w, name_h) = label_extent(&node.name, name_px);
    let visual_w = name_h;
    let top = node.y - hh + DAG_NODE_EDGE_INSET;
    let bottom = node.y + hh * 0.12;
    let x0 = node.x - visual_w * 0.5;
    let x1 = node.x + visual_w * 0.5;
    (x0, top, x1, bottom)
}

fn computation_channel_row_count(node: &DagNodeSpec) -> usize {
    match &node.kind {
        DagNodeKind::Computation { inputs, outputs, variadic_inputs, variadic_outputs } => computation_io_row_count(inputs.len(), outputs.len(), *variadic_inputs, *variadic_outputs) + DAG_COMPUTATION_HEADER_ROWS,
        DagNodeKind::Cluster { inputs, outputs } => computation_io_row_count(inputs.len(), outputs.len(), false, false) + DAG_COMPUTATION_HEADER_ROWS,
        _ => 0,
    }
}

pub fn fit_node_size(node: &mut DagNodeSpec) {
    match &node.kind {
        DagNodeKind::Computation { inputs, outputs, variadic_inputs, variadic_outputs } => {
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
        DagNodeKind::Select { .. } | DagNodeKind::Action { .. } | DagNodeKind::Export { .. } => {
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
        DagNodeKind::AppInstance { inputs, outputs, .. } => {
            let port_count = inputs.len().max(outputs.len()).max(1);
            node.width = 180.0;
            node.height = 56.0 + port_count as f64 * 18.0;
        }
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

fn variadic_output_insert_positions(node: &DagNodeSpec) -> Vec<(usize, f64, f64)> {
    if !node.variadic_outputs() {
        return vec![];
    }
    let outputs = node.outputs();
    let row = outputs.len();
    let port_y = computation_port_center_y(node, row);
    let port_x = computation_output_label_x(node, "+", DAG_LABEL_COMPACT_SCREEN_PX);
    vec![(outputs.len(), port_x, port_y)]
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
    use cavas::Point;
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
    use cavas::Point;
    use graph::rectangle_handle_angle_toward;
    let hw = node.width * 0.5;
    let count = if left { node.inputs().len() } else { node.outputs().len() };
    let port_y = port_center_y(node, port_index, count);
    let port_x = if left { node.x - hw } else { node.x + hw };
    rectangle_handle_angle_toward(Point::new(node.x, node.y), node.width, node.height, Point::new(port_x, port_y))
}
// #endregion 🔖IoNode

// #region 🔖Acyclicity
/// 🚫 Returns true when adding `source -> target` would create a cycle.
pub fn would_create_cycle(existing: &[(String, String)], source: &str, target: &str) -> bool {
    mathematical_graph::algorithms::would_create_cycle_ids(existing, source, target)
}
// #endregion 🔖Acyclicity

// #region 🔖Layout
use mathematical_graph_drawing::tidy_tree::buchheim_positions;
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
    if let Some(base) = key.split('@').next() {
        if node_ids.contains(base) {
            return base.to_string();
        }
    }
    key.to_string()
}

impl Default for DagLayoutOptions {
    fn default() -> Self {
        Self { layer_spacing: default_layer_spacing(), sibling_gap: default_sibling_gap(), orientation: DagLayoutOrientation::default(), center_x: None, center_y: None }
    }
}

/// 🌳 Writes node centers from a layered DAG layout into `dag.fixture`.
pub fn apply_dag_layout_to_fixture_v1_value(fixture: &mut Value, opts: &DagLayoutOptions) -> Result<(), DagError> {
    let Some(root) = fixture.as_object_mut() else {
        return Err(DagError::FixtureRootNotObject);
    };
    if root.get("schema").and_then(|v| v.as_str()) != Some("dag.fixture") {
        return Err(DagError::SchemaMismatch);
    }
    let edges_json = root.get("edges").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    let Some(nodes) = root.get_mut("nodes").and_then(|v| v.as_array_mut()) else {
        return Err(DagError::NodesMissing);
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
    for (x, y) in pos.values() {
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
    Lod { id: "minimap", name: "Minimap", description: "Whole-graph silhouette; fill only.", max_zoom: 0.15 },
    Lod { id: "overview", name: "Overview", description: "Node icons only.", max_zoom: 0.35 },
    Lod { id: "compact", name: "Compact", description: "Horizontal abbreviations.", max_zoom: 0.55 },
    Lod { id: "normal", name: "Normal", description: "Vertical names with sections; channel abbreviations on ports.", max_zoom: 1.25 },
    Lod { id: "detail", name: "Detail", description: "Channel names on ports, port handles, and control text.", max_zoom: 2.5 },
    Lod { id: "micro", name: "Micro", description: "Full channel names on ports and maximum node fidelity.", max_zoom: f64::INFINITY },
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

fn dag_node_body_fill(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> cavas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.node_fill_disabled, ui_styling::opacities::DISABLED_FILL_ALPHA)
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

pub(crate) fn dag_node_body_stroke(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> cavas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.node_stroke, ui_styling::opacities::DIM_STROKE_ALPHA)
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

pub(crate) fn dag_node_label_fill(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> cavas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.label_fill, ui_styling::opacities::DIM_LABEL_ALPHA)
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
pub(crate) fn dag_node_internal_chrome_stroke(body_stroke: cavas::Color, label_fill: cavas::Color, emphasized: bool) -> cavas::Color {
    if emphasized {
        label_fill
    } else {
        body_stroke
    }
}

pub(crate) fn dag_handle_body_fill(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> cavas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.handle_fill_disabled, ui_styling::opacities::DISABLED_FILL_ALPHA)
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

pub(crate) fn dag_handle_body_stroke(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> cavas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.handle_stroke_disabled, ui_styling::opacities::DISABLED_STROKE_ALPHA)
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

pub(crate) fn dag_edge_body_stroke(theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> cavas::Color {
    if dimmed {
        canvas_color_with_alpha(theme.edge_stroke_disabled, ui_styling::opacities::DISABLED_STROKE_ALPHA)
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
pub(crate) fn dag_node_paint_fill(lod: DagDrawLod, theme: &CanvasPalette, dimmed: bool, selected: bool, highlighted: bool, hovered: bool) -> Option<cavas::Color> {
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
        matches!(self, Self::Normal | Self::Detail | Self::Micro)
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

impl IoPortSpec {
    /// 🏷️ Channel label for the active draw LOD (normal → abbreviation, detail → name, micro → fullName).
    pub fn display_label(&self, lod: DagDrawLod) -> &str {
        match lod {
            DagDrawLod::Micro => {
                if !self.full_name.is_empty() {
                    return self.full_name.as_str();
                }
                self.id.as_str()
            }
            DagDrawLod::Detail => self.id.as_str(),
            DagDrawLod::Normal => {
                if !self.abbreviation.is_empty() {
                    return self.abbreviation.as_str();
                }
                if !self.label.is_empty() {
                    return self.label.as_str();
                }
                self.id.as_str()
            }
            _ => self.display_code(),
        }
    }

    pub fn display_label_layout_width(&self, px: f64) -> f64 {
        [self.label_with_cardinality(DagDrawLod::Normal), self.label_with_cardinality(DagDrawLod::Detail), self.label_with_cardinality(DagDrawLod::Micro)].into_iter().map(|label| port_label_text_width(&label, px)).fold(0.0, f64::max)
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
            let max_zoom = if lod.max_zoom.is_finite() { lod.max_zoom + DAG_LOD_ZOOM_SHIFT } else { lod.max_zoom };
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

//#region 🔖Grid
const GRID_WORLD_LARGE: f64 = ui_styling::metrics::board::GRID_WORLD_LARGE;
const GRID_WORLD_MEDIUM: f64 = ui_styling::metrics::board::GRID_WORLD_MEDIUM;
const GRID_WORLD_SMALL: f64 = ui_styling::metrics::board::GRID_WORLD_SMALL;
const GRID_WORLD_MICRO: f64 = ui_styling::metrics::board::GRID_WORLD_MICRO;
const GRID_FACTOR_DEFAULT: f64 = ui_styling::metrics::board::GRID_FACTOR_DEFAULT;
//#endregion 🔖Grid

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

// #region 🔖NoteEdit

#[derive(Clone, Debug)]
struct NoteEditState {
    node_id: String,
    caret: usize,
    anchor: usize,
}

// #endregion 🔖NoteEdit

// #region 🔖DagHost

/// 🌳 Retained DAG host: typed nodes, edges, engine, camera.
pub struct DagHost {
    pub fixture: DagFixture,
    pub engine: DagBoardEngine,
    pub canvas_theme: CanvasPalette,
    width: u32,
    height: u32,
    dpr: f64,
    last_screen_x: f64,
    last_screen_y: f64,
    node_id_map: HashMap<NodeId, usize>,
    handle_key_map: HashMap<HandleId, String>,
    handle_port_shape: HashMap<HandleId, PortShape>,
    handle_port_visible: HashMap<HandleId, bool>,
    edge_id_map: HashMap<EdgeId, String>,
    edge_route_style: HashMap<EdgeId, EdgeRouteStyle>,
    widget_drag: Option<usize>,
    pending_port_insert: Option<(DagPortSide, String, usize)>,
    last_logged_lod: Cell<i8>,
    dimmed: HashSet<NodeId>,
    wheel_zoom_active: bool,
    wheel_zoom_render_lod: Option<DagDrawLod>,
    automatic_lod: bool,
    forced_draw_lod: Option<DagDrawLod>,
    grid_visible: bool,
    grid_snap_enabled: bool,
    grid_factor: f64,
    icon_paint_cache: graph::IconPaintCache,
    ghost_node: Option<DagNodeSpec>,
    pending_cluster_explode: Option<String>,
    pending_export_click: Option<String>,
    pending_open_instance_id: Option<String>,
    last_pointer_down_at_ms: f64,
    last_pointer_down_world: (f64, f64),
    last_pointer_down_node_id: Option<String>,
    computing_active: Option<NodeId>,
    computing_stale: HashSet<NodeId>,
    computing_active_anim_phase: Cell<f64>,
    computing_stale_anim_phase: Cell<f64>,
    editing_note: Option<NoteEditState>,
    caret_visible: bool,
    pan_anchor: Option<(f64, f64, f64, f64)>,
}

fn pointer_event_now_ms() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0.0
    }
}

/// 🧭 Screen coordinates to world coordinates for the current viewport.
pub fn dag_screen_to_world(host: &DagHost, sx: f64, sy: f64) -> (f64, f64) {
    let point = host.screen_to_world_point(sx, sy);
    (point.x, point.y)
}

/// 🪟 Takes a pending double-click open request for an app instance node.
pub fn dag_take_pending_open_instance_id(host: &mut DagHost) -> Option<String> {
    host.pending_open_instance_id.take()
}

fn canvas_color_with_alpha(color: cavas::Color, alpha: u8) -> cavas::Color {
    use cavas::Color;
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
        Self { is_dimmed: false, is_selected: false, is_highlighted: false, is_hovered: false, is_computing: false, is_stale: false, body_fill_alpha: 255, ghost_tint: true }
    }

    fn tint_highlighted(self) -> bool {
        self.is_highlighted || self.ghost_tint
    }

    fn has_interaction_chrome(self) -> bool {
        self.is_dimmed || self.is_selected || self.is_highlighted || self.is_hovered
    }
}

/// 📦 `dag.fixture` document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagFixture {
    pub schema: String,
    pub camera: DagCamera,
    pub nodes: Vec<DagNodeSpec>,
    pub edges: Vec<DagFixtureEdge>,
}

/// 📷 Fixture camera snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

/// 🔗 Edge between port handles.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DagFixtureEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub route_style: EdgeRouteStyle,
    #[serde(default)]
    pub properties: PropertyBag,
}

impl Default for DagFixture {
    fn default() -> Self {
        // 📜 the demo board is handcrafted `.dag` DSL text (see `//#region 🔖Dsl`), not JSON — it is
        // compiled into the binary, so a parse failure here is a bug in the bundled fixture itself.
        let document = <DagDocument as vcs::DocumentDsl>::parse_dsl(include_str!("../example/demo.dag")).expect("bundled example/demo.dag is valid DagDocument DSL text");
        Self { schema: document.schema, camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: document.nodes, edges: document.edges }
    }
}

fn split_dag_endpoint(endpoint: &str) -> (String, String) {
    if let Some((node, port)) = endpoint.rsplit_once('@') {
        return (node.to_string(), port.to_string());
    }
    (endpoint.to_string(), "out".into())
}

fn dag_visual_kind(node: &DagNodeSpec) -> String {
    node.operator_kind.clone().unwrap_or_else(|| dag_node_kind_tag(&node.kind).to_string())
}

/// 📝 Render a DAG fixture as wire-literal compiled text.
pub fn dag_fixture_to_wire_literal(fixture: &DagFixture) -> String {
    use mathematical_graph_dsl::{wire_literal_from_dag, WireEdge, WireNode};
    let nodes = fixture.nodes.iter().map(|node| WireNode { id: node.id.clone(), kind: dag_visual_kind(node), port: None, properties: node.properties.clone() }).collect::<Vec<_>>();
    let edges = fixture
        .edges
        .iter()
        .map(|edge| {
            let (from, from_port) = split_dag_endpoint(&edge.source);
            let (to, to_port) = split_dag_endpoint(&edge.target);
            WireEdge { from, from_port, to, to_port, directed: true, properties: edge.properties.clone() }
        })
        .collect::<Vec<_>>();
    wire_literal_from_dag(&nodes, &edges)
}

/// 🧵 Build execution wire rows from an enriched DAG fixture.
pub fn dag_fixture_execution_rows(fixture: &DagFixture) -> (Vec<mathematical_graph_dsl::WireNode>, Vec<mathematical_graph_dsl::WireEdge>) {
    use mathematical_graph_dsl::{WireEdge, WireNode};
    use std::collections::HashSet;
    let executable: HashSet<String> = fixture.nodes.iter().filter_map(|node| node.operator_kind.as_ref().map(|_| node.id.clone())).collect();
    let nodes = fixture
        .nodes
        .iter()
        .filter_map(|node| {
            let kind = node.operator_kind.clone()?;
            Some(WireNode { id: node.id.clone(), kind, port: None, properties: node.properties.clone() })
        })
        .collect();
    let edges = fixture
        .edges
        .iter()
        .filter_map(|edge| {
            let (from, from_port) = split_dag_endpoint(&edge.source);
            let (to, to_port) = split_dag_endpoint(&edge.target);
            if !executable.contains(&from) || !executable.contains(&to) {
                return None;
            }
            Some(WireEdge { from, from_port, to, to_port, directed: true, properties: edge.properties.clone() })
        })
        .collect();
    (nodes, edges)
}

impl DagHost {
    pub fn default_demo() -> Self {
        Self::from_fixture(DagFixture::default())
    }

    pub fn from_fixture(fixture: DagFixture) -> Self {
        Self::from_fixture_with_layout(fixture, false)
    }

    /// 🌳 Builds a host without running auto-layout (preserves node positions).
    pub fn from_fixture_without_layout(fixture: DagFixture) -> Self {
        Self::from_fixture(fixture)
    }

    fn from_fixture_with_layout(fixture: DagFixture, apply_layout: bool) -> Self {
        let mut host = Self {
            fixture,
            engine: DagBoardEngine::new(),
            canvas_theme: CanvasPalette::default(),
            width: 1,
            height: 1,
            dpr: 1.0,
            last_screen_x: 0.0,
            last_screen_y: 0.0,
            node_id_map: HashMap::new(),
            handle_key_map: HashMap::new(),
            handle_port_shape: HashMap::new(),
            handle_port_visible: HashMap::new(),
            edge_id_map: HashMap::new(),
            edge_route_style: HashMap::new(),
            widget_drag: None,
            pending_port_insert: None,
            last_logged_lod: Cell::new(-1),
            dimmed: HashSet::new(),
            wheel_zoom_active: false,
            wheel_zoom_render_lod: None,
            automatic_lod: true,
            forced_draw_lod: None,
            grid_visible: true,
            grid_snap_enabled: false,
            grid_factor: GRID_FACTOR_DEFAULT,
            icon_paint_cache: graph::IconPaintCache::new(),
            ghost_node: None,
            pending_cluster_explode: None,
            pending_export_click: None,
            pending_open_instance_id: None,
            last_pointer_down_at_ms: 0.0,
            last_pointer_down_world: (0.0, 0.0),
            last_pointer_down_node_id: None,
            computing_active: None,
            computing_stale: HashSet::new(),
            computing_active_anim_phase: Cell::new(0.0),
            computing_stale_anim_phase: Cell::new(0.0),
            editing_note: None,
            caret_visible: true,
            pan_anchor: None,
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
        matches!(self.engine.interaction, InteractionMode::SelectionPending { .. } | InteractionMode::AreaSelect { .. })
            || !self.engine.preselect.node_ids.is_empty()
            || !self.engine.preselect.handle_ids.is_empty()
            || !self.engine.preselect.edge_ids.is_empty()
    }

    fn node_interaction_chrome(&self, node_id: NodeId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (self.is_node_preselected(node_id), self.is_node_preselect_removed(node_id), false);
        }
        (self.is_node_selected(node_id), false, self.is_node_hovered(node_id))
    }

    fn handle_interaction_chrome(&self, handle_id: HandleId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (self.engine.preselect.handle_ids.contains(&handle_id), self.engine.preselect_removed.handle_ids.contains(&handle_id), false);
        }
        (self.engine.selection.handle_ids.contains(&handle_id), false, self.engine.hover == Some(handle_id))
    }

    fn edge_interaction_chrome(&self, edge_id: EdgeId) -> (bool, bool, bool) {
        if self.is_preselect_active() {
            return (self.engine.preselect.edge_ids.contains(&edge_id), self.engine.preselect_removed.edge_ids.contains(&edge_id), false);
        }
        (self.engine.selection.edge_ids.contains(&edge_id), false, self.engine.hover == Some(edge_id))
    }

    fn sync_camera_from_engine(&mut self) {
        let cam = self.engine.camera;
        self.fixture.camera = DagCamera { x: cam.x, y: cam.y, zoom: cam.zoom };
    }

    /// 🎯 Selected fixture node ids from the engine selection snapshot.
    pub fn selected_node_ids(&self) -> Vec<String> {
        self.engine.selection.node_ids.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect()
    }

    /// ✅ Whether the engine has any committed node, edge, or handle selection.
    pub fn has_selection(&self) -> bool {
        !self.engine.selection.node_ids.is_empty() || !self.engine.selection.edge_ids.is_empty() || !self.engine.selection.handle_ids.is_empty()
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
        let (node_id, port_id) = key.split_once('@')?;
        let node = self.fixture.nodes.iter().find(|entry| entry.id == node_id)?;
        let direction = if node.inputs().iter().any(|port| port.id == port_id) {
            "in"
        } else if node.outputs().iter().any(|port| port.id == port_id) {
            "out"
        } else {
            return None;
        };
        Some(DagChannelRef { widget_id: node_id.to_string(), port: port_id.to_string(), direction: direction.to_string() })
    }

    /// 🔌 Hovered fixture channel when the pointer is over a port row or handle.
    pub fn hovered_channel(&self) -> Option<DagChannelRef> {
        let hover = self.engine.hover?;
        self.decode_channel_ref(hover)
    }

    /// 🔌 Selected fixture channels from handle picks in the current selection snapshot.
    pub fn selected_channels(&self) -> Vec<DagChannelRef> {
        self.engine.selection.handle_ids.iter().filter_map(|&handle_id| self.decode_channel_ref(handle_id)).collect()
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

    /// @emoji 🎯 All pick targets under a screen point as JSON (`domain`, `id`, `generality`).
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        #[derive(serde::Serialize)]
        struct Row {
            domain: String,
            id: String,
            generality: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            label: Option<String>,
        }
        let world = self.screen_to_world_point(sx, sy);
        let targets = self.engine.hit_test_pick_targets(world);
        let rows: Vec<Row> = targets
            .into_iter()
            .filter_map(|target| {
                let row = match target.domain.as_str() {
                    "node" => self.widget_id_for_node_id(target.id).map(|id| Row { domain: target.domain, id, generality: target.generality, label: None }),
                    "edge" => self.edge_id_map.get(&target.id).cloned().map(|id| Row { domain: target.domain, id, generality: target.generality, label: None }),
                    "handle" => self.decode_channel_ref(target.id).map(|channel| Row {
                        domain: "handle".into(),
                        id: format!("{}@{}", channel.widget_id, channel.port),
                        generality: target.generality,
                        label: Some(format!("{} · {}", channel.widget_id, channel.port)),
                    }),
                    _ => None,
                };
                row
            })
            .collect();
        serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
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
        let points: Vec<[f64; 2]> = self.engine.selection_preview_points().iter().map(|p| [p.x, p.y]).collect();
        serde_json::to_string(&points).unwrap_or_else(|_| "[]".into())
    }

    pub fn selection_preview_crossing(&self) -> bool {
        self.engine.selection_preview_crossing()
    }

    pub fn selection_preview_method(&self) -> &str {
        self.engine.selection_preview_method()
    }

    /// 👁️ Preselected widget ids during an in-flight marquee gesture.
    pub fn preselect_widget_ids(&self) -> Vec<String> {
        self.engine.preselect.node_ids.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect()
    }

    /// 👁️ Widget ids highlighted as marquee-exit candidates.
    pub fn preselect_removed_widget_ids(&self) -> Vec<String> {
        self.engine.preselect_removed.node_ids.iter().filter_map(|&nid| self.widget_id_for_node_id(nid)).collect()
    }

    /// ↩️ Cancels an in-flight area select and restores the pre-drag selection.
    pub fn cancel_area_select(&mut self) -> bool {
        self.engine.cancel_area_select()
    }

    // #region 🔖SelectionAlign
    fn dag_node_world_bounds(node: &DagNodeSpec) -> WorldBox {
        let hw = node.width * 0.5;
        let hh = node.height * 0.5;
        WorldBox { min_x: node.x - hw, min_y: node.y - hh, max_x: node.x + hw, max_y: node.y + hh }
    }

    fn selected_fixture_nodes(&self) -> Vec<(usize, DagNodeSpec)> {
        let ids = self.selected_node_ids();
        ids.into_iter().filter_map(|id| self.fixture.nodes.iter().enumerate().find(|(_, node)| node.id == id).map(|(idx, node)| (idx, node.clone()))).collect()
    }

    fn sync_fixture_node_center_to_engine(&mut self, idx: usize) {
        let node = &self.fixture.nodes[idx];
        let Some(nid) = self.node_id_for_widget_id(&node.id) else {
            return;
        };
        if let Some(engine_node) = self.engine.nodes.get_mut(&nid) {
            engine_node.center = cavas::Point::new(node.x, node.y);
        }
    }

    /// 📦 Screen-space union bounds of the current node selection for DOM chrome overlays.
    pub fn selection_union_bounds_screen_json(&self) -> String {
        let selected = self.selected_fixture_nodes();
        if selected.is_empty() {
            return "null".into();
        }
        use cavas::camera::{world_to_screen, Camera as CavasCamera, Viewport};
        use cavas::Point;
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
        let cam = CavasCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: self.width.max(1), height: self.height.max(1), dpr: self.dpr.max(1.0) };
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

    /// @emoji 🎯 Screen-space geometry (canvas-local px) for a live entity in the shell's pick-target
    /// grammar (`domain`: `"node"` | `"handle"` | `"edge"`; `id`: a node's widget id, `"widgetId@port"`
    /// for a handle, or a mirrored edge id — `"*"` picks whichever matching entity's screen anchor is
    /// nearest the viewport center) — powers introduction-demonstration semantic targeting
    /// (`IntroductionPoint::Entity`/`Curve`). Never errors: an unresolved domain/id returns
    /// `{"visible":false}`.
    pub fn entity_screen_json(&self, domain: &str, id: &str) -> String {
        #[derive(serde::Serialize)]
        struct EntityGeometry {
            visible: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            x: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            y: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            rect: Option<[f64; 4]>,
            #[serde(skip_serializing_if = "Option::is_none")]
            polyline: Option<Vec<[f64; 2]>>,
        }
        use cavas::camera::{world_to_screen, Camera as CavasCamera, Viewport};
        use cavas::Point;
        let unresolved = EntityGeometry { visible: false, x: None, y: None, rect: None, polyline: None };
        let cam = CavasCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: self.width.max(1), height: self.height.max(1), dpr: self.dpr.max(1.0) };
        let viewport_center = (self.width as f64 * 0.5, self.height as f64 * 0.5);

        let world_rect_to_screen = |min_x: f64, min_y: f64, max_x: f64, max_y: f64| -> ([f64; 4], (f64, f64)) {
            let tl = world_to_screen(&cam, &viewport, Point::new(min_x, min_y));
            let br = world_to_screen(&cam, &viewport, Point::new(max_x, max_y));
            ([tl.x, tl.y, (br.x - tl.x).max(1.0), (br.y - tl.y).max(1.0)], ((tl.x + br.x) * 0.5, (tl.y + br.y) * 0.5))
        };
        let handle_world_bounds = |widget_id: &str, port: &str| -> Option<(f64, f64, f64, f64)> {
            let node = self.fixture.nodes.iter().find(|node| node.id == widget_id)?;
            if let Some(index) = node.inputs().iter().position(|candidate| candidate.id == port) {
                return input_port_row_hit_bounds(node, index);
            }
            let index = node.outputs().iter().position(|candidate| candidate.id == port)?;
            output_port_row_hit_bounds(node, index)
        };
        let nearest_center_screen = |candidates: &[(f64, f64, f64, f64)]| -> Option<(f64, f64, f64, f64)> {
            let mut best: Option<((f64, f64, f64, f64), f64)> = None;
            for &bounds in candidates {
                let (_, screen_center) = world_rect_to_screen(bounds.0, bounds.1, bounds.2, bounds.3);
                let distance = (screen_center.0 - viewport_center.0).hypot(screen_center.1 - viewport_center.1);
                if best.map(|(_, best_distance)| distance < best_distance).unwrap_or(true) {
                    best = Some((bounds, distance));
                }
            }
            best.map(|(bounds, _)| bounds)
        };

        let bounds_result: Option<(f64, f64, f64, f64)> = match domain {
            "node" => {
                if id == "*" {
                    let all: Vec<(f64, f64, f64, f64)> = self.fixture.nodes.iter().map(|node| { let b = Self::dag_node_world_bounds(node); (b.min_x, b.min_y, b.max_x, b.max_y) }).collect();
                    nearest_center_screen(&all)
                } else {
                    self.fixture.nodes.iter().find(|node| node.id == id).map(|node| { let b = Self::dag_node_world_bounds(node); (b.min_x, b.min_y, b.max_x, b.max_y) })
                }
            }
            "handle" => {
                if id == "*" {
                    let mut all: Vec<(f64, f64, f64, f64)> = Vec::new();
                    for node in &self.fixture.nodes {
                        for port in node.inputs().iter().chain(node.outputs().iter()) {
                            if let Some(bounds) = handle_world_bounds(&node.id, &port.id) {
                                all.push(bounds);
                            }
                        }
                    }
                    nearest_center_screen(&all)
                } else {
                    id.split_once('@').and_then(|(widget_id, port)| handle_world_bounds(widget_id, port))
                }
            }
            "edge" => {
                let edge = if id == "*" { self.fixture.edges.first() } else { self.fixture.edges.iter().find(|edge| edge.id == id) };
                let Some(edge) = edge else { return serde_json::to_string(&unresolved).unwrap_or_default() };
                let Some((source_widget, source_port)) = edge.source.split_once('@') else { return serde_json::to_string(&unresolved).unwrap_or_default() };
                let Some((target_widget, target_port)) = edge.target.split_once('@') else { return serde_json::to_string(&unresolved).unwrap_or_default() };
                let Some(source_bounds) = handle_world_bounds(source_widget, source_port) else { return serde_json::to_string(&unresolved).unwrap_or_default() };
                let Some(target_bounds) = handle_world_bounds(target_widget, target_port) else { return serde_json::to_string(&unresolved).unwrap_or_default() };
                let (_, source_center) = world_rect_to_screen(source_bounds.0, source_bounds.1, source_bounds.2, source_bounds.3);
                let (_, target_center) = world_rect_to_screen(target_bounds.0, target_bounds.1, target_bounds.2, target_bounds.3);
                let midpoint = ((source_center.0 + target_center.0) * 0.5, (source_center.1 + target_center.1) * 0.5);
                return serde_json::to_string(&EntityGeometry {
                    visible: true,
                    x: Some(midpoint.0),
                    y: Some(midpoint.1),
                    rect: None,
                    polyline: Some(vec![[source_center.0, source_center.1], [target_center.0, target_center.1]]),
                })
                .unwrap_or_default();
            }
            _ => None,
        };

        let Some(bounds) = bounds_result else { return serde_json::to_string(&unresolved).unwrap_or_default() };
        let (rect, center) = world_rect_to_screen(bounds.0, bounds.1, bounds.2, bounds.3);
        serde_json::to_string(&EntityGeometry { visible: true, x: Some(center.0), y: Some(center.1), rect: Some(rect), polyline: None }).unwrap_or_default()
    }

    /// 📐 Aligns or distributes the current multi-node selection.
    pub fn align_selection(&mut self, mode: &str) -> Result<(), DagError> {
        use cavas::Point;
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
                let raw_gap = (right - left - total_width) / (selected.len() as f64 - 1.0);
                let gap = if raw_gap.is_finite() { raw_gap.max(DAG_DISTRIBUTE_MIN_GAP) } else { DAG_DISTRIBUTE_MIN_GAP };
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
                let raw_gap = (bottom - top - total_height) / (selected.len() as f64 - 1.0);
                let gap = if raw_gap.is_finite() { raw_gap.max(DAG_DISTRIBUTE_MIN_GAP) } else { DAG_DISTRIBUTE_MIN_GAP };
                let mut cursor = top;
                for (_, node) in &mut selected {
                    node.y = cursor + node.height * 0.5;
                    cursor += node.height + gap;
                }
            }
            other => return Err(DagError::UnknownAlignMode(other.to_string())),
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
    pub fn set_widget_position(&mut self, widget_id: &str, x: f64, y: f64) -> Result<(), DagError> {
        let idx = self.fixture.nodes.iter().position(|node| node.id == widget_id).ok_or_else(|| DagError::UnknownWidget(widget_id.to_string()))?;
        self.fixture.nodes[idx].x = x;
        self.fixture.nodes[idx].y = y;
        let Some(nid) = self.engine_node_id_for_index(idx) else {
            return Ok(());
        };
        if let Some(node) = self.engine.nodes.get_mut(&nid) {
            node.center = cavas::Point::new(x, y);
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
        let Some(widget_id) = widget_id else {
            self.set_hover(None);
            return;
        };
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

    /// ➕ Returns and clears a pending variadic insert request from the last pointer down.
    pub fn take_pending_port_insert(&mut self) -> Option<(DagPortSide, String, usize)> {
        self.pending_port_insert.take()
    }

    /// 🎯 Hit-tests variadic `+` controls; returns side, node id, and insert index.
    pub fn port_insert_hit(&self, world_x: f64, world_y: f64, zoom: f64) -> Option<(DagPortSide, String, usize)> {
        if zoom < DAG_VARIADIC_PLUS_ZOOM_THRESHOLD {
            return None;
        }
        for node in self.fixture.nodes.iter().rev() {
            if node.variadic_inputs() {
                let inputs = node.inputs();
                let hw = node.width * 0.5;
                let row = inputs.len() + DAG_COMPUTATION_HEADER_ROWS;
                let (x0, y0, hit_x1, y1) = {
                    let (x0, y0, _x1, y1) = channel_row_bounds(node, row);
                    (x0, y0, node.x - hw * 0.5, y1)
                };
                if point_in_rect(world_x, world_y, x0, y0, hit_x1, y1) {
                    return Some((DagPortSide::Input, node.id.clone(), inputs.len()));
                }
            }
            if node.variadic_outputs() {
                let outputs = node.outputs();
                let row = outputs.len() + DAG_COMPUTATION_HEADER_ROWS;
                let (x0, y0, x1, y1) = channel_row_bounds(node, row);
                let hit_x0 = computation_output_column_x_bounds(node).map(|(left, _)| left).unwrap_or(x0);
                if point_in_rect(world_x, world_y, hit_x0, y0, x1, y1) {
                    return Some((DagPortSide::Output, node.id.clone(), outputs.len()));
                }
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

    /// 🔲 Toggles LOD-tiered world grid painting.
    pub fn set_grid_visible(&mut self, visible: bool) {
        self.grid_visible = visible;
    }

    /// 🧲 Toggles node-drag snap to the finest visible LOD grid step.
    pub fn set_grid_snap_enabled(&mut self, enabled: bool) {
        self.grid_snap_enabled = enabled;
    }

    /// 📐 Sets the positive multiplier for LOD world grid steps.
    pub fn set_grid_factor(&mut self, factor: f64) -> Result<(), DagError> {
        if !factor.is_finite() || factor <= 0.0 || factor > 1e6 {
            return Err(DagError::GridFactorOutOfRange);
        }
        self.grid_factor = factor;
        Ok(())
    }

    /// 🔍 Frames the current node selection in the viewport camera.
    pub fn focus_selection_camera(&self, pad: f64) -> Option<DagCamera> {
        let selected = self.selected_fixture_nodes();
        if selected.is_empty() {
            return None;
        }
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for (_, node) in &selected {
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            min_x = min_x.min(node.x - hw);
            min_y = min_y.min(node.y - hh);
            max_x = max_x.max(node.x + hw);
            max_y = max_y.max(node.y + hh);
        }
        if !min_x.is_finite() {
            return None;
        }
        let pad = pad.max(1.05);
        let cx = (min_x + max_x) * 0.5;
        let cy = (min_y + max_y) * 0.5;
        let span_w = (max_x - min_x).max(1.0);
        let span_h = (max_y - min_y).max(1.0);
        let vw = self.width.max(1) as f64;
        let vh = self.height.max(1) as f64;
        let zoom = (vw / (span_w * pad))
            .min(vh / (span_h * pad))
            .clamp(ui_styling::metrics::camera::ZOOM_MIN, ui_styling::metrics::camera::FLOW_ZOOM_MAX);
        Some(DagCamera { x: cx, y: cy, zoom })
    }

    fn grid_step_large_world(&self) -> f64 {
        GRID_WORLD_LARGE * self.grid_factor
    }

    fn grid_step_medium_world(&self) -> f64 {
        GRID_WORLD_MEDIUM * self.grid_factor
    }

    fn grid_step_small_world(&self) -> f64 {
        GRID_WORLD_SMALL * self.grid_factor
    }

    fn grid_step_micro_world(&self) -> f64 {
        GRID_WORLD_MICRO * self.grid_factor
    }

    fn lod_visible_grid_snap_step_world(&self) -> Option<f64> {
        match self.draw_lod_for_frame() {
            DagDrawLod::Minimap => None,
            DagDrawLod::Overview | DagDrawLod::Compact => Some(self.grid_step_large_world()),
            DagDrawLod::Normal => Some(self.grid_step_medium_world()),
            DagDrawLod::Detail => Some(self.grid_step_small_world()),
            DagDrawLod::Micro => Some(self.grid_step_micro_world()),
        }
    }

    fn snap_world_scalar(&self, value: f64) -> f64 {
        if !self.grid_snap_enabled {
            return value;
        }
        let Some(step) = self.lod_visible_grid_snap_step_world() else {
            return value;
        };
        (value / step).round() * step
    }

    fn snap_world_pair(&self, x: f64, y: f64) -> (f64, f64) {
        (self.snap_world_scalar(x), self.snap_world_scalar(y))
    }

    fn stroke_world_step_grid(
        &self,
        scene: &mut cavas::Scene,
        cam: &cavas::camera::Camera,
        viewport: &cavas::camera::Viewport,
        color: cavas::Color,
        stroke_px: f64,
        world_step: f64,
        min_step_screen: f64,
    ) {
        use cavas::camera::world_to_screen;
        use cavas::{Affine, Point, Stroke};
        let step = world_step * cam.zoom;
        if step < min_step_screen {
            return;
        }
        let stroke = Stroke::new(stroke_px);
        let w = viewport.width as f64;
        let h = viewport.height as f64;
        let origin = world_to_screen(cam, viewport, Point::new(0.0, 0.0));
        let x_off = ((origin.x % step) + step) % step;
        let y_off = ((origin.y % step) + step) % step;
        let mut path = cavas::BezPath::new();
        let mut x = x_off;
        while x <= w {
            path.move_to(Point::new(x, 0.0));
            path.line_to(Point::new(x, h));
            x += step;
        }
        let mut y = y_off;
        while y <= h {
            path.move_to(Point::new(0.0, y));
            path.line_to(Point::new(w, y));
            y += step;
        }
        scene.stroke(&stroke, Affine::IDENTITY, color, None, &path);
    }

    fn paint_lod_grid(&self, scene: &mut cavas::Scene, cam: &cavas::camera::Camera, viewport: &cavas::camera::Viewport, lod: DagDrawLod) {
        if !self.grid_visible || self.wheel_zoom_active || lod == DagDrawLod::Minimap {
            return;
        }
        let grid_color = self.canvas_theme.grid_minor_stroke;
        self.stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_LARGE, self.grid_step_large_world(), 0.0);
        match lod {
            DagDrawLod::Normal | DagDrawLod::Detail | DagDrawLod::Micro => {
                self.stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_MEDIUM, self.grid_step_medium_world(), 0.0);
            }
            DagDrawLod::Minimap | DagDrawLod::Overview | DagDrawLod::Compact => {}
        }
        if matches!(lod, DagDrawLod::Detail | DagDrawLod::Micro) {
            self.stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_SMALL, self.grid_step_small_world(), 0.0);
        }
        if lod == DagDrawLod::Micro {
            self.stroke_world_step_grid(scene, cam, viewport, grid_color, ui_styling::strokes::GRID_MICRO, self.grid_step_micro_world(), 0.0);
        }
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

    pub fn load_fixture_json(json: &str) -> Result<Self, DagError> {
        let fixture: DagFixture = serde_json::from_str(json)?;
        if fixture.schema != "dag.fixture" {
            return Err(DagError::SchemaMismatch);
        }
        validate_dag_fixture_node_kinds(&fixture.nodes)?;
        Ok(Self::from_fixture(fixture))
    }

    pub fn fixture_json(&self) -> Result<String, DagError> {
        Ok(serde_json::to_string(&self.fixture)?)
    }

    /// 🌳 Recomputes node positions from the current graph using layered tree layout.
    pub fn reorganize(&mut self, opts: &DagLayoutOptions) -> Result<(), DagError> {
        let mut fixture_value = serde_json::to_value(&self.fixture)?;
        apply_dag_layout_to_fixture_v1_value(&mut fixture_value, opts)?;
        self.fixture = serde_json::from_value(fixture_value)?;
        self.rebuild_engine_with_layout(false);
        Ok(())
    }

    fn rebuild_engine_with_layout(&mut self, apply_layout: bool) {
        self.engine = DagBoardEngine::new();
        self.engine.enforce_acyclic = true;
        self.node_id_map.clear();
        self.handle_key_map.clear();
        self.handle_port_shape.clear();
        self.handle_port_visible.clear();
        self.edge_id_map.clear();
        self.edge_route_style.clear();
        for node in &mut self.fixture.nodes {
            fit_node_size(node);
        }
        let (cx, cy, zoom) = (self.fixture.camera.x, self.fixture.camera.y, self.fixture.camera.zoom);
        self.engine.set_camera(cx, cy, zoom);
        if apply_layout {
            let mut fixture_value = serde_json::to_value(&self.fixture).unwrap_or_else(|_| serde_json::json!({}));
            let _ = apply_dag_layout_to_fixture_v1_value(&mut fixture_value, &DagLayoutOptions::default());
            if let Ok(updated) = serde_json::from_value::<DagFixture>(fixture_value.clone()) {
                self.fixture = updated;
            }
        }
        let mut next_handle: u64 = 10;
        let mut handle_map: HashMap<String, u64> = HashMap::new();
        for (idx, node) in self.fixture.nodes.iter().enumerate() {
            let nid = idx as u64 + 1;
            self.node_id_map.insert(nid, idx);
            self.engine.create_rect_node(nid, node.x, node.y, node.width, node.height, true);
            let inputs = node.inputs();
            let outputs = node.outputs();
            for (port_idx, port) in inputs.iter().enumerate() {
                let in_a = io_node_rect_port_angle_for_node(node, port_idx, true);
                let hid = next_handle;
                next_handle += 1;
                let public_key = format!("{}@{}", node.id, port.id);
                handle_map.insert(Self::dag_port_handle_key(&node.id, &port.id, true), hid);
                self.handle_key_map.insert(hid, public_key);
                self.handle_port_shape.insert(hid, port.shape);
                self.handle_port_visible.insert(hid, port.visible);
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
                let public_key = format!("{}@{}", node.id, port.id);
                handle_map.insert(Self::dag_port_handle_key(&node.id, &port.id, false), hid);
                self.handle_key_map.insert(hid, public_key);
                self.handle_port_shape.insert(hid, port.shape);
                self.handle_port_visible.insert(hid, port.visible);
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
                let src = e.source.split('@').next()?.to_string();
                let tgt = e.target.split('@').next()?.to_string();
                Some((src, tgt))
            })
            .collect();
        let mut eid: u64 = 100;
        for edge in &self.fixture.edges {
            if would_create_cycle(&existing, edge.source.split('@').next().unwrap_or(""), edge.target.split('@').next().unwrap_or("")) {
                continue;
            }
            let (source_node, source_port) = Self::dag_port_endpoint_parts(&edge.source);
            let (target_node, target_port) = Self::dag_port_endpoint_parts(&edge.target);
            let src = handle_map.get(&Self::dag_port_handle_key(&source_node, &source_port, false)).copied();
            let tgt = handle_map.get(&Self::dag_port_handle_key(&target_node, &target_port, true)).copied();
            if let (Some(s), Some(t)) = (src, tgt) {
                let id = Self::parse_fixture_edge_numeric_id(&edge.id).unwrap_or(eid);
                eid = eid.max(id).saturating_add(1);
                self.engine.create_edge(id, s, t);
                self.edge_id_map.insert(id, edge.id.clone());
                self.edge_route_style.insert(id, edge.route_style);
            }
        }
        self.engine.set_next_edge_id(eid);
    }

    fn parse_fixture_edge_numeric_id(id: &str) -> Option<u64> {
        id.strip_prefix('e').and_then(|s| s.parse().ok())
    }

    fn dag_port_endpoint_parts(endpoint: &str) -> (String, String) {
        endpoint.split_once('@').map(|(node, port)| (node.to_string(), port.to_string())).unwrap_or_else(|| (endpoint.to_string(), String::new()))
    }

    fn dag_port_handle_key(node_id: &str, port_id: &str, input: bool) -> String {
        format!("{}:{}:{}", node_id, if input { "in" } else { "out" }, port_id)
    }

    fn screen_to_world_point(&self, sx: f64, sy: f64) -> cavas::Point {
        use cavas::camera::{screen_to_world, Camera as CavasCamera, Viewport};
        use cavas::Point;
        let cam = CavasCamera { x: self.fixture.camera.x, y: self.fixture.camera.y, zoom: self.fixture.camera.zoom };
        let viewport = Viewport { width: self.width, height: self.height, dpr: self.dpr };
        screen_to_world(&cam, &viewport, Point::new(sx, sy))
    }

    fn sync_node_positions_from_engine(&mut self) {
        for (&nid, &idx) in &self.node_id_map {
            let Some(engine_node) = self.engine.nodes.get(&nid) else {
                continue;
            };
            let (mut x, mut y) = (engine_node.center.x, engine_node.center.y);
            if self.grid_snap_enabled {
                (x, y) = self.snap_world_pair(x, y);
            }
            self.fixture.nodes[idx].x = x;
            self.fixture.nodes[idx].y = y;
            if self.grid_snap_enabled {
                if let Some(engine_node) = self.engine.nodes.get_mut(&nid) {
                    engine_node.center = cavas::Point::new(x, y);
                }
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
            edges.push(DagFixtureEdge { id, source, target, ..Default::default() });
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
                    let label = id.and_then(|nid| self.widget_id_for_node_id(nid).or_else(|| self.engine.handles.get(&nid).and_then(|handle| self.widget_id_for_node_id(handle.node_id))));
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
        self.fixture.camera = DagCamera { x, y, zoom };
        self.engine.set_camera(x, y, zoom);
    }

    fn handle_id_for_port(&self, node_id: &str, port_id: &str) -> Option<HandleId> {
        let key = format!("{node_id}@{port_id}");
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
        use cavas::Point;
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

    fn rim_handle_anchor_hit(&self, world_x: f64, world_y: f64) -> Option<HandleId> {
        let hid = self.handle_anchor_hit(world_x, world_y)?;
        let lod = self.draw_lod_for_frame();
        if (lod.uses_input_row_connection_hitbox() || lod.uses_channel_row_pick()) && self.port_row_handle_hit(world_x, world_y, true, true).is_some() {
            let handle = self.engine.handles.get(&hid)?;
            let node = self.engine.nodes.get(&handle.node_id)?;
            let pos = handle_position(node, handle);
            let dx = world_x - pos.x;
            let dy = world_y - pos.y;
            let rim_tol = (handle.radius + 1.5).max(3.0);
            if dx * dx + dy * dy > rim_tol * rim_tol {
                return None;
            }
        }
        Some(hid)
    }

    fn fixture_draggable_node_hit(&self, world_x: f64, world_y: f64) -> Option<NodeId> {
        for idx in (0..self.fixture.nodes.len()).rev() {
            let node = &self.fixture.nodes[idx];
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            if world_x < node.x - hw || world_x > node.x + hw || world_y < node.y - hh || world_y > node.y + hh {
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
        let Some(hid) = self.rim_handle_anchor_hit(world_x, world_y) else {
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
        self.rim_handle_anchor_hit(world_x, world_y).is_some()
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
        use cavas::Point;
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
                DagNodeKind::Export { .. } => {
                    let (x0, y0, x1, y1) = action_control_bounds(node);
                    if point_in_rect(world_x, world_y, x0, y0, x1, y1) {
                        return Some((idx, WidgetPointerKind::ExportClick));
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

    /// ✏️ Begins inline note text editing at a world-space click position.
    pub fn begin_note_edit(&mut self, node_id: &str, world_x: f64, _world_y: f64) -> bool {
        let Some(node) = self.fixture.nodes.iter().find(|n| n.id == node_id) else {
            return false;
        };
        let DagNodeKind::Note { text, .. } = &node.kind else {
            return false;
        };
        let lod_index = dag_lod_index(self.fixture.camera.zoom);
        let font_px = dag_label_paint_px(self.fixture.camera.zoom, lod_index) * 1.05;
        let origin_x = note_text_origin_x(node);
        let offset = hit_byte_in_note_line(text, world_x, origin_x, font_px);
        self.editing_note = Some(NoteEditState { node_id: node_id.to_string(), caret: offset, anchor: offset });
        self.caret_visible = true;
        true
    }

    /// ✏️ Inserts text at the active note caret, replacing any selection.
    pub fn note_insert_text(&mut self, chunk: &str) -> bool {
        let Some(node_id) = self.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
            return false;
        };
        let Some(idx) = self.fixture.nodes.iter().position(|n| n.id == node_id) else {
            return false;
        };
        let DagNodeKind::Note { text, .. } = &mut self.fixture.nodes[idx].kind else {
            return false;
        };
        let Some(edit) = self.editing_note.as_mut() else {
            return false;
        };
        let start = edit.caret.min(edit.anchor);
        let end = edit.caret.max(edit.anchor);
        text.replace_range(start..end, chunk);
        let pos = start + chunk.len();
        edit.caret = pos;
        edit.anchor = pos;
        true
    }

    /// ✏️ Deletes the selection or the character before the note caret.
    pub fn note_backspace(&mut self) -> bool {
        let Some(node_id) = self.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
            return false;
        };
        let Some(idx) = self.fixture.nodes.iter().position(|n| n.id == node_id) else {
            return false;
        };
        let DagNodeKind::Note { text, .. } = &mut self.fixture.nodes[idx].kind else {
            return false;
        };
        let Some(edit) = self.editing_note.as_mut() else {
            return false;
        };
        let start = edit.caret.min(edit.anchor);
        let end = edit.caret.max(edit.anchor);
        if start < end {
            text.replace_range(start..end, "");
            edit.caret = start;
            edit.anchor = start;
            return true;
        }
        if start == 0 {
            return false;
        }
        let prev = text[..start].char_indices().last().map(|(i, _)| i).unwrap_or(0);
        text.replace_range(prev..start, "");
        edit.caret = prev;
        edit.anchor = prev;
        true
    }

    /// ✏️ Deletes the selection or the character after the note caret.
    pub fn note_delete_forward(&mut self) -> bool {
        let Some(node_id) = self.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
            return false;
        };
        let Some(idx) = self.fixture.nodes.iter().position(|n| n.id == node_id) else {
            return false;
        };
        let DagNodeKind::Note { text, .. } = &mut self.fixture.nodes[idx].kind else {
            return false;
        };
        let Some(edit) = self.editing_note.as_mut() else {
            return false;
        };
        let start = edit.caret.min(edit.anchor);
        let end = edit.caret.max(edit.anchor);
        if start < end {
            text.replace_range(start..end, "");
            edit.caret = start;
            edit.anchor = start;
            return true;
        }
        if start >= text.len() {
            return false;
        }
        let next = text[start..].char_indices().nth(1).map(|(i, _)| start + i).unwrap_or(text.len());
        text.replace_range(start..next, "");
        edit.caret = start;
        edit.anchor = start;
        true
    }

    /// ✏️ Moves the note caret (`left` | `right` | `home` | `end`).
    pub fn note_move_caret(&mut self, direction: &str, extend: bool) -> bool {
        let Some(node_id) = self.editing_note.as_ref().map(|edit| edit.node_id.clone()) else {
            return false;
        };
        let Some(idx) = self.fixture.nodes.iter().position(|n| n.id == node_id) else {
            return false;
        };
        let DagNodeKind::Note { text, .. } = &self.fixture.nodes[idx].kind else {
            return false;
        };
        let Some(edit) = self.editing_note.as_mut() else {
            return false;
        };
        let pos = match direction {
            "left" => {
                if edit.caret == 0 {
                    0
                } else {
                    text[..edit.caret].char_indices().last().map(|(i, _)| i).unwrap_or(0)
                }
            }
            "right" => text[edit.caret..].char_indices().nth(1).map(|(i, _)| edit.caret + i).unwrap_or(text.len()),
            "home" => 0,
            "end" => text.len(),
            _ => return false,
        };
        if extend {
            edit.caret = pos;
        } else {
            edit.caret = pos;
            edit.anchor = pos;
        }
        true
    }

    /// ✏️ Ends inline note editing.
    pub fn note_commit_edit(&mut self) {
        self.editing_note = None;
    }

    /// ✏️ Toggles native caret visibility for the active note editor.
    pub fn set_note_caret_visible(&mut self, visible: bool) {
        self.caret_visible = visible;
    }

    /// ✏️ Returns the widget id currently being edited inline, if any.
    pub fn editing_note_id(&self) -> Option<&str> {
        self.editing_note.as_ref().map(|edit| edit.node_id.as_str())
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
            WidgetPointerKind::ExportClick => {
                self.pending_export_click = Some(self.fixture.nodes[idx].id.clone());
            }
        }
        true
    }

    /// 💥 Takes a pending cluster explode request from the last widget hit.
    pub fn take_pending_cluster_explode(&mut self) -> Option<String> {
        self.pending_cluster_explode.take()
    }

    /// 📤 Takes a pending export control click from the last widget hit.
    pub fn take_pending_export_click(&mut self) -> Option<String> {
        self.pending_export_click.take()
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
        self.pointer_down_screen(x, y, 0, extend, false, false, false);
    }

    #[allow(clippy::too_many_arguments, reason = "flat args mirror the shared WASM host-bridge pointer-event contract used across all `pointer_*_screen` methods in this repo (see infinite/board/rs, framework/editor/rs, layout/rs, etc.)")]
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool, alt: bool, pan: bool) {
        if pan {
            self.pan_anchor = Some((sx, sy, self.fixture.camera.x, self.fixture.camera.y));
            return;
        }
        self.sync_connection_hit_picking_for_lod();
        self.last_screen_x = sx;
        self.last_screen_y = sy;
        let world = self.screen_to_world_point(sx, sy);
        if button == 0 && !shift && !ctrl_or_meta && !alt {
            if let Some(node_id) = self.fixture_draggable_node_hit(world.x, world.y) {
                if !self.world_hits_handle(world.x, world.y) {
                    if let Some(widget_id) = self.widget_id_for_node_id(node_id) {
                        if let Some(node) = self.fixture.nodes.iter().find(|entry| entry.id == widget_id) {
                            if let DagNodeKind::AppInstance { instance_id, .. } = &node.kind {
                                let now = pointer_event_now_ms();
                                let dist = ((world.x - self.last_pointer_down_world.0).powi(2) + (world.y - self.last_pointer_down_world.1).powi(2)).sqrt();
                                let zoom = self.fixture.camera.zoom.max(1e-9);
                                if self.last_pointer_down_node_id.as_deref() == Some(widget_id.as_str()) && now - self.last_pointer_down_at_ms < 350.0 && dist < 8.0 / zoom {
                                    self.pending_open_instance_id = Some(instance_id.clone());
                                    return;
                                }
                                self.last_pointer_down_at_ms = now;
                                self.last_pointer_down_world = (world.x, world.y);
                                self.last_pointer_down_node_id = Some(widget_id);
                            }
                        }
                    }
                }
            }
        }
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
        if button == 0 && !merge_from_modifiers && self.lod_uses_bounded_drag() {
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
        if let Some((start_sx, start_sy, cam_x, cam_y)) = self.pan_anchor {
            let zoom = self.fixture.camera.zoom.max(1e-9);
            let dx = (sx - start_sx) / zoom;
            let dy = (sy - start_sy) / zoom;
            self.set_camera(cam_x - dx, cam_y - dy, zoom);
            self.last_screen_x = sx;
            self.last_screen_y = sy;
            return;
        }
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
        if matches!(self.engine.interaction, InteractionMode::DragNode { .. } | InteractionMode::DragNodes { .. }) {
            self.sync_node_positions_from_engine();
        }
        self.process_engine_events();
        self.sync_camera_from_engine();
    }

    pub fn pointer_up(&mut self, x: f64, y: f64) {
        self.pointer_up_screen(x, y, false, false, false);
    }

    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
        self.pan_anchor = None;
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

    pub fn set_canvas_theme_from_json(&mut self, json: &str) -> Result<(), DagError> {
        self.canvas_theme.merge_from_json(json).map_err(DagError::CanvasTheme)?;
        self.icon_paint_cache.clear();
        Ok(())
    }

    /// 🖼 Screen-node overlay rects in CSS pixel space for DOM media layers.
    pub fn node_overlays_json(&self) -> Result<String, DagError> {
        use cavas::camera::{world_to_screen, Camera as CavasCamera, Viewport};
        use cavas::Point;
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
        Ok(serde_json::to_string(&overlays)?)
    }

    fn handle_cap_peak(&self, center: cavas::Point, outward: cavas::Vec2, radius: f64, shape: PortShape) -> cavas::Point {
        match shape {
            PortShape::Semicircle => handle_exterior_cap_peak(center, outward, radius),
            PortShape::Triangle => handle_exterior_cap_triangle_peak(center, outward, radius),
        }
    }

    fn handle_cap_fill_path(&self, center: cavas::Point, outward: cavas::Vec2, radius: f64, shape: PortShape) -> cavas::BezPath {
        match shape {
            PortShape::Semicircle => handle_exterior_cap_fill_path(center, outward, radius),
            PortShape::Triangle => handle_exterior_cap_triangle_fill_path(center, outward, radius),
        }
    }

    fn handle_cap_stroke_path(&self, center: cavas::Point, outward: cavas::Vec2, radius: f64, shape: PortShape) -> cavas::BezPath {
        match shape {
            PortShape::Semicircle => handle_exterior_cap_stroke_path(center, outward, radius),
            PortShape::Triangle => handle_exterior_cap_triangle_stroke_path(center, outward, radius),
        }
    }

    fn edge_sharp_path(&self, eid: EdgeId) -> Option<cavas::BezPath> {
        let edge = self.engine.edges.get(&eid)?;
        let source_handle = self.engine.handles.get(&edge.source)?;
        let target_handle = self.engine.handles.get(&edge.target)?;
        let source_node = self.engine.nodes.get(&source_handle.node_id)?;
        let target_node = self.engine.nodes.get(&target_handle.node_id)?;
        let source_position = handle_position(source_node, source_handle);
        let target_position = handle_position(target_node, target_handle);
        let source_out = handle_outward_at_node_rim(source_position, source_node.center, source_node.shape, source_node.radius, source_node.width, source_node.height)?;
        let target_out = handle_outward_at_node_rim(target_position, target_node.center, target_node.shape, target_node.radius, target_node.width, target_node.height)?;
        let source_shape = self.handle_port_shape.get(&edge.source).copied().unwrap_or_default();
        let target_shape = self.handle_port_shape.get(&edge.target).copied().unwrap_or_default();
        let source_wire = self.handle_cap_peak(source_position, source_out, source_handle.radius, source_shape);
        let target_wire = self.handle_cap_peak(target_position, target_out, target_handle.radius, target_shape);
        Some(compute_edge_sharp_sz_path(source_wire, target_wire, source_out, target_out))
    }

    fn paint_node_handles_for_spec(&self, scene: &mut cavas::Scene, aff: &cavas::Affine, cam: &cavas::camera::Camera, node: &DagNodeSpec, chrome: &DagNodePaintChrome) {
        use cavas::FillRule;
        use cavas::Stroke;
        use cavas::{Circle, Point};
        use graph::{handle_outward_at_node_rim, handle_position_on_rectangle, NodeShape};

        let theme = &self.canvas_theme;
        let handle_stroke_px = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX, cam.zoom);
        let tint = chrome.tint_highlighted();
        let fill = dag_handle_body_fill(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
        let stroke_c = dag_handle_body_stroke(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
        let handle_chrome = chrome.has_interaction_chrome();
        let center = Point::new(node.x, node.y);
        let paint_port = |scene: &mut cavas::Scene, port_idx: usize, inputs: bool, port: &IoPortSpec| {
            if !port.visible {
                return;
            }
            let angle = io_node_rect_port_angle_for_node(node, port_idx, inputs);
            let handle_center = handle_position_on_rectangle(center, node.width, node.height, angle);
            let outward = handle_outward_at_node_rim(handle_center, center, NodeShape::Rectangle, 0.0, node.width, node.height);
            if let Some(out) = outward {
                if handle_chrome {
                    scene.fill(FillRule::NonZero, *aff, fill, None, &self.handle_cap_fill_path(handle_center, out, DAG_HANDLE_WORLD_RADIUS, port.shape));
                }
                scene.stroke(&Stroke::new(handle_stroke_px), *aff, stroke_c, None, &self.handle_cap_stroke_path(handle_center, out, DAG_HANDLE_WORLD_RADIUS, port.shape));
            } else {
                let circle = Circle::new(handle_center, DAG_HANDLE_WORLD_RADIUS);
                if handle_chrome {
                    scene.fill(FillRule::NonZero, *aff, fill, None, &circle);
                }
                scene.stroke(&Stroke::new(handle_stroke_px), *aff, stroke_c, None, &circle);
            }
        };
        for (port_idx, port) in node.inputs().iter().enumerate() {
            paint_port(scene, port_idx, true, port);
        }
        for (port_idx, port) in node.outputs().iter().enumerate() {
            paint_port(scene, port_idx, false, port);
        }
    }

    pub fn label_overlay_rows_for_node_spec(&self, node: &DagNodeSpec, ghost: bool) -> Vec<serde_json::Value> {
        let lod = self.draw_lod_for_frame();
        let zoom = self.fixture.camera.zoom;
        let lod_index = dag_lod_index(zoom);
        Self::label_overlay_rows_for_node(node, lod, zoom, lod_index, ghost)
    }

    fn label_overlay_rows_for_node(node: &DagNodeSpec, lod: DagDrawLod, zoom: f64, lod_index: usize, ghost: bool) -> Vec<serde_json::Value> {
        let paint_px = dag_label_paint_px(zoom, lod_index);
        let mut labels = Vec::new();
        if let Some(text) = Self::node_label_text(node, lod).map(str::to_string) {
            let (layout, x, y) = if lod.node_label_is_horizontal() {
                ("horizontal", node.x, node.y)
            } else if (uses_computation_layout(&node.kind) && lod.shows_computation_layout()) || (matches!(node.kind, DagNodeKind::Slider { .. }) && lod.shows_controls()) {
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
            for mut row in Self::port_label_overlay_rows(node, lod, zoom, lod_index) {
                if let Some(obj) = row.as_object_mut() {
                    obj.insert("ghost".into(), serde_json::Value::Bool(ghost));
                }
                labels.push(row);
            }
        }
        labels
    }

    fn port_label_overlay_rows(node: &DagNodeSpec, lod: DagDrawLod, zoom: f64, lod_index: usize) -> Vec<serde_json::Value> {
        use cavas::text::label_extent;
        let hw = node.width * 0.5;
        let handle_inset = 8.0 / zoom.max(0.05);
        let inputs = node.inputs();
        let outputs = node.outputs();
        let computation = uses_computation_layout(&node.kind);
        let port_layout_px = if computation { dag_label_compact_paint_px(zoom, lod_index) } else { dag_label_paint_px(zoom, lod_index) };
        let mut rows = Vec::new();
        let input_column_w = if computation { io_port_column_width(inputs, port_layout_px) } else { (hw - handle_inset).max(8.0) };
        let output_column_w = if computation { io_port_column_width(outputs, port_layout_px) } else { (hw - handle_inset).max(8.0) };
        for (i, port) in inputs.iter().enumerate() {
            if port.shape == PortShape::Triangle || !port.visible {
                continue;
            }
            let label = port.label_with_cardinality(lod);
            if label.trim().is_empty() {
                continue;
            }
            let world_y = port_center_y(node, i, inputs.len());
            let world_x = if computation { computation_input_label_x(node) } else { node.x - hw + handle_inset };
            rows.push(serde_json::json!({
                "id": node.id,
                "kind": "port",
                "text": label,
                "layout": "horizontal",
                "align": "left",
                "x": world_x,
                "y": world_y,
                "nodeW": input_column_w,
                "nodeH": DAG_CHANNEL_ROW_HEIGHT,
                "fontScreenPx": port_layout_px,
                "maxScreenH": port_layout_px * 1.3,
            }));
        }
        for (i, port) in outputs.iter().enumerate() {
            if port.shape == PortShape::Triangle || !port.visible {
                continue;
            }
            let label = port.label_with_cardinality(lod);
            if label.trim().is_empty() {
                continue;
            }
            let world_y = port_center_y(node, i, outputs.len());
            let (world_x, column_w) = if computation {
                let left = computation_output_label_x(node, &label, port_layout_px);
                (left + port_label_text_width(&label, port_layout_px), output_column_w)
            } else {
                let (label_w, _) = label_extent(&label, port_layout_px);
                (node.x + hw - handle_inset, label_w / zoom.max(0.05))
            };
            rows.push(serde_json::json!({
                "id": node.id,
                "kind": "port",
                "text": label,
                "layout": "horizontal",
                "align": "right",
                "x": world_x,
                "y": world_y,
                "nodeW": column_w,
                "nodeH": DAG_CHANNEL_ROW_HEIGHT,
                "fontScreenPx": port_layout_px,
                "maxScreenH": port_layout_px * 1.3,
            }));
        }
        rows
    }

    /// 🎚️ Slider track anchors for the HTML slider overlay.
    pub fn slider_overlay_state_json(&self) -> Result<String, DagError> {
        let cam = &self.fixture.camera;
        let mut sliders: Vec<serde_json::Value> = Vec::new();
        for (idx, fixture_node) in self.fixture.nodes.iter().enumerate() {
            let node = self.node_spec_for_paint(idx, fixture_node);
            let DagNodeKind::Slider { min, max, step, value, .. } = &node.kind else {
                continue;
            };
            let (x0, y0, x1, y1) = slider_track_bounds(&node);
            sliders.push(serde_json::json!({
                "widgetId": fixture_node.id,
                "value": value,
                "min": min,
                "max": max,
                "step": step,
                "x": (x0 + x1) * 0.5,
                "y": (y0 + y1) * 0.5,
                "w": (x1 - x0).max(1.0),
                "h": (y1 - y0).max(1.0),
            }));
        }
        serde_json::to_string(&serde_json::json!({
            "camera": { "x": cam.x, "y": cam.y, "zoom": cam.zoom },
            "width": self.width,
            "height": self.height,
            "sliders": sliders,
        }))
        .map_err(DagError::from)
    }

    /// 🏷️ Camera, draw LOD, and node label anchors for the JS canvas text overlay (must match the last GPU frame).
    pub fn label_overlay_paint_state_json(&self) -> Result<String, DagError> {
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
        .map_err(DagError::from)
    }

    fn paint_variadic_plus_controls(scene: &mut cavas::Scene, cam: &cavas::camera::Camera, viewport: &cavas::camera::Viewport, node: &DagNodeSpec, px: f64, fill: cavas::Color, halo: cavas::Color) {
        use cavas::camera::world_to_screen;
        use cavas::text::append_label;
        for (_, px_world, py_world) in variadic_input_insert_positions(node) {
            let screen = world_to_screen(cam, viewport, cavas::Point::new(px_world, py_world));
            append_label(scene, "+", screen, px * 0.95, fill, halo);
        }
        for (_, px_world, py_world) in variadic_output_insert_positions(node) {
            let screen = world_to_screen(cam, viewport, cavas::Point::new(px_world, py_world));
            append_label(scene, "+", screen, px * 0.95, fill, halo);
        }
    }

    fn paint_node_name_vertical(scene: &mut cavas::Scene, center_screen: cavas::Point, name: &str, px: f64, label_fill: cavas::Color, label_halo: cavas::Color) {
        use cavas::text::{append_label, label_extent};
        use cavas::{Affine, Point};
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return;
        }
        let (w, h) = label_extent(trimmed, px);
        let mut label_scene = cavas::Scene::new();
        append_label(&mut label_scene, trimmed, Point::new(0.0, 0.0), px, label_fill, label_halo);
        let rot = Affine::IDENTITY.translate((center_screen.x, center_screen.y)) * Affine::IDENTITY.rotate(-std::f64::consts::FRAC_PI_2) * Affine::IDENTITY.translate((-w * 0.5, -h * 0.5));
        scene.append(&label_scene, Some(rot));
    }

    fn paint_computation_column_divider(scene: &mut cavas::Scene, aff: cavas::Affine, node: &DagNodeSpec, chrome_stroke: f64, stroke: cavas::Color) {
        use cavas::{Line, Point, Stroke};
        let Some(divider_x) = computation_column_divider_x(node) else {
            return;
        };
        let hh = node.height * 0.5;
        let top = node.y - hh;
        let bottom = node.y + hh;
        let stroke_style = Stroke::new(chrome_stroke);
        scene.stroke(&stroke_style, aff, stroke, None, &Line::new(Point::new(divider_x, top), Point::new(divider_x, bottom)));
    }

    fn paint_computation_channel_row_highlights(&self, scene: &mut cavas::Scene, aff: &cavas::Affine, node: &DagNodeSpec, theme: &CanvasPalette, is_dimmed: bool) {
        use cavas::FillRule;
        use cavas::Rect;
        let mut paint_bounds = |(x0, y0, x1, y1): (f64, f64, f64, f64), selected: bool, highlighted: bool, hovered: bool| {
            if !selected && !highlighted && !hovered {
                return;
            }
            let fill = dag_handle_body_fill(theme, is_dimmed, selected, highlighted, hovered);
            scene.fill(FillRule::NonZero, *aff, fill, None, &Rect::new(x0, y0, x1, y1));
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

    fn computation_channel_row_divider_stroke(&self, node: &DagNodeSpec, port_id: &str, body_stroke: cavas::Color, label_fill: cavas::Color, default_stroke: cavas::Color) -> cavas::Color {
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

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_computation_channel_row_dividers(&self, scene: &mut cavas::Scene, aff: cavas::Affine, node: &DagNodeSpec, chrome_stroke: f64, stroke: cavas::Color, body_stroke: cavas::Color, label_fill: cavas::Color, channel_row_pick: bool) {
        use cavas::{Line, Point, Stroke};
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
                scene.stroke(&stroke_style, aff, row_stroke(port_id), None, &Line::new(Point::new(left, y), Point::new(right, y)));
            }
        }
        if computation_output_column_x_bounds(node).is_some() {
            let (left, right) = computation_channel_row_divider_x_span(node, ComputationChannelRowSide::Output);
            let outputs = node.outputs();
            for row in computation_io_side_row_divider_indices(output_rows, grid_rows) {
                let y = channel_row_divider_y(node.y, node.height, row);
                let port_id = outputs.get(row.saturating_sub(1)).map(|port| port.id.as_str()).unwrap_or("");
                scene.stroke(&stroke_style, aff, row_stroke(port_id), None, &Line::new(Point::new(left, y), Point::new(right, y)));
            }
        }
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_preview_image_content(&self, scene: &mut cavas::Scene, cam: &cavas::camera::Camera, viewport: &cavas::camera::Viewport, node: &DagNodeSpec, src: &str, label_fill: cavas::Color, bg: cavas::Color) {
        use cavas::camera::world_to_screen;
        use cavas::text::append_label;
        use cavas::Point;
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

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_preview_content(
        &self,
        scene: &mut cavas::Scene,
        cam: &cavas::camera::Camera,
        viewport: &cavas::camera::Viewport,
        node: &DagNodeSpec,
        content: &DagPreviewContent,
        expanded: &BTreeSet<String>,
        paint_px: f64,
        label_fill: cavas::Color,
        label_halo: cavas::Color,
        bg: cavas::Color,
    ) {
        use cavas::camera::world_to_screen;
        use cavas::text::append_label;
        use cavas::Point;
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
                let (x0, y0, x1, _) = preview_content_bounds(node);
                let rows = preview_tree_rows(json, expanded, "", 0);
                let row_font_px = paint_px * 0.9;
                for (index, row) in rows.iter().enumerate() {
                    let row_y = y0 + index as f64 * DAG_PREVIEW_ROW_HEIGHT + DAG_PREVIEW_ROW_HEIGHT * 0.5;
                    let indent = row.depth as f64 * DAG_PREVIEW_TREE_INDENT;
                    if row.has_children {
                        let glyph = if row.expanded { "▾" } else { "▸" };
                        let toggle_pos = world_to_screen(cam, viewport, Point::new(x0 + indent + DAG_PREVIEW_TOGGLE_WIDTH * 0.5, row_y));
                        append_label(scene, glyph, toggle_pos, row_font_px, label_fill, label_halo);
                    }
                    let text_x = x0 + indent + if row.has_children { DAG_PREVIEW_TOGGLE_WIDTH } else { 0.0 } + 2.0;
                    let line = if row.has_children && row.expanded { row.label.clone() } else { format!("{}: {}", row.label, row.summary) };
                    let max_w = (x1 - text_x).max(1.0);
                    let shown = truncate_label_to_fit_width(&line, max_w, row_font_px);
                    let text_pos = world_to_screen(cam, viewport, Point::new(text_x, row_y));
                    append_label(scene, &shown, text_pos, row_font_px, label_fill, label_halo);
                }
            }
        }
    }

    fn paint_io_widget_channel_borders(scene: &mut cavas::Scene, aff: cavas::Affine, node: &DagNodeSpec, px: f64, chrome_stroke: f64, stroke: cavas::Color) {
        use cavas::{Line, Point, Stroke};
        let (name_left, top, name_right, bottom) = io_widget_name_column_bounds(node, px);
        let stroke_style = Stroke::new(chrome_stroke);
        scene.stroke(&stroke_style, aff, stroke, None, &Line::new(Point::new(name_left, top), Point::new(name_left, bottom)));
        scene.stroke(&stroke_style, aff, stroke, None, &Line::new(Point::new(name_right, top), Point::new(name_right, bottom)));
    }

    fn node_label_text(node: &DagNodeSpec, lod: DagDrawLod) -> Option<&str> {
        if matches!(node.kind, DagNodeKind::Preview { .. } | DagNodeKind::Note { .. }) {
            return None;
        }
        let text = match lod.node_label() {
            DagNodeLabel::None => return None,
            DagNodeLabel::Abbreviation => node.abbreviation.trim(),
            DagNodeLabel::Name => node.name.trim(),
        };
        if text.is_empty() {
            None
        } else {
            Some(text)
        }
    }

    fn node_caption_delegated_to_js_overlay(node: &DagNodeSpec, lod: DagDrawLod) -> bool {
        Self::node_label_text(node, lod).is_some()
    }

    fn should_paint_node_lod_icon(node: &DagNodeSpec, lod: DagDrawLod) -> bool {
        if !lod.node_icon_visible() {
            return false;
        }
        !uses_computation_layout(&node.kind) || !lod.shows_computation_layout()
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_node_lod_icon(&self, scene: &mut cavas::Scene, lod: DagDrawLod, center_screen: cavas::Point, node: &DagNodeSpec, zoom: f64, fg: cavas::Color, bg: cavas::Color) {
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

    fn paint_cluster_affordances(scene: &mut cavas::Scene, cam: &cavas::camera::Camera, viewport: &cavas::camera::Viewport, node: &DagNodeSpec, paint_px: f64, label_fill: cavas::Color, label_halo: cavas::Color) {
        use cavas::camera::world_to_screen;
        use cavas::text::append_label;
        use cavas::Point;
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

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_computation_node_name(scene: &mut cavas::Scene, cam: &cavas::camera::Camera, viewport: &cavas::camera::Viewport, node: &DagNodeSpec, label: &str, px: f64, label_fill: cavas::Color, label_halo: cavas::Color) {
        use cavas::camera::world_to_screen;
        use cavas::Point;
        let (label_x, label_y) = computation_name_world_center(node, label, px, cam.zoom);
        let anchor = world_to_screen(cam, viewport, Point::new(label_x, label_y));
        Self::paint_node_name_horizontal(scene, anchor, label, px, label_fill, label_halo);
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_slider_name(scene: &mut cavas::Scene, cam: &cavas::camera::Camera, viewport: &cavas::camera::Viewport, node: &DagNodeSpec, label: &str, px: f64, label_fill: cavas::Color, label_halo: cavas::Color) {
        Self::paint_computation_node_name(scene, cam, viewport, node, label, px, label_fill, label_halo);
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_io_widget_name(scene: &mut cavas::Scene, cam: &cavas::camera::Camera, viewport: &cavas::camera::Viewport, node: &DagNodeSpec, lod: DagDrawLod, label: &str, px: f64, label_fill: cavas::Color, label_halo: cavas::Color) {
        use cavas::camera::world_to_screen;
        use cavas::Point;
        if lod.node_label() == DagNodeLabel::None && !lod.shows_controls() {
            return;
        }
        let (label_x, label_y) = io_widget_label_center(node);
        let name_anchor = world_to_screen(cam, viewport, Point::new(label_x, label_y));
        Self::paint_node_name_vertical(scene, name_anchor, label, px, label_fill, label_halo);
    }

    fn paint_node_name_horizontal(scene: &mut cavas::Scene, center_screen: cavas::Point, name: &str, px: f64, label_fill: cavas::Color, label_halo: cavas::Color) {
        use cavas::text::{append_label, label_extent};
        use cavas::Point;
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return;
        }
        let (w, h) = label_extent(trimmed, px);
        append_label(scene, trimmed, Point::new(center_screen.x - w * 0.5, center_screen.y - h * 0.5), px, label_fill, label_halo);
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_computing_border_arc(&self, scene: &mut cavas::Scene, aff: &cavas::Affine, rect: &cavas::Rect, cam_zoom: f64, color: cavas::Color, start_t: f64, dashed: bool) {
        use cavas::{BezPath, Stroke};
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
            stroke.set_dash_pattern(vec![stroke_px * 2.5, stroke_px * 2.0]);
        }
        scene.stroke(&stroke, *aff, color, None, &path);
    }

    fn paint_computing_active_border(&self, scene: &mut cavas::Scene, aff: &cavas::Affine, rect: &cavas::Rect, cam_zoom: f64, theme: &CanvasPalette) {
        self.paint_computing_border_arc(scene, aff, rect, cam_zoom, theme.node_stroke_selected, self.computing_active_anim_phase.get(), false);
    }

    fn paint_computing_stale_border(&self, scene: &mut cavas::Scene, aff: &cavas::Affine, rect: &cavas::Rect, cam_zoom: f64, theme: &CanvasPalette) {
        let highlight = canvas_color_with_alpha(theme.node_stroke_selected, 220);
        self.paint_computing_border_arc(scene, aff, rect, cam_zoom, highlight, self.computing_stale_anim_phase.get(), true);
    }

    fn rect_perimeter_point(rect: &cavas::Rect, t: f64) -> cavas::Point {
        use cavas::Point;
        let t = t.fract();
        let w = rect.width();
        let h = rect.height();
        let perim = 2.0 * (w + h);
        let mut d = t * perim;
        if d <= w {
            return Point::new(rect.x0() + d, rect.y0());
        }
        d -= w;
        if d <= h {
            return Point::new(rect.x1(), rect.y0() + d);
        }
        d -= h;
        if d <= w {
            return Point::new(rect.x1() - d, rect.y1());
        }
        d -= w;
        Point::new(rect.x0(), rect.y1() - d)
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_note_caret_bar(scene: &mut cavas::Scene, aff: &cavas::Affine, node: &DagNodeSpec, caret_byte: usize, text: &str, font_px: f64, fill: cavas::Color, zoom: f64) {
        use cavas::text::label_byte_world_x;
        use cavas::FillRule;
        use cavas::Rect;
        let (x0, y0, _x1, y1) = preview_content_bounds(node);
        let origin_x = x0 + DAG_PREVIEW_PAD;
        let caret_x = label_byte_world_x(text, caret_byte.min(text.len()), origin_x, font_px);
        let caret_y = (y0 + y1) * 0.5;
        let lh = font_px * 1.2;
        let bar_w = 1.5 / zoom.max(0.05);
        let rect = Rect::new(caret_x, caret_y - lh * 0.4, caret_x + bar_w, caret_y + lh * 0.4);
        scene.fill(FillRule::NonZero, *aff, fill, None, &rect);
    }

    #[allow(clippy::too_many_arguments, reason = "internal rendering helper takes scene/camera/viewport/geometry/color context flatly, matching this crate's paint_* convention")]
    fn paint_node_visual(
        &self,
        scene: &mut cavas::Scene,
        aff: &cavas::Affine,
        cam: &cavas::camera::Camera,
        viewport: &cavas::camera::Viewport,
        lod: DagDrawLod,
        lod_index: usize,
        node: &DagNodeSpec,
        chrome: DagNodePaintChrome,
    ) {
        use cavas::camera::world_to_screen;
        use cavas::text::append_label;
        use cavas::FillRule;
        use cavas::{Point, Rect, Stroke};

        let theme = &self.canvas_theme;
        let label_halo = theme.label_halo;
        let hw = node.width * 0.5;
        let hh = node.height * 0.5;
        let rect = Rect::new(node.x - hw, node.y - hh, node.x + hw, node.y + hh);
        let tint = chrome.tint_highlighted();
        let fill = dag_node_paint_fill(lod, theme, chrome.is_dimmed, chrome.is_selected, chrome.is_highlighted, chrome.is_hovered).map(|color| canvas_color_with_alpha(color, chrome.body_fill_alpha));
        let stroke = dag_node_body_stroke(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
        let label_fill = dag_node_label_fill(theme, chrome.is_dimmed, chrome.is_selected, tint, chrome.is_hovered);
        let internal_chrome_stroke = dag_node_internal_chrome_stroke(stroke, label_fill, chrome.is_hovered || chrome.is_selected || chrome.is_highlighted);
        let stroke_screen_px = dag_node_stroke_screen_px(chrome.is_dimmed, chrome.is_selected, chrome.is_highlighted, chrome.is_hovered);
        if let Some(fill) = fill {
            scene.fill(FillRule::NonZero, *aff, fill, None, &rect);
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
                        self.paint_computation_channel_row_dividers(scene, *aff, node, chrome_stroke, internal_chrome_stroke, stroke, label_fill, channel_row_pick);
                        if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                            Self::paint_computation_node_name(scene, cam, viewport, node, label, paint_px, label_fill, label_halo);
                        }
                        if matches!(node.kind, DagNodeKind::Cluster { .. }) {
                            Self::paint_cluster_affordances(scene, cam, viewport, node, paint_px, label_fill, label_halo);
                        }
                        if let DagNodeKind::Computation { variadic_inputs, variadic_outputs, .. } = &node.kind {
                            if cam.zoom >= DAG_VARIADIC_PLUS_ZOOM_THRESHOLD && (*variadic_inputs || *variadic_outputs) {
                                Self::paint_variadic_plus_controls(scene, cam, viewport, node, paint_px, label_fill, label_halo);
                            }
                        }
                    }
                }
                DagNodeKind::Slider { value, .. } => {
                    if let Some(label) = label_text.filter(|_| lod.shows_controls() && !caption_on_overlay) {
                        Self::paint_slider_name(scene, cam, viewport, node, label, paint_px, label_fill, label_halo);
                    }
                    // 🎚️ Track + knob live on the HTML `GraphSliderOverlays` control — painting them here
                    // doubles a pixelated GPU ghost above the crisp DOM slider. Only the left value readout
                    // stays on the GPU so the overlay can stay track-bounds-aligned (`showValue={false}`).
                    if lod.shows_controls() {
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
                            append_label(scene, kind_label, hint, paint_px * 0.85, canvas_color_with_alpha(label_fill, ui_styling::opacities::KIND_HINT_ALPHA), label_halo);
                        }
                    }
                }
                DagNodeKind::Note { text, .. } => {
                    if lod.shows_detail_text() || lod.shows_controls() {
                        let (x0, y0, x1, y1) = preview_content_bounds(node);
                        let font_px = paint_px * 1.05;
                        let text_x = x0 + DAG_PREVIEW_PAD;
                        let max_w = (x1 - text_x).max(1.0);
                        let display = if text.is_empty() { "…" } else { text.as_str() };
                        let shown = truncate_label_to_fit_width(display, max_w, font_px);
                        let pos = world_to_screen(cam, viewport, Point::new(text_x, (y0 + y1) * 0.5));
                        append_label(scene, &shown, pos, font_px, label_fill, label_halo);
                        if let Some(edit) = &self.editing_note {
                            if edit.node_id == node.id && self.caret_visible {
                                Self::paint_note_caret_bar(scene, aff, node, edit.caret, text, font_px, label_fill, cam.zoom);
                            }
                        }
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
                DagNodeKind::Export { format, .. } => {
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
                            let export_label = format.to_uppercase();
                            let pos = world_to_screen(cam, viewport, Point::new(node.x, node.y));
                            append_label(scene, &export_label, pos, paint_px * 0.95, label_fill, label_halo);
                        }
                    }
                }
                DagNodeKind::AppInstance { program_id, app_id, .. } => {
                    if let Some(label) = label_text.filter(|_| !caption_on_overlay) {
                        Self::paint_node_name_horizontal(scene, center_screen, label, paint_px, label_fill, label_halo);
                    }
                    if lod.shows_detail_text() {
                        let subtitle = format!("{program_id}/{app_id}");
                        let subtitle_pos = world_to_screen(cam, viewport, Point::new(node.x, node.y + hh * 0.22));
                        append_label(scene, &subtitle, subtitle_pos, paint_px * 0.85, label_fill, label_halo);
                    }
                }
            }
        }
    }

    pub fn paint_scene(&self, scene: &mut cavas::Scene, viewport_w: u32, viewport_h: u32, dpr: f64) {
        use cavas::camera::{camera_content_affine, Camera as CavasCamera, Viewport};
        use cavas::FillRule;
        use cavas::{Circle, Rect, Stroke};

        let theme = &self.canvas_theme;
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
            dag_debug_log(&format!("[DEBUG] dag draw lod={} zoom={:.3} icon={} label={:?}", lod.label(), cam.zoom, lod.node_icon_visible(), lod.node_label()));
        }
        self.paint_lod_grid(scene, &cam, &viewport, lod);
        let snap = self.engine.render_snapshot();
        let edge_stroke = dag_world_stroke(lod.edge_stroke_screen_px(), cam.zoom);
        for &eid in self.engine.edges.keys() {
            let (is_selected, is_highlighted, is_hovered) = self.edge_interaction_chrome(eid);
            let stroke_c = dag_edge_body_stroke(theme, false, is_selected, is_highlighted, is_hovered);
            let style = self.edge_route_style.get(&eid).copied().unwrap_or_default();
            match style {
                EdgeRouteStyle::Bezier => {
                    if let Some(curve) = self.engine.edge_curve(eid) {
                        scene.stroke(&Stroke::new(edge_stroke), aff, stroke_c, None, &curve);
                    }
                }
                EdgeRouteStyle::SharpSz => {
                    if let Some(path) = self.edge_sharp_path(eid) {
                        scene.stroke(&Stroke::new(edge_stroke), aff, stroke_c, None, &path);
                    }
                }
            }
        }
        if let Some(preview) = snap.pending_edge {
            scene.stroke(&Stroke::new(edge_stroke), aff, dag_edge_body_stroke(theme, false, true, false, false), None, &preview);
        }
        let handle_stroke_px = dag_world_stroke(DAG_CHROME_STROKE_SCREEN_PX, cam.zoom);
        let paint_snap_handle = |scene: &mut cavas::Scene, hid: &HandleId, center: &cavas::Point, shape_filter: Option<PortShape>| {
            if self.handle_port_visible.get(hid) == Some(&false) {
                return;
            }
            let shape = self.handle_port_shape.get(hid).copied().unwrap_or_default();
            if let Some(filter) = shape_filter {
                if shape != filter {
                    return;
                }
            }
            if !lod.shows_handles() && shape != PortShape::Triangle {
                return;
            }
            let node_id = self.engine.handles.get(hid).map(|handle| handle.node_id);
            let is_dimmed = node_id.is_some_and(|nid| self.dimmed.contains(&nid));
            let (is_selected, is_highlighted, is_hovered) = self.handle_interaction_chrome(*hid);
            let fill = dag_handle_body_fill(theme, is_dimmed, is_selected, is_highlighted, is_hovered);
            let stroke_c = dag_handle_body_stroke(theme, is_dimmed, is_selected, is_highlighted, is_hovered);
            let chrome = is_dimmed || is_selected || is_highlighted || is_hovered;
            let outward = node_id.and_then(|nid| self.engine.nodes.get(&nid).and_then(|node| handle_outward_at_node_rim(*center, node.center, node.shape, node.radius, node.width, node.height)));
            if let Some(out) = outward {
                if chrome {
                    scene.fill(FillRule::NonZero, aff, fill, None, &self.handle_cap_fill_path(*center, out, DAG_HANDLE_WORLD_RADIUS, shape));
                }
                scene.stroke(&Stroke::new(handle_stroke_px), aff, stroke_c, None, &self.handle_cap_stroke_path(*center, out, DAG_HANDLE_WORLD_RADIUS, shape));
            } else {
                let circle = Circle::new(*center, DAG_HANDLE_WORLD_RADIUS);
                if chrome {
                    scene.fill(FillRule::NonZero, aff, fill, None, &circle);
                }
                scene.stroke(&Stroke::new(handle_stroke_px), aff, stroke_c, None, &circle);
            }
        };
        for (hid, center, _radius) in &snap.handles {
            paint_snap_handle(scene, hid, center, Some(PortShape::Semicircle));
        }
        let paint_minimap_node = |scene: &mut cavas::Scene, idx: usize, fixture_node: &DagNodeSpec| {
            let node = self.node_spec_for_paint(idx, fixture_node);
            let node = node.as_ref();
            let hw = node.width * 0.5;
            let hh = node.height * 0.5;
            let rect = Rect::new(node.x - hw, node.y - hh, node.x + hw, node.y + hh);
            let engine_nid = self.engine_node_id_for_index(idx);
            let is_dimmed = engine_nid.is_some_and(|nid| self.dimmed.contains(&nid));
            let (is_selected, is_highlighted, is_hovered) = engine_nid.map(|nid| self.node_interaction_chrome(nid)).unwrap_or((false, false, false));
            if let Some(fill) = dag_node_paint_fill(lod, theme, is_dimmed, is_selected, is_highlighted, is_hovered) {
                scene.fill(FillRule::NonZero, aff, fill, None, &rect);
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
            let (is_selected, is_highlighted, is_hovered) = engine_nid.map(|nid| self.node_interaction_chrome(nid)).unwrap_or((false, false, false));
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
                DagNodePaintChrome { is_dimmed, is_selected, is_highlighted, is_hovered, is_computing, is_stale, body_fill_alpha: 255, ghost_tint: false },
            );
        }
        if let Some(ghost) = self.ghost_node.as_ref() {
            self.paint_node_visual(scene, &aff, &cam, &viewport, lod, lod_index, ghost, DagNodePaintChrome::ghost_preview());
        }
        for (hid, center, _radius) in &snap.handles {
            paint_snap_handle(scene, hid, center, Some(PortShape::Triangle));
        }
        if let Some(ghost) = self.ghost_node.as_ref() {
            let paint_ghost_handles = lod.shows_handles() || ghost.inputs().iter().any(|port| port.shape == PortShape::Triangle) || ghost.outputs().iter().any(|port| port.shape == PortShape::Triangle);
            if paint_ghost_handles {
                self.paint_node_handles_for_spec(scene, &aff, &cam, ghost, &DagNodePaintChrome::ghost_preview());
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
            let host = DagHost::load_fixture_json(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            self.state.borrow_mut().host = host;
            Ok(())
        }

        #[wasm_bindgen(js_name = fixtureJson)]
        pub fn fixture_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.fixture_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = nodeOverlaysJson)]
        pub fn node_overlays_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.node_overlays_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = labelOverlayPaintStateJson)]
        pub fn label_overlay_paint_state_json(&self) -> Result<String, JsValue> {
            self.state.borrow().host.label_overlay_paint_state_json().map_err(|e| JsValue::from_str(&e.to_string()))
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
                let (render_ctx, renderer, surface) = cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|err| JsValue::from_str(&err))?;
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

        #[wasm_bindgen(js_name = takePendingOpenInstanceId)]
        pub fn take_pending_open_instance_id(&self) -> Option<String> {
            dag_take_pending_open_instance_id(&mut self.state.borrow_mut().host)
        }

        #[wasm_bindgen(js_name = screenToWorld)]
        pub fn screen_to_world(&self, x: f64, y: f64) -> js_sys::Array {
            let (wx, wy) = dag_screen_to_world(&self.state.borrow().host, x, y);
            let out = js_sys::Array::new();
            out.push(&JsValue::from_f64(wx));
            out.push(&JsValue::from_f64(wy));
            out
        }

        #[wasm_bindgen(js_name = reorganize)]
        pub fn reorganize(&self, options_json: &str) -> Result<(), JsValue> {
            let opts = if options_json.trim().is_empty() { DagLayoutOptions::default() } else { serde_json::from_str(options_json).unwrap_or_default() };
            self.state.borrow_mut().host.reorganize(&opts).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = setCanvasThemeJson)]
        pub fn set_canvas_theme_json(&mut self, json: &str) {
            let _ = self.state.borrow_mut().host.set_canvas_theme_from_json(json);
        }

        #[wasm_bindgen(js_name = renderFrame)]
        pub fn render_frame(&self) -> Result<(), JsValue> {
            let mut inner = self.state.borrow_mut();
            let mut scene = cavas::Scene::new();
            let clear = inner.host.canvas_theme.raster_clear;
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
    fn app_instance_node_serializes_and_sizes_n_ports() {
        let node = DagNodeSpec {
            id: "node-a".into(),
            name: "Draw".into(),
            abbreviation: "drw".into(),
            icon: "emoji:draw".into(),
            x: 100.0,
            y: 80.0,
            width: 180.0,
            height: 92.0,
            kind: DagNodeKind::AppInstance {
                instance_id: "app-1".into(),
                program_id: "draw".into(),
                app_id: "draw".into(),
                icon: "emoji:draw".into(),
                inputs: vec![IoPortSpec::simple("in-a", "In")],
                outputs: vec![IoPortSpec::simple("out-a", "Out"), IoPortSpec::simple("out-b", "Mesh")],
            },
            ..Default::default()
        };
        assert_eq!(dag_node_kind_tag(&node.kind), "appInstance");
        assert_eq!(node.inputs().len(), 1);
        assert_eq!(node.outputs().len(), 2);
        let json = serde_json::to_string(&node).expect("serialize app instance");
        assert!(json.contains("appInstance"));
        assert!(json.contains("instanceId"));
        let mut sized = node.clone();
        fit_node_size(&mut sized);
        assert!(sized.height >= 56.0);
    }

    #[test]
    fn dag_selection_hover_and_dimmed_map_widget_ids() {
        let fixture = DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                DagNodeSpec::computation("a".into(), "A".into(), "A".into(), "emoji:🔷".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }], false, false, 0.0, 0.0, 160.0, 24.0),
                DagNodeSpec::computation(
                    "b".into(),
                    "B".into(),
                    "B".into(),
                    "emoji:🔷".into(),
                    vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }],
                    vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
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
            "schema": "dag.fixture",
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
            "schema": "dag.fixture",
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
            "schema": "dag.fixture",
            "nodes": [
                {"id": "a", "x": 0, "y": 0, "handles": []},
                {"id": "b", "x": 0, "y": 0, "handles": []}
            ],
            "edges": [{"id": "e1", "source": "a", "target": "b"}]
        });
        apply_dag_layout_to_fixture_v1_value(&mut fixture, &DagLayoutOptions::default()).unwrap();
        let default_gap = (fixture["nodes"][1]["x"].as_f64().unwrap() - fixture["nodes"][0]["x"].as_f64().unwrap()).abs();
        let mut wide: Value = fixture.clone();
        apply_dag_layout_to_fixture_v1_value(&mut wide, &DagLayoutOptions { layer_spacing: 240.0, sibling_gap: 80.0, ..DagLayoutOptions::default() }).unwrap();
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
                vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }],
                vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
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
                kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 3.0, output: IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() } },
                ..Default::default()
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
                kind: DagNodeKind::Select { options: vec!["A".into(), "B".into()], selected: 1, output: IoPortSpec { id: "out".into(), label: "mode".into(), ..Default::default() } },
                ..Default::default()
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
                kind: DagNodeKind::Screen { media: Some(DagMedia { kind: DagMediaKind::Svg, src: "data:image/svg+xml,test".into() }), input: IoPortSpec { id: "in".into(), label: "result".into(), ..Default::default() } },
                ..Default::default()
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
        let fixture = DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec::computation(
                "merge".into(),
                "Merge".into(),
                "M".into(),
                "emoji:🔀".into(),
                vec![IoPortSpec { id: "0".into(), label: "0".into(), ..Default::default() }],
                vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
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
        use cavas::Point;

        let mut host = DagHost::default_demo();
        host.set_viewport(800, 600, 1.0);
        let camera = Camera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
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
            kind: DagNodeKind::Slider { min: 0.0, max: 1.0, step: 0.1, value: 0.5, output: IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() } },
            ..Default::default()
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
            kind: DagNodeKind::Screen { media: None, input: IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() } },
            ..Default::default()
        };
        assert_eq!(screen.inputs().len(), 1);
        assert!(screen.outputs().is_empty());
    }

    #[test]
    fn dag_host_delete_selected_preserves_remaining_positions() {
        let mut host = DagHost::from_fixture_without_layout(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                DagNodeSpec::computation("a".into(), "A".into(), "A".into(), "emoji:🔷".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }], false, false, 100.0, 200.0, 160.0, 56.0),
                DagNodeSpec::computation(
                    "b".into(),
                    "B".into(),
                    "B".into(),
                    "emoji:🔷".into(),
                    vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }],
                    vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
                    false,
                    false,
                    400.0,
                    500.0,
                    160.0,
                    56.0,
                ),
                DagNodeSpec::computation("c".into(), "C".into(), "C".into(), "emoji:🔷".into(), vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }], vec![], false, false, 700.0, 300.0, 160.0, 56.0),
            ],
            edges: vec![DagFixtureEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() }, DagFixtureEdge { id: "e2".into(), source: "b@out".into(), target: "c@in".into(), ..Default::default() }],
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
        let mut host = DagHost::from_fixture_without_layout(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                DagNodeSpec::computation("a".into(), "A".into(), "A".into(), "emoji:🔷".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }], false, false, 100.0, 200.0, 160.0, 56.0),
                DagNodeSpec::computation(
                    "b".into(),
                    "B".into(),
                    "B".into(),
                    "emoji:🔷".into(),
                    vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }],
                    vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
                    false,
                    false,
                    400.0,
                    500.0,
                    160.0,
                    56.0,
                ),
                DagNodeSpec::computation("c".into(), "C".into(), "C".into(), "emoji:🔷".into(), vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }], vec![], false, false, 700.0, 300.0, 160.0, 56.0),
            ],
            edges: vec![DagFixtureEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() }, DagFixtureEdge { id: "e2".into(), source: "b@out".into(), target: "c@in".into(), ..Default::default() }],
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
        let mut host = DagHost::from_fixture_without_layout(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                DagNodeSpec::computation("a".into(), "A".into(), "A".into(), "emoji:🔷".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }], false, false, 500.0, 500.0, 160.0, 56.0),
                DagNodeSpec::computation("b".into(), "B".into(), "B".into(), "emoji:🔷".into(), vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }], vec![], false, false, 500.0, 500.0, 160.0, 56.0),
            ],
            edges: vec![DagFixtureEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() }],
        });
        host.reorganize(&DagLayoutOptions::default()).unwrap();
        let a = host.fixture.nodes.iter().find(|n| n.id == "a").expect("a");
        let b = host.fixture.nodes.iter().find(|n| n.id == "b").expect("b");
        assert!(b.x > a.x);
    }

    #[test]
    fn dag_host_loads_demo_fixture() {
        let host = DagHost::default_demo();
        assert_eq!(host.fixture.schema, "dag.fixture");
        assert_eq!(host.fixture.nodes.len(), 5);
        assert_eq!(host.fixture.edges.len(), 4);
        assert!(!host.engine.render_snapshot().edges.is_empty());
    }

    #[test]
    fn slider_track_bounds_stay_inside_node_rect() {
        let output = IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() };
        let node = DagNodeSpec {
            id: "slider".into(),
            name: "Amount".into(),
            abbreviation: "Amount".into(),
            icon: "emoji:🎚️".into(),
            x: 100.0,
            y: 50.0,
            width: slider_widget_width("Amount", &output),
            height: slider_widget_height(),
            kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 2.0, output },
            ..Default::default()
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
        let output = IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() };
        let mut host = DagHost::from_fixture_without_layout(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
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
                ..Default::default()
            }],
            edges: vec![],
        });
        host.set_viewport(800, 600, 1.0);
        let (x0, y0, x1, y1) = slider_track_bounds(&host.fixture.nodes[0]);
        let mid_y = (y0 + y1) * 0.5;
        let (sx, sy) = world_to_screen_px(&host, cavas::Point::new((x0 + x1) * 0.5, mid_y));
        host.pointer_down(sx, sy, false);
        host.pointer_up(sx, sy);
        let DagNodeKind::Slider { value, .. } = host.fixture.nodes[0].kind else {
            panic!("expected slider");
        };
        assert!((value - 2.0).abs() > 0.1);
    }

    #[test]
    fn dag_host_slider_drag_ignored_when_controls_hidden() {
        let output = IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() };
        let mut host = DagHost::from_fixture_without_layout(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
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
                ..Default::default()
            }],
            edges: vec![],
        });
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("minimap");
        host.set_viewport(800, 600, 1.0);
        let (x0, y0, x1, y1) = slider_track_bounds(&host.fixture.nodes[0]);
        let mid_y = (y0 + y1) * 0.5;
        let (sx, sy) = world_to_screen_px(&host, cavas::Point::new((x0 + x1) * 0.5, mid_y));
        host.pointer_down(sx, sy, false);
        host.pointer_up(sx, sy);
        let DagNodeKind::Slider { value, .. } = host.fixture.nodes[0].kind else {
            panic!("expected slider");
        };
        assert!((value - 2.0).abs() < 1e-6, "minimap LOD should only move the node rectangle, not adjust the value");
    }

    #[test]
    fn dag_host_select_click_advances_option() {
        let mut host = DagHost::from_fixture_without_layout(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec {
                id: "mode".into(),
                name: "Mode".into(),
                abbreviation: "Mode".into(),
                icon: "emoji:📋".into(),
                x: 0.0,
                y: 0.0,
                width: 180.0,
                height: 80.0,
                kind: DagNodeKind::Select { options: vec!["Add".into(), "Multiply".into()], selected: 0, output: IoPortSpec { id: "out".into(), label: "mode".into(), ..Default::default() } },
                ..Default::default()
            }],
            edges: vec![],
        });
        host.set_viewport(800, 600, 1.0);
        let (x0, y0, x1, y1) = select_control_bounds(&host.fixture.nodes[0]);
        let (sx, sy) = world_to_screen_px(&host, cavas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5));
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
        assert!(!labels[0]["text"].as_str().unwrap_or("").is_empty());
    }

    #[test]
    fn dag_host_label_overlay_paint_state_json_includes_slider_name() {
        let mut host = DagHost::from_fixture_without_layout(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 2.0 },
            nodes: vec![DagNodeSpec {
                id: "slider".into(),
                name: "Radius".into(),
                abbreviation: "Radius".into(),
                icon: "emoji:🎚️".into(),
                x: 0.0,
                y: 0.0,
                width: 120.0,
                height: 32.0,
                kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 3.0, output: IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() } },
                ..Default::default()
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
    fn dag_host_slider_overlay_state_json_includes_slider_track() {
        let mut host = DagHost::from_fixture_without_layout(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 2.0 },
            nodes: vec![DagNodeSpec {
                id: "slider".into(),
                name: "Radius".into(),
                abbreviation: "Radius".into(),
                icon: "emoji:🎚️".into(),
                x: 0.0,
                y: 0.0,
                width: 120.0,
                height: 32.0,
                kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 3.0, output: IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() } },
                ..Default::default()
            }],
            edges: vec![],
        });
        host.set_viewport(1280, 800, 1.0);
        let raw: serde_json::Value = serde_json::from_str(&host.slider_overlay_state_json().unwrap()).unwrap();
        let sliders = raw["sliders"].as_array().expect("sliders");
        assert_eq!(sliders.len(), 1);
        assert_eq!(sliders[0]["widgetId"], "slider");
        assert_eq!(sliders[0]["value"], 3.0);
        assert_eq!(sliders[0]["min"], 0.0);
        assert_eq!(sliders[0]["max"], 10.0);
    }

    #[test]
    fn label_overlay_port_rows_are_not_duplicated_in_json() {
        let mut host = DagHost::from_fixture_without_layout(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 2.0 },
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
                    inputs: vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "b".into(), ..Default::default() }],
                    outputs: vec![IoPortSpec { id: "out".into(), label: "merged".into(), ..Default::default() }],
                    variadic_inputs: false,
                    variadic_outputs: false,
                },
                ..Default::default()
            }],
            edges: vec![],
        });
        host.set_viewport(1280, 800, 1.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("normal");
        let raw: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
        let labels = raw["labels"].as_array().expect("labels");
        let port_rows: Vec<_> = labels.iter().filter(|row| row["kind"].as_str() == Some("port")).map(|row| (row["text"].as_str().unwrap_or(""), row["align"].as_str().unwrap_or(""))).collect();
        assert_eq!(port_rows.len(), 3);
        assert_eq!(port_rows.iter().filter(|(text, _)| *text == "! a").count(), 1);
        assert_eq!(port_rows.iter().filter(|(text, _)| *text == "! b").count(), 1);
        assert_eq!(port_rows.iter().filter(|(text, _)| *text == "! merged").count(), 1);
    }

    #[test]
    fn io_port_label_with_cardinality_prefixes_symbol() {
        let mut port = IoPortSpec::named("S", "Sld", "solid", "ExtrudedSolid");
        port.cardinality = "*".into();
        assert_eq!(port.label_with_cardinality(DagDrawLod::Normal), "* Sld");
    }

    #[test]
    fn io_port_display_label_follows_draw_lod() {
        let port = IoPortSpec::named("S", "Sld", "solid", "ExtrudedSolid");
        assert_eq!(port.display_label(DagDrawLod::Normal), "Sld");
        assert_eq!(port.display_label(DagDrawLod::Detail), "solid");
        assert_eq!(port.display_label(DagDrawLod::Micro), "ExtrudedSolid");
    }

    #[test]
    fn dag_host_label_overlay_port_text_follows_draw_lod() {
        let node = DagNodeSpec {
            id: "box".into(),
            name: "Box".into(),
            abbreviation: "Box".into(),
            icon: "emoji:📦".into(),
            x: 0.0,
            y: 0.0,
            width: 96.0,
            height: 42.0,
            kind: DagNodeKind::Computation { inputs: vec![IoPortSpec::named("W", "Wid", "width", "BoxWidth")], outputs: vec![IoPortSpec::named("S", "Sld", "solid", "BoxSolid")], variadic_inputs: false, variadic_outputs: false },
            ..Default::default()
        };
        let port_texts = |lod: &str| -> Vec<String> {
            let mut host = DagHost::from_fixture_without_layout(DagFixture { schema: "dag.fixture".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 2.0 }, nodes: vec![node.clone()], edges: vec![] });
            host.set_viewport(1280, 800, 1.0);
            host.set_automatic_lod(false);
            host.set_forced_draw_lod_label(lod);
            let raw: serde_json::Value = serde_json::from_str(&host.label_overlay_paint_state_json().unwrap()).unwrap();
            raw["labels"].as_array().expect("labels").iter().filter(|row| row["align"].as_str().is_some()).filter_map(|row| row["text"].as_str().map(str::to_string)).collect()
        };
        assert!(port_texts("normal").contains(&"! Wid".into()));
        assert!(port_texts("normal").contains(&"! Sld".into()));
        assert!(port_texts("detail").contains(&"! width".into()));
        assert!(port_texts("detail").contains(&"! solid".into()));
        assert!(port_texts("micro").contains(&"! BoxWidth".into()));
        assert!(port_texts("micro").contains(&"! BoxSolid".into()));
    }

    #[test]
    fn dag_host_exports_screen_overlay_rect() {
        let host = DagHost::from_fixture_without_layout(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec {
                id: "screen".into(),
                name: "Preview".into(),
                abbreviation: "Preview".into(),
                icon: "emoji:🖥️".into(),
                x: 100.0,
                y: 50.0,
                width: 200.0,
                height: 140.0,
                kind: DagNodeKind::Screen { media: Some(DagMedia { kind: DagMediaKind::Svg, src: "data:image/svg+xml,test".into() }), input: IoPortSpec { id: "in".into(), label: "result".into(), ..Default::default() } },
                ..Default::default()
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

    fn handle_world(host: &DagHost, port_key: &str) -> cavas::Point {
        let hid = host.handle_key_map.iter().find(|(_, key)| key.as_str() == port_key).map(|(id, _)| *id).expect("handle");
        host.engine.render_snapshot().handles.iter().find(|(id, _, _)| *id == hid).map(|(_, p, _)| *p).expect("handle pos")
    }

    fn world_to_screen_px(host: &DagHost, p: cavas::Point) -> (f64, f64) {
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
        host.pointer_down_screen(start_sx, start_sy, 0, false, false, false, false);
        host.pointer_move_screen(end_sx, end_sy, false, false, false);
        assert!(matches!(host.engine.interaction, InteractionMode::AreaSelect { .. }), "expected area-select after marquee threshold");
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
        host.set_selection(&["scale".into(), "combine".into(), "screen".into()]);
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
        host.set_selection(&["scale".into(), "combine".into(), "screen".into()]);
        host.align_selection("alignLeft").unwrap();
        let left_edges: Vec<f64> = host.selected_fixture_nodes().into_iter().map(|(_, node)| node.x - node.width * 0.5).collect();
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
    fn dag_host_entity_screen_json_resolves_node_by_id_and_wildcard() {
        let mut host = DagHost::default_demo();
        host.set_viewport(800, 600, 1.0);
        let json = host.entity_screen_json("node", "scale");
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["visible"], true);
        assert!(parsed["x"].is_number());
        assert!(parsed["rect"].is_array());

        let wildcard: serde_json::Value = serde_json::from_str(&host.entity_screen_json("node", "*")).unwrap();
        assert_eq!(wildcard["visible"], true);
    }

    #[test]
    fn dag_host_entity_screen_json_resolves_handle_by_widget_and_port() {
        let mut host = DagHost::default_demo();
        host.set_viewport(800, 600, 1.0);
        let input_json: serde_json::Value = serde_json::from_str(&host.entity_screen_json("handle", "scale@in")).unwrap();
        assert_eq!(input_json["visible"], true);
        let output_json: serde_json::Value = serde_json::from_str(&host.entity_screen_json("handle", "combine@b")).unwrap();
        assert_eq!(output_json["visible"], true);
        // 🐢 A malformed id (no "@port") or a port that doesn't exist on the node must degrade to
        // unresolved, never panic.
        let malformed: serde_json::Value = serde_json::from_str(&host.entity_screen_json("handle", "scale")).unwrap();
        assert_eq!(malformed["visible"], false);
        let missing_port: serde_json::Value = serde_json::from_str(&host.entity_screen_json("handle", "scale@nope")).unwrap();
        assert_eq!(missing_port["visible"], false);
    }

    #[test]
    fn dag_host_entity_screen_json_resolves_edge_with_a_two_point_polyline() {
        let mut host = DagHost::default_demo();
        host.set_viewport(800, 600, 1.0);
        let json: serde_json::Value = serde_json::from_str(&host.entity_screen_json("edge", "e1")).unwrap();
        assert_eq!(json["visible"], true);
        let polyline = json["polyline"].as_array().expect("edge geometry carries a polyline");
        assert_eq!(polyline.len(), 2);
    }

    #[test]
    fn dag_host_entity_screen_json_unresolved_domain_or_id_never_panics() {
        let mut host = DagHost::default_demo();
        host.set_viewport(800, 600, 1.0);
        for (domain, id) in [("node", "nonexistent"), ("handle", "*"), ("edge", "nonexistent"), ("bogus-domain", "*")] {
            let json: serde_json::Value = serde_json::from_str(&host.entity_screen_json(domain, id)).unwrap();
            if json["visible"] == true {
                continue; // "handle":"*" may legitimately resolve to the demo fixture's first port.
            }
            assert_eq!(json["visible"], false, "domain={domain} id={id}");
        }
        let empty_fixture = DagFixture { schema: "dag.fixture".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![], edges: vec![] };
        let empty: serde_json::Value = serde_json::from_str(&DagHost::from_fixture(empty_fixture).entity_screen_json("node", "*")).unwrap();
        assert_eq!(empty["visible"], false);
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
        let gap = cavas::Point::new(0.0, 0.0);
        use cavas::camera::{world_to_screen, Camera as CavasCamera, Viewport};
        let cam = CavasCamera { x: host.fixture.camera.x, y: host.fixture.camera.y, zoom: host.fixture.camera.zoom };
        let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
        let start = world_to_screen(&cam, &viewport, gap);
        host.pointer_down_screen(start.x, start.y, 0, false, false, false, false);
        assert!(matches!(host.engine.interaction, InteractionMode::DragNodes { .. }), "expected bounded drag inside selection union at minimap LOD");
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
            let grab = cavas::Point::new(node.center.x - node.width * 0.4, node.center.y);
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
    fn dag_host_grid_snap_aligns_dragged_node() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("normal");
        host.set_grid_snap_enabled(true);
        host.set_grid_factor(10.0).expect("grid factor");
        let mut dragged = false;
        for (nid, node) in host.engine.nodes.clone() {
            let grab = cavas::Point::new(node.center.x - node.width * 0.4, node.center.y);
            let (sx, sy) = world_to_screen_px(&host, grab);
            host.pointer_down(sx, sy, false);
            if !matches!(host.engine.interaction, InteractionMode::DragNode { node_id, .. } if node_id == nid) {
                host.pointer_up(sx, sy);
                continue;
            }
            host.pointer_move(sx + 37.0, sy + 23.0);
            host.pointer_up(sx + 37.0, sy + 23.0);
            let idx = *host.node_id_map.get(&nid).expect("fixture index");
            let fixture = &host.fixture.nodes[idx];
            let step = GRID_WORLD_MEDIUM * 10.0;
            assert!(((fixture.x / step).round() * step - fixture.x).abs() < 1e-6);
            assert!(((fixture.y / step).round() * step - fixture.y).abs() < 1e-6);
            dragged = true;
            break;
        }
        assert!(dragged, "expected draggable node for grid snap test");
    }

    #[test]
    fn dag_host_focus_selection_camera_frames_selection() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        let ids: Vec<String> = host.fixture.nodes.iter().take(2).map(|node| node.id.clone()).collect();
        host.set_selection(&ids);
        let camera = host.focus_selection_camera(1.2).expect("camera");
        assert!(camera.zoom > 0.0);
        assert!(camera.x.is_finite() && camera.y.is_finite());
    }

    #[test]
    fn dag_host_reconnects_edge_endpoint() {
        let mut host = DagHost::default_demo();
        host.set_viewport(1280, 800, 1.0);
        let in_w = handle_world(&host, "combine@b");
        let out_w = handle_world(&host, "scale@out");
        let (in_sx, in_sy) = world_to_screen_px(&host, in_w);
        let (out_sx, out_sy) = world_to_screen_px(&host, out_w);
        host.pointer_down(in_sx, in_sy, false);
        assert!(matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }));
        host.pointer_move(out_sx, out_sy);
        host.pointer_up(out_sx, out_sy);
        let e3 = host.fixture.edges.iter().find(|e| e.id == "e3").expect("e3");
        assert_eq!(e3.source, "scale@out");
    }

    #[test]
    fn dag_host_node_drag_proximity_preview_and_connects() {
        let inputs = vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
        let src_w = computation_node_width("Src", &[], &outputs);
        let tgt_w = computation_node_width("Tgt", &inputs, &outputs);
        let src_h = computation_node_height(0, 1, false, false);
        let tgt_h = computation_node_height(1, 1, false, false);
        let mut host = DagHost::from_fixture_without_layout(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                DagNodeSpec::computation("src".into(), "Src".into(), "Src".into(), "emoji:🔢".into(), vec![], outputs.clone(), false, false, 0.0, 0.0, src_w, src_h),
                DagNodeSpec::computation("tgt".into(), "Tgt".into(), "Tgt".into(), "emoji:🔢".into(), inputs, outputs, false, false, 220.0, 0.0, tgt_w, tgt_h),
            ],
            edges: vec![],
        });
        host.set_viewport(1280, 800, 1.0);
        host.set_proximity_distance(120.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("normal");
        let src_center = cavas::Point::new(0.0, 0.0);
        let (sx, sy) = world_to_screen_px(&host, src_center);
        host.pointer_down_screen(sx, sy, 0, false, false, false, false);
        host.pointer_move_screen(sx + 200.0, sy, false, false, false);
        assert!(host.engine.render_snapshot().pending_edge.is_some(), "proximity drag should preview edge");
        host.pointer_up_screen(sx + 200.0, sy, false, false, false);
        assert!(host.fixture.edges.iter().any(|edge| edge.source == "src@out" && edge.target == "tgt@in"), "proximity drag should commit edge");
    }

    #[test]
    fn dag_host_node_drag_skips_wired_cut_inputs() {
        let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "b".into(), ..Default::default() }];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
        let src_w = computation_node_width("Src", &[], &outputs);
        let cut_w = computation_node_width("Cut", &inputs, &outputs);
        let src_h = computation_node_height(0, 1, false, false);
        let cut_h = computation_node_height(2, 1, false, false);
        let mut host = DagHost::from_fixture_without_layout(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                DagNodeSpec::computation("sphere".into(), "Sphere".into(), "Sphere".into(), "emoji:🔵".into(), vec![], outputs.clone(), false, false, 0.0, -60.0, src_w, src_h),
                DagNodeSpec::computation("torus".into(), "Torus".into(), "Torus".into(), "emoji:🍩".into(), vec![], outputs.clone(), false, false, 0.0, 60.0, src_w, src_h),
                DagNodeSpec::computation("cut".into(), "Cut".into(), "Cut".into(), "emoji:✂️".into(), inputs, outputs, false, false, 240.0, 0.0, cut_w, cut_h),
            ],
            edges: vec![DagFixtureEdge { id: "e1".into(), source: "sphere@out".into(), target: "cut@a".into(), ..Default::default() }, DagFixtureEdge { id: "e2".into(), source: "torus@out".into(), target: "cut@b".into(), ..Default::default() }],
        });
        assert_eq!(host.engine.edges.len(), 2, "fixture edges should load into engine");
        host.set_viewport(1280, 800, 1.0);
        host.set_proximity_distance(160.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("normal");
        let cut_center = cavas::Point::new(240.0, 0.0);
        let (sx, sy) = world_to_screen_px(&host, cut_center);
        host.pointer_down_screen(sx, sy, 0, false, false, false, false);
        host.pointer_move_screen(sx - 180.0, sy, false, false, false);
        assert!(host.engine.render_snapshot().pending_edge.is_none(), "dragging wired cut near sources must not preview proximity edges to occupied inputs");
        host.pointer_up_screen(sx - 180.0, sy, false, false, false);
        assert_eq!(host.engine.edges.len(), 2);
        assert_eq!(host.fixture.edges.len(), 2);
    }

    #[test]
    fn dag_host_keeps_same_named_input_and_output_handles_distinct() {
        let solid = vec![IoPortSpec { id: "solid".into(), label: "solid".into(), ..Default::default() }];
        let brep = vec![IoPortSpec { id: "brep".into(), label: "brep".into(), ..Default::default() }];
        let list = vec![IoPortSpec { id: "list".into(), label: "list".into(), ..Default::default() }];
        let mut host = DagHost::from_fixture_without_layout(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                DagNodeSpec::computation(
                    "extrude".into(),
                    "Extrude".into(),
                    "Extrude".into(),
                    "emoji:⬆️".into(),
                    vec![],
                    solid.clone(),
                    false,
                    false,
                    0.0,
                    0.0,
                    computation_node_width("Extrude", &[], &solid),
                    computation_node_height(0, 1, false, false),
                ),
                DagNodeSpec::computation("brep".into(), "Brep".into(), "Brep".into(), "emoji:🧊".into(), brep.clone(), brep.clone(), false, false, 200.0, 0.0, computation_node_width("Brep", &brep, &brep), computation_node_height(1, 1, false, false)),
                DagNodeSpec::computation(
                    "get".into(),
                    "Get".into(),
                    "Get".into(),
                    "emoji:📋".into(),
                    list,
                    vec![],
                    false,
                    false,
                    400.0,
                    0.0,
                    computation_node_width("Get", &[IoPortSpec { id: "list".into(), label: "list".into(), ..Default::default() }], &[]),
                    computation_node_height(1, 0, false, false),
                ),
            ],
            edges: vec![
                DagFixtureEdge { id: "e100".into(), source: "extrude@solid".into(), target: "brep@brep".into(), ..Default::default() },
                DagFixtureEdge { id: "e101".into(), source: "brep@brep".into(), target: "get@list".into(), ..Default::default() },
            ],
        });
        host.sync_edges_from_engine();
        let incoming = host.engine.edges.get(&100).expect("incoming brep edge");
        let outgoing = host.engine.edges.get(&101).expect("outgoing brep edge");
        let incoming_target = host.engine.handles.get(&incoming.target).expect("incoming target handle");
        let outgoing_source = host.engine.handles.get(&outgoing.source).expect("outgoing source handle");
        assert_eq!(incoming_target.role, HandleRole::Target);
        assert_eq!(outgoing_source.role, HandleRole::Source);
        assert_eq!(host.fixture.edges.iter().find(|edge| edge.id == "e100").map(|edge| edge.target.as_str()), Some("brep@brep"));
        assert_eq!(host.fixture.edges.iter().find(|edge| edge.id == "e101").map(|edge| edge.source.as_str()), Some("brep@brep"));
    }

    #[test]
    fn dag_host_proximity_zero_disables_node_drag_connect() {
        let inputs = vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
        let src_w = computation_node_width("Src", &[], &outputs);
        let tgt_w = computation_node_width("Tgt", &inputs, &outputs);
        let src_h = computation_node_height(0, 1, false, false);
        let tgt_h = computation_node_height(1, 1, false, false);
        let mut host = DagHost::from_fixture_without_layout(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![
                DagNodeSpec::computation("src".into(), "Src".into(), "Src".into(), "emoji:🔢".into(), vec![], outputs.clone(), false, false, 0.0, 0.0, src_w, src_h),
                DagNodeSpec::computation("tgt".into(), "Tgt".into(), "Tgt".into(), "emoji:🔢".into(), inputs, outputs, false, false, 220.0, 0.0, tgt_w, tgt_h),
            ],
            edges: vec![],
        });
        host.set_viewport(1280, 800, 1.0);
        host.set_proximity_distance(0.0);
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label("normal");
        let (sx, sy) = world_to_screen_px(&host, cavas::Point::new(0.0, 0.0));
        host.pointer_down_screen(sx, sy, 0, false, false, false, false);
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
        let row_center = cavas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let handle = handle_world(&host, "combine@b");
        for lod in ["minimap", "overview", "compact"] {
            host.set_forced_draw_lod_label(lod);
            assert!(!host.draw_lod_for_frame().allows_connection_hit_picking(), "{lod}");
            let (sx, sy) = world_to_screen_px(&host, row_center);
            host.pointer_down(sx, sy, false);
            assert!(!matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }), "{lod} input row should not start edge draw");
            host.pointer_up(sx, sy);
            let (hsx, hsy) = world_to_screen_px(&host, handle);
            host.pointer_down(hsx, hsy, false);
            assert!(!matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }), "{lod} handle anchor should not start edge draw");
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
        let row_center = cavas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let handle = handle_world(&host, "combine@b");
        assert!((row_center.x - handle.x).abs() > 4.0, "row center should sit away from the painted handle anchor");
        let (sx, sy) = world_to_screen_px(&host, row_center);
        host.pointer_down(sx, sy, false);
        assert!(matches!(host.engine.interaction, InteractionMode::DragNode { .. }), "interior rectangle drag should move the node");
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
        assert_eq!(host.hovered_channel(), Some(DagChannelRef { widget_id: "combine".into(), port: "b".into(), direction: "in".into() }));
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
        let row_center = cavas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let (sx, sy) = world_to_screen_px(&host, row_center);
        host.pointer_move_screen(sx, sy, false, false, false);
        assert_eq!(host.hovered_channel(), Some(DagChannelRef { widget_id: "combine".into(), port: "b".into(), direction: "in".into() }));
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
        let row_center = cavas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let (sx, sy) = world_to_screen_px(&host, row_center);
        host.pointer_move_screen(sx, sy, false, false, false);
        assert!(host.hovered_node_id().as_deref() == Some("combine"));
        assert!(host.engine.hover.is_some());
        assert!(!host.engine.selection.node_ids.contains(&host.node_id_for_widget_id("combine").expect("combine node id")));
        let divider_x = computation_column_divider_x(&combine).expect("divider");
        let (_, header_top, _, header_bottom) = channel_row_bounds(&combine, 0);
        let title_probe = cavas::Point::new(divider_x, (header_top + header_bottom) * 0.5);
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
        let row_center = cavas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let handle = handle_world(&host, "scale@out");
        assert!((row_center.x - handle.x).abs() > 4.0, "row center should sit away from the painted handle anchor");
        let (sx, sy) = world_to_screen_px(&host, row_center);
        host.pointer_down(sx, sy, false);
        assert!(!matches!(host.engine.interaction, InteractionMode::DrawEdge { .. }), "visible handles require anchor hit for wire draw");
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
        let row_center = cavas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let handle = handle_world(&host, "combine@b");
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
        let title_probe = cavas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
        let (sx, sy) = world_to_screen_px(&host, title_probe);
        host.pointer_down(sx, sy, false);
        assert!(matches!(host.engine.interaction, InteractionMode::DragNode { .. }));
    }

    #[test]
    fn input_port_row_hit_bounds_span_input_channel() {
        let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "b".into(), ..Default::default() }];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
        let width = computation_node_width("Node", &inputs, &outputs);
        let height = computation_node_height(2, 1, false, false);
        let node = DagNodeSpec::computation("n".into(), "Node".into(), "Node".into(), "emoji:🔢".into(), inputs, outputs, false, false, 0.0, 0.0, width, height);
        let hw = width * 0.5;
        let divider_x = computation_column_divider_x(&node).expect("divider");
        let (x0, _, x1, _) = input_port_row_hit_bounds(&node, 1).expect("row");
        assert!((x0 - (node.x - hw)).abs() < 1e-9);
        assert!((x1 - divider_x).abs() < 1e-9);
    }

    #[test]
    fn output_port_row_hit_bounds_span_output_channel() {
        let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }];
        let outputs = vec![IoPortSpec { id: "x".into(), label: "x".into(), ..Default::default() }, IoPortSpec { id: "y".into(), label: "y".into(), ..Default::default() }];
        let width = computation_node_width("Node", &inputs, &outputs);
        let height = computation_node_height(1, 2, false, false);
        let node = DagNodeSpec::computation("n".into(), "Node".into(), "Node".into(), "emoji:🔢".into(), inputs, outputs, false, false, 0.0, 0.0, width, height);
        let hw = width * 0.5;
        let divider_x = computation_column_divider_x(&node).expect("divider");
        let (x0, _, x1, _) = output_port_row_hit_bounds(&node, 1).expect("row");
        assert!((x0 - divider_x).abs() < 1e-9);
        assert!((x1 - (node.x + hw)).abs() < 1e-9);
    }

    #[test]
    fn variadic_plus_hit_maps_insert_index() {
        let inputs = vec![IoPortSpec { id: "0".into(), label: "0".into(), ..Default::default() }, IoPortSpec { id: "1".into(), label: "1".into(), ..Default::default() }];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
        let width = computation_node_width("dictionary.merge", &inputs, &outputs);
        let height = computation_node_height(2, 1, true, false);
        let host = DagHost::from_fixture_without_layout(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 2.0 },
            nodes: vec![DagNodeSpec::computation("merge".into(), "Merge".into(), "Merge".into(), "emoji:🔀".into(), inputs, outputs, true, false, 0.0, 0.0, width, height)],
            edges: vec![],
        });
        let positions = variadic_input_insert_positions(&host.fixture.nodes[0]);
        assert_eq!(positions.len(), 1);
        let (_, px, py) = positions[0];
        let hit = host.port_insert_hit(px, py, 2.0).expect("hit");
        assert_eq!(hit.0, DagPortSide::Input);
        assert_eq!(hit.1, "merge");
        assert_eq!(hit.2, 2);
        assert!(host.port_insert_hit(px, py, 1.0).is_none());
    }

    #[test]
    fn variadic_output_plus_hit_maps_insert_index() {
        let outputs = vec![IoPortSpec { id: "0".into(), label: "i".into(), ..Default::default() }];
        let inputs = vec![IoPortSpec { id: "list".into(), label: "list".into(), ..Default::default() }, IoPortSpec { id: "index".into(), label: "index".into(), ..Default::default() }];
        let width = computation_node_width("list.get", &inputs, &outputs);
        let height = computation_node_height(2, 1, false, true);
        let host = DagHost::from_fixture_without_layout(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 2.0 },
            nodes: vec![DagNodeSpec::computation("get".into(), "Get".into(), "Get".into(), "emoji:📋".into(), inputs, outputs, false, true, 0.0, 0.0, width, height)],
            edges: vec![],
        });
        let positions = variadic_output_insert_positions(&host.fixture.nodes[0]);
        assert_eq!(positions.len(), 1);
        let (_, px, py) = positions[0];
        let hit = host.port_insert_hit(px, py, 2.0).expect("hit");
        assert_eq!(hit.0, DagPortSide::Output);
        assert_eq!(hit.1, "get");
        assert_eq!(hit.2, 1);
    }

    #[test]
    fn component_width_is_twice_channel_width() {
        assert_eq!(DAG_COMPONENT_WIDTH, DAG_IO_COLUMN_WIDTH * 2.0);
    }

    #[test]
    fn computation_node_width_is_uniform_for_all_components() {
        let inputs = vec![
            IoPortSpec { id: "cornerA".into(), label: "cornerA".into(), ..Default::default() },
            IoPortSpec { id: "cornerB".into(), label: "cornerB".into(), ..Default::default() },
            IoPortSpec { id: "height".into(), label: "height".into(), ..Default::default() },
        ];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }];
        let width = computation_node_width("Box", &inputs, &outputs);
        assert_eq!(width, DAG_COMPONENT_WIDTH);
    }

    #[test]
    fn computation_io_columns_use_uniform_channel_width() {
        let inputs_short = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }];
        let inputs_long = vec![IoPortSpec { id: "cornerA".into(), label: "cornerA".into(), ..Default::default() }, IoPortSpec { id: "cornerB".into(), label: "cornerB".into(), ..Default::default() }];
        let outputs_short = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
        let outputs_long = vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }];
        let short = computation_node_width("n", &inputs_short, &outputs_short);
        let long = computation_node_width("n", &inputs_long, &outputs_long);
        assert_eq!(short, DAG_COMPONENT_WIDTH);
        assert_eq!(long, DAG_COMPONENT_WIDTH);
        assert_eq!(io_port_column_width(&inputs_long, DAG_LABEL_COMPACT_SCREEN_PX), DAG_IO_COLUMN_WIDTH);
        assert_eq!(io_port_column_width(&outputs_long, DAG_LABEL_COMPACT_SCREEN_PX), DAG_IO_COLUMN_WIDTH);
    }

    #[test]
    fn computation_column_divider_splits_io_columns() {
        let inputs = vec![IoPortSpec { id: "cornerA".into(), label: "cornerA".into(), ..Default::default() }, IoPortSpec { id: "cornerB".into(), label: "cornerB".into(), ..Default::default() }];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }];
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
        let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
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
    fn io_widget_width_matches_component_width() {
        let width = io_widget_width("Amount");
        let height = io_widget_height("Amount");
        assert_eq!(width, DAG_COMPONENT_WIDTH);
        assert!(height >= 40.0);
    }

    #[test]
    fn slider_widget_size_matches_function_row_metrics() {
        let input = IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() };
        let output = IoPortSpec { id: "out".into(), label: "value".into(), ..Default::default() };
        let width = slider_widget_width("Amount", &output);
        let height = slider_widget_height();
        assert_eq!(width, DAG_COMPONENT_WIDTH);
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
                IoPortSpec { id: "cornerA".into(), label: "cornerA".into(), ..Default::default() },
                IoPortSpec { id: "cornerB".into(), label: "cornerB".into(), ..Default::default() },
                IoPortSpec { id: "height".into(), label: "height".into(), ..Default::default() },
            ],
            vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }],
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
        let three_inputs =
            vec![IoPortSpec { id: "a".into(), label: "cornerA".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "cornerB".into(), ..Default::default() }, IoPortSpec { id: "c".into(), label: "height".into(), ..Default::default() }];
        let one_output = vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }];
        let three_outputs = vec![
            IoPortSpec { id: "outA".into(), label: "geometry".into(), ..Default::default() },
            IoPortSpec { id: "outB".into(), label: "mesh".into(), ..Default::default() },
            IoPortSpec { id: "outC".into(), label: "curve".into(), ..Default::default() },
        ];
        let one_input = vec![IoPortSpec { id: "a".into(), label: "cornerA".into(), ..Default::default() }];
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
        let inputs =
            vec![IoPortSpec { id: "a".into(), label: "cornerA".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "cornerB".into(), ..Default::default() }, IoPortSpec { id: "c".into(), label: "height".into(), ..Default::default() }];
        let outputs = vec![IoPortSpec { id: "outA".into(), label: "geometry".into(), ..Default::default() }, IoPortSpec { id: "outB".into(), label: "mesh".into(), ..Default::default() }];
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
        let divider_x = computation_column_divider_x(&node).expect("divider");
        let (input_span_left, input_span_right) = computation_channel_row_divider_x_span(&node, ComputationChannelRowSide::Input);
        let (output_span_left, output_span_right) = computation_channel_row_divider_x_span(&node, ComputationChannelRowSide::Output);
        assert!((input_span_left - (node.x - width * 0.5)).abs() < 1e-6);
        assert!((input_span_right - divider_x).abs() < 1e-6);
        assert!((output_span_left - divider_x).abs() < 1e-6);
        assert!((output_span_right - (node.x + width * 0.5)).abs() < 1e-6);
        assert!((divider_x - node.x).abs() < 1e-6, "2× channel width nodes split IO at center");
        let divider_y = channel_row_divider_y(node.y, node.height, 1);
        let (_, _row0_top, _, row0_bottom) = channel_row_bounds(&node, 0);
        let (_, row1_top, _, _row1_bottom) = channel_row_bounds(&node, 1);
        assert!((divider_y - row0_bottom).abs() < 1e-6);
        assert!((divider_y - row1_top).abs() < 1e-6);
    }

    #[test]
    fn computation_node_size_fits_io_labels() {
        let inputs = vec![
            IoPortSpec { id: "width".into(), label: "width".into(), ..Default::default() },
            IoPortSpec { id: "depth".into(), label: "depth".into(), ..Default::default() },
            IoPortSpec { id: "height".into(), label: "height".into(), ..Default::default() },
        ];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "geometry".into(), ..Default::default() }];
        let width = computation_node_width("brep.prim3d.box", &inputs, &outputs);
        let height = computation_node_height(3, 1, false, false);
        assert!(height <= 42.0, "expected compact height, got {height}");
        assert!(height < 96.0, "expected shorter than legacy 4-row layout");
        assert_eq!(width, DAG_COMPONENT_WIDTH);
    }

    #[test]
    fn io_node_rect_port_angles_on_edges() {
        use cavas::Point;
        use graph::handle_position_on_rectangle;
        let inputs = vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }, IoPortSpec { id: "b".into(), label: "b".into(), ..Default::default() }];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
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
        use cavas::Point;
        use graph::{handle_exterior_cap_fill_path, handle_outward_at_node_rim, handle_position_on_rectangle, NodeShape};
        let inputs = vec![IoPortSpec { id: "0".into(), label: "0".into(), ..Default::default() }, IoPortSpec { id: "1".into(), label: "1".into(), ..Default::default() }];
        let outputs = vec![IoPortSpec { id: "out".into(), label: "dictionary".into(), ..Default::default() }];
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
        assert!(left_cap.bounding_box().x0() < left_pos.x - 1.0, "left input cap must bulge outside the west edge");
        assert!(right_cap.bounding_box().x1() > right_pos.x + 1.0, "right output cap must bulge outside the east edge");
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
        assert!(DagDrawLod::Normal.shows_port_labels());
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
    fn pointer_pan_gesture_moves_camera() {
        let mut host = DagHost::default_demo();
        host.set_camera(0.0, 0.0, 2.0);
        host.pointer_down_screen(100.0, 100.0, 1, false, false, false, true);
        host.pointer_move_screen(150.0, 100.0, false, false, false);
        assert!((host.fixture.camera.x - -25.0).abs() < 1e-6);
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
        let mut scene = cavas::Scene::new();
        host.paint_scene(&mut scene, 1280, 800, 1.0);
        assert!(scene.path_count() > 12, "compact LOD at low zoom should still paint abbreviation labels");
    }

    #[test]
    fn dag_label_colors_use_theme_label_fields() {
        use cavas::Color;
        let theme = CanvasPalette { label_fill: Color::from_rgba8(240, 241, 245, 255), label_halo: Color::from_rgba8(10, 12, 16, 180), node_stroke: Color::from_rgba8(90, 100, 110, 255), ..CanvasPalette::default() };
        assert_ne!(theme.label_fill.to_rgba8(), theme.node_stroke.to_rgba8());
    }

    #[test]
    fn dag_node_paint_fill_matches_puzzle2d_lod_chrome() {
        use cavas::Color;
        let theme = CanvasPalette {
            node_fill: Color::from_rgba8(10, 20, 30, 255),
            node_stroke: Color::from_rgba8(200, 210, 220, 255),
            node_fill_hovered: Color::from_rgba8(40, 50, 60, 255),
            node_stroke_hovered: Color::from_rgba8(90, 100, 110, 255),
            node_fill_selected: Color::from_rgba8(70, 80, 90, 255),
            node_stroke_selected: Color::from_rgba8(120, 130, 140, 255),
            node_fill_selection_exit: Color::from_rgba8(196, 228, 213, 255),
            node_stroke_selection_exit: Color::from_rgba8(80, 140, 110, 255),
            ..CanvasPalette::default()
        };
        assert_eq!(dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, false, false, false).expect("minimap neutral").to_rgba8(), theme.node_stroke.to_rgba8());
        assert!(dag_node_paint_fill(DagDrawLod::Overview, &theme, false, false, false, false).is_none());
        assert!(dag_node_paint_fill(DagDrawLod::Normal, &theme, false, false, false, false).is_none());
        assert_eq!(dag_node_paint_fill(DagDrawLod::Normal, &theme, false, true, false, false).expect("selected").to_rgba8(), theme.node_fill_selected.to_rgba8());
        assert_eq!(dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, false, false, true).expect("minimap hovered").to_rgba8(), theme.node_stroke_hovered.to_rgba8());
        assert_eq!(dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, false, true, false).expect("minimap highlighted").to_rgba8(), theme.node_stroke_selection_exit.to_rgba8());
        assert_eq!(dag_node_body_stroke(&theme, false, true, false, true).to_rgba8(), theme.node_stroke_selected.to_rgba8());
        assert_eq!(dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, true, false, true).expect("minimap selected").to_rgba8(), theme.node_stroke_selected.to_rgba8());
        assert_ne!(dag_node_paint_fill(DagDrawLod::Minimap, &theme, false, false, false, true).expect("minimap hovered").to_rgba8(), theme.node_fill_hovered.to_rgba8());
        assert_eq!(dag_node_body_stroke(&theme, false, false, false, true).to_rgba8(), theme.node_stroke_hovered.to_rgba8());
        assert_eq!(dag_node_body_stroke(&theme, false, false, true, false).to_rgba8(), theme.node_stroke_selection_exit.to_rgba8());
        assert_eq!(dag_node_label_fill(&theme, false, false, false, true).to_rgba8(), theme.label_fill_hovered.to_rgba8());
        assert_eq!(dag_node_label_fill(&theme, false, false, true, false).to_rgba8(), theme.node_stroke_selection_exit.to_rgba8());
        assert_eq!(dag_node_label_fill(&theme, false, false, false, false).to_rgba8(), theme.label_fill.to_rgba8());
        assert_eq!(dag_node_label_fill(&theme, false, true, false, false).to_rgba8(), theme.label_fill_hovered.to_rgba8());
        let body = dag_node_body_stroke(&theme, false, false, false, false);
        let label = dag_node_label_fill(&theme, false, false, false, true);
        assert_eq!(dag_node_internal_chrome_stroke(body, label, true).to_rgba8(), label.to_rgba8());
        assert_eq!(dag_node_internal_chrome_stroke(body, label, false).to_rgba8(), body.to_rgba8());
        let body_selected = dag_node_body_stroke(&theme, false, true, false, false);
        let label_selected = dag_node_label_fill(&theme, false, true, false, false);
        assert_eq!(dag_node_internal_chrome_stroke(body_selected, label_selected, true).to_rgba8(), label_selected.to_rgba8());
        assert_eq!(dag_node_internal_chrome_stroke(body_selected, label_selected, false).to_rgba8(), body_selected.to_rgba8());
    }

    #[test]
    fn dag_handle_and_edge_stroke_use_theme_defaults() {
        use cavas::Color;
        let theme = CanvasPalette {
            edge_stroke: Color::from_rgba8(100, 110, 120, 255),
            edge_stroke_hovered: Color::from_rgba8(10, 20, 30, 255),
            edge_stroke_selected: Color::from_rgba8(40, 50, 60, 255),
            handle_stroke: Color::from_rgba8(130, 140, 150, 255),
            handle_stroke_hovered: Color::from_rgba8(20, 30, 40, 255),
            handle_stroke_selected: Color::from_rgba8(50, 60, 70, 255),
            ..CanvasPalette::default()
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
    fn note_widget_size_uses_uniform_component_width() {
        let short = note_widget_size("hi");
        let long = note_widget_size("some longer note text");
        assert_eq!(short.0, DAG_COMPONENT_WIDTH);
        assert_eq!(long.0, DAG_COMPONENT_WIDTH);
        assert_eq!(short.1, DAG_CHANNEL_ROW_HEIGHT);
        assert_eq!(long.1, DAG_CHANNEL_ROW_HEIGHT);
    }

    #[test]
    fn fit_note_sizes_keeps_slider_height() {
        let mut host = DagHost::from_fixture(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec {
                id: "note".into(),
                name: "Note".into(),
                abbreviation: "Note".into(),
                icon: "emoji:📝".into(),
                x: 0.0,
                y: 0.0,
                width: 80.0,
                height: 80.0,
                kind: DagNodeKind::Note { text: "hi".into(), output: IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() } },
                ..Default::default()
            }],
            edges: vec![],
        });
        host.fit_note_sizes();
        assert_eq!(host.fixture.nodes[0].height, DAG_CHANNEL_ROW_HEIGHT);
        let DagNodeKind::Note { text, .. } = &mut host.fixture.nodes[0].kind else {
            panic!("expected note");
        };
        *text = "a much longer note body".into();
        host.fit_note_sizes();
        assert_eq!(host.fixture.nodes[0].height, DAG_CHANNEL_ROW_HEIGHT);
        assert_eq!(host.fixture.nodes[0].width, DAG_COMPONENT_WIDTH);
    }

    #[test]
    fn truncate_label_to_fit_width_adds_ellipsis() {
        let px = 12.0;
        let max_w = 40.0;
        let truncated = truncate_label_to_fit_width("alpha beta gamma delta", max_w, px);
        assert!(truncated.ends_with('…'));
        assert!(truncated.len() < "alpha beta gamma delta".len());
    }

    #[test]
    fn begin_note_edit_inserts_and_backspaces_text() {
        let mut host = DagHost::from_fixture(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec {
                id: "note".into(),
                name: "Note".into(),
                abbreviation: "Note".into(),
                icon: "emoji:📝".into(),
                x: 0.0,
                y: 0.0,
                width: note_widget_size("hi").0,
                height: note_widget_size("hi").1,
                kind: DagNodeKind::Note { text: "hi".into(), output: IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() } },
                ..Default::default()
            }],
            edges: vec![],
        });
        let origin_x = note_text_origin_x(&host.fixture.nodes[0]);
        assert!(host.begin_note_edit("note", origin_x + 100.0, 0.0));
        assert_eq!(host.editing_note_id(), Some("note"));
        assert!(host.note_insert_text("!"));
        {
            let DagNodeKind::Note { text, .. } = &host.fixture.nodes[0].kind else {
                panic!("expected note");
            };
            assert_eq!(text, "hi!");
        }
        assert!(host.note_backspace());
        {
            let DagNodeKind::Note { text, .. } = &host.fixture.nodes[0].kind else {
                panic!("expected note");
            };
            assert_eq!(text, "hi");
        }
        host.note_commit_edit();
        assert_eq!(host.editing_note_id(), None);
    }

    #[test]
    fn note_label_overlay_skips_title_and_ports() {
        let mut host = DagHost::from_fixture(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec {
                id: "note".into(),
                name: "Note".into(),
                abbreviation: "Note".into(),
                icon: "emoji:📝".into(),
                x: 0.0,
                y: 0.0,
                width: note_widget_size("hello").0,
                height: note_widget_size("hello").1,
                kind: DagNodeKind::Note { text: "hello".into(), output: IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() } },
                ..Default::default()
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
            kind: DagNodeKind::Note { text: "hi".into(), output: IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() } },
            ..Default::default()
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
            kind: DagNodeKind::Preview { content: DagPreviewContent::Scalar { text: "3".into() }, expanded: BTreeSet::new(), input: IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() } },
            ..Default::default()
        };
        assert_eq!(preview.inputs().len(), 1);
        assert!(preview.outputs().is_empty());
        let export = DagNodeSpec {
            id: "export".into(),
            name: "Export SVG".into(),
            abbreviation: "SVG".into(),
            icon: "emoji:📤".into(),
            x: 0.0,
            y: 0.0,
            width: 120.0,
            height: 48.0,
            kind: DagNodeKind::Export { label: "SVG".into(), format: "svg".into(), input: IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() } },
            ..Default::default()
        };
        assert_eq!(export.inputs().len(), 1);
        assert!(export.outputs().is_empty());
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
            vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }],
            vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
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
        let node = DagNodeSpec::computation("n".into(), "pass through".into(), "pass".into(), "emoji:➡️".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }], false, false, 0.0, 0.0, 80.0, 24.0);
        let json = serde_json::to_string(&node).unwrap();
        let back: DagNodeSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(back.name, "PassThrough");
        assert_eq!(back.abbreviation, "Pass");
        assert_eq!(back.icon, "emoji:➡️");
    }

    #[test]
    fn preview_tree_toggle_expands_and_resizes() {
        let json = serde_json::json!({ "alpha": { "beta": 1 }, "gamma": "x" });
        let mut host = DagHost::from_fixture(DagFixture {
            schema: "dag.fixture".into(),
            camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            nodes: vec![DagNodeSpec {
                id: "preview".into(),
                name: "Preview".into(),
                abbreviation: "Preview".into(),
                icon: "emoji:👁️".into(),
                x: 0.0,
                y: 0.0,
                width: 80.0,
                height: 80.0,
                kind: DagNodeKind::Preview { content: DagPreviewContent::Tree { json }, expanded: BTreeSet::new(), input: IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() } },
                ..Default::default()
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
        use cavas::Point;
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
        let mut scene = cavas::Scene::new();
        host.fixture.camera.zoom = 0.3;
        host.paint_scene(&mut scene, 1280, 800, 1.0);
        host.fixture.camera.zoom = 5.0;
        host.paint_scene(&mut scene, 1280, 800, 1.0);
    }

    #[test]
    fn cluster_node_round_trips_serde() {
        let inputs = vec![IoPortSpec::simple("a", "a")];
        let outputs = vec![IoPortSpec::simple("out", "out")];
        let node = DagNodeSpec::cluster("cluster".into(), "Cluster".into(), "Cluster".into(), "emoji:🧩".into(), inputs, outputs, 10.0, 20.0, 120.0, 80.0);
        let json = serde_json::to_string(&node).unwrap();
        let back: DagNodeSpec = serde_json::from_str(&json).unwrap();
        assert!(matches!(back.kind, DagNodeKind::Cluster { .. }));
    }

    #[test]
    fn cluster_explode_hit_rect_detects_top_right_affordance() {
        let inputs = vec![IoPortSpec::simple("a", "a")];
        let outputs = vec![IoPortSpec::simple("out", "out")];
        let node = DagNodeSpec::cluster("cluster".into(), "Cluster".into(), "Cluster".into(), "emoji:🧩".into(), inputs, outputs, 0.0, 0.0, 120.0, 80.0);
        let (x0, y0, x1, y1) = cluster_explode_hit_rect(&node).expect("rect");
        assert!(cluster_explode_hit(&node, (x0 + x1) * 0.5, (y0 + y1) * 0.5));
        assert!(!cluster_explode_hit(&node, node.x - 50.0, node.y - 50.0));
    }
}
// #endregion 🔖Tests

// #region 🔖DocumentVcs
use protocol::{collection_diff_from_operation, invert_collection_operation, CollectionDiff, CollectionOperation, Identified, Operation, OperationDiff, OpText, Patchable};
use vcs::{DocumentVcsEnvelope, DocumentVcsStore};
#[cfg(any(test, target_arch = "wasm32"))]
use vcs::create_document_vcs_envelope;
#[cfg(test)]
use vcs::DocumentVcsCommand;

pub const DAG_DOCUMENT_SCHEMA: &str = "dag.fixture";

fn dag_document_schema() -> String {
    DAG_DOCUMENT_SCHEMA.into()
}

/// 🧾 The persistent DAG projection — nodes and edges only. Camera/viewport and selection are
/// ephemeral view state kept in the plugin runtime, never recorded in the document's undo history.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagDocument {
    #[serde(default = "dag_document_schema")]
    pub schema: String,
    #[serde(default)]
    pub nodes: Vec<DagNodeSpec>,
    #[serde(default)]
    pub edges: Vec<DagFixtureEdge>,
}

pub fn empty_dag_document() -> DagDocument {
    DagDocument { schema: DAG_DOCUMENT_SCHEMA.into(), nodes: Vec::new(), edges: Vec::new() }
}

/// 🌱 The demo document seed (nodes/edges from `demo.dag`), sharing the fixture's example.
pub fn default_dag_document() -> DagDocument {
    dag_document_from_fixture(&DagFixture::default())
}

/// 🔁 Projects a {@link DagFixture} (which also carries a camera) down to the persistent {@link DagDocument}.
pub fn dag_document_from_fixture(fixture: &DagFixture) -> DagDocument {
    DagDocument { schema: fixture.schema.clone(), nodes: fixture.nodes.clone(), edges: fixture.edges.clone() }
}

/// 🔁 Rehydrates a full {@link DagFixture} from a {@link DagDocument} plus a runtime `camera`, so the
/// existing fixture-shaped helpers (`dag_fixture_to_wire_literal`, `DagHost`, …) can be reused.
pub fn dag_fixture_from_document(document: &DagDocument, camera: DagCamera) -> DagFixture {
    DagFixture { schema: document.schema.clone(), camera, nodes: document.nodes.clone(), edges: document.edges.clone() }
}

//#region 🔖CollectionSupport
impl Identified<String> for DagNodeSpec {
    fn id(&self) -> &String {
        &self.id
    }
}

impl Identified<String> for DagFixtureEdge {
    fn id(&self) -> &String {
        &self.id
    }
}

/// 🩹 Sparse patch of a {@link DagNodeSpec} — layout fields plus a whole-`kind` replacement for
/// kind-specific edits (slider value/min/max, note text, …).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagNodePatch {
    pub name: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub kind: Option<DagNodeKind>,
}

impl Patchable<DagNodePatch> for DagNodeSpec {
    fn apply_patch(&mut self, patch: &DagNodePatch) {
        if let Some(name) = &patch.name {
            self.name = name.clone();
        }
        if let Some(x) = patch.x {
            self.x = x;
        }
        if let Some(y) = patch.y {
            self.y = y;
        }
        if let Some(width) = patch.width {
            self.width = width;
        }
        if let Some(height) = patch.height {
            self.height = height;
        }
        if let Some(kind) = &patch.kind {
            self.kind = kind.clone();
        }
    }

    /// 🧮 `self`-relative-to-`other` diff (protocol::Patchable convention) — used by
    /// `invert_collection_operation` as `after.diff_patch(&prior)` to recover the pre-patch state.
    /// Always returns `Some` (even an all-`None` patch) per that function's no-op-patch guidance.
    fn diff_patch(&self, other: &Self) -> Option<DagNodePatch> {
        Some(DagNodePatch {
            name: (self.name != other.name).then(|| other.name.clone()),
            x: (self.x != other.x).then_some(other.x),
            y: (self.y != other.y).then_some(other.y),
            width: (self.width != other.width).then_some(other.width),
            height: (self.height != other.height).then_some(other.height),
            kind: (self.kind != other.kind).then(|| other.kind.clone()),
        })
    }
}

/// 🩹 Sparse patch of a {@link DagFixtureEdge}'s endpoints.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DagEdgePatch {
    pub source: Option<String>,
    pub target: Option<String>,
}

impl Patchable<DagEdgePatch> for DagFixtureEdge {
    fn apply_patch(&mut self, patch: &DagEdgePatch) {
        if let Some(source) = &patch.source {
            self.source = source.clone();
        }
        if let Some(target) = &patch.target {
            self.target = target.clone();
        }
    }

    /// 🧮 `self`-relative-to-`other` diff — see {@link DagNodeSpec}'s `Patchable` impl above.
    fn diff_patch(&self, other: &Self) -> Option<DagEdgePatch> {
        Some(DagEdgePatch { source: (self.source != other.source).then(|| other.source.clone()), target: (self.target != other.target).then(|| other.target.clone()) })
    }
}

/// ▶️ Applies a `CollectionDiff` (removed → modified → added) to an owned `Vec` — `vcs::CollectionDiff`
/// has no generic apply of its own since `modified` patches require the item's `Patchable` impl.
fn apply_collection_diff<TId, TItem, TPatch>(items: &mut Vec<TItem>, diff: &CollectionDiff<TId, TPatch, TItem>)
where
    TId: PartialEq,
    TItem: Identified<TId> + Clone + Patchable<TPatch>,
{
    for id in &diff.removed {
        items.retain(|item| item.id() != id);
    }
    for patch in &diff.modified {
        if let Some(item) = items.iter_mut().find(|item| item.id() == &patch.id) {
            item.apply_patch(&patch.patch);
        }
    }
    for added in &diff.added {
        items.push(added.clone());
    }
}

/// ➕ Merges an incoming `CollectionDiff` into an existing one (coalescing two edits' diffs).
fn absorb_collection_diff<TId: Clone, TItem: Clone, TPatch: Clone>(target: &mut Option<CollectionDiff<TId, TPatch, TItem>>, incoming: Option<CollectionDiff<TId, TPatch, TItem>>) {
    if let Some(b) = incoming {
        match target {
            Some(a) => {
                a.removed.extend(b.removed);
                a.modified.extend(b.modified);
                a.added.extend(b.added);
            }
            None => *target = Some(b),
        }
    }
}
//#endregion 🔖CollectionSupport

/// 📦 Typed DAG operation. Node/edge add/remove/patch/move flow through a generic {@link CollectionOperation}
/// for granular convergence; `SetNodes`/`SetEdges` are bulk writes (relayout/rename) and `SetDocument`
/// replaces the whole projection (import/reset).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
#[allow(
    clippy::large_enum_variant,
    reason = "boxing `Nodes`'s CollectionOperation would require rewrapping every construction/match site across this crate and infinite/board/port/directed/dag/plugin/rs (10+ call sites, out of this crate's scope); DagOperation values are short-lived per-dispatch operations, not stored in bulk"
)]
pub enum DagOperation {
    Nodes(CollectionOperation<String, DagNodeSpec, DagNodePatch>),
    Edges(CollectionOperation<String, DagFixtureEdge, DagEdgePatch>),
    SetNodes { nodes: Vec<DagNodeSpec> },
    SetEdges { edges: Vec<DagFixtureEdge> },
    SetDocument { document: DagDocument },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DagDiff {
    pub document: Option<DagDocument>,
    pub nodes: Option<CollectionDiff<String, DagNodePatch, DagNodeSpec>>,
    pub edges: Option<CollectionDiff<String, DagEdgePatch, DagFixtureEdge>>,
    pub set_nodes: Option<Vec<DagNodeSpec>>,
    pub set_edges: Option<Vec<DagFixtureEdge>>,
}

impl OperationDiff<DagDocument> for DagDiff {
    fn apply(&self, projection: &DagDocument) -> DagDocument {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        if let Some(nodes) = &self.set_nodes {
            next.nodes = nodes.clone();
        }
        if let Some(edges) = &self.set_edges {
            next.edges = edges.clone();
        }
        if let Some(diff) = &self.nodes {
            apply_collection_diff(&mut next.nodes, diff);
        }
        if let Some(diff) = &self.edges {
            apply_collection_diff(&mut next.edges, diff);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            self.document = other.document;
            return;
        }
        if other.set_nodes.is_some() {
            self.set_nodes = other.set_nodes;
        }
        if other.set_edges.is_some() {
            self.set_edges = other.set_edges;
        }
        absorb_collection_diff(&mut self.nodes, other.nodes);
        absorb_collection_diff(&mut self.edges, other.edges);
    }
}

impl Operation<DagDocument> for DagOperation {
    type Diff = DagDiff;

    fn diff(&self, projection: &DagDocument) -> DagDiff {
        match self {
            DagOperation::Nodes(operation) => DagDiff { nodes: Some(collection_diff_from_operation(&projection.nodes, operation)), ..Default::default() },
            DagOperation::Edges(operation) => DagDiff { edges: Some(collection_diff_from_operation(&projection.edges, operation)), ..Default::default() },
            DagOperation::SetNodes { nodes } => DagDiff { set_nodes: Some(nodes.clone()), ..Default::default() },
            DagOperation::SetEdges { edges } => DagDiff { set_edges: Some(edges.clone()), ..Default::default() },
            DagOperation::SetDocument { document } => DagDiff { document: Some(document.clone()), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &DagDocument) -> Vec<Self> {
        match self {
            DagOperation::Nodes(operation) => vec![DagOperation::Nodes(invert_collection_operation(&projection.nodes, operation))],
            DagOperation::Edges(operation) => vec![DagOperation::Edges(invert_collection_operation(&projection.edges, operation))],
            DagOperation::SetNodes { .. } => vec![DagOperation::SetNodes { nodes: projection.nodes.clone() }],
            DagOperation::SetEdges { .. } => vec![DagOperation::SetEdges { edges: projection.edges.clone() }],
            DagOperation::SetDocument { .. } => vec![DagOperation::SetDocument { document: projection.clone() }],
        }
    }
}

pub type DagEnvelope = DocumentVcsEnvelope<DagDocument, DagOperation>;
pub type DagStore = DocumentVcsStore<DagDocument, DagOperation>;

//#region 🔖Dsl
// 🧬 `.dag` document DSL via the `dsl::` derive engine (see `🔖DslMirror` below) — every persisted
// type (`DagDocument`/`DagNodeSpec`/`DagNodeKind`/`DagFixtureEdge`/`IoPortSpec`/`DagMedia`/
// `DagPreviewContent`/`PortShape`/`EdgeRouteStyle`/`DagMediaKind`/patches) either derives a
// `dsl::Dsl*` macro directly or, where the real Rust field shape can't satisfy the derive engine
// (a bare tagged-enum field where the engine requires `Box<T>`), converts through a small local
// mirror type at the `parse_dsl`/`print_dsl`/`parse_op`/`print_op` boundary. This replaces the old
// hand-rolled `mathematical_graph_dsl` wire-literal-based printer/parser that used to live in this
// region (deleted; `dag_fixture_to_wire_literal`/`dag_fixture_execution_rows` near {@link DagHost}
// still use the wire-literal grammar directly for their own, unrelated purpose and are untouched).

//#region 🔖DslMirror
// 🧬 `DagNodeKind` is `#[serde(flatten)]`-merged onto `DagNodeSpec` at the JSON level, and its own
// `Preview` variant carries a nested tagged enum (`DagPreviewContent`). The dsl:: derive engine
// represents "exactly one nested tagged value" via `#[dsl(statements)] Box<T>` (`RequiredStatements`),
// which needs a `Box` wrapper the REAL `DagNodeKind`/`DagNodeSpec` fields deliberately don't carry
// (dozens of call sites here and in `dag-plugin`/`framework/surface/node-graph`/`flow/core` destructure
// `node.kind`/`DagNodeKind::Preview { content, .. }` directly — boxing those fields would ripple far
// outside this crate's ownership). So, exactly like `imperative/core/rs`'s `ImperativeOperationDsl`
// mirror, `DagNodeKindDsl`/`DagNodeSpecDsl`/`DagNodePatchDsl`/`DagDocumentDsl`/`DagOperationDsl` are
// LOCAL structural twins that box only where the derive requires it; the real domain types keep their
// original unboxed shape and never leave this crate — conversion happens right at this boundary.
#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum DagNodeKindDsl {
    Computation {
        #[dsl(table)]
        inputs: Vec<IoPortSpec>,
        #[dsl(table)]
        outputs: Vec<IoPortSpec>,
        variadic_inputs: bool,
        variadic_outputs: bool,
    },
    Slider { min: f64, max: f64, step: f64, value: f64, output: IoPortSpec },
    Select { options: Vec<String>, selected: usize, output: IoPortSpec },
    Screen { media: Option<DagMedia>, input: IoPortSpec },
    Note { text: String, output: IoPortSpec },
    Image { src: String, output: IoPortSpec },
    Preview {
        #[dsl(statements)]
        content: Box<DagPreviewContent>,
        expanded: Vec<String>,
        input: IoPortSpec,
    },
    Action { label: String, input: IoPortSpec },
    Export { label: String, format: String, input: IoPortSpec },
    Cluster {
        #[dsl(table)]
        inputs: Vec<IoPortSpec>,
        #[dsl(table)]
        outputs: Vec<IoPortSpec>,
    },
    AppInstance {
        instance_id: String,
        program_id: String,
        app_id: String,
        icon: String,
        #[dsl(table)]
        inputs: Vec<IoPortSpec>,
        #[dsl(table)]
        outputs: Vec<IoPortSpec>,
    },
}

fn dag_node_kind_to_dsl(kind: &DagNodeKind) -> DagNodeKindDsl {
    match kind {
        DagNodeKind::Computation { inputs, outputs, variadic_inputs, variadic_outputs } => {
            DagNodeKindDsl::Computation { inputs: inputs.clone(), outputs: outputs.clone(), variadic_inputs: *variadic_inputs, variadic_outputs: *variadic_outputs }
        }
        DagNodeKind::Slider { min, max, step, value, output } => DagNodeKindDsl::Slider { min: *min, max: *max, step: *step, value: *value, output: output.clone() },
        DagNodeKind::Select { options, selected, output } => DagNodeKindDsl::Select { options: options.clone(), selected: *selected, output: output.clone() },
        DagNodeKind::Screen { media, input } => DagNodeKindDsl::Screen { media: media.clone(), input: input.clone() },
        DagNodeKind::Note { text, output } => DagNodeKindDsl::Note { text: text.clone(), output: output.clone() },
        DagNodeKind::Image { src, output } => DagNodeKindDsl::Image { src: src.clone(), output: output.clone() },
        DagNodeKind::Preview { content, expanded, input } => DagNodeKindDsl::Preview { content: Box::new(content.clone()), expanded: expanded.iter().cloned().collect(), input: input.clone() },
        DagNodeKind::Action { label, input } => DagNodeKindDsl::Action { label: label.clone(), input: input.clone() },
        DagNodeKind::Export { label, format, input } => DagNodeKindDsl::Export { label: label.clone(), format: format.clone(), input: input.clone() },
        DagNodeKind::Cluster { inputs, outputs } => DagNodeKindDsl::Cluster { inputs: inputs.clone(), outputs: outputs.clone() },
        DagNodeKind::AppInstance { instance_id, program_id, app_id, icon, inputs, outputs } => {
            DagNodeKindDsl::AppInstance { instance_id: instance_id.clone(), program_id: program_id.clone(), app_id: app_id.clone(), icon: icon.clone(), inputs: inputs.clone(), outputs: outputs.clone() }
        }
    }
}

fn dag_node_kind_from_dsl(kind: DagNodeKindDsl) -> DagNodeKind {
    match kind {
        DagNodeKindDsl::Computation { inputs, outputs, variadic_inputs, variadic_outputs } => DagNodeKind::Computation { inputs, outputs, variadic_inputs, variadic_outputs },
        DagNodeKindDsl::Slider { min, max, step, value, output } => DagNodeKind::Slider { min, max, step, value, output },
        DagNodeKindDsl::Select { options, selected, output } => DagNodeKind::Select { options, selected, output },
        DagNodeKindDsl::Screen { media, input } => DagNodeKind::Screen { media, input },
        DagNodeKindDsl::Note { text, output } => DagNodeKind::Note { text, output },
        DagNodeKindDsl::Image { src, output } => DagNodeKind::Image { src, output },
        DagNodeKindDsl::Preview { content, expanded, input } => DagNodeKind::Preview { content: *content, expanded: expanded.into_iter().collect(), input },
        DagNodeKindDsl::Action { label, input } => DagNodeKind::Action { label, input },
        DagNodeKindDsl::Export { label, format, input } => DagNodeKind::Export { label, format, input },
        DagNodeKindDsl::Cluster { inputs, outputs } => DagNodeKind::Cluster { inputs, outputs },
        DagNodeKindDsl::AppInstance { instance_id, program_id, app_id, icon, inputs, outputs } => DagNodeKind::AppInstance { instance_id, program_id, app_id, icon, inputs, outputs },
    }
}

/// 🧬 Mirror of {@link DagNodeSpec} — every field identical except `kind`, boxed only here (see the
/// region's opening doc comment).
#[derive(Clone, Debug, PartialEq, dsl::DslRecord)]
struct DagNodeSpecDsl {
    id: String,
    name: String,
    abbreviation: String,
    icon: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    operator_kind: Option<String>,
    properties: PropertyBag,
    #[dsl(statements)]
    kind: Box<DagNodeKindDsl>,
}

fn dag_node_spec_to_dsl(node: &DagNodeSpec) -> DagNodeSpecDsl {
    DagNodeSpecDsl {
        id: node.id.clone(),
        name: node.name.clone(),
        abbreviation: node.abbreviation.clone(),
        icon: node.icon.clone(),
        x: node.x,
        y: node.y,
        width: node.width,
        height: node.height,
        operator_kind: node.operator_kind.clone(),
        properties: node.properties.clone(),
        kind: Box::new(dag_node_kind_to_dsl(&node.kind)),
    }
}

fn dag_node_spec_from_dsl(mirror: DagNodeSpecDsl) -> DagNodeSpec {
    DagNodeSpec {
        id: mirror.id,
        name: mirror.name,
        abbreviation: mirror.abbreviation,
        icon: mirror.icon,
        x: mirror.x,
        y: mirror.y,
        width: mirror.width,
        height: mirror.height,
        operator_kind: mirror.operator_kind,
        properties: mirror.properties,
        kind: dag_node_kind_from_dsl(*mirror.kind),
    }
}

/// 🧬 Mirror of {@link DagNodePatch} — `kind` unwraps to `Option<DagNodeKindDsl>` directly (no `Box`
/// needed: `#[dsl(statements)] Option<T>` is already the `OptionStatements` shape, not `RequiredStatements`).
#[derive(Clone, Debug, Default, PartialEq, dsl::DslRecord)]
struct DagNodePatchDsl {
    name: Option<String>,
    x: Option<f64>,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
    #[dsl(statements)]
    kind: Option<DagNodeKindDsl>,
}

fn dag_node_patch_to_dsl(patch: &DagNodePatch) -> DagNodePatchDsl {
    DagNodePatchDsl { name: patch.name.clone(), x: patch.x, y: patch.y, width: patch.width, height: patch.height, kind: patch.kind.as_ref().map(dag_node_kind_to_dsl) }
}

fn dag_node_patch_from_dsl(mirror: DagNodePatchDsl) -> DagNodePatch {
    DagNodePatch { name: mirror.name, x: mirror.x, y: mirror.y, width: mirror.width, height: mirror.height, kind: mirror.kind.map(dag_node_kind_from_dsl) }
}

/// 🧬 Mirror of {@link DagDocument} — `nodes: Vec<DagNodeSpecDsl>` instead of `Vec<DagNodeSpec>` since
/// `DagNodeSpec` itself can't implement `dsl::DslField` (its `kind` field isn't boxed).
#[derive(Clone, Debug, PartialEq, dsl::DslDocument)]
#[dsl(extension = "dag")]
#[dsl(layout = "lines")]
struct DagDocumentDsl {
    schema: String,
    nodes: Vec<DagNodeSpecDsl>,
    #[dsl(table)]
    edges: Vec<DagFixtureEdge>,
}

fn dag_document_to_dsl(document: &DagDocument) -> DagDocumentDsl {
    DagDocumentDsl { schema: document.schema.clone(), nodes: document.nodes.iter().map(dag_node_spec_to_dsl).collect(), edges: document.edges.clone() }
}

fn dag_document_from_dsl(mirror: DagDocumentDsl) -> DagDocument {
    DagDocument { schema: mirror.schema, nodes: mirror.nodes.into_iter().map(dag_node_spec_from_dsl).collect(), edges: mirror.edges }
}

impl vcs::DocumentDsl for DagDocument {
    const EXTENSION: &'static str = "dag";

    fn parse_dsl(text: &str) -> Result<Self, vcs::TextError> {
        Ok(dag_document_from_dsl(<DagDocumentDsl as vcs::DocumentDsl>::parse_dsl(text)?))
    }

    fn print_dsl(&self) -> String {
        <DagDocumentDsl as vcs::DocumentDsl>::print_dsl(&dag_document_to_dsl(self))
    }
}

/// 📦 Binary counterpart of the `DocumentDsl` impl above — `DagDocument` can't `#[derive(dsl::
/// DslDocument)]` directly (see this region's opening doc comment), so `DocumentPack` is hand-routed
/// through the same `DagDocumentDsl` mirror, which does derive it.
impl vcs::DocumentPack for DagDocument {
    fn encode_pack_with(&self, options: &vcs::PackEncodeOptions) -> Result<Vec<u8>, vcs::PackError> {
        <DagDocumentDsl as vcs::DocumentPack>::encode_pack_with(&dag_document_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &vcs::PackDecodeOptions) -> Result<Self, vcs::PackError> {
        Ok(dag_document_from_dsl(<DagDocumentDsl as vcs::DocumentPack>::decode_pack_with(bytes, options)?))
    }
}
//#endregion 🔖DslMirror
//#endregion 🔖Dsl

//#region 🔖OpText

//#region 🔖OpTextMirror
/// 🧬 Mirror of {@link DagOperation} — see `🔖DslMirror`'s doc comment for why `DagNodeSpec`/
/// `DagNodePatch` need their own Dsl twins here too. `DagFixtureEdge`/`DagEdgePatch` are used
/// directly (no twin needed): neither has a boxed-tagged-enum field, so both already implement
/// `dsl::DslField` from their own direct `#[derive(dsl::DslRecord)]` above.
#[derive(Clone, Debug, PartialEq, dsl::DslOps)]
enum DagOperationDsl {
    NodesAdd { index: usize, item: DagNodeSpecDsl },
    NodesRemove { id: String },
    NodesMove {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    NodesPatch { id: String, patch: DagNodePatchDsl },
    EdgesAdd { index: usize, item: DagFixtureEdge },
    EdgesRemove { id: String },
    EdgesMove {
        id: String,
        #[dsl(key = "to")]
        to_index: usize,
    },
    EdgesPatch { id: String, patch: DagEdgePatch },
    SetNodes { nodes: Vec<DagNodeSpecDsl> },
    SetEdges {
        #[dsl(table)]
        edges: Vec<DagFixtureEdge>,
    },
    SetDocument { document: DagDocumentDsl },
}

fn dag_operation_to_dsl(operation: &DagOperation) -> DagOperationDsl {
    match operation {
        DagOperation::Nodes(CollectionOperation::Add { id: _id, item, at }) => DagOperationDsl::NodesAdd { index: *at, item: dag_node_spec_to_dsl(item) },
        DagOperation::Nodes(CollectionOperation::Remove { id }) => DagOperationDsl::NodesRemove { id: id.clone() },
        DagOperation::Nodes(CollectionOperation::Move { id, to }) => DagOperationDsl::NodesMove { id: id.clone(), to_index: *to },
        DagOperation::Nodes(CollectionOperation::Patch { id, patch }) => DagOperationDsl::NodesPatch { id: id.clone(), patch: dag_node_patch_to_dsl(patch) },
        DagOperation::Edges(CollectionOperation::Add { id: _id, item, at }) => DagOperationDsl::EdgesAdd { index: *at, item: item.clone() },
        DagOperation::Edges(CollectionOperation::Remove { id }) => DagOperationDsl::EdgesRemove { id: id.clone() },
        DagOperation::Edges(CollectionOperation::Move { id, to }) => DagOperationDsl::EdgesMove { id: id.clone(), to_index: *to },
        DagOperation::Edges(CollectionOperation::Patch { id, patch }) => DagOperationDsl::EdgesPatch { id: id.clone(), patch: patch.clone() },
        DagOperation::SetNodes { nodes } => DagOperationDsl::SetNodes { nodes: nodes.iter().map(dag_node_spec_to_dsl).collect() },
        DagOperation::SetEdges { edges } => DagOperationDsl::SetEdges { edges: edges.clone() },
        DagOperation::SetDocument { document } => DagOperationDsl::SetDocument { document: dag_document_to_dsl(document) },
    }
}

fn dag_operation_from_dsl(mirror: DagOperationDsl) -> DagOperation {
    match mirror {
        DagOperationDsl::NodesAdd { index, item } => {
            let item = dag_node_spec_from_dsl(item);
            DagOperation::Nodes(CollectionOperation::Add { id: item.id.clone(), item, at: index })
        }
        DagOperationDsl::NodesRemove { id } => DagOperation::Nodes(CollectionOperation::Remove { id }),
        DagOperationDsl::NodesMove { id, to_index } => DagOperation::Nodes(CollectionOperation::Move { id, to: to_index }),
        DagOperationDsl::NodesPatch { id, patch } => DagOperation::Nodes(CollectionOperation::Patch { id, patch: dag_node_patch_from_dsl(patch) }),
        DagOperationDsl::EdgesAdd { index, item } => DagOperation::Edges(CollectionOperation::Add { id: item.id.clone(), item, at: index }),
        DagOperationDsl::EdgesRemove { id } => DagOperation::Edges(CollectionOperation::Remove { id }),
        DagOperationDsl::EdgesMove { id, to_index } => DagOperation::Edges(CollectionOperation::Move { id, to: to_index }),
        DagOperationDsl::EdgesPatch { id, patch } => DagOperation::Edges(CollectionOperation::Patch { id, patch }),
        DagOperationDsl::SetNodes { nodes } => DagOperation::SetNodes { nodes: nodes.into_iter().map(dag_node_spec_from_dsl).collect() },
        DagOperationDsl::SetEdges { edges } => DagOperation::SetEdges { edges },
        DagOperationDsl::SetDocument { document } => DagOperation::SetDocument { document: dag_document_from_dsl(document) },
    }
}

impl OpText for DagOperation {
    fn parse_op(line: &str) -> Result<Self, vcs::TextError> {
        Ok(dag_operation_from_dsl(<DagOperationDsl as OpText>::parse_op(line)?))
    }

    fn print_op(&self) -> String {
        <DagOperationDsl as OpText>::print_op(&dag_operation_to_dsl(self))
    }
}
//#endregion 🔖OpTextMirror
//#endregion 🔖OpText

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct DagDocumentVcs {
        store: RefCell<DagStore>,
    }

    #[wasm_bindgen]
    impl DagDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<DagDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: DagEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    DagStore::new(envelope)
                }
                None => DagStore::new(create_document_vcs_envelope(DAG_DOCUMENT_SCHEMA, "dag", empty_dag_document(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_json(command_json).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
//#endregion 🔖WasmBridge

#[cfg(test)]
mod dag_vcs_tests {
    use super::*;

    fn sample_node(id: &str) -> DagNodeSpec {
        DagNodeSpec { id: id.into(), name: id.into(), ..Default::default() }
    }

    fn round_trip(document: &DagDocument, operation: &DagOperation) -> DagDocument {
        let forward = vcs::apply_operation(document, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(document) {
            restored = vcs::apply_operation(&restored, &back);
        }
        assert_eq!(&restored, document, "backwards() must exactly restore the pre-operation document");
        forward
    }

    #[test]
    fn dag_document_vcs_replays_node_operations() {
        let mut store = DagStore::new(create_document_vcs_envelope(DAG_DOCUMENT_SCHEMA, "dag", empty_dag_document(), None));
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![DagOperation::Nodes(CollectionOperation::Add { id: "n1".into(), item: sample_node("n1"), at: 0 })], description: None }).expect("apply");
        assert_eq!(store.projection().expect("projection").nodes.len(), 1);
    }

    #[test]
    fn node_add_patch_remove_round_trip() {
        let document = empty_dag_document();
        let added = round_trip(&document, &DagOperation::Nodes(CollectionOperation::Add { id: "n1".into(), item: sample_node("n1"), at: 0 }));
        assert_eq!(added.nodes.len(), 1);
        let patched = round_trip(&added, &DagOperation::Nodes(CollectionOperation::Patch { id: "n1".into(), patch: DagNodePatch { name: Some("Renamed".into()), x: Some(42.0), ..Default::default() } }));
        assert_eq!(patched.nodes[0].name, "Renamed");
        assert_eq!(patched.nodes[0].x, 42.0);
        let removed = round_trip(&patched, &DagOperation::Nodes(CollectionOperation::Remove { id: "n1".into() }));
        assert!(removed.nodes.is_empty());
    }

    #[test]
    fn edge_add_remove_round_trip() {
        let mut document = empty_dag_document();
        document.nodes = vec![sample_node("a"), sample_node("b")];
        let edge = DagFixtureEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() };
        let added = round_trip(&document, &DagOperation::Edges(CollectionOperation::Add { id: "e1".into(), item: edge, at: 0 }));
        assert_eq!(added.edges.len(), 1);
        let removed = round_trip(&added, &DagOperation::Edges(CollectionOperation::Remove { id: "e1".into() }));
        assert!(removed.edges.is_empty());
    }

    //#region 🔖DslTests
    /// 🧩 One node per `DagNodeKind` tag (safe field values only — no raw JSON literals in
    /// `default`/`value`/`Tree.json`, see {@link json_to_property}'s docstring), so the DSL round trip
    /// exercises every kind-specific payload shape the wire-literal property bag needs to carry.
    fn kitchen_sink_document() -> DagDocument {
        let port = |id: &str, label: &str| IoPortSpec::simple(id, label);
        let nodes = vec![
            DagNodeSpec { id: "comp".into(), name: "Comp".into(), abbreviation: "Cmp".into(), icon: "emoji:🧮".into(), x: -120.0, y: -40.0, width: 104.0, height: 14.0, kind: DagNodeKind::Computation { inputs: vec![port("in", "In")], outputs: vec![port("out", "Out")], variadic_inputs: true, variadic_outputs: false }, ..Default::default() },
            DagNodeSpec { id: "slider".into(), name: "Amount".into(), x: -400.0, y: -40.0, width: 70.0, height: 14.0, kind: DagNodeKind::Slider { min: 0.0, max: 10.0, step: 0.5, value: 5.0, output: port("out", "value") }, ..Default::default() },
            DagNodeSpec { id: "mode".into(), name: "Mode".into(), x: -400.0, y: 80.0, width: 56.0, height: 28.0, kind: DagNodeKind::Select { options: vec!["Add".into(), "Multiply".into()], selected: 1, output: port("out", "mode") }, ..Default::default() },
            DagNodeSpec { id: "screen".into(), name: "Preview".into(), x: 400.0, y: 0.0, width: 200.0, height: 140.0, kind: DagNodeKind::Screen { media: Some(DagMedia { kind: DagMediaKind::Svg, src: "data:image/svg+xml,%3Csvg viewBox='0 0 1 1'%3E%3C/svg%3E".into() }), input: port("in", "result") }, ..Default::default() },
            DagNodeSpec { id: "note".into(), name: "Note".into(), x: 0.0, y: 200.0, kind: DagNodeKind::Note { text: "line one\nline two — with a ' quote and a % sign".into(), output: port("out", "text") }, ..Default::default() },
            DagNodeSpec { id: "image".into(), name: "Image".into(), x: 0.0, y: 260.0, kind: DagNodeKind::Image { src: "data:image/png;base64,AAA=".into(), output: port("out", "img") }, ..Default::default() },
            DagNodeSpec { id: "preview".into(), name: "Preview2".into(), x: 0.0, y: 320.0, kind: DagNodeKind::Preview { content: DagPreviewContent::Scalar { text: "42".into() }, expanded: BTreeSet::from(["a.b".to_string()]), input: port("in", "value") }, ..Default::default() },
            DagNodeSpec { id: "action".into(), name: "Action".into(), x: 0.0, y: 380.0, kind: DagNodeKind::Action { label: "Run".into(), input: port("in", "trigger") }, ..Default::default() },
            DagNodeSpec { id: "export".into(), name: "Export".into(), x: 0.0, y: 440.0, kind: DagNodeKind::Export { label: "Save".into(), format: "png".into(), input: port("in", "value") }, ..Default::default() },
            DagNodeSpec { id: "cluster".into(), name: "Cluster".into(), x: 0.0, y: 500.0, kind: DagNodeKind::Cluster { inputs: vec![port("in", "In")], outputs: vec![port("out", "Out")] }, ..Default::default() },
            DagNodeSpec { id: "app".into(), name: "App".into(), x: 0.0, y: 560.0, kind: DagNodeKind::AppInstance { instance_id: "inst-1".into(), program_id: "prog-1".into(), app_id: "note".into(), icon: "emoji:📦".into(), inputs: vec![], outputs: vec![port("out", "Out")] }, ..Default::default() },
        ];
        let edges = vec![
            DagFixtureEdge { id: "e1".into(), source: "slider@out".into(), target: "comp@in".into(), ..Default::default() },
            DagFixtureEdge { id: "e2".into(), source: "comp@out".into(), target: "screen@in".into(), route_style: EdgeRouteStyle::SharpSz, properties: PropertyBag::from([("weight".to_string(), PropertyValue::Number(2.0))]) },
        ];
        DagDocument { schema: DAG_DOCUMENT_SCHEMA.into(), nodes, edges }
    }

    #[test]
    fn dag_document_dsl_round_trips_the_demo_fixture() {
        vcs::test_support::assert_dsl_round_trip(&default_dag_document());
        vcs::test_support::assert_dsl_pack_equivalence(&default_dag_document());
    }

    #[test]
    fn dag_document_dsl_round_trips_every_node_kind() {
        vcs::test_support::assert_dsl_round_trip(&kitchen_sink_document());
        vcs::test_support::assert_dsl_pack_equivalence(&kitchen_sink_document());
    }

    #[test]
    fn dag_document_dsl_round_trips_the_empty_document() {
        vcs::test_support::assert_dsl_round_trip(&empty_dag_document());
        vcs::test_support::assert_dsl_pack_equivalence(&empty_dag_document());
    }

    #[test]
    fn op_text_round_trips_nodes_add() {
        vcs::test_support::assert_op_line_round_trip(&DagOperation::Nodes(CollectionOperation::Add { id: "n1".into(), item: sample_node("n1"), at: 0 }));
    }

    #[test]
    fn op_text_round_trips_nodes_remove() {
        vcs::test_support::assert_op_line_round_trip(&DagOperation::Nodes(CollectionOperation::Remove { id: "n1".into() }));
    }

    #[test]
    fn op_text_round_trips_nodes_move() {
        vcs::test_support::assert_op_line_round_trip(&DagOperation::Nodes(CollectionOperation::Move { id: "n1".into(), to: 2 }));
    }

    #[test]
    fn op_text_round_trips_nodes_patch() {
        vcs::test_support::assert_op_line_round_trip(&DagOperation::Nodes(CollectionOperation::Patch {
            id: "n1".into(),
            patch: DagNodePatch { name: Some("Renamed".into()), x: Some(42.0), y: None, width: None, height: Some(10.0), kind: Some(DagNodeKind::Slider { min: 0.0, max: 1.0, step: 0.1, value: 0.5, output: IoPortSpec::simple("out", "value") }) },
        }));
    }

    #[test]
    fn op_text_round_trips_edges_add() {
        let edge = DagFixtureEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() };
        vcs::test_support::assert_op_line_round_trip(&DagOperation::Edges(CollectionOperation::Add { id: "e1".into(), item: edge, at: 0 }));
    }

    #[test]
    fn op_text_round_trips_edges_remove() {
        vcs::test_support::assert_op_line_round_trip(&DagOperation::Edges(CollectionOperation::Remove { id: "e1".into() }));
    }

    #[test]
    fn op_text_round_trips_edges_move() {
        vcs::test_support::assert_op_line_round_trip(&DagOperation::Edges(CollectionOperation::Move { id: "e1".into(), to: 3 }));
    }

    #[test]
    fn op_text_round_trips_edges_patch() {
        vcs::test_support::assert_op_line_round_trip(&DagOperation::Edges(CollectionOperation::Patch { id: "e1".into(), patch: DagEdgePatch { source: Some("a@out".into()), target: None } }));
    }

    #[test]
    fn op_text_round_trips_set_nodes() {
        vcs::test_support::assert_op_line_round_trip(&DagOperation::SetNodes { nodes: vec![sample_node("n1"), sample_node("n2")] });
    }

    #[test]
    fn op_text_round_trips_set_edges() {
        let edge = DagFixtureEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() };
        vcs::test_support::assert_op_line_round_trip(&DagOperation::SetEdges { edges: vec![edge] });
    }

    #[test]
    fn op_text_round_trips_set_document() {
        vcs::test_support::assert_op_line_round_trip(&DagOperation::SetDocument { document: kitchen_sink_document() });
    }

    #[test]
    fn document_text_round_trips_a_store_with_an_applied_operation() {
        let mut store = DagStore::new(create_document_vcs_envelope(DAG_DOCUMENT_SCHEMA, "dag", kitchen_sink_document(), None));
        store
            .dispatch(DocumentVcsCommand::Apply { operations: vec![DagOperation::Nodes(CollectionOperation::Add { id: "extra".into(), item: sample_node("extra"), at: 0 })], description: None })
            .expect("apply");
        vcs::test_support::assert_document_text_round_trip(&store);
        vcs::test_support::assert_document_pack_round_trip(&store);
    }
    //#endregion 🔖DslTests
}
// #endregion 🔖DocumentVcs
