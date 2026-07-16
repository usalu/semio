//! ✏️ Draw document domain + typed VCS on `vcs`.

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
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
pub const DRAW_UTILITY_IDS: &[&str] = &[
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_rule: Option<String>,
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

fn scene_node_for_path(base: &DrawLayerBase, segments: Vec<PathSegment>) -> DrawSceneNode {
    DrawSceneNode {
        id: base.id.clone(),
        transform: draw_transform_to_matrix(&base.transform),
        segments,
        fill: base.attributes.fill.clone(),
        stroke: base.attributes.stroke.clone(),
        opacity: base.opacity,
        blend_mode: base.blend_mode.clone(),
        visible: base.visible,
        fill_rule: Some("evenodd".into()),
        text: None,
        image: None,
    }
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
                DrawLayerNode::Boolean(boolean) => {
                    let segments = resolve_boolean_layer_segments(doc, boolean);
                    if segments.is_empty() {
                        continue;
                    }
                    out.push(scene_node_for_path(&boolean.base, segments));
                }
                DrawLayerNode::Trace(trace) => {
                    let segments = resolve_trace_layer_segments(doc, trace);
                    if segments.is_empty() {
                        continue;
                    }
                    let mut node = scene_node_for_path(&trace.base, segments);
                    if node.fill.is_none() {
                        if let Some(stroke) = node.stroke.take() {
                            node.fill = Some(FillStyle::Solid { color: stroke.color });
                        }
                    }
                    out.push(node);
                }
                DrawLayerNode::Text(text) => out.push(DrawSceneNode {
                    id: text.base.id.clone(),
                    transform: draw_transform_to_matrix(&text.base.transform),
                    segments: Vec::new(),
                    fill: text.base.attributes.fill.clone(),
                    stroke: text.base.attributes.stroke.clone(),
                    opacity: text.base.opacity,
                    blend_mode: text.base.blend_mode.clone(),
                    visible: text.base.visible,
                    fill_rule: None,
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
                        fill_rule: None,
                        text: None,
                        image: Some(DrawSceneImage { src, width: image.width, height: image.height }),
                    });
                }
                _ => {
                    let segments = flatten_curve_segments(&layer_to_path_segments(layer));
                    if segments.is_empty() {
                        continue;
                    }
                    out.push(scene_node_for_path(base, segments));
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

//#region 🔖SegmentGeometry
fn draw_map_point_by_matrix(matrix: [f64; 6], point: [f64; 2]) -> [f64; 2] {
    let [a, b, c, d, e, f] = matrix;
    [a * point[0] + c * point[1] + e, b * point[0] + d * point[1] + f]
}

pub fn transform_path_segments(segments: &[PathSegment], transform: &DrawTransform) -> Vec<PathSegment> {
    let matrix = draw_transform_to_matrix(transform);
    segments
        .iter()
        .map(|segment| match segment {
            PathSegment::Move { to } => PathSegment::Move { to: draw_map_point_by_matrix(matrix, *to) },
            PathSegment::Line { to } => PathSegment::Line { to: draw_map_point_by_matrix(matrix, *to) },
            PathSegment::Quad { ctrl, to } => PathSegment::Quad {
                ctrl: draw_map_point_by_matrix(matrix, *ctrl),
                to: draw_map_point_by_matrix(matrix, *to),
            },
            PathSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic {
                ctrl1: draw_map_point_by_matrix(matrix, *ctrl1),
                ctrl2: draw_map_point_by_matrix(matrix, *ctrl2),
                to: draw_map_point_by_matrix(matrix, *to),
            },
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::Arc {
                rx: *rx,
                ry: *ry,
                rotation: *rotation,
                large_arc: *large_arc,
                sweep: *sweep,
                to: draw_map_point_by_matrix(matrix, *to),
            },
            PathSegment::Close => PathSegment::Close,
        })
        .collect()
}

pub fn scale_path_segments(segments: &[PathSegment], scale_x: f64, scale_y: f64) -> Vec<PathSegment> {
    if scale_x == 1.0 && scale_y == 1.0 {
        return segments.to_vec();
    }
    transform_path_segments(segments, &DrawTransform { x: 0.0, y: 0.0, scale_x, scale_y, rotation: 0.0 })
}

pub fn split_path_segments_by_contour(segments: &[PathSegment]) -> Vec<Vec<PathSegment>> {
    let mut contours = Vec::new();
    let mut current: Vec<PathSegment> = Vec::new();
    for segment in segments {
        if matches!(segment, PathSegment::Move { .. }) && !current.is_empty() {
            contours.push(std::mem::take(&mut current));
        }
        current.push(segment.clone());
    }
    if !current.is_empty() {
        contours.push(current);
    }
    if contours.is_empty() {
        contours.push(Vec::new());
    }
    contours
}

pub fn path_segments_bounds(segments: &[PathSegment]) -> Option<(f64, f64, f64, f64)> {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for segment in segments {
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
    Some((min_x, min_y, max_x - min_x, max_y - min_y))
}

pub fn filter_path_segments_by_contour_area(segments: &[PathSegment], min_area: f64) -> Vec<PathSegment> {
    if min_area <= 0.0 {
        return segments.to_vec();
    }
    let mut kept = Vec::new();
    for contour in split_path_segments_by_contour(segments) {
        let Some((_, _, width, height)) = path_segments_bounds(&contour) else { continue };
        if width * height < min_area {
            continue;
        }
        kept.extend(contour);
    }
    kept
}

fn arc_ellipse_point(unit: [f64; 2], rx: f64, ry: f64, cos_phi: f64, sin_phi: f64, cx: f64, cy: f64) -> [f64; 2] {
    let x = unit[0] * rx;
    let y = unit[1] * ry;
    [cos_phi * x - sin_phi * y + cx, sin_phi * x + cos_phi * y + cy]
}

fn arc_vector_angle(ux: f64, uy: f64, vx: f64, vy: f64) -> f64 {
    let sign = if ux * vy - uy * vx < 0.0 { -1.0 } else { 1.0 };
    let dot = (ux * vx + uy * vy).clamp(-1.0, 1.0);
    sign * dot.acos()
}

fn arc_approx_unit_arc(ang1: f64, ang2: f64) -> ([f64; 2], [f64; 2], [f64; 2]) {
    let a = (4.0 / 3.0) * ((ang2 - ang1) / 4.0).tan();
    let (sin1, cos1) = ang1.sin_cos();
    let (sin2, cos2) = ang2.sin_cos();
    ([cos1 - sin1 * a, sin1 + cos1 * a], [cos2 + sin2 * a, sin2 - cos2 * a], [cos2, sin2])
}

/// 🌙 Converts one SVG endpoint-parameterized arc into cubic Bézier control triples (SVG spec F.6.5).
fn arc_segment_to_cubics(from: [f64; 2], rx: f64, ry: f64, rotation_deg: f64, large_arc: bool, sweep: bool, to: [f64; 2]) -> Vec<([f64; 2], [f64; 2], [f64; 2])> {
    if rx.abs() < 1e-9 || ry.abs() < 1e-9 {
        return Vec::new();
    }
    let mut rx = rx.abs();
    let mut ry = ry.abs();
    let phi = rotation_deg.to_radians();
    let (sin_phi, cos_phi) = phi.sin_cos();
    let dx = (from[0] - to[0]) / 2.0;
    let dy = (from[1] - to[1]) / 2.0;
    let pxp = cos_phi * dx + sin_phi * dy;
    let pyp = -sin_phi * dx + cos_phi * dy;
    if pxp == 0.0 && pyp == 0.0 {
        return Vec::new();
    }
    let lambda = (pxp * pxp) / (rx * rx) + (pyp * pyp) / (ry * ry);
    if lambda > 1.0 {
        let factor = lambda.sqrt();
        rx *= factor;
        ry *= factor;
    }
    let rx_sq = rx * rx;
    let ry_sq = ry * ry;
    let pxp_sq = pxp * pxp;
    let pyp_sq = pyp * pyp;
    let mut radicand = rx_sq * ry_sq - rx_sq * pyp_sq - ry_sq * pxp_sq;
    if radicand < 0.0 {
        radicand = 0.0;
    }
    radicand /= rx_sq * pyp_sq + ry_sq * pxp_sq;
    let coef = radicand.sqrt() * if large_arc == sweep { -1.0 } else { 1.0 };
    let centerxp = coef * (rx / ry) * pyp;
    let centeryp = coef * -(ry / rx) * pxp;
    let cx = cos_phi * centerxp - sin_phi * centeryp + (from[0] + to[0]) / 2.0;
    let cy = sin_phi * centerxp + cos_phi * centeryp + (from[1] + to[1]) / 2.0;
    let vx1 = (pxp - centerxp) / rx;
    let vy1 = (pyp - centeryp) / ry;
    let vx2 = (-pxp - centerxp) / rx;
    let vy2 = (-pyp - centeryp) / ry;
    let ang1 = arc_vector_angle(1.0, 0.0, vx1, vy1);
    let mut ang2 = arc_vector_angle(vx1, vy1, vx2, vy2);
    if !sweep && ang2 > 0.0 {
        ang2 -= std::f64::consts::TAU;
    }
    if sweep && ang2 < 0.0 {
        ang2 += std::f64::consts::TAU;
    }
    let mut ratio = ang2.abs() / std::f64::consts::FRAC_PI_2;
    if (1.0 - ratio).abs() < 1e-7 {
        ratio = 1.0;
    }
    let segment_count = ratio.ceil().max(1.0) as usize;
    let delta = ang2 / segment_count as f64;
    let mut cubics = Vec::with_capacity(segment_count);
    let mut angle = ang1;
    for _ in 0..segment_count {
        let (unit_ctrl1, unit_ctrl2, unit_to) = arc_approx_unit_arc(angle, angle + delta);
        cubics.push((
            arc_ellipse_point(unit_ctrl1, rx, ry, cos_phi, sin_phi, cx, cy),
            arc_ellipse_point(unit_ctrl2, rx, ry, cos_phi, sin_phi, cx, cy),
            arc_ellipse_point(unit_to, rx, ry, cos_phi, sin_phi, cx, cy),
        ));
        angle += delta;
    }
    cubics
}

/// 🌙 Flattens `Arc` segments into `Cubic` runs so downstream consumers (booleans, canvas hosts) never see SVG endpoint arcs.
pub fn flatten_curve_segments(segments: &[PathSegment]) -> Vec<PathSegment> {
    let mut out = Vec::with_capacity(segments.len());
    let mut cursor = [0.0, 0.0];
    for segment in segments {
        match segment {
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => {
                let cubics = arc_segment_to_cubics(cursor, *rx, *ry, *rotation, *large_arc, *sweep, *to);
                if cubics.is_empty() {
                    out.push(PathSegment::Line { to: *to });
                } else {
                    for (ctrl1, ctrl2, point) in cubics {
                        out.push(PathSegment::Cubic { ctrl1, ctrl2, to: point });
                    }
                }
                cursor = *to;
            }
            other => {
                if let Some(to) = segment_to_point(other) {
                    cursor = to;
                }
                out.push(other.clone());
            }
        }
    }
    out
}

pub fn draw_layer_descendant_leaf_ids(layer: &DrawLayerNode) -> Vec<String> {
    match layer {
        DrawLayerNode::Group(group) => group.children.iter().flat_map(draw_layer_descendant_leaf_ids).collect(),
        _ => vec![layer_id(layer).to_string()],
    }
}

const CURVE_LINE_SAMPLE_STEPS: usize = 16;

fn sample_quad_points(from: [f64; 2], ctrl: [f64; 2], to: [f64; 2], steps: usize) -> Vec<[f64; 2]> {
    (1..=steps)
        .map(|step| {
            let t = step as f64 / steps as f64;
            let mt = 1.0 - t;
            [
                mt * mt * from[0] + 2.0 * mt * t * ctrl[0] + t * t * to[0],
                mt * mt * from[1] + 2.0 * mt * t * ctrl[1] + t * t * to[1],
            ]
        })
        .collect()
}

fn sample_cubic_points(from: [f64; 2], ctrl1: [f64; 2], ctrl2: [f64; 2], to: [f64; 2], steps: usize) -> Vec<[f64; 2]> {
    (1..=steps)
        .map(|step| {
            let t = step as f64 / steps as f64;
            let mt = 1.0 - t;
            [
                mt * mt * mt * from[0] + 3.0 * mt * mt * t * ctrl1[0] + 3.0 * mt * t * t * ctrl2[0] + t * t * t * to[0],
                mt * mt * mt * from[1] + 3.0 * mt * mt * t * ctrl1[1] + 3.0 * mt * t * t * ctrl2[1] + t * t * t * to[1],
            ]
        })
        .collect()
}

/// 📏 Flattens `Quad`/`Cubic`/`Arc` segments into `Line` segments — the planar boolean kernel only understands polygons.
pub fn flatten_segments_to_lines(segments: &[PathSegment]) -> Vec<PathSegment> {
    let curved = flatten_curve_segments(segments);
    let mut out = Vec::with_capacity(curved.len());
    let mut cursor = [0.0, 0.0];
    for segment in &curved {
        match segment {
            PathSegment::Quad { ctrl, to } => {
                for point in sample_quad_points(cursor, *ctrl, *to, CURVE_LINE_SAMPLE_STEPS) {
                    out.push(PathSegment::Line { to: point });
                }
                cursor = *to;
            }
            PathSegment::Cubic { ctrl1, ctrl2, to } => {
                for point in sample_cubic_points(cursor, *ctrl1, *ctrl2, *to, CURVE_LINE_SAMPLE_STEPS) {
                    out.push(PathSegment::Line { to: point });
                }
                cursor = *to;
            }
            other => {
                if let Some(to) = segment_to_point(other) {
                    cursor = to;
                }
                out.push(other.clone());
            }
        }
    }
    out
}
//#endregion 🔖SegmentGeometry

//#region 🔖KernelResolve
fn to_kernel_segment(segment: &PathSegment) -> kernel_2d_engine::PathSegment {
    use kernel_2d_engine::PathSegment as KernelSegment;
    match segment {
        PathSegment::Move { to } => KernelSegment::Move { to: *to },
        PathSegment::Line { to } => KernelSegment::Line { to: *to },
        PathSegment::Quad { ctrl, to } => KernelSegment::Quad { ctrl: *ctrl, to: *to },
        PathSegment::Cubic { ctrl1, ctrl2, to } => KernelSegment::Cubic { ctrl1: *ctrl1, ctrl2: *ctrl2, to: *to },
        PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => {
            KernelSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to }
        }
        PathSegment::Close => KernelSegment::Close,
    }
}

fn from_kernel_segment(segment: &kernel_2d_engine::PathSegment) -> PathSegment {
    use kernel_2d_engine::PathSegment as KernelSegment;
    match segment {
        KernelSegment::Move { to } => PathSegment::Move { to: *to },
        KernelSegment::Line { to } => PathSegment::Line { to: *to },
        KernelSegment::Quad { ctrl, to } => PathSegment::Quad { ctrl: *ctrl, to: *to },
        KernelSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic { ctrl1: *ctrl1, ctrl2: *ctrl2, to: *to },
        KernelSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => {
            PathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to }
        }
        KernelSegment::Close => PathSegment::Close,
    }
}

fn to_kernel_segments(segments: &[PathSegment]) -> Vec<kernel_2d_engine::PathSegment> {
    segments.iter().map(to_kernel_segment).collect()
}

fn from_kernel_segments(segments: &[kernel_2d_engine::PathSegment]) -> Vec<PathSegment> {
    segments.iter().map(from_kernel_segment).collect()
}

/// 🪢 Resolves a boolean layer's children (each transformed by its own local transform) through the planar kernel.
fn resolve_boolean_layer_segments(doc: &DrawDocument, boolean: &DrawBooleanBody) -> Vec<PathSegment> {
    let child_segments: Vec<Vec<PathSegment>> = boolean
        .children
        .iter()
        .filter_map(|child_id| find_draw_layer(doc, child_id))
        .map(|child| transform_path_segments(&flatten_segments_to_lines(&layer_to_path_segments(child)), &layer_base(child).transform))
        .filter(|segments| !segments.is_empty())
        .collect();
    if child_segments.is_empty() {
        return Vec::new();
    }
    let kernel_inputs: Vec<Vec<kernel_2d_engine::PathSegment>> = child_segments.iter().map(|segments| to_kernel_segments(segments)).collect();
    match kernel_2d_rs::booleans::boolean_paths_many(&kernel_inputs, &boolean.op) {
        Ok(result) => from_kernel_segments(&result),
        Err(_) => Vec::new(),
    }
}

/// 🖼️ Decodes a (possibly resized) PNG asset into an 8-bit luma buffer, matching the premigration canvas-based decode.
fn decode_draw_image_asset_luma(asset: &DrawImageAsset) -> Option<(u32, u32, Vec<u8>)> {
    let base64_data = match asset.data.strip_prefix("data:") {
        Some(rest) => rest.split_once(',').map(|(_, data)| data).unwrap_or(rest),
        None => asset.data.as_str(),
    };
    let bytes = BASE64.decode(base64_data).ok()?;
    let decoded = image::load_from_memory(&bytes).ok()?;
    let target_width = asset.width.unwrap_or(decoded.width());
    let target_height = asset.height.unwrap_or(decoded.height());
    let rgba = if target_width == decoded.width() && target_height == decoded.height() {
        decoded.to_rgba8()
    } else {
        image::imageops::resize(&decoded.to_rgba8(), target_width, target_height, image::imageops::FilterType::Triangle)
    };
    let mut luma = vec![0u8; (target_width as usize) * (target_height as usize)];
    for (index, pixel) in rgba.pixels().enumerate() {
        let [r, g, b, a] = pixel.0;
        luma[index] = ((r as f64 * 0.299 + g as f64 * 0.587 + b as f64 * 0.114) * (a as f64 / 255.0)).round() as u8;
    }
    Some((target_width, target_height, luma))
}

/// 📐 Premigration artboard resolution: explicit artboard wins, else layer bounds excluding group/boolean/trace kinds.
pub fn resolve_draw_artboard(doc: &DrawDocument) -> Option<DrawArtboard> {
    if let Some(artboard) = &doc.artboard {
        if artboard.width > 0.0 && artboard.height > 0.0 {
            return Some(artboard.clone());
        }
    }
    let mut max_x = 0.0_f64;
    let mut max_y = 0.0_f64;
    for layer in flatten_draw_layers(&doc.layers) {
        if matches!(layer, DrawLayerNode::Trace(_) | DrawLayerNode::Boolean(_) | DrawLayerNode::Group(_)) {
            continue;
        }
        if let Some((x, y, width, height)) = draw_layer_world_bounds(layer) {
            max_x = max_x.max(x + width);
            max_y = max_y.max(y + height);
        }
    }
    if max_x <= 0.0 || max_y <= 0.0 {
        return None;
    }
    Some(DrawArtboard { width: max_x, height: max_y })
}

/// 🔍 Resolves a trace layer's bitmap source into simplified, artboard-scaled contour segments.
fn resolve_trace_layer_segments(doc: &DrawDocument, trace: &DrawTraceBody) -> Vec<PathSegment> {
    let Some(assets) = &doc.assets else { return Vec::new() };
    let Some(asset) = assets.get(&trace.source_key) else { return Vec::new() };
    let Some((width, height, luma)) = decode_draw_image_asset_luma(asset) else { return Vec::new() };
    let traced = match kernel_2d_rs::trace::trace_bitmap_paths(width, height, &luma, trace.params.threshold, trace.params.simplify_epsilon) {
        Ok(segments) => from_kernel_segments(&segments),
        Err(_) => return Vec::new(),
    };
    let scaled = match resolve_draw_artboard(doc) {
        Some(artboard) if width > 0 && height > 0 => scale_path_segments(&traced, artboard.width / width as f64, artboard.height / height as f64),
        _ => traced,
    };
    filter_path_segments_by_contour_area(&scaled, 6.0)
}
//#endregion 🔖KernelResolve

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
    SetCamera { camera: DrawCamera },
    SetDocument { document: DrawDocument },
}

pub fn apply_draw_edit_op(doc: &DrawDocument, edit: &DrawOp) -> DrawDocument {
    match edit {
        DrawOp::SetDocument { document } => document.clone(),
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

/// 🩹 Resolves an inspector field write (`name`/`opacity`/`fillColor`/`transformX`/…) to the granular
/// {@link DrawOp} that carries it, so field edits flow through the typed VCS as convergent operations
/// instead of whole-document snapshots. Returns `None` for a missing layer or an unmapped field.
pub fn draw_op_for_layer_field(doc: &DrawDocument, layer_id: &str, field: &str, value: &serde_json::Value) -> Option<DrawOp> {
    let layer = find_draw_layer(doc, layer_id)?;
    let op = match field {
        "name" => DrawOp::SetLayerName { layer_id: layer_id.into(), name: value.as_str().unwrap_or("").into() },
        "opacity" => DrawOp::SetLayerOpacity { layer_id: layer_id.into(), opacity: value.as_f64().unwrap_or(1.0) },
        "visible" => DrawOp::SetLayerVisible { layer_id: layer_id.into(), visible: value.as_bool().unwrap_or(true) },
        "locked" => DrawOp::SetLayerLocked { layer_id: layer_id.into(), locked: value.as_bool().unwrap_or(false) },
        "blendMode" => DrawOp::SetLayerBlendMode { layer_id: layer_id.into(), blend_mode: value.as_str().unwrap_or("normal").into() },
        "booleanOp" => DrawOp::SetBooleanOp { layer_id: layer_id.into(), boolean_op: value.as_str().unwrap_or("union").into() },
        "transformX" | "transformY" | "transformScaleX" | "transformScaleY" | "transformRotation" => {
            let mut transform = layer_base(layer).transform.clone();
            match field {
                "transformX" => transform.x = value.as_f64().unwrap_or(0.0),
                "transformY" => transform.y = value.as_f64().unwrap_or(0.0),
                "transformScaleX" => transform.scale_x = value.as_f64().unwrap_or(1.0),
                "transformScaleY" => transform.scale_y = value.as_f64().unwrap_or(1.0),
                _ => transform.rotation = value.as_f64().unwrap_or(0.0),
            }
            DrawOp::SetLayerTransform { layer_id: layer_id.into(), transform }
        }
        "fillColor" => {
            let alpha = layer_base(layer).attributes.fill.as_ref().and_then(|fill| match fill {
                FillStyle::Solid { color } => Some(color[3]),
                FillStyle::LinearGradient { .. } | FillStyle::RadialGradient { .. } => Some(1.0),
            }).unwrap_or(1.0);
            DrawOp::SetFill {
                layer_id: layer_id.into(),
                fill: Some(FillStyle::Solid { color: hex_to_rgba(value.as_str().unwrap_or("#000000"), alpha) }),
            }
        }
        "strokeWidth" => {
            let stroke = layer_base(layer).attributes.stroke.clone().unwrap_or(StrokeStyle {
                color: [0.0, 0.0, 0.0, 1.0],
                width: 1.0,
                cap: "butt".into(),
                join: "miter".into(),
                dash: None,
            });
            DrawOp::SetStroke {
                layer_id: layer_id.into(),
                stroke: Some(StrokeStyle { width: value.as_f64().unwrap_or(1.0), ..stroke }),
            }
        }
        "traceThreshold" => {
            let DrawLayerNode::Trace(trace) = layer else { return None };
            let mut params = trace.params.clone();
            params.threshold = value.as_f64().unwrap_or(0.5);
            DrawOp::SetTraceParams { layer_id: layer_id.into(), params }
        }
        "traceSimplify" => {
            let DrawLayerNode::Trace(trace) = layer else { return None };
            let mut params = trace.params.clone();
            params.simplify_epsilon = value.as_f64().unwrap_or(1.5);
            DrawOp::SetTraceParams { layer_id: layer_id.into(), params }
        }
        _ => return None,
    };
    Some(op)
}

pub fn patch_layer_field(doc: &DrawDocument, layer_id: &str, field: &str, value: &serde_json::Value) -> DrawDocument {
    match draw_op_for_layer_field(doc, layer_id, field, value) {
        Some(op) => apply_draw_edit_op(doc, &op),
        None => doc.clone(),
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
    pub camera: Option<DrawCamera>,
    pub layer_patches: Vec<DrawLayerTreePatch>,
    pub layers_removed: Vec<String>,
    pub layers_added: Vec<DrawLayerTreeAdd>,
}

impl Default for DrawDiff {
    fn default() -> Self {
        Self {
            document: None,
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

//#region 🔖MediaExport
fn escape_svg_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn rgba_to_svg_color(color: [f64; 4]) -> String {
    let r = (color[0].clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (color[1].clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (color[2].clamp(0.0, 1.0) * 255.0).round() as u8;
    let a = color[3].clamp(0.0, 1.0);
    if (a - 1.0).abs() < f64::EPSILON {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    } else {
        format!("rgba({r},{g},{b},{a:.3})")
    }
}

fn fill_style_to_svg(fill: &FillStyle) -> String {
    match fill {
        FillStyle::Solid { color } => rgba_to_svg_color(*color),
        FillStyle::LinearGradient { .. } | FillStyle::RadialGradient { .. } => "none".into(),
    }
}

fn path_segments_to_svg_d(segments: &[PathSegment]) -> String {
    let mut out = String::new();
    for segment in segments {
        match segment {
            PathSegment::Move { to } => out.push_str(&format!("M {} {} ", to[0], to[1])),
            PathSegment::Line { to } => out.push_str(&format!("L {} {} ", to[0], to[1])),
            PathSegment::Quad { ctrl, to } => out.push_str(&format!("Q {} {} {} {} ", ctrl[0], ctrl[1], to[0], to[1])),
            PathSegment::Cubic { ctrl1, ctrl2, to } => {
                out.push_str(&format!(
                    "C {} {} {} {} {} {} ",
                    ctrl1[0], ctrl1[1], ctrl2[0], ctrl2[1], to[0], to[1]
                ));
            }
            PathSegment::Arc {
                rx,
                ry,
                rotation,
                large_arc,
                sweep,
                to,
            } => out.push_str(&format!(
                "A {} {} {} {} {} {} {} ",
                rx,
                ry,
                rotation,
                if *large_arc { 1 } else { 0 },
                if *sweep { 1 } else { 0 },
                to[0],
                to[1]
            )),
            PathSegment::Close => out.push('Z'),
        }
    }
    out.trim().to_string()
}

fn resolve_draw_document_artboard(doc: &DrawDocument) -> (u32, u32) {
    if let Some(artboard) = &doc.artboard {
        return (
            artboard.width.max(1.0).round() as u32,
            artboard.height.max(1.0).round() as u32,
        );
    }
    let mut max_x: f64 = 1024.0;
    let mut max_y: f64 = 1024.0;
    for layer in flatten_draw_layers(&doc.layers) {
        if let Some((x, y, width, height)) = draw_layer_world_bounds(layer) {
            max_x = max_x.max(x + width);
            max_y = max_y.max(y + height);
        }
    }
    (max_x.max(1.0).round() as u32, max_y.max(1.0).round() as u32)
}

/// @emoji 💾 Serializes a draw document to SVG markup and raster dimensions.
pub fn draw_document_to_svg(doc: &DrawDocument) -> (String, u32, u32) {
    let (width, height) = resolve_draw_document_artboard(doc);
    let shapes = flatten_draw_document_to_scene_nodes(doc)
        .into_iter()
        .map(|node| {
            let matrix = node
                .transform
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            if let Some(text) = node.text {
                let fill = node.fill.as_ref().map(fill_style_to_svg).unwrap_or_else(|| "black".into());
                return format!(
                    r#"<g transform="matrix({matrix})" opacity="{}"><text x="0" y="{}" font-size="{}" fill="{fill}">{}</text></g>"#,
                    node.opacity,
                    text.size,
                    text.size,
                    escape_svg_text(&text.content)
                );
            }
            if let Some(image) = node.image {
                return format!(
                    r#"<g transform="matrix({matrix})" opacity="{}"><image href="{}" width="{}" height="{}"/></g>"#,
                    node.opacity, image.src, image.width, image.height
                );
            }
            let d = path_segments_to_svg_d(&node.segments);
            if d.is_empty() {
                return String::new();
            }
            let fill = node
                .fill
                .as_ref()
                .map(fill_style_to_svg)
                .unwrap_or_else(|| "none".into());
            let stroke = node
                .stroke
                .as_ref()
                .map(|style| rgba_to_svg_color(style.color))
                .unwrap_or_else(|| "none".into());
            let stroke_width = node.stroke.as_ref().map(|style| style.width).unwrap_or(0.0);
            format!(
                r#"<g transform="matrix({matrix})" opacity="{}"><path d="{d}" fill="{fill}" stroke="{stroke}" stroke-width="{stroke_width}" fill-rule="evenodd"/></g>"#,
                node.opacity
            )
        })
        .filter(|shape| !shape.is_empty())
        .collect::<Vec<_>>()
        .join("");
    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="{width}" height="{height}">{shapes}</svg>"#
    );
    (svg, width, height)
}

pub fn draw_document_json_to_svg(value: &serde_json::Value) -> Result<(String, u32, u32), String> {
    let doc: DrawDocument = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    Ok(draw_document_to_svg(&doc))
}

fn apply_draw_transform_point(m: [f64; 6], p: [f64; 2]) -> [f64; 2] {
    [m[0] * p[0] + m[2] * p[1] + m[4], m[1] * p[0] + m[3] * p[1] + m[5]]
}

fn draw_path_segment_to_dwg(segment: &PathSegment, transform: [f64; 6]) -> semio_framework_core::DwgPathSegment {
    use semio_framework_core::DwgPathSegment;
    match segment {
        PathSegment::Move { to } => DwgPathSegment::Move { to: apply_draw_transform_point(transform, *to) },
        PathSegment::Line { to } => DwgPathSegment::Line { to: apply_draw_transform_point(transform, *to) },
        PathSegment::Quad { ctrl, to } => DwgPathSegment::Quad { ctrl: apply_draw_transform_point(transform, *ctrl), to: apply_draw_transform_point(transform, *to) },
        PathSegment::Cubic { ctrl1, ctrl2, to } => DwgPathSegment::Cubic {
            ctrl1: apply_draw_transform_point(transform, *ctrl1),
            ctrl2: apply_draw_transform_point(transform, *ctrl2),
            to: apply_draw_transform_point(transform, *to),
        },
        PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => {
            DwgPathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: apply_draw_transform_point(transform, *to) }
        }
        PathSegment::Close => DwgPathSegment::Close,
    }
}

fn dwg_path_segment_to_draw(segment: &semio_framework_core::DwgPathSegment) -> PathSegment {
    use semio_framework_core::DwgPathSegment;
    match segment {
        DwgPathSegment::Move { to } => PathSegment::Move { to: *to },
        DwgPathSegment::Line { to } => PathSegment::Line { to: *to },
        DwgPathSegment::Quad { ctrl, to } => PathSegment::Quad { ctrl: *ctrl, to: *to },
        DwgPathSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic { ctrl1: *ctrl1, ctrl2: *ctrl2, to: *to },
        DwgPathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to },
        DwgPathSegment::Close => PathSegment::Close,
    }
}

/// 📐 Converts a draw document to DWG bytes with native fidelity: circular/elliptical arcs become bulges (not flattened cubics) and text stays a DWG TEXT entity.
pub fn draw_document_json_to_dwg_bytes(value: &serde_json::Value) -> Result<Vec<u8>, String> {
    let doc: DrawDocument = serde_json::from_value(value.clone()).map_err(|error| error.to_string())?;
    let mut path_groups: Vec<Vec<semio_framework_core::DwgPathSegment>> = Vec::new();
    let mut text_entities: Vec<(f64, f64, f64, String)> = Vec::new();
    for node in flatten_draw_document_to_scene_nodes(&doc) {
        if !node.visible {
            continue;
        }
        if let Some(text) = &node.text {
            let at = apply_draw_transform_point(node.transform, [0.0, 0.0]);
            text_entities.push((at[0], at[1], text.size, text.content.clone()));
            continue;
        }
        if node.segments.is_empty() {
            continue;
        }
        path_groups.push(node.segments.iter().map(|segment| draw_path_segment_to_dwg(segment, node.transform)).collect());
    }
    let mut drawing = semio_framework_core::paths_to_dwg_drawing(&path_groups);
    let layer = drawing.ensure_layer("0");
    for (x, y, size, content) in text_entities {
        drawing.entities.push(semio_framework_core::DwgEntity {
            layer,
            color: semio_framework_core::DwgColor::ByLayer,
            geometry: semio_framework_core::DwgGeometry::Text { at: [x, y, 0.0], height: size, rotation: 0.0, content },
        });
    }
    semio_framework_core::dwg_to_bytes(&drawing)
}

/// 📐 Rebuilds a draw document from an imported DWG drawing: one path layer per polyline/spline entity, DWG text entities become draw text layers.
pub fn draw_document_json_from_dwg(drawing: &semio_framework_core::DwgDrawing) -> Result<serde_json::Value, String> {
    let mut doc = default_draw_document("imported-dwg", Some("Imported DWG"));
    let mut layers: Vec<DrawLayerNode> = semio_framework_core::dwg_drawing_to_paths(drawing)
        .iter()
        .enumerate()
        .map(|(index, segments)| {
            let draw_segments: Vec<PathSegment> = segments.iter().map(dwg_path_segment_to_draw).collect();
            create_draw_path_layer(&format!("Path {}", index + 1), draw_segments)
        })
        .collect();
    for entity in &drawing.entities {
        if let semio_framework_core::DwgGeometry::Text { at, height, content, .. } = &entity.geometry {
            layers.push(DrawLayerNode::Text(DrawTextBody { base: default_layer_base("Text"), x: at[0], y: at[1], content: content.clone(), size: *height }));
        }
    }
    if layers.is_empty() {
        layers.push(create_draw_path_layer("Layer 1", Vec::new()));
    }
    doc.layers = layers;
    doc.artboard = Some(DrawArtboard {
        width: (drawing.extmax[0] - drawing.extmin[0]).max(1.0),
        height: (drawing.extmax[1] - drawing.extmin[1]).max(1.0),
    });
    serde_json::to_value(&doc).map_err(|error| error.to_string())
}
//#endregion 🔖MediaExport

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
    fn dwg_export_import_round_trips_a_path_and_text_layer() {
        let path_layer = create_draw_path_layer(
            "Outline",
            vec![
                PathSegment::Move { to: [0.0, 0.0] },
                PathSegment::Line { to: [10.0, 0.0] },
                PathSegment::Cubic { ctrl1: [12.0, 2.0], ctrl2: [12.0, 6.0], to: [10.0, 8.0] },
                PathSegment::Close,
            ],
        );
        let text_layer = DrawLayerNode::Text(DrawTextBody { base: default_layer_base("Label"), x: 1.0, y: 2.0, content: "semio".into(), size: 3.0 });
        let doc = DrawDocument { layers: vec![path_layer, text_layer], ..default_draw_document("dwg-test", None) };
        let value = serde_json::to_value(&doc).unwrap();

        let bytes = draw_document_json_to_dwg_bytes(&value).expect("export dwg");
        assert!(!bytes.is_empty());
        let drawing = semio_framework_core::dwg_from_bytes(&bytes).expect("decode dwg");
        assert!(drawing.entities.iter().any(|entity| matches!(entity.geometry, semio_framework_core::DwgGeometry::Text { .. })));
        assert!(drawing.entities.iter().any(|entity| matches!(entity.geometry, semio_framework_core::DwgGeometry::LwPolyline { .. } | semio_framework_core::DwgGeometry::Spline { .. })));

        let reimported = draw_document_json_from_dwg(&drawing).expect("import dwg");
        let reimported_doc: DrawDocument = serde_json::from_value(reimported).expect("valid draw document");
        assert!(reimported_doc.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Text(text) if text.content == "semio")));
        assert!(reimported_doc.layers.iter().any(|layer| matches!(layer, DrawLayerNode::Path(_))));
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

    #[test]
    fn resolve_boolean_layer_segments_unions_two_rects() {
        let mut doc = default_draw_document("bool-test", None);
        doc.layers.clear();
        let mut rect_a = create_draw_shape_layer_rect("A");
        if let DrawLayerNode::Shape(shape) = &mut rect_a {
            shape.rect = Some(DrawRect { x: 0.0, y: 0.0, width: 10.0, height: 10.0 });
        }
        let id_a = layer_id(&rect_a).to_string();
        let mut rect_b = create_draw_shape_layer_rect("B");
        if let DrawLayerNode::Shape(shape) = &mut rect_b {
            shape.rect = Some(DrawRect { x: 5.0, y: 5.0, width: 10.0, height: 10.0 });
        }
        let id_b = layer_id(&rect_b).to_string();
        doc.layers.push(rect_a);
        doc.layers.push(rect_b);
        let boolean = create_draw_boolean_layer("Union", "union", vec![id_a, id_b]);
        let boolean_id = layer_id(&boolean).to_string();
        doc.layers.push(boolean);
        let nodes = flatten_draw_document_to_scene_nodes(&doc);
        let boolean_node = nodes.iter().find(|node| node.id == boolean_id).expect("boolean scene node");
        assert!(!boolean_node.segments.is_empty());
        assert_eq!(boolean_node.fill_rule.as_deref(), Some("evenodd"));
    }

    #[test]
    fn resolve_boolean_layer_segments_flattens_arcs_before_boolean_op() {
        let mut doc = default_draw_document("bool-arc-test", None);
        doc.layers.clear();
        let path_a = create_draw_path_layer(
            "A",
            vec![
                PathSegment::Move { to: [0.0, 0.0] },
                PathSegment::Line { to: [10.0, 0.0] },
                PathSegment::Arc { rx: 10.0, ry: 10.0, rotation: 0.0, large_arc: false, sweep: true, to: [0.0, 10.0] },
                PathSegment::Close,
            ],
        );
        let id_a = layer_id(&path_a).to_string();
        let rect_b = {
            let mut layer = create_draw_shape_layer_rect("B");
            if let DrawLayerNode::Shape(shape) = &mut layer {
                shape.rect = Some(DrawRect { x: 2.0, y: 2.0, width: 4.0, height: 4.0 });
            }
            layer
        };
        let id_b = layer_id(&rect_b).to_string();
        doc.layers.push(path_a);
        doc.layers.push(rect_b);
        let boolean = create_draw_boolean_layer("Union", "union", vec![id_a, id_b]);
        let boolean_id = layer_id(&boolean).to_string();
        doc.layers.push(boolean);
        let nodes = flatten_draw_document_to_scene_nodes(&doc);
        let boolean_node = nodes.iter().find(|node| node.id == boolean_id).expect("boolean scene node");
        assert!(!boolean_node.segments.is_empty());
    }

    #[test]
    fn arc_segment_flattens_to_cubics_preserving_endpoints() {
        let segments = vec![
            PathSegment::Move { to: [10.0, 0.0] },
            PathSegment::Arc { rx: 10.0, ry: 10.0, rotation: 0.0, large_arc: false, sweep: true, to: [0.0, 10.0] },
        ];
        let flattened = flatten_curve_segments(&segments);
        assert!(flattened.iter().all(|segment| !matches!(segment, PathSegment::Arc { .. })));
        match flattened.last() {
            Some(PathSegment::Cubic { to, .. }) => {
                assert!((to[0] - 0.0).abs() < 1e-6);
                assert!((to[1] - 10.0).abs() < 1e-6);
            }
            other => panic!("expected trailing cubic segment, got {other:?}"),
        }
    }

    #[test]
    fn resolve_trace_layer_segments_traces_solid_square_png() {
        let mut image_buffer = image::RgbaImage::new(8, 8);
        for y in 2..6 {
            for x in 2..6 {
                image_buffer.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
            }
        }
        let mut bytes = Vec::new();
        image::DynamicImage::ImageRgba8(image_buffer)
            .write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png)
            .expect("encode png");
        let mut doc = default_draw_document("trace-test", None);
        doc.layers.clear();
        let mut assets = std::collections::HashMap::new();
        assets.insert(
            "source".to_string(),
            DrawImageAsset { mime: "image/png".into(), data: BASE64.encode(&bytes), width: None, height: None },
        );
        doc.assets = Some(assets);
        doc.artboard = Some(DrawArtboard { width: 16.0, height: 16.0 });
        doc.layers.push(create_draw_trace_layer("Trace", "source"));
        let nodes = flatten_draw_document_to_scene_nodes(&doc);
        assert_eq!(nodes.len(), 1);
        assert!(!nodes[0].segments.is_empty());
        assert_eq!(nodes[0].fill_rule.as_deref(), Some("evenodd"));
    }

    #[test]
    fn resolve_draw_artboard_skips_group_boolean_trace_kinds() {
        let mut doc = default_draw_document("artboard-test", None);
        doc.layers.clear();
        let mut rect = create_draw_shape_layer_rect("R");
        if let DrawLayerNode::Shape(shape) = &mut rect {
            shape.rect = Some(DrawRect { x: 0.0, y: 0.0, width: 20.0, height: 30.0 });
        }
        doc.layers.push(rect);
        doc.layers.push(create_draw_trace_layer("Trace", "missing-source"));
        let artboard = resolve_draw_artboard(&doc).expect("artboard bounds");
        assert_eq!(artboard.width, 20.0);
        assert_eq!(artboard.height, 30.0);
    }
}
//#endregion 🧪Tests
