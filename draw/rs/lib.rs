//! ✏️ Draw document domain + typed VCS on `vcs`.

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
#[cfg(target_arch = "wasm32")]
use vcs::create_document_vcs_envelope;
use vcs::{DocumentDsl, DocumentVcsEnvelope, DocumentVcsStore, OpText, Operation, OperationDiff, TextError, TextSpan};
/// 🔁 Reexported so downstream crates (e.g. `draw-plugin`) can call `DrawDocument::parse_dsl`/
/// `.print_dsl()` without taking a direct `vcs` dependency just for the trait.
pub use vcs::DocumentDsl;

pub const DRAW_DOCUMENT_SCHEMA: &str = "draw.document";
pub const DRAW_BLEND_MODES: &[&str] = &["normal", "multiply", "screen", "overlay", "darken", "lighten", "colorDodge", "colorBurn", "hardLight", "softLight", "difference", "exclusion", "hue", "saturation", "color", "luminosity"];
pub const DRAW_BOOLEAN_OPERATIONS: &[&str] = &["union", "difference", "intersection", "xor"];
pub const DRAW_SHAPE_KINDS: &[&str] = &["rect", "ellipse", "circle", "line", "polygon"];
pub const DRAW_UTILITY_IDS: &[&str] = &["selectMarquee", "selectLasso", "selectDirect", "pen", "shapeRect", "shapeEllipse", "shapeLine", "shapePolygon", "booleanCombine", "trace", "transformMove"];

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
    pub operation: String,
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
    Arc { rx: f64, ry: f64, rotation: f64, large_arc: bool, sweep: bool, to: [f64; 2] },
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

pub type DrawEnvelope = DocumentVcsEnvelope<DrawDocument, DrawOperation>;
pub type DrawStore = DocumentVcsStore<DrawDocument, DrawOperation>;

static DRAW_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn create_draw_id(prefix: &str) -> String {
    let next = DRAW_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("{prefix}-{next}")
}

pub fn default_draw_transform() -> DrawTransform {
    DrawTransform { x: 0.0, y: 0.0, scale_x: 1.0, scale_y: 1.0, rotation: 0.0 }
}

pub fn default_draw_trace_params() -> DrawTraceParams {
    DrawTraceParams { threshold: 0.5, simplify_epsilon: 1.5 }
}

pub fn default_layer_base(name: &str) -> DrawLayerBase {
    DrawLayerBase { id: create_draw_id("layer"), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_draw_transform(), attributes: DrawAttributes::default() }
}

pub fn create_draw_path_layer(name: &str, segments: Vec<PathSegment>) -> DrawLayerNode {
    DrawLayerNode::Path(DrawPathBody {
        base: DrawLayerBase { id: create_draw_id("path"), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_draw_transform(), attributes: DrawAttributes::default() },
        segments,
    })
}

pub fn create_draw_group_layer(name: &str) -> DrawLayerNode {
    DrawLayerNode::Group(DrawGroupBody {
        base: DrawLayerBase { id: create_draw_id("group"), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_draw_transform(), attributes: DrawAttributes::default() },
        children: Vec::new(),
    })
}

pub fn create_draw_boolean_layer(name: &str, operation: &str, children: Vec<String>) -> DrawLayerNode {
    DrawLayerNode::Boolean(DrawBooleanBody {
        base: DrawLayerBase { id: create_draw_id("boolean"), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_draw_transform(), attributes: DrawAttributes::default() },
        operation: operation.into(),
        children,
    })
}

pub fn create_draw_trace_layer(name: &str, source_key: &str) -> DrawLayerNode {
    DrawLayerNode::Trace(DrawTraceBody {
        base: DrawLayerBase { id: create_draw_id("trace"), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_draw_transform(), attributes: DrawAttributes::default() },
        source_key: source_key.into(),
        params: default_draw_trace_params(),
    })
}

pub fn create_draw_shape_layer_rect(name: &str) -> DrawLayerNode {
    DrawLayerNode::Shape(DrawShapeBody {
        base: DrawLayerBase { id: create_draw_id("shape"), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_draw_transform(), attributes: DrawAttributes::default() },
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
            attributes: DrawAttributes { fill: Some(FillStyle::Solid { color: [0.0, 0.0, 0.0, 1.0] }), stroke: None },
        },
        x: 0.0,
        y: 0.0,
        content: "Text".into(),
        size: 24.0,
    })
}

pub fn create_draw_image_layer(name: &str, image_key: &str) -> DrawLayerNode {
    DrawLayerNode::Image(DrawImageBody {
        base: DrawLayerBase { id: create_draw_id("image"), name: name.into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: default_draw_transform(), attributes: DrawAttributes::default() },
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
        camera: DrawCamera { x: 512.0, y: 512.0, zoom: 0.75 },
        layers: vec![create_draw_path_layer("Layer 1", Vec::new())],
        assets: None,
        artboard: Some(DrawArtboard { width: 1024.0, height: 1024.0 }),
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
        "line" => shape.line.as_ref().map(|line| vec![PathSegment::Move { to: [line.x1, line.y1] }, PathSegment::Line { to: [line.x2, line.y2] }]),
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
        "ellipse" => shape.ellipse.as_ref().map(|ellipse| ellipse_path_segments(ellipse.cx, ellipse.cy, ellipse.rx, ellipse.ry)),
        "circle" => shape.circle.as_ref().map(|circle| ellipse_path_segments(circle.cx, circle.cy, circle.r, circle.r)),
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
    let corners = [(local.0, local.1), (local.0 + local.2, local.1), (local.0 + local.2, local.1 + local.3), (local.0, local.1 + local.3)];
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (x, y) in corners {
        let world = transform_world_point(&base.transform, x, y);
        xs.push(world.0);
        ys.push(world.1);
    }
    Some((
        xs.iter().cloned().fold(f64::INFINITY, f64::min),
        ys.iter().cloned().fold(f64::INFINITY, f64::min),
        xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - xs.iter().cloned().fold(f64::INFINITY, f64::min),
        ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max) - ys.iter().cloned().fold(f64::INFINITY, f64::min),
    ))
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
                    let src =
                        doc.assets.as_ref().and_then(|assets| assets.get(&image.image_key)).map(|asset| if asset.data.starts_with("data:") { asset.data.clone() } else { format!("data:{};base64,{}", asset.mime, asset.data) }).unwrap_or_default();
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
            DrawCanvasLayerRecord { id: base.id.clone(), kind: layer_kind_label(layer), name: base.name.clone(), x: bounds.map(|b| b.0), y: bounds.map(|b| b.1), width: bounds.map(|b| b.2), height: bounds.map(|b| b.3) }
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

//#region 🔖Dsl
// 🔤 Handcrafted textual DSL for `DrawDocument` (`vcs::DocumentDsl`) and one-line op-text for
// `DrawOperation` (`vcs::OpText`, see `🔖OpText`) — replaces the JSON fixture format. Grammar is a
// small hand-rolled tokenizer + recursive-descent parser (no external parser crate), in the spirit of
// `mathematical_graph_dsl::wire`. Layers nest via `{ }`, tuples via `( )`, lists via `[ ]`, and every
// statement is self-delimiting so the same printer/parser pair works whether chunks are newline-joined
// (pretty `print_dsl`) or space-joined (one-line op-text embedding a whole layer or document).

//#region 🔖DslLexer
#[derive(Clone, Debug, PartialEq)]
enum DrawTok {
    Ident(String),
    Str(String),
    Num(f64),
    Eq,
    Colon,
    Comma,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    At,
    Arrow,
    Eof,
}

#[derive(Clone, Debug)]
struct DrawSpannedTok {
    tok: DrawTok,
    line: u32,
    column: u32,
}

/// 🔍 Hand-rolled char-by-char tokenizer for the draw DSL/op-text grammar; tracks line/column so parse
/// errors carry a `TextSpan` a dev can jump to.
fn lex_draw_dsl(input: &str) -> Result<Vec<DrawSpannedTok>, TextError> {
    let chars: Vec<char> = input.chars().collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    let mut line: u32 = 1;
    let mut col: u32 = 1;
    while i < chars.len() {
        let c = chars[i];
        if c == '\n' {
            i += 1;
            line += 1;
            col = 1;
            continue;
        }
        if c.is_whitespace() {
            i += 1;
            col += 1;
            continue;
        }
        let (start_line, start_col) = (line, col);
        match c {
            '=' => {
                out.push(DrawSpannedTok { tok: DrawTok::Eq, line: start_line, column: start_col });
                i += 1;
                col += 1;
            }
            ':' => {
                out.push(DrawSpannedTok { tok: DrawTok::Colon, line: start_line, column: start_col });
                i += 1;
                col += 1;
            }
            ',' => {
                out.push(DrawSpannedTok { tok: DrawTok::Comma, line: start_line, column: start_col });
                i += 1;
                col += 1;
            }
            '(' => {
                out.push(DrawSpannedTok { tok: DrawTok::LParen, line: start_line, column: start_col });
                i += 1;
                col += 1;
            }
            ')' => {
                out.push(DrawSpannedTok { tok: DrawTok::RParen, line: start_line, column: start_col });
                i += 1;
                col += 1;
            }
            '[' => {
                out.push(DrawSpannedTok { tok: DrawTok::LBracket, line: start_line, column: start_col });
                i += 1;
                col += 1;
            }
            ']' => {
                out.push(DrawSpannedTok { tok: DrawTok::RBracket, line: start_line, column: start_col });
                i += 1;
                col += 1;
            }
            '{' => {
                out.push(DrawSpannedTok { tok: DrawTok::LBrace, line: start_line, column: start_col });
                i += 1;
                col += 1;
            }
            '}' => {
                out.push(DrawSpannedTok { tok: DrawTok::RBrace, line: start_line, column: start_col });
                i += 1;
                col += 1;
            }
            '@' => {
                out.push(DrawSpannedTok { tok: DrawTok::At, line: start_line, column: start_col });
                i += 1;
                col += 1;
            }
            '-' if i + 1 < chars.len() && chars[i + 1] == '>' => {
                out.push(DrawSpannedTok { tok: DrawTok::Arrow, line: start_line, column: start_col });
                i += 2;
                col += 2;
            }
            '"' => {
                i += 1;
                col += 1;
                let mut value = String::new();
                loop {
                    if i >= chars.len() {
                        return Err(TextError::new("unterminated string literal", TextSpan::at(start_line, start_col)));
                    }
                    let ch = chars[i];
                    if ch == '"' {
                        i += 1;
                        col += 1;
                        break;
                    }
                    if ch == '\\' && i + 1 < chars.len() {
                        match chars[i + 1] {
                            'n' => value.push('\n'),
                            '"' => value.push('"'),
                            '\\' => value.push('\\'),
                            other => value.push(other),
                        }
                        i += 2;
                        col += 2;
                    } else if ch == '\n' {
                        value.push('\n');
                        i += 1;
                        line += 1;
                        col = 1;
                    } else {
                        value.push(ch);
                        i += 1;
                        col += 1;
                    }
                }
                out.push(DrawSpannedTok { tok: DrawTok::Str(value), line: start_line, column: start_col });
            }
            '-' | '0'..='9' => {
                let start = i;
                if c == '-' {
                    i += 1;
                    col += 1;
                }
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                    col += 1;
                }
                if i < chars.len() && (chars[i] == 'e' || chars[i] == 'E') {
                    i += 1;
                    col += 1;
                    if i < chars.len() && (chars[i] == '+' || chars[i] == '-') {
                        i += 1;
                        col += 1;
                    }
                    while i < chars.len() && chars[i].is_ascii_digit() {
                        i += 1;
                        col += 1;
                    }
                }
                let text: String = chars[start..i].iter().collect();
                let value: f64 = text.parse().map_err(|_| TextError::new(format!("invalid number '{text}'"), TextSpan::at(start_line, start_col)))?;
                out.push(DrawSpannedTok { tok: DrawTok::Num(value), line: start_line, column: start_col });
            }
            other if other.is_ascii_alphabetic() || other == '_' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_' || chars[i] == '-' || chars[i] == '.') {
                    i += 1;
                    col += 1;
                }
                let text: String = chars[start..i].iter().collect();
                out.push(DrawSpannedTok { tok: DrawTok::Ident(text), line: start_line, column: start_col });
            }
            other => return Err(TextError::new(format!("unexpected character '{other}'"), TextSpan::at(start_line, start_col))),
        }
    }
    out.push(DrawSpannedTok { tok: DrawTok::Eof, line, column: col });
    Ok(out)
}

/// 🔐 Escapes `\`, `"` and newlines for embedding a string inside a `"..."` DSL literal.
fn escape_str(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            _ => out.push(ch),
        }
    }
    out
}

/// 🔢 Prints an `f64` via its shortest round-trippable `Display` form (Rust's float formatter already
/// guarantees `s.parse::<f64>() == value`), named for call-site clarity next to `escape_str`.
fn fmt_num(value: f64) -> String {
    value.to_string()
}
//#endregion 🔖DslLexer

//#region 🔖DslParser
struct DrawDslParser {
    toks: Vec<DrawSpannedTok>,
    pos: usize,
}

impl DrawDslParser {
    fn new(toks: Vec<DrawSpannedTok>) -> Self {
        Self { toks, pos: 0 }
    }

    fn peek(&self) -> &DrawTok {
        &self.toks[self.pos].tok
    }

    fn peek_at(&self, offset: usize) -> &DrawTok {
        let idx = (self.pos + offset).min(self.toks.len() - 1);
        &self.toks[idx].tok
    }

    fn span(&self) -> TextSpan {
        let tok = &self.toks[self.pos];
        TextSpan::at(tok.line, tok.column)
    }

    fn bump(&mut self) -> DrawTok {
        let tok = self.toks[self.pos].tok.clone();
        if self.pos + 1 < self.toks.len() {
            self.pos += 1;
        }
        tok
    }

    fn expect_tok(&mut self, expected: &DrawTok, label: &str) -> Result<(), TextError> {
        let span = self.span();
        let got = self.bump();
        if &got == expected {
            Ok(())
        } else {
            Err(TextError::expected(format!("expected '{label}'"), span, format!("{got:?}")))
        }
    }

    fn expect_ident(&mut self) -> Result<String, TextError> {
        let span = self.span();
        match self.bump() {
            DrawTok::Ident(value) => Ok(value),
            other => Err(TextError::expected("expected identifier", span, format!("{other:?}"))),
        }
    }

    fn expect_str(&mut self) -> Result<String, TextError> {
        let span = self.span();
        match self.bump() {
            DrawTok::Str(value) => Ok(value),
            other => Err(TextError::expected("expected string literal", span, format!("{other:?}"))),
        }
    }

    fn expect_num(&mut self) -> Result<f64, TextError> {
        let span = self.span();
        match self.bump() {
            DrawTok::Num(value) => Ok(value),
            other => Err(TextError::expected("expected number", span, format!("{other:?}"))),
        }
    }

    fn expect_bool(&mut self) -> Result<bool, TextError> {
        let span = self.span();
        match self.bump() {
            DrawTok::Ident(value) if value == "true" => Ok(true),
            DrawTok::Ident(value) if value == "false" => Ok(false),
            other => Err(TextError::expected("expected 'true' or 'false'", span, format!("{other:?}"))),
        }
    }

    fn at_ident(&self, value: &str) -> bool {
        matches!(self.peek(), DrawTok::Ident(candidate) if candidate == value)
    }

    fn eat_keyword(&mut self, value: &str) -> Result<(), TextError> {
        let span = self.span();
        match self.bump() {
            DrawTok::Ident(candidate) if candidate == value => Ok(()),
            other => Err(TextError::expected(format!("expected '{value}'"), span, format!("{other:?}"))),
        }
    }

    /// 🔎 True when the parser sits at the start of a `key=value` attribute (one token of lookahead
    /// past the identifier), the signal every attr-loop uses to decide whether to keep consuming.
    fn at_attr(&self) -> bool {
        matches!(self.peek(), DrawTok::Ident(_)) && matches!(self.peek_at(1), DrawTok::Eq)
    }

    fn peek_attr_key(&self) -> Option<&str> {
        if let DrawTok::Ident(value) = self.peek() {
            Some(value.as_str())
        } else {
            None
        }
    }
}

/// 🏷️ Consumes `key=` (assumes `at_attr()` already confirmed the shape) and returns `key`.
fn take_attr_key(p: &mut DrawDslParser) -> Result<String, TextError> {
    let key = p.expect_ident()?;
    p.expect_tok(&DrawTok::Eq, "=")?;
    Ok(key)
}

/// 🏷️ Consumes `expected=` and errors if the attribute name doesn't match (keeps op-text/DSL parsing
/// honest about field order without a full generic key-value bag).
fn expect_key(p: &mut DrawDslParser, expected: &str) -> Result<(), TextError> {
    let span = p.span();
    let key = take_attr_key(p)?;
    if key == expected {
        Ok(())
    } else {
        Err(TextError::expected(format!("expected key '{expected}'"), span, key))
    }
}

/// 🏷️ Consumes `expected:` — the marker used before an inline nested layer/document expression, since
/// those values aren't a simple scalar/tuple/list and instead parse their own self-delimited grammar.
fn expect_key_colon(p: &mut DrawDslParser, expected: &str) -> Result<(), TextError> {
    let span = p.span();
    let key = p.expect_ident()?;
    if key != expected {
        return Err(TextError::expected(format!("expected '{expected}:'"), span, key));
    }
    p.expect_tok(&DrawTok::Colon, ":")?;
    Ok(())
}

fn parse_kv_ident(p: &mut DrawDslParser, key: &str) -> Result<String, TextError> {
    expect_key(p, key)?;
    p.expect_ident()
}

fn parse_kv_str(p: &mut DrawDslParser, key: &str) -> Result<String, TextError> {
    expect_key(p, key)?;
    p.expect_str()
}

fn parse_kv_num(p: &mut DrawDslParser, key: &str) -> Result<f64, TextError> {
    expect_key(p, key)?;
    p.expect_num()
}

fn parse_kv_bool(p: &mut DrawDslParser, key: &str) -> Result<bool, TextError> {
    expect_key(p, key)?;
    p.expect_bool()
}

/// 🕳️ `none` sentinel for an absent `Option<String>` op-text field (`parentId`, …).
fn parse_kv_opt_ident(p: &mut DrawDslParser, key: &str) -> Result<Option<String>, TextError> {
    expect_key(p, key)?;
    match p.peek().clone() {
        DrawTok::Ident(value) if value == "none" => {
            p.bump();
            Ok(None)
        }
        DrawTok::Ident(value) => {
            p.bump();
            Ok(Some(value))
        }
        other => Err(TextError::expected("expected identifier or 'none'", p.span(), format!("{other:?}"))),
    }
}

fn parse_kv_opt_num(p: &mut DrawDslParser, key: &str) -> Result<Option<f64>, TextError> {
    expect_key(p, key)?;
    match p.peek().clone() {
        DrawTok::Ident(value) if value == "none" => {
            p.bump();
            Ok(None)
        }
        DrawTok::Num(value) => {
            p.bump();
            Ok(Some(value))
        }
        other => Err(TextError::expected("expected number or 'none'", p.span(), format!("{other:?}"))),
    }
}

fn print_opt_ident(value: &Option<String>) -> String {
    value.clone().unwrap_or_else(|| "none".to_string())
}

fn print_opt_num(value: &Option<usize>) -> String {
    value.map(|v| v.to_string()).unwrap_or_else(|| "none".to_string())
}

fn parse_point(p: &mut DrawDslParser) -> Result<[f64; 2], TextError> {
    let x = p.expect_num()?;
    p.expect_tok(&DrawTok::Comma, ",")?;
    let y = p.expect_num()?;
    Ok([x, y])
}

fn parse_color4_bare(p: &mut DrawDslParser) -> Result<[f64; 4], TextError> {
    let r = p.expect_num()?;
    p.expect_tok(&DrawTok::Comma, ",")?;
    let g = p.expect_num()?;
    p.expect_tok(&DrawTok::Comma, ",")?;
    let b = p.expect_num()?;
    p.expect_tok(&DrawTok::Comma, ",")?;
    let a = p.expect_num()?;
    Ok([r, g, b, a])
}

fn print_color4(color: [f64; 4]) -> String {
    format!("{},{},{},{}", fmt_num(color[0]), fmt_num(color[1]), fmt_num(color[2]), fmt_num(color[3]))
}

fn parse_num_list(p: &mut DrawDslParser) -> Result<Vec<f64>, TextError> {
    p.expect_tok(&DrawTok::LBracket, "[")?;
    let mut out = Vec::new();
    while !matches!(p.peek(), DrawTok::RBracket) {
        out.push(p.expect_num()?);
    }
    p.expect_tok(&DrawTok::RBracket, "]")?;
    Ok(out)
}

fn print_num_list(values: &[f64]) -> String {
    format!("[{}]", values.iter().map(|value| fmt_num(*value)).collect::<Vec<_>>().join(" "))
}

fn parse_point_list(p: &mut DrawDslParser) -> Result<Vec<[f64; 2]>, TextError> {
    p.expect_tok(&DrawTok::LBracket, "[")?;
    let mut out = Vec::new();
    while !matches!(p.peek(), DrawTok::RBracket) {
        out.push(parse_point(p)?);
    }
    p.expect_tok(&DrawTok::RBracket, "]")?;
    Ok(out)
}

fn parse_id_list(p: &mut DrawDslParser) -> Result<Vec<String>, TextError> {
    p.expect_tok(&DrawTok::LBracket, "[")?;
    let mut out = Vec::new();
    while !matches!(p.peek(), DrawTok::RBracket) {
        out.push(p.expect_ident()?);
    }
    p.expect_tok(&DrawTok::RBracket, "]")?;
    Ok(out)
}

fn parse_transform(p: &mut DrawDslParser) -> Result<DrawTransform, TextError> {
    p.expect_tok(&DrawTok::LParen, "(")?;
    let x = p.expect_num()?;
    p.expect_tok(&DrawTok::Comma, ",")?;
    let y = p.expect_num()?;
    p.expect_tok(&DrawTok::Comma, ",")?;
    let scale_x = p.expect_num()?;
    p.expect_tok(&DrawTok::Comma, ",")?;
    let scale_y = p.expect_num()?;
    p.expect_tok(&DrawTok::Comma, ",")?;
    let rotation = p.expect_num()?;
    p.expect_tok(&DrawTok::RParen, ")")?;
    Ok(DrawTransform { x, y, scale_x, scale_y, rotation })
}

fn print_transform(t: &DrawTransform) -> String {
    format!("({},{},{},{},{})", fmt_num(t.x), fmt_num(t.y), fmt_num(t.scale_x), fmt_num(t.scale_y), fmt_num(t.rotation))
}

fn parse_gradient_stops(p: &mut DrawDslParser) -> Result<Vec<GradientStop>, TextError> {
    p.expect_tok(&DrawTok::LBracket, "[")?;
    let mut stops = Vec::new();
    while !matches!(p.peek(), DrawTok::RBracket) {
        let offset = p.expect_num()?;
        p.expect_tok(&DrawTok::At, "@")?;
        let color = parse_color4_bare(p)?;
        stops.push(GradientStop { offset, color });
    }
    p.expect_tok(&DrawTok::RBracket, "]")?;
    Ok(stops)
}

fn print_gradient_stops(stops: &[GradientStop]) -> String {
    let items: Vec<String> = stops.iter().map(|stop| format!("{}@{}", fmt_num(stop.offset), print_color4(stop.color))).collect();
    format!("[{}]", items.join(" "))
}

fn parse_fill(p: &mut DrawDslParser) -> Result<Option<FillStyle>, TextError> {
    let span = p.span();
    let kind = p.expect_ident()?;
    match kind.as_str() {
        "none" => Ok(None),
        "solid" => {
            p.expect_tok(&DrawTok::LParen, "(")?;
            let color = parse_color4_bare(p)?;
            p.expect_tok(&DrawTok::RParen, ")")?;
            Ok(Some(FillStyle::Solid { color }))
        }
        "linear" => {
            p.expect_tok(&DrawTok::LParen, "(")?;
            let x1 = p.expect_num()?;
            p.expect_tok(&DrawTok::Comma, ",")?;
            let y1 = p.expect_num()?;
            p.expect_tok(&DrawTok::Comma, ",")?;
            let x2 = p.expect_num()?;
            p.expect_tok(&DrawTok::Comma, ",")?;
            let y2 = p.expect_num()?;
            p.expect_tok(&DrawTok::RParen, ")")?;
            let stops = parse_gradient_stops(p)?;
            Ok(Some(FillStyle::LinearGradient { x1, y1, x2, y2, stops }))
        }
        "radial" => {
            p.expect_tok(&DrawTok::LParen, "(")?;
            let cx = p.expect_num()?;
            p.expect_tok(&DrawTok::Comma, ",")?;
            let cy = p.expect_num()?;
            p.expect_tok(&DrawTok::Comma, ",")?;
            let r = p.expect_num()?;
            p.expect_tok(&DrawTok::RParen, ")")?;
            let stops = parse_gradient_stops(p)?;
            Ok(Some(FillStyle::RadialGradient { cx, cy, r, stops }))
        }
        other => Err(TextError::expected(format!("unknown fill kind '{other}'"), span, "none|solid|linear|radial")),
    }
}

fn print_fill(fill: &Option<FillStyle>) -> String {
    match fill {
        None => "none".to_string(),
        Some(FillStyle::Solid { color }) => format!("solid({})", print_color4(*color)),
        Some(FillStyle::LinearGradient { x1, y1, x2, y2, stops }) => format!("linear({},{},{},{}){}", fmt_num(*x1), fmt_num(*y1), fmt_num(*x2), fmt_num(*y2), print_gradient_stops(stops)),
        Some(FillStyle::RadialGradient { cx, cy, r, stops }) => format!("radial({},{},{}){}", fmt_num(*cx), fmt_num(*cy), fmt_num(*r), print_gradient_stops(stops)),
    }
}

/// 🖌️ Parses `stroke=none` or `stroke=(r,g,b,a) width=.. cap=.. join=.. [dash=[..]]` — the shared shape
/// used both for a layer's base `stroke=` attribute and for the `setStroke` op's payload.
fn parse_stroke_value(p: &mut DrawDslParser) -> Result<Option<StrokeStyle>, TextError> {
    match p.peek().clone() {
        DrawTok::Ident(value) if value == "none" => {
            p.bump();
            Ok(None)
        }
        DrawTok::LParen => {
            p.bump();
            let color = parse_color4_bare(p)?;
            p.expect_tok(&DrawTok::RParen, ")")?;
            let width = parse_kv_num(p, "width")?;
            let cap = parse_kv_ident(p, "cap")?;
            let join = parse_kv_ident(p, "join")?;
            let dash = if p.at_ident("dash") {
                p.bump();
                p.expect_tok(&DrawTok::Eq, "=")?;
                Some(parse_num_list(p)?)
            } else {
                None
            };
            Ok(Some(StrokeStyle { color, width, cap, join, dash }))
        }
        other => Err(TextError::expected("expected stroke value", p.span(), format!("{other:?}"))),
    }
}

fn print_stroke_value(stroke: &Option<StrokeStyle>) -> String {
    match stroke {
        None => "none".to_string(),
        Some(s) => format!("({})", print_color4(s.color)),
    }
}

/// 🖌️ The `width=`/`cap=`/`join=`/`dash=` attrs that accompany a non-`none` `stroke=`, shared by the
/// layer base printer and the `setStroke` op printer.
fn print_stroke_trailing_attrs(stroke: &StrokeStyle) -> Vec<String> {
    let mut out = vec![format!("width={}", fmt_num(stroke.width)), format!("cap={}", stroke.cap), format!("join={}", stroke.join)];
    if let Some(dash) = &stroke.dash {
        out.push(format!("dash={}", print_num_list(dash)));
    }
    out
}
//#endregion 🔖DslParser

//#region 🔖DslSegments
fn parse_segments(p: &mut DrawDslParser) -> Result<Vec<PathSegment>, TextError> {
    p.expect_tok(&DrawTok::LBrace, "{")?;
    let mut segments = Vec::new();
    while !matches!(p.peek(), DrawTok::RBrace) {
        let span = p.span();
        let keyword = p.expect_ident()?;
        let segment = match keyword.as_str() {
            "move" => PathSegment::Move { to: parse_point(p)? },
            "line" => PathSegment::Line { to: parse_point(p)? },
            "quad" => {
                let ctrl = parse_point(p)?;
                p.expect_tok(&DrawTok::Arrow, "->")?;
                let to = parse_point(p)?;
                PathSegment::Quad { ctrl, to }
            }
            "cubic" => {
                let ctrl1 = parse_point(p)?;
                let ctrl2 = parse_point(p)?;
                p.expect_tok(&DrawTok::Arrow, "->")?;
                let to = parse_point(p)?;
                PathSegment::Cubic { ctrl1, ctrl2, to }
            }
            "arc" => {
                let rx = p.expect_num()?;
                p.expect_tok(&DrawTok::Comma, ",")?;
                let ry = p.expect_num()?;
                p.expect_tok(&DrawTok::Comma, ",")?;
                let rotation = p.expect_num()?;
                p.expect_tok(&DrawTok::Comma, ",")?;
                let large_arc = p.expect_bool()?;
                p.expect_tok(&DrawTok::Comma, ",")?;
                let sweep = p.expect_bool()?;
                p.expect_tok(&DrawTok::Arrow, "->")?;
                let to = parse_point(p)?;
                PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to }
            }
            "close" => PathSegment::Close,
            other => return Err(TextError::expected(format!("unknown path segment '{other}'"), span, "move|line|quad|cubic|arc|close")),
        };
        segments.push(segment);
    }
    p.expect_tok(&DrawTok::RBrace, "}")?;
    Ok(segments)
}

fn print_segment(segment: &PathSegment) -> String {
    match segment {
        PathSegment::Move { to } => format!("move {},{}", fmt_num(to[0]), fmt_num(to[1])),
        PathSegment::Line { to } => format!("line {},{}", fmt_num(to[0]), fmt_num(to[1])),
        PathSegment::Quad { ctrl, to } => format!("quad {},{} -> {},{}", fmt_num(ctrl[0]), fmt_num(ctrl[1]), fmt_num(to[0]), fmt_num(to[1])),
        PathSegment::Cubic { ctrl1, ctrl2, to } => format!("cubic {},{} {},{} -> {},{}", fmt_num(ctrl1[0]), fmt_num(ctrl1[1]), fmt_num(ctrl2[0]), fmt_num(ctrl2[1]), fmt_num(to[0]), fmt_num(to[1])),
        PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => format!("arc {},{},{},{},{} -> {},{}", fmt_num(*rx), fmt_num(*ry), fmt_num(*rotation), large_arc, sweep, fmt_num(to[0]), fmt_num(to[1])),
        PathSegment::Close => "close".to_string(),
    }
}
//#endregion 🔖DslSegments

//#region 🔖DslLayer
/// 🧱 Fields shared by every layer kind's `DrawLayerBase`, accumulated by `parse_base_and_extra_attrs`
/// before the caller wraps them (plus its own kind-specific fields) into a `DrawLayerNode` variant.
struct DrawBaseAttrs {
    visible: bool,
    locked: bool,
    opacity: f64,
    blend_mode: String,
    transform: DrawTransform,
    fill: Option<FillStyle>,
    stroke: Option<StrokeStyle>,
}

fn build_base(id: String, name: String, attrs: DrawBaseAttrs) -> DrawLayerBase {
    DrawLayerBase {
        id,
        name,
        visible: attrs.visible,
        locked: attrs.locked,
        opacity: attrs.opacity,
        blend_mode: attrs.blend_mode,
        transform: attrs.transform,
        attributes: DrawAttributes { fill: attrs.fill, stroke: attrs.stroke },
    }
}

fn print_base_attrs(base: &DrawLayerBase) -> Vec<String> {
    let mut out = vec![
        format!("visible={}", base.visible),
        format!("locked={}", base.locked),
        format!("opacity={}", fmt_num(base.opacity)),
        format!("blend={}", base.blend_mode),
        format!("transform={}", print_transform(&base.transform)),
        format!("fill={}", print_fill(&base.attributes.fill)),
        format!("stroke={}", print_stroke_value(&base.attributes.stroke)),
    ];
    if let Some(stroke) = &base.attributes.stroke {
        out.extend(print_stroke_trailing_attrs(stroke));
    }
    out
}

/// 🔁 Parses every base `DrawLayerBase` attribute (`visible=`/`locked=`/`opacity=`/`blend=`/
/// `transform=`/`fill=`/`stroke=` + its trailing `width=`/`cap=`/`join=`/`dash=`), delegating any key it
/// doesn't recognize to `handle_extra` so each layer kind supplies only its own extra fields.
fn parse_base_and_extra_attrs(p: &mut DrawDslParser, mut handle_extra: impl FnMut(&mut DrawDslParser, &str) -> Result<bool, TextError>) -> Result<DrawBaseAttrs, TextError> {
    let mut visible = true;
    let mut locked = false;
    let mut opacity = 1.0;
    let mut blend_mode = "normal".to_string();
    let mut transform = default_draw_transform();
    let mut fill = None;
    let mut stroke: Option<StrokeStyle> = None;
    while p.at_attr() {
        let key = p.peek_attr_key().expect("at_attr confirmed an identifier").to_string();
        match key.as_str() {
            "visible" => {
                take_attr_key(p)?;
                visible = p.expect_bool()?;
            }
            "locked" => {
                take_attr_key(p)?;
                locked = p.expect_bool()?;
            }
            "opacity" => {
                take_attr_key(p)?;
                opacity = p.expect_num()?;
            }
            "blend" => {
                take_attr_key(p)?;
                blend_mode = p.expect_ident()?;
            }
            "transform" => {
                take_attr_key(p)?;
                transform = parse_transform(p)?;
            }
            "fill" => {
                take_attr_key(p)?;
                fill = parse_fill(p)?;
            }
            "stroke" => {
                take_attr_key(p)?;
                stroke = parse_stroke_value(p)?;
            }
            "width" | "cap" | "join" | "dash" if stroke.is_some() => {
                // 🩹 Trailing stroke attrs land here; re-parse them onto the just-parsed stroke.
                let mut s = stroke.take().expect("stroke.is_some() checked above");
                match key.as_str() {
                    "width" => {
                        take_attr_key(p)?;
                        s.width = p.expect_num()?;
                    }
                    "cap" => {
                        take_attr_key(p)?;
                        s.cap = p.expect_ident()?;
                    }
                    "join" => {
                        take_attr_key(p)?;
                        s.join = p.expect_ident()?;
                    }
                    _ => {
                        take_attr_key(p)?;
                        s.dash = Some(parse_num_list(p)?);
                    }
                }
                stroke = Some(s);
            }
            _ => {
                if !handle_extra(p, &key)? {
                    break;
                }
            }
        }
    }
    Ok(DrawBaseAttrs { visible, locked, opacity, blend_mode, transform, fill, stroke })
}

fn parse_shape_layer(p: &mut DrawDslParser, id: String, name: String) -> Result<DrawLayerNode, TextError> {
    let mut shape_kind = String::new();
    let mut rect = None;
    let mut ellipse = None;
    let mut circle = None;
    let mut line = None;
    let mut polygon = None;
    let base_attrs = parse_base_and_extra_attrs(p, |p, key| -> Result<bool, TextError> {
        match key {
            "shapeKind" => {
                take_attr_key(p)?;
                shape_kind = p.expect_ident()?;
                Ok(true)
            }
            "rect" => {
                take_attr_key(p)?;
                p.expect_tok(&DrawTok::LParen, "(")?;
                let x = p.expect_num()?;
                p.expect_tok(&DrawTok::Comma, ",")?;
                let y = p.expect_num()?;
                p.expect_tok(&DrawTok::Comma, ",")?;
                let width = p.expect_num()?;
                p.expect_tok(&DrawTok::Comma, ",")?;
                let height = p.expect_num()?;
                p.expect_tok(&DrawTok::RParen, ")")?;
                rect = Some(DrawRect { x, y, width, height });
                Ok(true)
            }
            "ellipse" => {
                take_attr_key(p)?;
                p.expect_tok(&DrawTok::LParen, "(")?;
                let cx = p.expect_num()?;
                p.expect_tok(&DrawTok::Comma, ",")?;
                let cy = p.expect_num()?;
                p.expect_tok(&DrawTok::Comma, ",")?;
                let rx = p.expect_num()?;
                p.expect_tok(&DrawTok::Comma, ",")?;
                let ry = p.expect_num()?;
                p.expect_tok(&DrawTok::RParen, ")")?;
                ellipse = Some(DrawEllipse { cx, cy, rx, ry });
                Ok(true)
            }
            "circle" => {
                take_attr_key(p)?;
                p.expect_tok(&DrawTok::LParen, "(")?;
                let cx = p.expect_num()?;
                p.expect_tok(&DrawTok::Comma, ",")?;
                let cy = p.expect_num()?;
                p.expect_tok(&DrawTok::Comma, ",")?;
                let r = p.expect_num()?;
                p.expect_tok(&DrawTok::RParen, ")")?;
                circle = Some(DrawCircle { cx, cy, r });
                Ok(true)
            }
            "line" => {
                take_attr_key(p)?;
                p.expect_tok(&DrawTok::LParen, "(")?;
                let x1 = p.expect_num()?;
                p.expect_tok(&DrawTok::Comma, ",")?;
                let y1 = p.expect_num()?;
                p.expect_tok(&DrawTok::Comma, ",")?;
                let x2 = p.expect_num()?;
                p.expect_tok(&DrawTok::Comma, ",")?;
                let y2 = p.expect_num()?;
                p.expect_tok(&DrawTok::RParen, ")")?;
                line = Some(DrawLine { x1, y1, x2, y2 });
                Ok(true)
            }
            "polygon" => {
                take_attr_key(p)?;
                polygon = Some(DrawPolygon { points: parse_point_list(p)? });
                Ok(true)
            }
            _ => Ok(false),
        }
    })?;
    Ok(DrawLayerNode::Shape(DrawShapeBody { base: build_base(id, name, base_attrs), shape_kind, rect, ellipse, circle, line, polygon }))
}

fn print_shape(body: &DrawShapeBody) -> String {
    let mut parts = vec![format!("shape {} \"{}\"", body.base.id, escape_str(&body.base.name))];
    parts.extend(print_base_attrs(&body.base));
    parts.push(format!("shapeKind={}", body.shape_kind));
    if let Some(rect) = &body.rect {
        parts.push(format!("rect=({},{},{},{})", fmt_num(rect.x), fmt_num(rect.y), fmt_num(rect.width), fmt_num(rect.height)));
    }
    if let Some(ellipse) = &body.ellipse {
        parts.push(format!("ellipse=({},{},{},{})", fmt_num(ellipse.cx), fmt_num(ellipse.cy), fmt_num(ellipse.rx), fmt_num(ellipse.ry)));
    }
    if let Some(circle) = &body.circle {
        parts.push(format!("circle=({},{},{})", fmt_num(circle.cx), fmt_num(circle.cy), fmt_num(circle.r)));
    }
    if let Some(line) = &body.line {
        parts.push(format!("line=({},{},{},{})", fmt_num(line.x1), fmt_num(line.y1), fmt_num(line.x2), fmt_num(line.y2)));
    }
    if let Some(polygon) = &body.polygon {
        let points: Vec<String> = polygon.points.iter().map(|point| format!("{},{}", fmt_num(point[0]), fmt_num(point[1]))).collect();
        parts.push(format!("polygon=[{}]", points.join(" ")));
    }
    parts.join(" ")
}

fn parse_path_layer(p: &mut DrawDslParser, id: String, name: String) -> Result<DrawLayerNode, TextError> {
    let base_attrs = parse_base_and_extra_attrs(p, |_p, _key| Ok(false))?;
    let segments = parse_segments(p)?;
    Ok(DrawLayerNode::Path(DrawPathBody { base: build_base(id, name, base_attrs), segments }))
}

fn print_path(body: &DrawPathBody) -> String {
    let mut parts = vec![format!("path {} \"{}\"", body.base.id, escape_str(&body.base.name))];
    parts.extend(print_base_attrs(&body.base));
    let segments: Vec<String> = body.segments.iter().map(print_segment).collect();
    parts.push(format!("{{ {} }}", segments.join(" ")));
    parts.join(" ")
}

fn parse_text_layer(p: &mut DrawDslParser, id: String, name: String) -> Result<DrawLayerNode, TextError> {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut content = String::new();
    let mut size = 0.0;
    let base_attrs = parse_base_and_extra_attrs(p, |p, key| -> Result<bool, TextError> {
        match key {
            "x" => {
                take_attr_key(p)?;
                x = p.expect_num()?;
                Ok(true)
            }
            "y" => {
                take_attr_key(p)?;
                y = p.expect_num()?;
                Ok(true)
            }
            "content" => {
                take_attr_key(p)?;
                content = p.expect_str()?;
                Ok(true)
            }
            "size" => {
                take_attr_key(p)?;
                size = p.expect_num()?;
                Ok(true)
            }
            _ => Ok(false),
        }
    })?;
    Ok(DrawLayerNode::Text(DrawTextBody { base: build_base(id, name, base_attrs), x, y, content, size }))
}

fn print_text(body: &DrawTextBody) -> String {
    let mut parts = vec![format!("text {} \"{}\"", body.base.id, escape_str(&body.base.name))];
    parts.extend(print_base_attrs(&body.base));
    parts.push(format!("x={}", fmt_num(body.x)));
    parts.push(format!("y={}", fmt_num(body.y)));
    parts.push(format!("content=\"{}\"", escape_str(&body.content)));
    parts.push(format!("size={}", fmt_num(body.size)));
    parts.join(" ")
}

fn parse_image_layer(p: &mut DrawDslParser, id: String, name: String) -> Result<DrawLayerNode, TextError> {
    let mut image_key = String::new();
    let mut width = 0.0;
    let mut height = 0.0;
    let base_attrs = parse_base_and_extra_attrs(p, |p, key| -> Result<bool, TextError> {
        match key {
            "imageKey" => {
                take_attr_key(p)?;
                image_key = p.expect_ident()?;
                Ok(true)
            }
            "width" => {
                take_attr_key(p)?;
                width = p.expect_num()?;
                Ok(true)
            }
            "height" => {
                take_attr_key(p)?;
                height = p.expect_num()?;
                Ok(true)
            }
            _ => Ok(false),
        }
    })?;
    Ok(DrawLayerNode::Image(DrawImageBody { base: build_base(id, name, base_attrs), image_key, width, height }))
}

fn print_image(body: &DrawImageBody) -> String {
    let mut parts = vec![format!("image {} \"{}\"", body.base.id, escape_str(&body.base.name))];
    parts.extend(print_base_attrs(&body.base));
    parts.push(format!("imageKey={}", body.image_key));
    parts.push(format!("width={}", fmt_num(body.width)));
    parts.push(format!("height={}", fmt_num(body.height)));
    parts.join(" ")
}

fn parse_group_layer(p: &mut DrawDslParser, id: String, name: String) -> Result<DrawLayerNode, TextError> {
    let base_attrs = parse_base_and_extra_attrs(p, |_p, _key| Ok(false))?;
    p.expect_tok(&DrawTok::LBrace, "{")?;
    let mut children = Vec::new();
    while !matches!(p.peek(), DrawTok::RBrace) {
        children.push(parse_layer(p)?);
    }
    p.expect_tok(&DrawTok::RBrace, "}")?;
    Ok(DrawLayerNode::Group(DrawGroupBody { base: build_base(id, name, base_attrs), children }))
}

fn print_group(body: &DrawGroupBody) -> String {
    let mut parts = vec![format!("group {} \"{}\"", body.base.id, escape_str(&body.base.name))];
    parts.extend(print_base_attrs(&body.base));
    let children: Vec<String> = body.children.iter().map(print_layer_node).collect();
    parts.push(format!("{{ {} }}", children.join(" ")));
    parts.join(" ")
}

fn parse_boolean_layer(p: &mut DrawDslParser, id: String, name: String) -> Result<DrawLayerNode, TextError> {
    let mut operation = String::new();
    let mut children = Vec::new();
    let base_attrs = parse_base_and_extra_attrs(p, |p, key| -> Result<bool, TextError> {
        match key {
            "operation" => {
                take_attr_key(p)?;
                operation = p.expect_ident()?;
                Ok(true)
            }
            "children" => {
                take_attr_key(p)?;
                children = parse_id_list(p)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    })?;
    Ok(DrawLayerNode::Boolean(DrawBooleanBody { base: build_base(id, name, base_attrs), operation, children }))
}

fn print_boolean(body: &DrawBooleanBody) -> String {
    let mut parts = vec![format!("boolean {} \"{}\"", body.base.id, escape_str(&body.base.name))];
    parts.extend(print_base_attrs(&body.base));
    parts.push(format!("operation={}", body.operation));
    parts.push(format!("children=[{}]", body.children.join(" ")));
    parts.join(" ")
}

fn parse_trace_layer(p: &mut DrawDslParser, id: String, name: String) -> Result<DrawLayerNode, TextError> {
    let mut source_key = String::new();
    let mut threshold = 0.0;
    let mut simplify_epsilon = 0.0;
    let base_attrs = parse_base_and_extra_attrs(p, |p, key| -> Result<bool, TextError> {
        match key {
            "sourceKey" => {
                take_attr_key(p)?;
                source_key = p.expect_ident()?;
                Ok(true)
            }
            "threshold" => {
                take_attr_key(p)?;
                threshold = p.expect_num()?;
                Ok(true)
            }
            "simplify" => {
                take_attr_key(p)?;
                simplify_epsilon = p.expect_num()?;
                Ok(true)
            }
            _ => Ok(false),
        }
    })?;
    Ok(DrawLayerNode::Trace(DrawTraceBody { base: build_base(id, name, base_attrs), source_key, params: DrawTraceParams { threshold, simplify_epsilon } }))
}

fn print_trace(body: &DrawTraceBody) -> String {
    let mut parts = vec![format!("trace {} \"{}\"", body.base.id, escape_str(&body.base.name))];
    parts.extend(print_base_attrs(&body.base));
    parts.push(format!("sourceKey={}", body.source_key));
    parts.push(format!("threshold={}", fmt_num(body.params.threshold)));
    parts.push(format!("simplify={}", fmt_num(body.params.simplify_epsilon)));
    parts.join(" ")
}

fn parse_layer(p: &mut DrawDslParser) -> Result<DrawLayerNode, TextError> {
    let span = p.span();
    let kind = p.expect_ident()?;
    let id = p.expect_ident()?;
    let name = p.expect_str()?;
    match kind.as_str() {
        "shape" => parse_shape_layer(p, id, name),
        "path" => parse_path_layer(p, id, name),
        "text" => parse_text_layer(p, id, name),
        "image" => parse_image_layer(p, id, name),
        "group" => parse_group_layer(p, id, name),
        "boolean" => parse_boolean_layer(p, id, name),
        "trace" => parse_trace_layer(p, id, name),
        other => Err(TextError::expected(format!("unknown layer kind '{other}'"), span, "shape|path|text|image|group|boolean|trace")),
    }
}

fn print_layer_node(node: &DrawLayerNode) -> String {
    match node {
        DrawLayerNode::Shape(body) => print_shape(body),
        DrawLayerNode::Path(body) => print_path(body),
        DrawLayerNode::Text(body) => print_text(body),
        DrawLayerNode::Image(body) => print_image(body),
        DrawLayerNode::Group(body) => print_group(body),
        DrawLayerNode::Boolean(body) => print_boolean(body),
        DrawLayerNode::Trace(body) => print_trace(body),
    }
}
//#endregion 🔖DslLayer

//#region 🔖DslDocument
fn print_asset(key: &str, asset: &DrawImageAsset) -> String {
    let mut s = format!("asset {} mime=\"{}\" data=\"{}\"", key, escape_str(&asset.mime), escape_str(&asset.data));
    if let Some(width) = asset.width {
        s.push_str(&format!(" width={width}"));
    }
    if let Some(height) = asset.height {
        s.push_str(&format!(" height={height}"));
    }
    s
}

/// 📤 Renders `doc` as a list of self-delimited top-level statements (`doc`/`camera`/`artboard`/
/// `assets`/one per top-level layer) — joined with `"\n"` for the pretty `print_dsl`, or with `" "` to
/// embed a whole document inline in a one-line `setDocument` op (see `🔖OpText`).
fn print_dsl_chunks(doc: &DrawDocument) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut header = format!("doc {} schema={}", doc.id, doc.schema);
    if let Some(title) = &doc.title {
        header.push_str(&format!(" title=\"{}\"", escape_str(title)));
    }
    chunks.push(header);
    chunks.push(format!("camera x={} y={} zoom={}", fmt_num(doc.camera.x), fmt_num(doc.camera.y), fmt_num(doc.camera.zoom)));
    if let Some(artboard) = &doc.artboard {
        chunks.push(format!("artboard width={} height={}", fmt_num(artboard.width), fmt_num(artboard.height)));
    }
    if let Some(assets) = &doc.assets {
        let mut keys: Vec<&String> = assets.keys().collect();
        keys.sort();
        let entries: Vec<String> = keys.iter().map(|key| print_asset(key, &assets[*key])).collect();
        chunks.push(format!("assets {{ {} }}", entries.join(" ")));
    }
    for layer in &doc.layers {
        chunks.push(print_layer_node(layer));
    }
    chunks
}

fn parse_document(p: &mut DrawDslParser) -> Result<DrawDocument, TextError> {
    p.eat_keyword("doc")?;
    let id = p.expect_ident()?;
    let mut schema = String::new();
    let mut title = None;
    while p.at_attr() {
        let key = p.peek_attr_key().expect("at_attr confirmed an identifier").to_string();
        match key.as_str() {
            "schema" => {
                take_attr_key(p)?;
                schema = p.expect_ident()?;
            }
            "title" => {
                take_attr_key(p)?;
                title = Some(p.expect_str()?);
            }
            _ => break,
        }
    }
    p.eat_keyword("camera")?;
    let mut camera_x = 0.0;
    let mut camera_y = 0.0;
    let mut camera_zoom = 1.0;
    while p.at_attr() {
        let key = p.peek_attr_key().expect("at_attr confirmed an identifier").to_string();
        match key.as_str() {
            "x" => {
                take_attr_key(p)?;
                camera_x = p.expect_num()?;
            }
            "y" => {
                take_attr_key(p)?;
                camera_y = p.expect_num()?;
            }
            "zoom" => {
                take_attr_key(p)?;
                camera_zoom = p.expect_num()?;
            }
            _ => break,
        }
    }
    let camera = DrawCamera { x: camera_x, y: camera_y, zoom: camera_zoom };
    let mut artboard = None;
    if p.at_ident("artboard") {
        p.bump();
        let mut width = 0.0;
        let mut height = 0.0;
        while p.at_attr() {
            let key = p.peek_attr_key().expect("at_attr confirmed an identifier").to_string();
            match key.as_str() {
                "width" => {
                    take_attr_key(p)?;
                    width = p.expect_num()?;
                }
                "height" => {
                    take_attr_key(p)?;
                    height = p.expect_num()?;
                }
                _ => break,
            }
        }
        artboard = Some(DrawArtboard { width, height });
    }
    let mut assets = None;
    if p.at_ident("assets") {
        p.bump();
        p.expect_tok(&DrawTok::LBrace, "{")?;
        let mut map = std::collections::HashMap::new();
        while !matches!(p.peek(), DrawTok::RBrace) {
            p.eat_keyword("asset")?;
            let key = p.expect_ident()?;
            let mut mime = String::new();
            let mut data = String::new();
            let mut width = None;
            let mut height = None;
            while p.at_attr() {
                let attr_key = p.peek_attr_key().expect("at_attr confirmed an identifier").to_string();
                match attr_key.as_str() {
                    "mime" => {
                        take_attr_key(p)?;
                        mime = p.expect_str()?;
                    }
                    "data" => {
                        take_attr_key(p)?;
                        data = p.expect_str()?;
                    }
                    "width" => {
                        take_attr_key(p)?;
                        width = Some(p.expect_num()? as u32);
                    }
                    "height" => {
                        take_attr_key(p)?;
                        height = Some(p.expect_num()? as u32);
                    }
                    _ => break,
                }
            }
            map.insert(key, DrawImageAsset { mime, data, width, height });
        }
        p.expect_tok(&DrawTok::RBrace, "}")?;
        assets = Some(map);
    }
    let mut layers = Vec::new();
    while !matches!(p.peek(), DrawTok::Eof) {
        layers.push(parse_layer(p)?);
    }
    Ok(DrawDocument { schema, id, title, camera, layers, assets, artboard })
}

impl DocumentDsl for DrawDocument {
    const EXTENSION: &'static str = "draw";

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let tokens = lex_draw_dsl(text)?;
        let mut parser = DrawDslParser::new(tokens);
        parse_document(&mut parser)
    }

    fn print_dsl(&self) -> String {
        print_dsl_chunks(self).join("\n")
    }
}
//#endregion 🔖DslDocument
//#endregion 🔖Dsl

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
            PathSegment::Quad { ctrl, to } => PathSegment::Quad { ctrl: draw_map_point_by_matrix(matrix, *ctrl), to: draw_map_point_by_matrix(matrix, *to) },
            PathSegment::Cubic { ctrl1, ctrl2, to } => PathSegment::Cubic { ctrl1: draw_map_point_by_matrix(matrix, *ctrl1), ctrl2: draw_map_point_by_matrix(matrix, *ctrl2), to: draw_map_point_by_matrix(matrix, *to) },
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: draw_map_point_by_matrix(matrix, *to) },
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
        cubics.push((arc_ellipse_point(unit_ctrl1, rx, ry, cos_phi, sin_phi, cx, cy), arc_ellipse_point(unit_ctrl2, rx, ry, cos_phi, sin_phi, cx, cy), arc_ellipse_point(unit_to, rx, ry, cos_phi, sin_phi, cx, cy)));
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
            [mt * mt * from[0] + 2.0 * mt * t * ctrl[0] + t * t * to[0], mt * mt * from[1] + 2.0 * mt * t * ctrl[1] + t * t * to[1]]
        })
        .collect()
}

fn sample_cubic_points(from: [f64; 2], ctrl1: [f64; 2], ctrl2: [f64; 2], to: [f64; 2], steps: usize) -> Vec<[f64; 2]> {
    (1..=steps)
        .map(|step| {
            let t = step as f64 / steps as f64;
            let mt = 1.0 - t;
            [mt * mt * mt * from[0] + 3.0 * mt * mt * t * ctrl1[0] + 3.0 * mt * t * t * ctrl2[0] + t * t * t * to[0], mt * mt * mt * from[1] + 3.0 * mt * mt * t * ctrl1[1] + 3.0 * mt * t * t * ctrl2[1] + t * t * t * to[1]]
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
        PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => KernelSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to },
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
        KernelSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => PathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: *to },
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
    match kernel_2d_rs::booleans::boolean_paths_many(&kernel_inputs, &boolean.operation) {
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
    let rgba = if target_width == decoded.width() && target_height == decoded.height() { decoded.to_rgba8() } else { image::imageops::resize(&decoded.to_rgba8(), target_width, target_height, image::imageops::FilterType::Triangle) };
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

//#region 🔖EditOperations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum DrawOperation {
    SetLayerVisible {
        layer_id: String,
        visible: bool,
    },
    SetLayerLocked {
        layer_id: String,
        locked: bool,
    },
    SetLayerOpacity {
        layer_id: String,
        opacity: f64,
    },
    SetLayerBlendMode {
        layer_id: String,
        blend_mode: String,
    },
    SetLayerName {
        layer_id: String,
        name: String,
    },
    SetLayerTransform {
        layer_id: String,
        transform: DrawTransform,
    },
    SetFill {
        layer_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        fill: Option<FillStyle>,
    },
    SetStroke {
        layer_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        stroke: Option<StrokeStyle>,
    },
    SetBooleanOperation {
        layer_id: String,
        boolean_operation: String,
    },
    SetTraceParams {
        layer_id: String,
        params: DrawTraceParams,
    },
    AddLayer {
        #[serde(skip_serializing_if = "Option::is_none")]
        parent_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<usize>,
        layer: Box<DrawLayerNode>,
    },
    DuplicateLayer {
        layer_id: String,
    },
    RemoveLayer {
        layer_id: String,
    },
    ReorderLayer {
        layer_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        parent_id: Option<String>,
        index: usize,
    },
    SetCamera {
        camera: DrawCamera,
    },
    SetDocument {
        document: DrawDocument,
    },
}

pub fn apply_draw_edit_operation(doc: &DrawDocument, edit: &DrawOperation) -> DrawDocument {
    match edit {
        DrawOperation::SetDocument { document } => document.clone(),
        DrawOperation::SetCamera { camera } => DrawDocument { camera: camera.clone(), ..doc.clone() },
        DrawOperation::SetLayerVisible { layer_id, visible } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).visible = *visible;
        }),
        DrawOperation::SetLayerLocked { layer_id, locked } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).locked = *locked;
        }),
        DrawOperation::SetLayerOpacity { layer_id, opacity } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).opacity = *opacity;
        }),
        DrawOperation::SetLayerBlendMode { layer_id, blend_mode } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).blend_mode = blend_mode.clone();
        }),
        DrawOperation::SetLayerName { layer_id, name } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).name = name.clone();
        }),
        DrawOperation::SetLayerTransform { layer_id, transform } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).transform = transform.clone();
        }),
        DrawOperation::SetFill { layer_id, fill } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).attributes.fill = fill.clone();
        }),
        DrawOperation::SetStroke { layer_id, stroke } => mutate_draw_layer(doc, layer_id, |layer| {
            layer_base_mut(layer).attributes.stroke = stroke.clone();
        }),
        DrawOperation::SetBooleanOperation { layer_id, boolean_operation } => mutate_draw_layer(doc, layer_id, |layer| {
            if let DrawLayerNode::Boolean(boolean) = layer {
                boolean.operation = boolean_operation.clone();
            }
        }),
        DrawOperation::SetTraceParams { layer_id, params } => mutate_draw_layer(doc, layer_id, |layer| {
            if let DrawLayerNode::Trace(trace) = layer {
                trace.params = params.clone();
            }
        }),
        DrawOperation::AddLayer { parent_id, index, layer } => {
            let mut next = doc.clone();
            let at = index.unwrap_or(next.layers.len());
            insert_layer(&mut next.layers, parent_id.as_deref(), at, layer.as_ref().clone());
            next
        }
        DrawOperation::DuplicateLayer { layer_id: source_id } => {
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
        DrawOperation::RemoveLayer { layer_id } => {
            let mut next = doc.clone();
            remove_layer_from_tree(&mut next.layers, layer_id);
            next
        }
        DrawOperation::ReorderLayer { layer_id, parent_id, index } => {
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
            "ellipse" => {
                DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Ellipse"), shape_kind: "ellipse".into(), rect: None, ellipse: Some(DrawEllipse { cx: 0.0, cy: 0.0, rx: 64.0, ry: 48.0 }), circle: None, line: None, polygon: None })
            }
            "line" => DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Line"), shape_kind: "line".into(), rect: None, ellipse: None, circle: None, line: Some(DrawLine { x1: 0.0, y1: 0.0, x2: 128.0, y2: 0.0 }), polygon: None }),
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
    let value = if normalized.len() == 3 { normalized.chars().map(|c| format!("{c}{c}")).collect::<String>() } else { normalized.to_string() };
    let parse = |start: usize| u8::from_str_radix(&value[start..start + 2], 16).unwrap_or(0) as f64 / 255.0;
    [parse(0), parse(2), parse(4), alpha]
}

pub fn rgba_to_hex(color: [f64; 4]) -> String {
    let channel = |value: f64| format!("{:02x}", (value.clamp(0.0, 1.0) * 255.0).round() as u8);
    format!("#{}{}{}", channel(color[0]), channel(color[1]), channel(color[2]))
}

/// 🩹 Resolves an inspector field write (`name`/`opacity`/`fillColor`/`transformX`/…) to the granular
/// {@link DrawOperation} that carries it, so field edits flow through the typed VCS as convergent operations
/// instead of whole-document snapshots. Returns `None` for a missing layer or an unmapped field.
pub fn draw_op_for_layer_field(doc: &DrawDocument, layer_id: &str, field: &str, value: &serde_json::Value) -> Option<DrawOperation> {
    let layer = find_draw_layer(doc, layer_id)?;
    let operation = match field {
        "name" => DrawOperation::SetLayerName { layer_id: layer_id.into(), name: value.as_str().unwrap_or("").into() },
        "opacity" => DrawOperation::SetLayerOpacity { layer_id: layer_id.into(), opacity: value.as_f64().unwrap_or(1.0) },
        "visible" => DrawOperation::SetLayerVisible { layer_id: layer_id.into(), visible: value.as_bool().unwrap_or(true) },
        "locked" => DrawOperation::SetLayerLocked { layer_id: layer_id.into(), locked: value.as_bool().unwrap_or(false) },
        "blendMode" => DrawOperation::SetLayerBlendMode { layer_id: layer_id.into(), blend_mode: value.as_str().unwrap_or("normal").into() },
        "booleanOperation" => DrawOperation::SetBooleanOperation { layer_id: layer_id.into(), boolean_operation: value.as_str().unwrap_or("union").into() },
        "transformX" | "transformY" | "transformScaleX" | "transformScaleY" | "transformRotation" => {
            let mut transform = layer_base(layer).transform.clone();
            match field {
                "transformX" => transform.x = value.as_f64().unwrap_or(0.0),
                "transformY" => transform.y = value.as_f64().unwrap_or(0.0),
                "transformScaleX" => transform.scale_x = value.as_f64().unwrap_or(1.0),
                "transformScaleY" => transform.scale_y = value.as_f64().unwrap_or(1.0),
                _ => transform.rotation = value.as_f64().unwrap_or(0.0),
            }
            DrawOperation::SetLayerTransform { layer_id: layer_id.into(), transform }
        }
        "fillColor" => {
            let alpha = layer_base(layer)
                .attributes
                .fill
                .as_ref()
                .map(|fill| match fill {
                    FillStyle::Solid { color } => color[3],
                    FillStyle::LinearGradient { .. } | FillStyle::RadialGradient { .. } => 1.0,
                })
                .unwrap_or(1.0);
            DrawOperation::SetFill { layer_id: layer_id.into(), fill: Some(FillStyle::Solid { color: hex_to_rgba(value.as_str().unwrap_or("#000000"), alpha) }) }
        }
        "strokeWidth" => {
            let stroke = layer_base(layer).attributes.stroke.clone().unwrap_or(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 1.0, cap: "butt".into(), join: "miter".into(), dash: None });
            DrawOperation::SetStroke { layer_id: layer_id.into(), stroke: Some(StrokeStyle { width: value.as_f64().unwrap_or(1.0), ..stroke }) }
        }
        "traceThreshold" => {
            let DrawLayerNode::Trace(trace) = layer else { return None };
            let mut params = trace.params.clone();
            params.threshold = value.as_f64().unwrap_or(0.5);
            DrawOperation::SetTraceParams { layer_id: layer_id.into(), params }
        }
        "traceSimplify" => {
            let DrawLayerNode::Trace(trace) = layer else { return None };
            let mut params = trace.params.clone();
            params.simplify_epsilon = value.as_f64().unwrap_or(1.5);
            DrawOperation::SetTraceParams { layer_id: layer_id.into(), params }
        }
        _ => return None,
    };
    Some(operation)
}

pub fn patch_layer_field(doc: &DrawDocument, layer_id: &str, field: &str, value: &serde_json::Value) -> DrawDocument {
    match draw_op_for_layer_field(doc, layer_id, field, value) {
        Some(operation) => apply_draw_edit_operation(doc, &operation),
        None => doc.clone(),
    }
}
//#endregion 🔖EditOperations

//#region 🔖OpText
/// ⚡ One-line textual encoding of every `DrawOperation` variant (`vcs::OpText`). Reuses the value
/// grammars from `🔖Dsl` (transform/fill/stroke/segments/layer) so a full `DrawLayerNode` subtree
/// (`addLayer`) or an entire `DrawDocument` (`setDocument`) embeds inline on one line — the DSL grammar
/// never depends on newlines, so joining its chunks with `" "` instead of `"\n"` round-trips identically.
impl OpText for DrawOperation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let tokens = lex_draw_dsl(line)?;
        let mut p = DrawDslParser::new(tokens);
        let span = p.span();
        let op_name = p.expect_ident()?;
        let operation = match op_name.as_str() {
            "setLayerVisible" => {
                let layer_id = parse_kv_ident(&mut p, "layerId")?;
                let visible = parse_kv_bool(&mut p, "visible")?;
                DrawOperation::SetLayerVisible { layer_id, visible }
            }
            "setLayerLocked" => {
                let layer_id = parse_kv_ident(&mut p, "layerId")?;
                let locked = parse_kv_bool(&mut p, "locked")?;
                DrawOperation::SetLayerLocked { layer_id, locked }
            }
            "setLayerOpacity" => {
                let layer_id = parse_kv_ident(&mut p, "layerId")?;
                let opacity = parse_kv_num(&mut p, "opacity")?;
                DrawOperation::SetLayerOpacity { layer_id, opacity }
            }
            "setLayerBlendMode" => {
                let layer_id = parse_kv_ident(&mut p, "layerId")?;
                let blend_mode = parse_kv_ident(&mut p, "blend")?;
                DrawOperation::SetLayerBlendMode { layer_id, blend_mode }
            }
            "setLayerName" => {
                let layer_id = parse_kv_ident(&mut p, "layerId")?;
                let name = parse_kv_str(&mut p, "name")?;
                DrawOperation::SetLayerName { layer_id, name }
            }
            "setLayerTransform" => {
                let layer_id = parse_kv_ident(&mut p, "layerId")?;
                expect_key(&mut p, "transform")?;
                let transform = parse_transform(&mut p)?;
                DrawOperation::SetLayerTransform { layer_id, transform }
            }
            "setFill" => {
                let layer_id = parse_kv_ident(&mut p, "layerId")?;
                expect_key(&mut p, "fill")?;
                let fill = parse_fill(&mut p)?;
                DrawOperation::SetFill { layer_id, fill }
            }
            "setStroke" => {
                let layer_id = parse_kv_ident(&mut p, "layerId")?;
                expect_key(&mut p, "stroke")?;
                let stroke = parse_stroke_value(&mut p)?;
                DrawOperation::SetStroke { layer_id, stroke }
            }
            "setBooleanOperation" => {
                let layer_id = parse_kv_ident(&mut p, "layerId")?;
                let boolean_operation = parse_kv_ident(&mut p, "operation")?;
                DrawOperation::SetBooleanOperation { layer_id, boolean_operation }
            }
            "setTraceParams" => {
                let layer_id = parse_kv_ident(&mut p, "layerId")?;
                let threshold = parse_kv_num(&mut p, "threshold")?;
                let simplify_epsilon = parse_kv_num(&mut p, "simplify")?;
                DrawOperation::SetTraceParams { layer_id, params: DrawTraceParams { threshold, simplify_epsilon } }
            }
            "addLayer" => {
                let parent_id = parse_kv_opt_ident(&mut p, "parentId")?;
                let index = parse_kv_opt_num(&mut p, "index")?.map(|value| value as usize);
                expect_key_colon(&mut p, "layer")?;
                let layer = parse_layer(&mut p)?;
                DrawOperation::AddLayer { parent_id, index, layer: Box::new(layer) }
            }
            "duplicateLayer" => {
                let layer_id = parse_kv_ident(&mut p, "layerId")?;
                DrawOperation::DuplicateLayer { layer_id }
            }
            "removeLayer" => {
                let layer_id = parse_kv_ident(&mut p, "layerId")?;
                DrawOperation::RemoveLayer { layer_id }
            }
            "reorderLayer" => {
                let layer_id = parse_kv_ident(&mut p, "layerId")?;
                let parent_id = parse_kv_opt_ident(&mut p, "parentId")?;
                let index = parse_kv_num(&mut p, "index")? as usize;
                DrawOperation::ReorderLayer { layer_id, parent_id, index }
            }
            "setCamera" => {
                expect_key(&mut p, "camera")?;
                p.expect_tok(&DrawTok::LParen, "(")?;
                let x = p.expect_num()?;
                p.expect_tok(&DrawTok::Comma, ",")?;
                let y = p.expect_num()?;
                p.expect_tok(&DrawTok::Comma, ",")?;
                let zoom = p.expect_num()?;
                p.expect_tok(&DrawTok::RParen, ")")?;
                DrawOperation::SetCamera { camera: DrawCamera { x, y, zoom } }
            }
            "setDocument" => {
                let document = parse_document(&mut p)?;
                DrawOperation::SetDocument { document }
            }
            other => return Err(TextError::expected(format!("unknown draw operation '{other}'"), span, "known DrawOperation variant")),
        };
        Ok(operation)
    }

    fn print_op(&self) -> String {
        match self {
            DrawOperation::SetLayerVisible { layer_id, visible } => format!("setLayerVisible layerId={layer_id} visible={visible}"),
            DrawOperation::SetLayerLocked { layer_id, locked } => format!("setLayerLocked layerId={layer_id} locked={locked}"),
            DrawOperation::SetLayerOpacity { layer_id, opacity } => format!("setLayerOpacity layerId={layer_id} opacity={}", fmt_num(*opacity)),
            DrawOperation::SetLayerBlendMode { layer_id, blend_mode } => format!("setLayerBlendMode layerId={layer_id} blend={blend_mode}"),
            DrawOperation::SetLayerName { layer_id, name } => format!("setLayerName layerId={layer_id} name=\"{}\"", escape_str(name)),
            DrawOperation::SetLayerTransform { layer_id, transform } => format!("setLayerTransform layerId={layer_id} transform={}", print_transform(transform)),
            DrawOperation::SetFill { layer_id, fill } => format!("setFill layerId={layer_id} fill={}", print_fill(fill)),
            DrawOperation::SetStroke { layer_id, stroke } => {
                let mut line = format!("setStroke layerId={layer_id} stroke={}", print_stroke_value(stroke));
                if let Some(s) = stroke {
                    line.push(' ');
                    line.push_str(&print_stroke_trailing_attrs(s).join(" "));
                }
                line
            }
            DrawOperation::SetBooleanOperation { layer_id, boolean_operation } => format!("setBooleanOperation layerId={layer_id} operation={boolean_operation}"),
            DrawOperation::SetTraceParams { layer_id, params } => format!("setTraceParams layerId={layer_id} threshold={} simplify={}", fmt_num(params.threshold), fmt_num(params.simplify_epsilon)),
            DrawOperation::AddLayer { parent_id, index, layer } => format!("addLayer parentId={} index={} layer:{}", print_opt_ident(parent_id), print_opt_num(index), print_layer_node(layer)),
            DrawOperation::DuplicateLayer { layer_id } => format!("duplicateLayer layerId={layer_id}"),
            DrawOperation::RemoveLayer { layer_id } => format!("removeLayer layerId={layer_id}"),
            DrawOperation::ReorderLayer { layer_id, parent_id, index } => format!("reorderLayer layerId={layer_id} parentId={} index={index}", print_opt_ident(parent_id)),
            DrawOperation::SetCamera { camera } => format!("setCamera camera=({},{},{})", fmt_num(camera.x), fmt_num(camera.y), fmt_num(camera.zoom)),
            DrawOperation::SetDocument { document } => format!("setDocument {}", print_dsl_chunks(document).join(" ")),
        }
    }
}
//#endregion 🔖OpText

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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawDiff {
    pub document: Option<DrawDocument>,
    pub camera: Option<DrawCamera>,
    pub layer_patches: Vec<DrawLayerTreePatch>,
    pub layers_removed: Vec<String>,
    pub layers_added: Vec<DrawLayerTreeAdd>,
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
            let index = add.index.unwrap_or(next.layers.len());
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

impl Operation<DrawDocument> for DrawOperation {
    type Diff = DrawDiff;

    fn diff(&self, _projection: &DrawDocument) -> DrawDiff {
        match self {
            DrawOperation::SetDocument { document } => DrawDiff { document: Some(document.clone()), ..Default::default() },
            DrawOperation::SetCamera { camera } => DrawDiff { camera: Some(camera.clone()), ..Default::default() },
            DrawOperation::SetLayerVisible { layer_id, visible } => DrawDiff { layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { visible: Some(*visible), ..Default::default() } }], ..Default::default() },
            DrawOperation::SetLayerLocked { layer_id, locked } => DrawDiff { layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { locked: Some(*locked), ..Default::default() } }], ..Default::default() },
            DrawOperation::SetLayerName { layer_id, name } => DrawDiff { layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { name: Some(name.clone()), ..Default::default() } }], ..Default::default() },
            DrawOperation::SetLayerOpacity { layer_id, opacity } => DrawDiff { layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { opacity: Some(*opacity), ..Default::default() } }], ..Default::default() },
            DrawOperation::SetLayerBlendMode { layer_id, blend_mode } => {
                DrawDiff { layer_patches: vec![DrawLayerTreePatch { layer_id: layer_id.clone(), base: DrawLayerBasePatch { blend_mode: Some(blend_mode.clone()), ..Default::default() } }], ..Default::default() }
            }
            DrawOperation::AddLayer { parent_id, index, layer } => DrawDiff { layers_added: vec![DrawLayerTreeAdd { parent_id: parent_id.clone(), index: *index, layer: layer.as_ref().clone() }], ..Default::default() },
            DrawOperation::RemoveLayer { layer_id } => DrawDiff { layers_removed: vec![layer_id.clone()], ..Default::default() },
            _ => DrawDiff { document: Some(apply_draw_edit_operation(_projection, self)), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &DrawDocument) -> Vec<Self> {
        vec![DrawOperation::SetDocument { document: projection.clone() }]
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
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
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
                out.push_str(&format!("C {} {} {} {} {} {} ", ctrl1[0], ctrl1[1], ctrl2[0], ctrl2[1], to[0], to[1]));
            }
            PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => out.push_str(&format!("A {} {} {} {} {} {} {} ", rx, ry, rotation, if *large_arc { 1 } else { 0 }, if *sweep { 1 } else { 0 }, to[0], to[1])),
            PathSegment::Close => out.push('Z'),
        }
    }
    out.trim().to_string()
}

fn resolve_draw_document_artboard(doc: &DrawDocument) -> (u32, u32) {
    if let Some(artboard) = &doc.artboard {
        return (artboard.width.max(1.0).round() as u32, artboard.height.max(1.0).round() as u32);
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
            let matrix = node.transform.iter().map(|value| value.to_string()).collect::<Vec<_>>().join(" ");
            if let Some(text) = node.text {
                let fill = node.fill.as_ref().map(fill_style_to_svg).unwrap_or_else(|| "black".into());
                return format!(r#"<g transform="matrix({matrix})" opacity="{}"><text x="0" y="{}" font-size="{}" fill="{fill}">{}</text></g>"#, node.opacity, text.size, text.size, escape_svg_text(&text.content));
            }
            if let Some(image) = node.image {
                return format!(r#"<g transform="matrix({matrix})" opacity="{}"><image href="{}" width="{}" height="{}"/></g>"#, node.opacity, image.src, image.width, image.height);
            }
            let d = path_segments_to_svg_d(&node.segments);
            if d.is_empty() {
                return String::new();
            }
            let fill = node.fill.as_ref().map(fill_style_to_svg).unwrap_or_else(|| "none".into());
            let stroke = node.stroke.as_ref().map(|style| rgba_to_svg_color(style.color)).unwrap_or_else(|| "none".into());
            let stroke_width = node.stroke.as_ref().map(|style| style.width).unwrap_or(0.0);
            format!(r#"<g transform="matrix({matrix})" opacity="{}"><path d="{d}" fill="{fill}" stroke="{stroke}" stroke-width="{stroke_width}" fill-rule="evenodd"/></g>"#, node.opacity)
        })
        .filter(|shape| !shape.is_empty())
        .collect::<Vec<_>>()
        .join("");
    let svg = format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="{width}" height="{height}">{shapes}</svg>"#);
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
        PathSegment::Cubic { ctrl1, ctrl2, to } => DwgPathSegment::Cubic { ctrl1: apply_draw_transform_point(transform, *ctrl1), ctrl2: apply_draw_transform_point(transform, *ctrl2), to: apply_draw_transform_point(transform, *to) },
        PathSegment::Arc { rx, ry, rotation, large_arc, sweep, to } => DwgPathSegment::Arc { rx: *rx, ry: *ry, rotation: *rotation, large_arc: *large_arc, sweep: *sweep, to: apply_draw_transform_point(transform, *to) },
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
        drawing.entities.push(semio_framework_core::DwgEntity { layer, color: semio_framework_core::DwgColor::ByLayer, geometry: semio_framework_core::DwgGeometry::Text { at: [x, y, 0.0], height: size, rotation: 0.0, content } });
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
    doc.artboard = Some(DrawArtboard { width: (drawing.extmax[0] - drawing.extmin[0]).max(1.0), height: (drawing.extmax[1] - drawing.extmin[1]).max(1.0) });
    serde_json::to_value(&doc).map_err(|error| error.to_string())
}
//#endregion 🔖MediaExport

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_document_has_path_layer() {
        let doc = default_draw_document("test", None);
        assert_eq!(doc.layers.len(), 1);
        assert!(matches!(doc.layers[0], DrawLayerNode::Path(_)));
    }

    #[test]
    fn dwg_export_import_round_trips_a_path_and_text_layer() {
        let path_layer = create_draw_path_layer("Outline", vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [10.0, 0.0] }, PathSegment::Cubic { ctrl1: [12.0, 2.0], ctrl2: [12.0, 6.0], to: [10.0, 8.0] }, PathSegment::Close]);
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
        let next = apply_draw_edit_operation(&doc, &DrawOperation::AddLayer { parent_id: None, index: None, layer: Box::new(layer) });
        assert_eq!(next.layers.len(), 2);
        let renamed = apply_draw_edit_operation(&next, &DrawOperation::SetLayerName { layer_id: id.clone(), name: "Box".into() });
        assert_eq!(find_draw_layer(&renamed, &id).map(|layer| layer_base(layer).name.as_str()), Some("Box"));
    }

    #[test]
    fn scene_nodes_include_shape_bounds() {
        let layer = create_draw_shape_layer_rect("Rect");
        let doc = DrawDocument { layers: vec![layer], ..default_draw_document("scene", None) };
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
    fn resolve_boolean_layer_segments_flattens_arcs_before_boolean_operation() {
        let mut doc = default_draw_document("bool-arc-test", None);
        doc.layers.clear();
        let path_a =
            create_draw_path_layer("A", vec![PathSegment::Move { to: [0.0, 0.0] }, PathSegment::Line { to: [10.0, 0.0] }, PathSegment::Arc { rx: 10.0, ry: 10.0, rotation: 0.0, large_arc: false, sweep: true, to: [0.0, 10.0] }, PathSegment::Close]);
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
        let segments = vec![PathSegment::Move { to: [10.0, 0.0] }, PathSegment::Arc { rx: 10.0, ry: 10.0, rotation: 0.0, large_arc: false, sweep: true, to: [0.0, 10.0] }];
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
        image::DynamicImage::ImageRgba8(image_buffer).write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).expect("encode png");
        let mut doc = default_draw_document("trace-test", None);
        doc.layers.clear();
        let mut assets = std::collections::HashMap::new();
        assets.insert("source".to_string(), DrawImageAsset { mime: "image/png".into(), data: BASE64.encode(&bytes), width: None, height: None });
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
        doc.artboard = None;
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

    #[test]
    fn default_draw_document_has_artboard_dimensions() {
        let doc = default_draw_document("blank", None);
        let artboard = doc.artboard.expect("default artboard");
        assert_eq!(artboard.width, 1024.0);
        assert_eq!(artboard.height, 1024.0);
    }

    //#region 🔖DslTests
    /// 🧬 Exercises every DSL construct (all 7 layer kinds, both gradient variants, solid fill, dashed
    /// stroke, all 6 path segment kinds, nested group, an asset, and quote/backslash/newline-bearing
    /// strings) so `assert_dsl_round_trip` is a real stress test, not just a smoke test.
    fn representative_draw_document() -> DrawDocument {
        let mut assets = std::collections::HashMap::new();
        assets.insert("src-1".to_string(), DrawImageAsset { mime: "image/png".into(), data: "aGVsbG8=".into(), width: Some(8), height: Some(8) });

        let mut rect_shape = create_draw_shape_layer_rect("Rect");
        if let DrawLayerNode::Shape(shape) = &mut rect_shape {
            shape.base.attributes.fill = Some(FillStyle::LinearGradient {
                x1: 0.0,
                y1: 0.0,
                x2: 10.0,
                y2: 10.0,
                stops: vec![GradientStop { offset: 0.0, color: [1.0, 0.0, 0.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 1.0, 1.0] }],
            });
            shape.base.attributes.stroke = Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 1.5, cap: "round".into(), join: "round".into(), dash: Some(vec![2.0, 4.0]) });
        }
        let rect_id = layer_id(&rect_shape).to_string();

        let line_shape = DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Line"), shape_kind: "line".into(), rect: None, ellipse: None, circle: None, line: Some(DrawLine { x1: 0.0, y1: 0.0, x2: 5.0, y2: 5.0 }), polygon: None });
        let line_id = layer_id(&line_shape).to_string();

        let polygon_shape = DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Polygon"), shape_kind: "polygon".into(), rect: None, ellipse: None, circle: None, line: None, polygon: Some(DrawPolygon { points: vec![[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]] }) });

        let mut radial_circle = DrawShapeBody { base: default_layer_base("RadialCircle"), shape_kind: "circle".into(), rect: None, ellipse: None, circle: Some(DrawCircle { cx: 1.0, cy: 2.0, r: 3.0 }), line: None, polygon: None };
        radial_circle.base.attributes.fill = Some(FillStyle::RadialGradient { cx: 1.0, cy: 2.0, r: 3.0, stops: vec![GradientStop { offset: 0.0, color: [1.0, 1.0, 1.0, 1.0] }, GradientStop { offset: 1.0, color: [0.0, 0.0, 0.0, 0.0] }] });
        let radial_circle = DrawLayerNode::Shape(radial_circle);

        let path_layer = create_draw_path_layer(
            "Path",
            vec![
                PathSegment::Move { to: [0.0, 0.0] },
                PathSegment::Line { to: [1.0, 0.0] },
                PathSegment::Quad { ctrl: [1.0, 1.0], to: [2.0, 1.0] },
                PathSegment::Cubic { ctrl1: [2.0, 2.0], ctrl2: [3.0, 2.0], to: [3.0, 3.0] },
                PathSegment::Arc { rx: 2.0, ry: 2.0, rotation: 0.0, large_arc: false, sweep: true, to: [1.0, -1.0] },
                PathSegment::Close,
            ],
        );

        let text_layer = DrawLayerNode::Text(DrawTextBody { base: default_layer_base("Label"), x: 4.0, y: 5.0, content: "semio \"draw\"\ndsl".into(), size: 12.0 });
        let image_layer = create_draw_image_layer("Image", "src-1");
        let trace_layer = create_draw_trace_layer("Trace", "src-1");
        let boolean_layer = create_draw_boolean_layer("Boolean", "xor", vec![rect_id.clone(), line_id]);

        let ellipse_shape = DrawLayerNode::Shape(DrawShapeBody { base: default_layer_base("Ellipse"), shape_kind: "ellipse".into(), rect: None, ellipse: Some(DrawEllipse { cx: 1.0, cy: 2.0, rx: 3.0, ry: 4.0 }), circle: None, line: None, polygon: None });
        let group_layer = DrawLayerNode::Group(DrawGroupBody { base: default_layer_base("Group \"nested\""), children: vec![ellipse_shape, radial_circle] });

        DrawDocument {
            schema: DRAW_DOCUMENT_SCHEMA.into(),
            id: "dsl-fixture".into(),
            title: Some("DSL Fixture \"Quotes\" \\ backslash".into()),
            camera: DrawCamera { x: 12.5, y: -3.0, zoom: 2.25 },
            layers: vec![rect_shape, line_shape, polygon_shape, path_layer, text_layer, image_layer, trace_layer, boolean_layer, group_layer],
            assets: Some(assets),
            artboard: Some(DrawArtboard { width: 640.0, height: 480.0 }),
        }
    }

    #[test]
    fn dsl_round_trips_representative_document() {
        vcs::test_support::assert_dsl_round_trip(&representative_draw_document());
    }

    #[test]
    fn dsl_round_trips_document_without_assets_or_artboard() {
        let mut doc = default_draw_document("no-extras", None);
        doc.assets = None;
        doc.artboard = None;
        doc.title = None;
        vcs::test_support::assert_dsl_round_trip(&doc);
    }

    #[test]
    fn dsl_round_trips_semio_example_fixture() {
        let dsl = include_str!("../example/semio.draw");
        let doc = DrawDocument::parse_dsl(dsl).expect("semio example fixture parses");
        assert_eq!(doc.id, "semio");
        assert_eq!(doc.title.as_deref(), Some("Semio Emblem"));
        assert_eq!(doc.layers.len(), 1);
        vcs::test_support::assert_dsl_round_trip(&doc);
    }

    #[test]
    fn op_text_round_trips_every_draw_operation_variant() {
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::SetLayerVisible { layer_id: "layer-1".into(), visible: false });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::SetLayerLocked { layer_id: "layer-1".into(), locked: true });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::SetLayerOpacity { layer_id: "layer-1".into(), opacity: 0.42 });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::SetLayerBlendMode { layer_id: "layer-1".into(), blend_mode: "multiply".into() });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::SetLayerName { layer_id: "layer-1".into(), name: "New \"Name\"\nline2".into() });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::SetLayerTransform { layer_id: "layer-1".into(), transform: DrawTransform { x: 1.0, y: -2.0, scale_x: 1.5, scale_y: 0.5, rotation: 0.3 } });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::SetFill { layer_id: "layer-1".into(), fill: None });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::SetFill { layer_id: "layer-1".into(), fill: Some(FillStyle::Solid { color: [0.1, 0.2, 0.3, 1.0] }) });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::SetStroke { layer_id: "layer-1".into(), stroke: None });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::SetStroke { layer_id: "layer-1".into(), stroke: Some(StrokeStyle { color: [0.0, 0.0, 0.0, 1.0], width: 2.0, cap: "butt".into(), join: "bevel".into(), dash: Some(vec![1.0, 2.0, 3.0]) }) });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::SetBooleanOperation { layer_id: "layer-1".into(), boolean_operation: "intersection".into() });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::SetTraceParams { layer_id: "layer-1".into(), params: DrawTraceParams { threshold: 0.33, simplify_epsilon: 1.1 } });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::AddLayer { parent_id: None, index: None, layer: Box::new(create_draw_shape_layer_rect("Added")) });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::AddLayer { parent_id: Some("group-1".into()), index: Some(2), layer: Box::new(create_draw_group_layer("Nested")) });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::DuplicateLayer { layer_id: "layer-1".into() });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::RemoveLayer { layer_id: "layer-1".into() });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::ReorderLayer { layer_id: "layer-1".into(), parent_id: None, index: 0 });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::ReorderLayer { layer_id: "layer-1".into(), parent_id: Some("group-1".into()), index: 3 });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::SetCamera { camera: DrawCamera { x: 10.0, y: 20.0, zoom: 1.5 } });
        vcs::test_support::assert_op_line_round_trip(&DrawOperation::SetDocument { document: representative_draw_document() });
    }

    #[test]
    fn document_text_round_trips_a_store_with_an_applied_operation() {
        let initial = default_draw_document("doc-text-test", None);
        let envelope = vcs::create_document_vcs_envelope(DRAW_DOCUMENT_SCHEMA, "doc-text-test", initial, None);
        let mut store: DrawStore = DocumentVcsStore::new(envelope);
        let layer = create_draw_shape_layer_rect("Added Rect");
        let layer_id_value = layer_id(&layer).to_string();
        store
            .dispatch(vcs::DocumentVcsCommand::Apply { operations: vec![DrawOperation::AddLayer { parent_id: None, index: None, layer: Box::new(layer) }], description: Some("add rect".into()) })
            .expect("apply add layer");
        store.dispatch(vcs::DocumentVcsCommand::Apply { operations: vec![DrawOperation::SetLayerOpacity { layer_id: layer_id_value, opacity: 0.5 }], description: Some("set opacity".into()) }).expect("apply set opacity");
        vcs::test_support::assert_document_text_round_trip(&store);
        vcs::test_support::assert_live_equals_replay(&store);
    }
    //#endregion 🔖DslTests
}
//#endregion 🧪Tests
