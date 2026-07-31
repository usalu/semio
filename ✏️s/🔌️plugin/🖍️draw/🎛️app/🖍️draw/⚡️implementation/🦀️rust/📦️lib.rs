//! ✏️ Draw app — document entities (constitutional: general).

use serde::{Deserialize, Serialize};
/// 🔁️ Reexported so downstream crates (e.g. `draw-plugin`) can call `DrawDocument::parse_dsl`/
/// `.print_dsl()` without taking a direct `vcs` dependency just for the trait.
pub use store::DocumentDsl;

pub const DRAW_DOCUMENT_SCHEMA: &str = "draw.document";
pub const DRAW_BLEND_MODES: &[&str] = &["normal", "multiply", "screen", "overlay", "darken", "lighten", "colorDodge", "colorBurn", "hardLight", "softLight", "difference", "exclusion", "hue", "saturation", "color", "luminosity"];
pub const DRAW_BOOLEAN_OPERATIONS: &[&str] = &["union", "difference", "intersection", "xor"];
pub const DRAW_SHAPE_KINDS: &[&str] = &["rect", "ellipse", "circle", "line", "polygon"];
pub const DRAW_UTILITY_IDS: &[&str] = &["selectMarquee", "selectLasso", "selectDirect", "pen", "shapeRect", "shapeEllipse", "shapeLine", "shapePolygon", "booleanCombine", "trace", "transformMove"];

//#region 🔖️Domain
// No `#[dsl(keyword = ...)]` on `DrawTransform`/`DrawTraceParams`/`DrawArtboard`: every field of
// these types is itself `#[dsl(block)]`, which already supplies the bare leading keyword from the
// FIELD's own name — an inner keyword too would double it (`transform { transform x=0 ... }`),
// same reasoning as `note`'s `NoteImageAsset`.
/// 🎥️ Camera pose (pan + zoom). Ephemeral view state owned by the `draw-plugin` app runtime struct
/// (`DrawInteractionState`), never a `DrawDocument` field — see `.🦑️repo/🎫️tickets/26/07/31/
/// MOVE-DRAW-PLUGIN-CAMERA-TO-RUNTIME-STATE`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DrawCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for DrawCamera {
    /// 🎯️ Matches the pre-migration `default_draw_document` camera: centered on its 1024x1024 artboard.
    fn default() -> Self {
        Self { x: 512.0, y: 512.0, zoom: 0.75 }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DrawTransform {
    pub x: f64,
    pub y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub rotation: f64,
}

// No keyword either: reached only through `Vec<GradientStop>` (a plain, un-tagged list) —
// `parse_record_body` self-terminates on the first unrecognized key regardless, the same reasoning
// verified for `note`'s `NoteImageAsset` nested inside a `Map` value slot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct GradientStop {
    pub offset: f64,
    pub color: [f64; 4],
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FillStyle {
    Solid { color: [f64; 4] },
    LinearGradient { x1: f64, y1: f64, x2: f64, y2: f64, #[dsl(table)] stops: Vec<GradientStop> },
    RadialGradient { cx: f64, cy: f64, r: f64, #[dsl(table)] stops: Vec<GradientStop> },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct StrokeStyle {
    pub color: [f64; 4],
    pub width: f64,
    pub cap: String,
    pub join: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash: Option<Vec<f64>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DrawAttributes {
    // `fill` is a sum type (`FillStyle` has several tagged variants), so it uses
    // `#[dsl(statements, block)]` — see `dsl::DslVariants`'s doc comment on `OptionStatements`.
    // `stroke` is a single record type, so a plain `#[dsl(block)]` scalar Option suffices.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[dsl(statements, block)]
    pub fill: Option<FillStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub stroke: Option<StrokeStyle>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DrawTraceParams {
    pub threshold: f64,
    pub simplify_epsilon: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DrawImageAsset {
    pub mime: String,
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DrawLayerBase {
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f64,
    pub blend_mode: String,
    #[dsl(block)]
    pub transform: DrawTransform,
    #[serde(default)]
    #[dsl(block)]
    pub attributes: DrawAttributes,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DrawRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DrawEllipse {
    pub cx: f64,
    pub cy: f64,
    pub rx: f64,
    pub ry: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DrawCircle {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DrawLine {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DrawPolygon {
    pub points: Vec<[f64; 2]>,
}

// Each body carries its own `#[dsl(keyword = ...)]` — required by the single-field tuple
// ("newtype") variants of `DrawLayerNode` below, which delegate their entire `RecordSpec` to the
// inner body's own spec (see `dsl::__rt::newtype_variant_spec`) rather than wrapping it in one more
// layer. `base: DrawLayerBase` replaces `#[serde(flatten)]` with `#[dsl(block)]` — the engine has no
// flatten-splice primitive (yet); a bare nested `base { ... }` line is the declarative equivalent.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "shape")]
pub struct DrawShapeBody {
    #[serde(flatten)]
    #[dsl(block)]
    pub base: DrawLayerBase,
    pub shape_kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub rect: Option<DrawRect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub ellipse: Option<DrawEllipse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub circle: Option<DrawCircle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub line: Option<DrawLine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub polygon: Option<DrawPolygon>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "path")]
pub struct DrawPathBody {
    #[serde(flatten)]
    #[dsl(block)]
    pub base: DrawLayerBase,
    #[dsl(statements, block)]
    pub segments: Vec<PathSegment>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "text")]
pub struct DrawTextBody {
    #[serde(flatten)]
    #[dsl(block)]
    pub base: DrawLayerBase,
    pub x: f64,
    pub y: f64,
    pub content: String,
    pub size: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "image")]
pub struct DrawImageBody {
    #[serde(flatten)]
    #[dsl(block)]
    pub base: DrawLayerBase,
    pub image_key: String,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "group")]
pub struct DrawGroupBody {
    #[serde(flatten)]
    #[dsl(block)]
    pub base: DrawLayerBase,
    #[dsl(statements, block)]
    pub children: Vec<DrawLayerNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "boolean")]
pub struct DrawBooleanBody {
    #[serde(flatten)]
    #[dsl(block)]
    pub base: DrawLayerBase,
    pub operation: String,
    pub children: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "trace")]
pub struct DrawTraceBody {
    #[serde(flatten)]
    #[dsl(block)]
    pub base: DrawLayerBase,
    pub source_key: String,
    #[dsl(block)]
    pub params: DrawTraceParams,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
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

// 🖊️ Keywords/field order are a genuine SUBSET of SVG path data's absolute commands
// (`M`/`L`/`Q`/`C`/`A`/`Z`), each field `#[dsl(positional)]` so a segment prints as compact
// command-then-args tokens — `M 1.25,196.933 L 36.25,161.125 ... Z` — instead of `move to=1.25,196.933`.
// Field order per variant mirrors the SVG spec's own argument order (e.g. `A rx ry rotation
// large-arc-flag sweep-flag x,y`) so it reads as real SVG path syntax, just space- instead of
// comma/space-mixed-delimited between commands.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum PathSegment {
    #[dsl(key = "M")]
    Move {
        #[dsl(positional)]
        to: [f64; 2],
    },
    #[dsl(key = "L")]
    Line {
        #[dsl(positional)]
        to: [f64; 2],
    },
    #[dsl(key = "Q")]
    Quad {
        #[dsl(positional)]
        ctrl: [f64; 2],
        #[dsl(positional)]
        to: [f64; 2],
    },
    #[dsl(key = "C")]
    Cubic {
        #[dsl(positional)]
        ctrl1: [f64; 2],
        #[dsl(positional)]
        ctrl2: [f64; 2],
        #[dsl(positional)]
        to: [f64; 2],
    },
    #[dsl(key = "A")]
    Arc {
        #[dsl(positional)]
        rx: f64,
        #[dsl(positional)]
        ry: f64,
        #[dsl(positional)]
        rotation: f64,
        #[dsl(positional)]
        large_arc: bool,
        #[dsl(positional)]
        sweep: bool,
        #[dsl(positional)]
        to: [f64; 2],
    },
    #[dsl(key = "Z")]
    Close,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct DrawArtboard {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "draw", layout = "lines")]
pub struct DrawDocument {
    pub schema: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[dsl(statements, block)]
    pub layers: Vec<DrawLayerNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<std::collections::BTreeMap<String, DrawImageAsset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[dsl(block)]
    pub artboard: Option<DrawArtboard>,
}


pub fn default_draw_transform() -> DrawTransform {
    DrawTransform { x: 0.0, y: 0.0, scale_x: 1.0, scale_y: 1.0, rotation: 0.0 }
}

pub fn default_draw_trace_params() -> DrawTraceParams {
    DrawTraceParams { threshold: 0.5, simplify_epsilon: 1.5 }
}
