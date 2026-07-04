//! ✏️ Draw document domain + typed VCS on `vcs`.

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use vcs::{create_document_vcs_envelope, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff};

pub const DRAW_DOCUMENT_SCHEMA: &str = "draw.document";
pub const DRAW_BLEND_MODES: &[&str] = &[
    "normal", "multiply", "screen", "overlay", "darken", "lighten", "colorDodge", "colorBurn", "hardLight",
    "softLight", "difference", "exclusion", "hue", "saturation", "color", "luminosity",
];
pub const DRAW_BOOLEAN_OPS: &[&str] = &["union", "difference", "intersection", "xor"];
pub const DRAW_SHAPE_KINDS: &[&str] = &["rect", "ellipse", "circle", "line", "polygon"];
pub const DRAW_TOOL_IDS: &[&str] = &[
    "selectMarquee", "selectLasso", "selectDirect", "pen", "shapeRect", "shapeEllipse", "shapeLine",
    "shapePolygon", "booleanCombine", "trace", "transformMove",
];

//#region 🔖Domain
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawTransform {
    pub x: f64,
    pub y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub rotation: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GradientStop {
    pub offset: f64,
    pub color: [f64; 4],
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FillStyle {
    Solid { color: [f64; 4] },
    LinearGradient { x1: f64, y1: f64, x2: f64, y2: f64, stops: Vec<GradientStop> },
    RadialGradient { cx: f64, cy: f64, r: f64, stops: Vec<GradientStop> },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrokeStyle {
    pub color: [f64; 4],
    pub width: f64,
    pub cap: String,
    pub join: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash: Option<Vec<f64>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<FillStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<StrokeStyle>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawTraceParams {
    pub threshold: f64,
    pub simplify_epsilon: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawImageAsset {
    pub mime: String,
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayerBase {
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f64,
    pub blend_mode: String,
    pub transform: DrawTransform,
    #[serde(default)]
    pub attributes: DrawAttributes,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawEllipse {
    pub cx: f64,
    pub cy: f64,
    pub rx: f64,
    pub ry: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawCircle {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLine {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawPolygon {
    pub points: Vec<[f64; 2]>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawShapeBody {
    #[serde(flatten)]
    pub base: DrawLayerBase,
    pub shape_kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rect: Option<DrawRect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ellipse: Option<DrawEllipse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle: Option<DrawCircle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<DrawLine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon: Option<DrawPolygon>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawPathBody {
    #[serde(flatten)]
    pub base: DrawLayerBase,
    pub segments: Vec<PathSegment>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawTextBody {
    #[serde(flatten)]
    pub base: DrawLayerBase,
    pub x: f64,
    pub y: f64,
    pub content: String,
    pub size: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawImageBody {
    #[serde(flatten)]
    pub base: DrawLayerBase,
    pub image_key: String,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawGroupBody {
    #[serde(flatten)]
    pub base: DrawLayerBase,
    pub children: Vec<DrawLayerNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawBooleanBody {
    #[serde(flatten)]
    pub base: DrawLayerBase,
    pub op: String,
    pub children: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawTraceBody {
    #[serde(flatten)]
    pub base: DrawLayerBase,
    pub source_key: String,
    pub params: DrawTraceParams,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum DrawLayerNode {
    #[serde(rename = "shape")]
    Shape(DrawShapeBody),
    #[serde(rename = "path")]
    Path(DrawPathBody),
    #[serde(rename = "text")]
    Text(DrawTextBody),
    #[serde(rename = "image")]
    Image(DrawImageBody),
    #[serde(rename = "group")]
    Group(DrawGroupBody),
    #[serde(rename = "boolean")]
    Boolean(DrawBooleanBody),
    #[serde(rename = "trace")]
    Trace(DrawTraceBody),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum PathSegment {
    Move { to: [f64; 2] },
    Line { to: [f64; 2] },
    Quad { ctrl: [f64; 2], to: [f64; 2] },
    Cubic { ctrl1: [f64; 2], ctrl2: [f64; 2], to: [f64; 2] },
    Arc {
        rx: f64,
        ry: f64,
        rotation: f64,
        large_arc: bool,
        sweep: bool,
        to: [f64; 2],
    },
    Close,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawArtboard {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawDocument {
    pub schema: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub camera: DrawCamera,
    pub layers: Vec<DrawLayerNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<std::collections::HashMap<String, DrawImageAsset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artboard: Option<DrawArtboard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_tool: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawSceneNode {
    pub id: String,
    pub transform: [f64; 6],
    pub segments: Vec<PathSegment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<FillStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<StrokeStyle>,
    pub opacity: f64,
    pub blend_mode: String,
    pub visible: bool,
    pub needs_kernel: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_payload: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<DrawSceneText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<DrawSceneImage>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawSceneText {
    pub content: String,
    pub size: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawSceneImage {
    pub src: String,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawCanvasLayerRecord {
    pub id: String,
    pub kind: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
}

pub type DrawEnvelope = DocumentVcsEnvelope<DrawDocument, DrawOp>;
pub type DrawStore = DocumentVcsStore<DrawDocument, DrawOp>;

static DRAW_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn create_draw_id(prefix: &str) -> String {
    let next = DRAW_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{next}")
}

pub fn default_draw_transform() -> DrawTransform {
    DrawTransform {
        x: 0.0,
        y: 0.0,
        scale_x: 1.0,
        scale_y: 1.0,
        rotation: 0.0,
    }
}

pub fn default_draw_trace_params() -> DrawTraceParams {
    DrawTraceParams {
        threshold: 0.5,
        simplify_epsilon: 1.5,
    }
}

pub fn default_layer_base(name: &str) -> DrawLayerBase {
    DrawLayerBase {
        id: create_draw_id("layer"),
        name: name.into(),
        visible: true,
        locked: false,
        opacity: 1.0,
        blend_mode: "normal".into(),
        transform: default_draw_transform(),
        attributes: DrawAttributes::default(),
    }
}

pub fn create_draw_path_layer(name: &str, segments: Vec<PathSegment>) -> DrawLayerNode {
    DrawLayerNode::Path(DrawPathBody {
        base: DrawLayerBase {
            id: create_draw_id("path"),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: default_draw_transform(),
            attributes: DrawAttributes::default(),
        },
        segments,
    })
}

pub fn create_draw_group_layer(name: &str) -> DrawLayerNode {
    DrawLayerNode::Group(DrawGroupBody {
        base: DrawLayerBase {
            id: create_draw_id("group"),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: default_draw_transform(),
            attributes: DrawAttributes::default(),
        },
        children: Vec::new(),
    })
}

pub fn create_draw_boolean_layer(name: &str, op: &str, children: Vec<String>) -> DrawLayerNode {
    DrawLayerNode::Boolean(DrawBooleanBody {
        base: DrawLayerBase {
            id: create_draw_id("boolean"),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: default_draw_transform(),
            attributes: DrawAttributes::default(),
        },
        op: op.into(),
        children,
    })
}

pub fn create_draw_trace_layer(name: &str, source_key: &str) -> DrawLayerNode {
    DrawLayerNode::Trace(DrawTraceBody {
        base: DrawLayerBase {
            id: create_draw_id("trace"),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: default_draw_transform(),
            attributes: DrawAttributes::default(),
        },
        source_key: source_key.into(),
        params: default_draw_trace_params(),
    })
}

pub fn create_draw_shape_layer_rect(name: &str) -> DrawLayerNode {
    DrawLayerNode::Shape(DrawShapeBody {
        base: DrawLayerBase {
            id: create_draw_id("shape"),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: default_draw_transform(),
            attributes: DrawAttributes::default(),
        },
        shape_kind: "rect".into(),
        rect: Some(DrawRect { x: 0.0, y: 0.0, width: 128.0, height: 96.0 }),
        ellipse: None,
        circle: None,
        line: None,
        polygon: None,
    })
}

pub fn create_draw_text_layer(name: &str) -> DrawLayerNode {
    DrawLayerNode::Text(DrawTextBody {
        base: DrawLayerBase {
            id: create_draw_id("text"),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: default_draw_transform(),
            attributes: DrawAttributes {
                fill: Some(FillStyle::Solid { color: [0.0, 0.0, 0.0, 1.0] }),
                stroke: None,
            },
        },
        x: 0.0,
        y: 0.0,
        content: "Text".into(),
        size: 24.0,
    })
}

pub fn create_draw_image_layer(name: &str, image_key: &str) -> DrawLayerNode {
    DrawLayerNode::Image(DrawImageBody {
        base: DrawLayerBase {
            id: create_draw_id("image"),
            name: name.into(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: "normal".into(),
            transform: default_draw_transform(),
            attributes: DrawAttributes::default(),
        },
        image_key: image_key.into(),
        width: 256.0,
        height: 256.0,
    })
}

pub fn default_draw_document(id: &str, title: Option<&str>) -> DrawDocument {
    DrawDocument {
        schema: DRAW_DOCUMENT_SCHEMA.into(),
        id: id.into(),
        title: title.map(str::to_string),
        camera: DrawCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        layers: vec![create_draw_path_layer("Layer 1", Vec::new())],
        assets: None,
        artboard: None,
        active_tool: Some("selectDirect".into()),
    }
}

pub fn empty_draw_projection() -> DrawDocument {
    default_draw_document("empty", None)
}

pub fn layer_id(layer: &DrawLayerNode) -> &str {
    match layer {
        DrawLayerNode::Shape(shape) => &shape.base.id,
        DrawLayerNode::Path(path) => &path.base.id,
        DrawLayerNode::Text(text) => &text.base.id,
        DrawLayerNode::Image(image) => &image.base.id,
        DrawLayerNode::Group(group) => &group.base.id,
        DrawLayerNode::Boolean(boolean) => &boolean.base.id,
        DrawLayerNode::Trace(trace) => &trace.base.id,
    }
}

pub fn layer_base(layer: &DrawLayerNode) -> &DrawLayerBase {
    match layer {
        DrawLayerNode::Shape(shape) => &shape.base,
        DrawLayerNode::Path(path) => &path.base,
        DrawLayerNode::Text(text) => &text.base,
        DrawLayerNode::Image(image) => &image.base,
        DrawLayerNode::Group(group) => &group.base,
        DrawLayerNode::Boolean(boolean) => &boolean.base,
        DrawLayerNode::Trace(trace) => &trace.base,
    }
}

pub fn layer_kind_label(layer: &DrawLayerNode) -> String {
    match layer {
        DrawLayerNode::Shape(shape) => format!("shape:{}", shape.shape_kind),
        DrawLayerNode::Path(_) => "path".into(),
        DrawLayerNode::Text(_) => "text".into(),
        DrawLayerNode::Image(_) => "image".into(),
        DrawLayerNode::Group(_) => "group".into(),
        DrawLayerNode::Boolean(_) => "boolean".into(),
        DrawLayerNode::Trace(_) => "trace".into(),
    }
}

pub fn find_draw_layer<'a>(doc: &'a DrawDocument, layer_id: &str) -> Option<&'a DrawLayerNode> {
    for layer in &doc.layers {
        if let Some(found) = find_draw_layer_in_node(layer, layer_id) {
            return Some(found);
        }
    }
    None
}

fn find_draw_layer_in_node<'a>(node: &'a DrawLayerNode, target_id: &str) -> Option<&'a DrawLayerNode> {
    if layer_id(node) == target_id {
        return Some(node);
    }
    if let DrawLayerNode::Group(group) = node {
        for child in &group.children {
            if let Some(found) = find_draw_layer_in_node(child, target_id) {
                return Some(found);
            }
        }
    }
    None
}

pub fn flatten_draw_layers(layers: &[DrawLayerNode]) -> Vec<&DrawLayerNode> {
    let mut out = Vec::new();
    fn walk<'a>(nodes: &'a [DrawLayerNode], out: &mut Vec<&'a DrawLayerNode>) {
        for node in nodes {
            out.push(node);
            if let DrawLayerNode::Group(group) = node {
                walk(&group.children, out);
            }
        }
    }
    walk(layers, &mut out);
    out
}

pub fn draw_transform_to_matrix(transform: &DrawTransform) -> [f64; 6] {
    let cos = transform.rotation.cos();
    let sin = transform.rotation.sin();
    let a = transform.scale_x * cos;
    let b = transform.scale_x * sin;
    let c = -transform.scale_y * sin;
    let d = transform.scale_y * cos;
    [a, b, c, d, transform.x, transform.y]
}

pub fn draw_matrix_to_transform(matrix: [f64; 6]) -> DrawTransform {
    let [a, b, c, d, e, f] = matrix;
    let scale_x = (a * a + b * b).sqrt();
    let rotation = b.atan2(a);
    let det = a * d - b * c;
    let scale_y = if scale_x != 0.0 { det / scale_x } else { 0.0 };
    DrawTransform { x: e, y: f, scale_x, scale_y, rotation }
}

pub fn draw_play_layers_tree_row_id(layer: &DrawLayerNode) -> String {
    let segment = match layer {
        DrawLayerNode::Group(_) => "group",
        DrawLayerNode::Boolean(_) => "boolean",
        DrawLayerNode::Trace(_) => "trace",
        DrawLayerNode::Path(_) => "path",
        DrawLayerNode::Shape(_) => "shape",
        DrawLayerNode::Text(_) => "text",
        DrawLayerNode::Image(_) => "image",
    };
    format!("draw-play-layers.{segment}.{}", layer_id(layer))
}

pub fn draw_play_boolean_child_row_id(boolean_id: &str, child_id: &str) -> String {
    format!("draw-play-layers.boolean.{boolean_id}.child.{child_id}")
}

pub fn draw_play_layer_id_from_tree_row_id(row_id: &str) -> Option<String> {
    if let Some(rest) = row_id.strip_prefix("draw-play-layers.") {
        let parts: Vec<&str> = rest.split('.').collect();
        if parts.len() >= 2 {
            return Some(parts[parts.len() - 1].to_string());
        }
    }
    None
}

pub fn layer_to_path_segments(layer: &DrawLayerNode) -> Vec<PathSegment> {
    match layer {
        DrawLayerNode::Path(path) => path.segments.clone(),
        DrawLayerNode::Shape(shape) => shape_to_path_segments(shape),
        _ => Vec::new(),
    }
}

fn ellipse_path_segments(cx: f64, cy: f64, rx: f64, ry: f64) -> Vec<PathSegment> {
    let k = 0.552_284_749_8;
    let crx = rx * k;
    let cry = ry * k;
    vec![
        PathSegment::Move { to: [cx, cy - ry] },
        PathSegment::Cubic { ctrl1: [cx + crx, cy - ry], ctrl2: [cx + rx, cy - cry], to: [cx + rx, cy] },
        PathSegment::Cubic { ctrl1: [cx + rx, cy + cry], ctrl2: [cx + crx, cy + ry], to: [cx, cy + ry] },
        PathSegment::Cubic { ctrl1: [cx - crx, cy + ry], ctrl2: [cx - rx, cy + cry], to: [cx - rx, cy] },
        PathSegment::Cubic { ctrl1: [cx - rx, cy - cry], ctrl2: [cx - crx, cy - ry], to: [cx, cy - ry] },
        PathSegment::Close,
    ]
}

fn shape_to_path_segments(shape: &DrawShapeBody) -> Vec<PathSegment> {
    match shape.shape_kind.as_str() {
        "rect" => shape.rect.as_ref().map(|rect| {
            vec![
                PathSegment::Move { to: [rect.x, rect.y] },
                PathSegment::Line { to: [rect.x + rect.width, rect.y] },
                PathSegment::Line { to: [rect.x + rect.width, rect.y + rect.height] },
                PathSegment::Line { to: [rect.x, rect.y + rect.height] },
                PathSegment::Close,
            ]
        }),
        "line" => shape.line.as_ref().map(|line| {
            vec![
                PathSegment::Move { to: [line.x1, line.y1] },
                PathSegment::Line { to: [line.x2, line.y2] },
            ]
        }),
        "polygon" => shape.polygon.as_ref().and_then(|polygon| {
            if polygon.points.is_empty() {
                return None;
            }
            let mut segments = vec![PathSegment::Move { to: polygon.points[0] }];
            for point in polygon.points.iter().skip(1) {
                segments.push(PathSegment::Line { to: *point });
            }
            segments.push(PathSegment::Close);
            Some(segments)
        }),
        "ellipse" => shape.ellipse.as_ref().map(|ellipse| {
            ellipse_path_segments(ellipse.cx, ellipse.cy, ellipse.rx, ellipse.ry)
        }),
        "circle" => shape.circle.as_ref().map(|circle| {
            ellipse_path_segments(circle.cx, circle.cy, circle.r, circle.r)
        }),
        _ => None,
    }
    .unwrap_or_default()
}

pub fn draw_layer_world_bounds(layer: &DrawLayerNode) -> Option<(f64, f64, f64, f64)> {
    let local = match layer {
        DrawLayerNode::Text(text) => {
            let width = (text.content.len() as f64 * text.size * 0.6).max(8.0);
            let height = (text.size * 1.2).max(8.0);
            (text.x, text.y, width, height)
        }
        DrawLayerNode::Image(image) => (0.0, 0.0, image.width, image.height),
        _ => {
            let segments = layer_to_path_segments(layer);
            if segments.is_empty() {
                return Some((-64.0, -64.0, 128.0, 128.0));
            }
            let mut min_x = f64::INFINITY;
            let mut min_y = f64::INFINITY;
            let mut max_x = f64::NEG_INFINITY;
            let mut max_y = f64::NEG_INFINITY;
            for segment in &segments {
                if let Some(to) = segment_to_point(segment) {
                    min_x = min_x.min(to[0]);
                    min_y = min_y.min(to[1]);
                    max_x = max_x.max(to[0]);
                    max_y = max_y.max(to[1]);
                }
            }
            if !min_x.is_finite() {
                return None;
            }
            (min_x, min_y, max_x - min_x, max_y - min_y)
        }
    };
    let base = layer_base(layer);
    let corners = [
        (local.0, local.1),
        (local.0 + local.2, local.1),
        (local.0 + local.2, local.1 + local.3),
        (local.0, local.1 + local.3),
    ];
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (x, y) in corners {
        let world = transform_world_point(&base.transform, x, y);
        xs.push(world.0);
        ys.push(world.1);
    }
    Some((xs.iter().cloned().fold(f64::INFINITY, f64::min), ys.iter().cloned().fold(f64::INFINITY, f64::min), xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - xs.iter().cloned().fold(f64::INFINITY, f64::min), ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - ys.iter().cloned().fold(f64::INFINITY, f64::min)))
}

fn segment_to_point(segment: &PathSegment) -> Option<[f64; 2]> {
    match segment {
        PathSegment::Move { to } | PathSegment::Line { to } | PathSegment::Quad { to, .. } | PathSegment::Cubic { to, .. } | PathSegment::Arc { to, .. } => Some(*to),
        PathSegment::Close => None,
    }
}

fn transform_world_point(transform: &DrawTransform, x: f64, y: f64) -> (f64, f64) {
    let sx = x * transform.scale_x;
    let sy = y * transform.scale_y;
    let cos = transform.rotation.cos();
    let sin = transform.rotation.sin();
    (transform.x + sx * cos - sy * sin, transform.y + sx * sin + sy * cos)
}

pub fn flatten_draw_document_to_scene_nodes(doc: &DrawDocument) -> Vec<DrawSceneNode> {
    let mut out = Vec::new();
    fn walk(doc: &DrawDocument, layers: &[DrawLayerNode], out: &mut Vec<DrawSceneNode>) {
        for layer in layers {
            let base = layer_base(layer);
            if !base.visible {
                continue;
            }
            match layer {
                DrawLayerNode::Group(group) => walk(doc, &group.children, out),
                DrawLayerNode::Boolean(boolean) => out.push(DrawSceneNode {
                    id: boolean.base.id.clone(),
                    transform: draw_transform_to_matrix(&boolean.base.transform),
                    segments: Vec::new(),
                    fill: boolean.base.attributes.fill.clone(),
                    stroke: boolean.base.attributes.stroke.clone(),
                    opacity: boolean.base.opacity,
                    blend_mode: boolean.base.blend_mode.clone(),
                    visible: boolean.base.visible,
                    needs_kernel: true,
                    kernel_kind: Some("boolean".into()),
                    kernel_payload: Some(serde_json::json!({ "op": boolean.op, "children": boolean.children })),
                    text: None,
                    image: None,
                }),
                DrawLayerNode::Trace(trace) => out.push(DrawSceneNode {
                    id: trace.base.id.clone(),
                    transform: draw_transform_to_matrix(&trace.base.transform),
                    segments: Vec::new(),
                    fill: trace.base.attributes.fill.clone(),
                    stroke: trace.base.attributes.stroke.clone(),
                    opacity: trace.base.opacity,
                    blend_mode: trace.base.blend_mode.clone(),
                    visible: trace.base.visible,
                    needs_kernel: true,
                    kernel_kind: Some("trace".into()),
                    kernel_payload: Some(serde_json::json!({
                        "sourceKey": trace.source_key,
                        "params": trace.params
                    })),
                    text: None,
                    image: None,
                }),
                DrawLayerNode::Text(text) => out.push(DrawSceneNode {
                    id: text.base.id.clone(),
                    transform: draw_transform_to_matrix(&text.base.transform),
                    segments: Vec::new(),
                    fill: text.base.attributes.fill.clone(),
                    stroke: text.base.attributes.stroke.clone(),
                    opacity: text.base.opacity,
                    blend_mode: text.base.blend_mode.clone(),
                    visible: text.base.visible,
                    needs_kernel: false,
                    kernel_kind: None,
                    kernel_payload: None,
                    text: Some(DrawSceneText { content: text.content.clone(), size: text.size }),
                    image: None,
                }),
                DrawLayerNode::Image(image) => {
                    let src = doc
                        .assets
                        .as_ref()
                        .and_then(|assets| assets.get(&image.image_key))
                        .map(|asset| {
                            if asset.data.starts_with("data:") {
                                asset.data.clone()
                            } else {
                                format!("data:{};base64,{}", asset.mime, asset.data)
                            }
                        })
                        .unwrap_or_default();
                    out.push(DrawSceneNode {
                        id: image.base.id.clone(),
                        transform: draw_transform_to_matrix(&image.base.transform),
                        segments: Vec::new(),
                        fill: image.base.attributes.fill.clone(),
                        stroke: image.base.attributes.stroke.clone(),
                        opacity: image.base.opacity,
                        blend_mode: image.base.blend_mode.clone(),
                        visible: image.base.visible,
                        needs_kernel: false,
                        kernel_kind: None,
                        kernel_payload: None,
                        text: None,
                        image: Some(DrawSceneImage { src, width: image.width, height: image.height }),
                    });
                }
                _ => {
                    let segments = layer_to_path_segments(layer);
                    if segments.is_empty() {
                        continue;
                    }
                    out.push(DrawSceneNode {
                        id: base.id.clone(),
                        transform: draw_transform_to_matrix(&base.transform),
                        segments,
                        fill: base.attributes.fill.clone(),
                        stroke: base.attributes.stroke.clone(),
                        opacity: base.opacity,
                        blend_mode: base.blend_mode.clone(),
                        visible: base.visible,
                        needs_kernel: false,
                        kernel_kind: None,
                        kernel_payload: None,
                        text: None,
                        image: None,
                    });
                }
            }
        }
    }
    walk(doc, &doc.layers, &mut out);
    out
}

pub fn canvas_layer_records(doc: &DrawDocument) -> Vec<DrawCanvasLayerRecord> {
    flatten_draw_layers(&doc.layers)
        .into_iter()
        .filter(|layer| !matches!(layer, DrawLayerNode::Group(_)))
        .map(|layer| {
            let base = layer_base(layer);
            let bounds = draw_layer_world_bounds(layer);
            DrawCanvasLayerRecord {
                id: base.id.clone(),
                kind: layer_kind_label(layer),
                name: base.name.clone(),
                x: bounds.map(|b| b.0),
                y: bounds.map(|b| b.1),
                width: bounds.map(|b| b.2),
                height: bounds.map(|b| b.3),
            }
        })
        .collect()
}

pub fn clone_draw_layer_node(node: &DrawLayerNode, name_suffix: &str) -> DrawLayerNode {
    let mut cloned = node.clone();
    match &mut cloned {
        DrawLayerNode::Shape(shape) => {
            shape.base.id = create_draw_id("shape");
            shape.base.name = format!("{}{name_suffix}", shape.base.name);
        }
        DrawLayerNode::Path(path) => {
            path.base.id = create_draw_id("path");
            path.base.name = format!("{}{name_suffix}", path.base.name);
        }
        DrawLayerNode::Text(text) => {
            text.base.id = create_draw_id("text");
            text.base.name = format!("{}{name_suffix}", text.base.name);
        }
        DrawLayerNode::Image(image) => {
            image.base.id = create_draw_id("image");
            image.base.name = format!("{}{name_suffix}", image.base.name);
        }
        DrawLayerNode::Group(group) => {
            group.base.id = create_draw_id("group");
            group.base.name = format!("{}{name_suffix}", group.base.name);
            group.children = group.children.iter().map(|child| clone_draw_layer_node(child, "")).collect();
        }
        DrawLayerNode::Boolean(boolean) => {
            boolean.base.id = create_draw_id("boolean");
            boolean.base.name = format!("{}{name_suffix}", boolean.base.name);
        }
        DrawLayerNode::Trace(trace) => {
            trace.base.id = create_draw_id("trace");
            trace.base.name = format!("{}{name_suffix}", trace.base.name);
        }
    }
    cloned
}
//#endregion 🔖Domain

//#region 🔖EditOps
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum DrawOp {
    SetLayerVisible { layer_id: String, visible: bool },
    SetLayerLocked { layer_id: String, locked: bool },
    SetLayerOpacity { layer_id: String, opacity: f64 },
    SetLayerBlendMode { layer_id: String, blend_mode: String },
    SetLayerName { layer_id: String, name: String },
    SetLayerTransform { layer_id: String, transform: DrawTransform },
    SetFill { layer_id: String, #[serde(skip_serializing_if = "Option::is_none")] fill: Option<FillStyle> },
    SetStroke { layer_id: String, #[serde(skip_serializing_if = "Option::is_none")] stroke: Option<StrokeStyle> },
    SetBooleanOp { layer_id: String, boolean_op: String },
    SetTraceParams { layer_id: String, params: DrawTraceParams },
    AddLayer { #[serde(skip_serializing_if = "Option::is_none")] parent_id: Option<String>, #[serde(skip_serializing_if = "Option::is_none")] index: Option<usize>, layer: DrawLayerNode },
    DuplicateLayer { layer_id: String },
    RemoveLayer { layer_id: String },
    ReorderLayer { layer_id: String, #[serde(skip_serializing_if = "Option::is_none")] parent_id: Option<String>, index: usize },
    SetActiveTool { tool: String },
    SetCamera { camera: DrawCamera },
    SetDocument { document: DrawDocument },
}

pub fn apply_draw_edit_op(doc: &DrawDocument, edit: &DrawOp) -> DrawDocument {
    match edit {
        DrawOp::SetDocument { document } => document.clone(),
        DrawOp::SetActiveTool { tool } => DrawDocument { active_tool: Some(tool.clone()), ..doc.clone() },
        DrawOp::SetCamera { camera } => DrawDocument { camera: camera.clone(), ..doc.clone() },
        DrawOp::SetLayerVisible { layer_id, visible } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).visible = *visible;
        }),
        DrawOp::SetLayerLocked { layer_id, locked } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).locked = *locked;
        }),
        DrawOp::SetLayerOpacity { layer_id, opacity } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).opacity = *opacity;
        }),
        DrawOp::SetLayerBlendMode { layer_id, blend_mode } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).blend_mode = blend_mode.clone();
        }),
        DrawOp::SetLayerName { layer_id, name } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).name = name.clone();
        }),
        DrawOp::SetLayerTransform { layer_id, transform } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).transform = transform.clone();
        }),
        DrawOp::SetFill { layer_id, fill } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).attributes.fill = fill.clone();
        }),
        DrawOp::SetStroke { layer_id, stroke } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).attributes.stroke = stroke.clone();
        }),
        DrawOp::SetBooleanOp { layer_id, boolean_op } => mutate_draw_layer(doc, layer_id, |layer| {
            if let DrawLayerNode::Boolean(boolean) = layer {
                boolean.op = boolean_op.clone();
            }
        }),
        DrawOp::SetTraceParams { layer_id, params } => mutate_draw_layer(doc, layer_id, |layer| {
            if let DrawLayerNode::Trace(trace) = layer {
                trace.params = params.clone();
            }
        }),
        DrawOp::AddLayer { parent_id, index, layer } => {
            let mut next = doc.clone();
            let at = index.unwrap_or(next.layers.len());
            insert_layer(&mut next.layers, parent_id.as_deref(), at, layer.clone());
            next
        }
        DrawOp::DuplicateLayer { layer_id: source_id } => {
            if let Some(layer) = find_draw_layer(doc, source_id).cloned() {
                let duplicate = clone_draw_layer_node(&layer, " copy");
                let mut next = doc.clone();
                if let Some(location) = find_draw_layer_location(doc, source_id) {
                    insert_layer(&mut next.layers, location.parent_id.as_deref(), location.index + 1, duplicate);
                } else {
                    next.layers.push(duplicate);
                }
                next
            } else {
                doc.clone()
            }
        }
        DrawOp::RemoveLayer { layer_id } => {
            let mut next = doc.clone();
            remove_layer_from_tree(&mut next.layers, layer_id);
            next
        }
        DrawOp::ReorderLayer { layer_id, parent_id, index } => {
            let mut next = doc.clone();
            if let Some(node) = extract_layer_node(&mut next.layers, layer_id) {
                insert_layer(&mut next.layers, parent_id.as_deref(), *index, node);
            }
            next
        }
    }
}

pub fn layer_base_mut(layer: &mut DrawLayerNode) -> &mut DrawLayerBase {
    match layer {
        DrawLayerNode::Shape(shape) => &mut shape.base,
        DrawLayerNode::Path(path) => &mut path.base,
        DrawLayerNode::Text(text) => &mut text.base,
        DrawLayerNode::Image(image) => &mut image.base,
        DrawLayerNode::Group(group) => &mut group.base,
        DrawLayerNode::Boolean(boolean) => &mut boolean.base,
        DrawLayerNode::Trace(trace) => &mut trace.base,
    }
}

pub fn mutate_draw_layer(doc: &DrawDocument, target_id: &str, mutator: impl FnMut(&mut DrawLayerNode)) -> DrawDocument {
    let mut next = doc.clone();
    let mut mutator = mutator;
    update_layer_in_tree(&mut next.layers, target_id, &mut mutator);
    next
}

fn update_layer_in_tree(layers: &mut [DrawLayerNode], target_id: &str, mutator: &mut impl FnMut(&mut DrawLayerNode)) -> bool {
    for layer in layers.iter_mut() {
        if layer_id(layer) == target_id {
            mutator(layer);
            return true;
        }
        if let DrawLayerNode::Group(group) = layer {
            if update_layer_in_tree(&mut group.children, target_id, mutator) {
                return true;
            }
        }
    }
    false
}

fn remove_layer_from_tree(layers: &mut Vec<DrawLayerNode>, target_id: &str) -> bool {
    if let Some(index) = layers.iter().position(|layer| layer_id(layer) == target_id) {
        layers.remove(index);
        return true;
    }
    for layer in layers.iter_mut() {
        if let DrawLayerNode::Group(group) = layer {
            if remove_layer_from_tree(&mut group.children, target_id) {
                return true;
            }
        }
    }
    false
}

fn extract_layer_node(layers: &mut Vec<DrawLayerNode>, target_id: &str) -> Option<DrawLayerNode> {
    if let Some(index) = layers.iter().position(|layer| layer_id(layer) == target_id) {
        return Some(layers.remove(index));
    }
    for layer in layers.iter_mut() {
        if let DrawLayerNode::Group(group) = layer {
            if let Some(node) = extract_layer_node(&mut group.children, target_id) {
                return Some(node);
            }
        }
    }
    None
}

fn insert_layer(layers: &mut Vec<DrawLayerNode>, parent_id: Option<&str>, index: usize, node: DrawLayerNode) {
    if let Some(parent_id) = parent_id {
        if !insert_layer_in_parent(layers, parent_id, index, node.clone()) {
            layers.push(node);
        }
    } else {
        let at = index.min(layers.len());
        layers.insert(at, node);
    }
}

fn insert_layer_in_parent(layers: &mut [DrawLayerNode], parent_id: &str, index: usize, node: DrawLayerNode) -> bool {
    for layer in layers.iter_mut() {
        if let DrawLayerNode::Group(group) = layer {
            if group.base.id == parent_id {
                let at = index.min(group.children.len());
                group.children.insert(at, node);
                return true;
            }
            if insert_layer_in_parent(&mut group.children, parent_id, index, node.clone()) {
                return true;
            }
        }
    }
    false
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayerLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub index: usize,
}

pub fn find_draw_layer_location(doc: &DrawDocument, target_id: &str) -> Option<DrawLayerLocation> {
    fn search(layers: &[DrawLayerNode], parent_id: Option<String>, target_id: &str) -> Option<DrawLayerLocation> {
        for (index, layer) in layers.iter().enumerate() {
            if layer_id(layer) == target_id {
                return Some(DrawLayerLocation { parent_id, index });
            }
            if let DrawLayerNode::Group(group) = layer {
                if let Some(found) = search(&group.children, Some(group.base.id.clone()), target_id) {
                    return Some(found);
                }
            }
        }
        None
    }
    search(&doc.layers, None, target_id)
}

pub fn create_layer_by_kind(kind: &str) -> DrawLayerNode {
    if let Some(shape_kind) = kind.strip_prefix("shape:") {
        return match shape_kind {
            "rect" => create_draw_shape_layer_rect("Rectangle"),
            "ellipse" => DrawLayerNode::Shape(DrawShapeBody {
                base: default_layer_base("Ellipse"),
                shape_kind: "ellipse".into(),
                rect: None,
                ellipse: Some(DrawEllipse { cx: 0.0, cy: 0.0, rx: 64.0, ry: 48.0 }),
                circle: None,
                line: None,
                polygon: None,
            }),
            "line" => DrawLayerNode::Shape(DrawShapeBody {
                base: default_layer_base("Line"),
                shape_kind: "line".into(),
                rect: None,
                ellipse: None,
                circle: None,
                line: Some(DrawLine { x1: 0.0, y1: 0.0, x2: 128.0, y2: 0.0 }),
                polygon: None,
            }),
            "polygon" => DrawLayerNode::Shape(DrawShapeBody {
                base: default_layer_base("Polygon"),
                shape_kind: "polygon".into(),
                rect: None,
                ellipse: None,
                circle: None,
                line: None,
                polygon: Some(DrawPolygon { points: vec![[0.0, 0.0], [64.0, 0.0], [32.0, 48.0]] }),
            }),
            _ => create_draw_shape_layer_rect("Shape"),
        };
    }
    match kind {
        "path" => create_draw_path_layer("Path", Vec::new()),
        "text" => create_draw_text_layer("Text"),
        "image" => create_draw_image_layer("Image", "image-source"),
        "group" => create_draw_group_layer("Group"),
        "boolean" => create_draw_boolean_layer("Boolean", "union", Vec::new()),
        "trace" => create_draw_trace_layer("Trace", "trace-source"),
        _ => create_draw_path_layer("Path", Vec::new()),
    }
}

pub fn hex_to_rgba(hex: &str, alpha: f64) -> [f64; 4] {
    let normalized = hex.trim_start_matches('#');
    let value = if normalized.len() == 3 {
        normalized.chars().map(|c| format!("{c}{c}")).collect::<String>()
    } else {
        normalized.to_string()
    };
    let parse = |start: usize| u8::from_str_radix(&value[start..start + 2], 16).unwrap_or(0) as f64 / 255.0;
    [parse(0), parse(2), parse(4), alpha]
}

pub fn rgba_to_hex(color: [f64; 4]) -> String {
    let channel = |value: f64| format!("{:02x}", (value.clamp(0.0, 1.0) * 255.0).round() as u8);
    format!("#{}{}{}", channel(color[0]), channel(color[1]), channel(color[2]))
}

pub fn patch_layer_field(doc: &DrawDocument, layer_id: &str, field: &str, value: &serde_json::Value) -> DrawDocument {
    let Some(layer) = find_draw_layer(doc, layer_id) else {
        return doc.clone();
    };
    match field {
        "name" => apply_draw_edit_op(doc, &DrawOp::SetLayerName { layer_id: layer_id.into(), name: value.as_str().unwrap_or("").into() }),
        "opacity" => apply_draw_edit_op(doc, &DrawOp::SetLayerOpacity { layer_id: layer_id.into(), opacity: value.as_f64().unwrap_or(1.0) }),
        "visible" => apply_draw_edit_op(doc, &DrawOp::SetLayerVisible { layer_id: layer_id.into(), visible: value.as_bool().unwrap_or(true) }),
        "locked" => apply_draw_edit_op(doc, &DrawOp::SetLayerLocked { layer_id: layer_id.into(), locked: value.as_bool().unwrap_or(false) }),
        "blendMode" => apply_draw_edit_op(doc, &DrawOp::SetLayerBlendMode { layer_id: layer_id.into(), blend_mode: value.as_str().unwrap_or("normal").into() }),
        "booleanOp" => apply_draw_edit_op(doc, &DrawOp::SetBooleanOp { layer_id: layer_id.into(), boolean_op: value.as_str().unwrap_or("union").into() }),
        "transformX" | "transformY" | "transformScaleX" | "transformScaleY" | "transformRotation" => {
            let mut transform = layer_base(layer).transform.clone();
            match field {
                "transformX" => transform.x = value.as_f64().unwrap_or(0.0),
                "transformY" => transform.y = value.as_f64().unwrap_or(0.0),
                "transformScaleX" => transform.scale_x = value.as_f64().unwrap_or(1.0),
                "transformScaleY" => transform.scale_y = value.as_f64().unwrap_or(1.0),
                _ => transform.rotation = value.as_f64().unwrap_or(0.0),
            }
            apply_draw_edit_op(doc, &DrawOp::SetLayerTransform { layer_id: layer_id.into(), transform })
        }
        "fillColor" => {
            let alpha = layer_base(layer).attributes.fill.as_ref().and_then(|fill| match fill {
                FillStyle::Solid { color } => Some(color[3]),
                FillStyle::LinearGradient { .. } | FillStyle::RadialGradient { .. } => Some(1.0),
            }).unwrap_or(1.0);
            apply_draw_edit_op(doc, &DrawOp::SetFill {
                layer_id: layer_id.into(),
                fill: Some(FillStyle::Solid { color: hex_to_rgba(value.as_str().unwrap_or("#000000"), alpha) }),
            })
        }
        "strokeWidth" => {
            let stroke = layer_base(layer).attributes.stroke.clone().unwrap_or(StrokeStyle {
                color: [0.0, 0.0, 0.0, 1.0],
                width: 1.0,
                cap: "butt".into(),
                join: "miter".into(),
                dash: None,
            });
            apply_draw_edit_op(doc, &DrawOp::SetStroke {
                layer_id: layer_id.into(),
                stroke: Some(StrokeStyle { width: value.as_f64().unwrap_or(1.0), ..stroke }),
            })
        }
        "traceThreshold" => {
            if let DrawLayerNode::Trace(trace) = layer {
                let mut params = trace.params.clone();
                params.threshold = value.as_f64().unwrap_or(0.5);
                apply_draw_edit_op(doc, &DrawOp::SetTraceParams { layer_id: layer_id.into(), params })
            } else {
                doc.clone()
            }
        }
        "traceSimplify" => {
            if let DrawLayerNode::Trace(trace) = layer {
                let mut params = trace.params.clone();
                params.simplify_epsilon = value.as_f64().unwrap_or(1.5);
                apply_draw_edit_op(doc, &DrawOp::SetTraceParams { layer_id: layer_id.into(), params })
            } else {
                doc.clone()
            }
        }
        _ => doc.clone(),
    }
}
//#endregion 🔖EditOps

//#region 🔖Vcs
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayerBasePatch {
    pub visible: Option<bool>,
    pub locked: Option<bool>,
    pub name: Option<String>,
    pub opacity: Option<f64>,
    pub blend_mode: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayerTreePatch {
    pub layer_id: String,
    pub base: DrawLayerBasePatch,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayerTreeAdd {
    pub parent_id: Option<String>,
    pub index: Option<usize>,
    pub layer: DrawLayerNode,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawDiff {
    pub document: Option<DrawDocument>,
    pub active_tool: Option<Option<String>>,
    pub camera: Option<DrawCamera>,
    pub layer_patches: Vec<DrawLayerTreePatch>,
    pub layers_removed: Vec<String>,
    pub layers_added: Vec<DrawLayerTreeAdd>,
}

impl Default for DrawDiff {
    fn default() -> Self {
        Self {
            document: None,
            active_tool: None,
            camera: None,
            layer_patches: Vec::new(),
            layers_removed: Vec::new(),
            layers_added: Vec::new(),
        }
    }
}

impl OperationDiff<DrawDocument> for DrawDiff {
    fn apply(&self, projection: &DrawDocument) -> DrawDocument {
        let mut next = projection.clone();
        if let Some(document) = &self.document {
            return document.clone();
        }
        if let Some(tool) = &self.active_tool {
            next.active_tool = tool.clone();
        }
        if let Some(camera) = &self.camera {
            next.camera = camera.clone();
        }
        for patch in &self.layer_patches {
            update_layer_in_tree(&mut next.layers, &patch.layer_id, &mut |layer| {
                let base = layer_base_mut(layer);
                if let Some(visible) = patch.base.visible {
                    base.visible = visible;
                }
                if let Some(locked) = patch.base.locked {
                    base.locked = locked;
                }
                if let Some(name) = &patch.base.name {
                    base.name = name.clone();
                }
                if let Some(opacity) = patch.base.opacity {
                    base.opacity = opacity;
                }
                if let Some(blend_mode) = &patch.base.blend_mode {
                    base.blend_mode = blend_mode.clone();
                }
            });
        }
        for layer_id in &self.layers_removed {
            remove_layer_from_tree(&mut next.layers, layer_id);
        }
        for add in &self.layers_added {
            let index = add.index.unwrap_or_else(|| next.layers.len());
            insert_layer(&mut next.layers, add.parent_id.as_deref(), index, add.layer.clone());
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            *self = other;
            return;
        }
        if other.active_tool.is_some() {
            self.active_tool = other.active_tool;
        }
        if other.camera.is_some() {
            self.camera = other.camera;
        }
        self.layer_patches.extend(other.layer_patches);
        self.layers_removed.extend(other.layers_removed);
        self.layers_added.extend(other.layers_added);
    }
}

impl Operation<DrawDocument> for DrawOp {
    type Diff = DrawDiff;

    fn diff(&self, _projection: &DrawDocument) -> DrawDiff {
        match self {
            DrawOp::SetDocument { document } => DrawDiff { document: Some(document.clone()), ..Default::default() },
            DrawOp::SetActiveTool { tool } => DrawDiff { active_tool: Some(Some(tool.clone())), ..Default::default() },
            DrawOp::SetCamera { camera } => DrawDiff { camera: Some(camera.clone()), ..Default::default() },
            DrawOp::SetLayerVisible { layer_id, visible } => DrawDiff {
                layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { visible: Some(*visible), ..Default::default() } }],
                ..Default::default()
            },
            DrawOp::SetLayerLocked { layer_id, locked } => DrawDiff {
                layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { locked: Some(*locked), ..Default::default() } }],
                ..Default::default()
            },
            DrawOp::SetLayerName { layer_id, name } => DrawDiff {
                layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { name: Some(name.clone()), ..Default::default() } }],
                ..Default::default()
            },
            DrawOp::SetLayerOpacity { layer_id, opacity } => DrawDiff {
                layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { opacity: Some(*opacity), ..Default::default() } }],
                ..Default::default()
            },
            DrawOp::SetLayerBlendMode { layer_id, blend_mode } => DrawDiff {
                layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { blend_mode: Some(blend_mode.clone()), ..Default::default() } }],
                ..Default::default()
            },
            DrawOp::AddLayer { parent_id, index, layer } => DrawDiff {
                layers_added: vec![DrawLayerTreeAdd { parent_id: parent_id.clone(), index: *index, layer: layer.clone() }],
                ..Default::default()
            },
            DrawOp::RemoveLayer { layer_id } => DrawDiff { layers_removed: vec![layer_id.clone()], ..Default::default() },
            _ => DrawDiff { document: Some(apply_draw_edit_op(_projection, self)), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &DrawDocument) -> Vec<Self> {
        vec![DrawOp::SetDocument { document: projection.clone() }]
    }
}
//#endregion 🔖Vcs

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct DrawDocumentVcs {
        store: RefCell<DrawStore>,
    }

    #[wasm_bindgen]
    impl DrawDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<DrawDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: DrawEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    DrawStore::new(envelope)
                }
                None => DrawStore::new(create_document_vcs_envelope(DRAW_DOCUMENT_SCHEMA, "draw", empty_draw_projection(), None)),
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

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use vcs::create_document_vcs_envelope;

    #[test]
    fn default_document_has_path_layer() {
        let doc = default_draw_document("test", None);
        assert_eq!(doc.layers.len(), 1);
        assert!(matches!(doc.layers[0], DrawLayerNode::Path(_)));
    }

    #[test]
    fn apply_add_and_patch_layer() {
        let doc = empty_draw_projection();
        let layer = create_draw_shape_layer_rect("Rect");
        let id = layer_id(&layer).to_string();
        let next = apply_draw_edit_op(&doc, &DrawOp::AddLayer { parent_id: None, index: None, layer });
        assert_eq!(next.layers.len(), 2);
        let renamed = apply_draw_edit_op(&next, &DrawOp::SetLayerName { layer_id: id.clone(), name: "Box".into() });
        assert_eq!(find_draw_layer(&renamed, &id).map(|layer| layer_base(layer).name.as_str()), Some("Box"));
    }

    #[test]
    fn scene_nodes_include_shape_bounds() {
        let layer = create_draw_shape_layer_rect("Rect");
        let doc = DrawDocument {
            layers: vec![layer],
            ..default_draw_document("scene", None)
        };
        let nodes = flatten_draw_document_to_scene_nodes(&doc);
        assert_eq!(nodes.len(), 1);
        assert!(!nodes[0].segments.is_empty());
    }
}
//#endregion 🧪Tests
