//! 📜️ Shooting artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! `pack::snapshot::ArtifactDsl for ShootingSnapshot` is implemented directly on the artifact type (see
//! `🗿️artifacts/🎥️shooting/🦀️.rs`'s doc comment for why). This component only adds the thin
//! artifact-facing `parse_dsl`/`print_dsl` wrappers plus the canonical example-fixture constant and its
//! round-trip law.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::ShootingSnapshot;

/// 🗄️ The base-icon example snapshot, handcrafted in `shooting`'s DSL (`pack::snapshot::ArtifactDsl`).
pub const SHOOTING_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.shooting` DSL text into a `ShootingSnapshot`.
pub fn parse_dsl(text: &str) -> Result<ShootingSnapshot, semio_framework_diagnostic::TextError> {
    <ShootingSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `ShootingSnapshot` back to `.shooting` DSL text.
pub fn print_dsl(snapshot: &ShootingSnapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(snapshot)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type ShootingSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod mutations_codec {
use crate::ShootingSnapshot;

/// 📥️ Decodes a committed snapshot document — the `📸️snapshot/⬅️before/🔣️.json` every leaf
/// fixture of this vocabulary shares — into a real [`ShootingSnapshot`].
pub fn decode_shooting_snapshot_json(text: &str) -> Result<ShootingSnapshot, String> {
    let json_value = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    let dsl_value = semio_framework_pack_json::to_dsl_value(&json_value);
    semio_framework_value::FromValue::from_value(dsl_value).map_err(|error| error.to_string())
}
}
pub use mutations_codec::*;


#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{ShootingAsset,ShootingEmblemChild,ShootingSavedCamera,ShootingSceneLighting,ShootingShot,SHOOTING_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;

impl pack::snapshot::ArtifactDsl for ShootingSnapshot{
 const EXTENSION:&'static str="shooting";
 fn envelope_id()->&'static str{"shooting.shooting"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{
  let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|e|semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?;
  if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Shooting native text envelope mismatch",semio_framework_diagnostic::TextSpan::at(1,1)))}
  let record=semio_framework_dsl_record::parse(body,&Self::__dsl_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Document})?;Self::__dsl_from_record(&record)
 }
 fn print_dsl(&self)->String{
  let body=semio_framework_dsl_record::print(&self.__dsl_to_record(),&Self::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("valid Shooting envelope");store::semio_format::wrap_text(&envelope,&body)
 }
}
}
#[cfg(test)] pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::{active_asset, active_shot, is_transparent_shooting_background, ShootingEmblemChild, ShootingSnapshot};
use semio_framework_pack_json::json;
use semio_framework_pack_json::Value;
use schema::ArtifactSchema;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::io::text::snapshot::write_svg_xml;
use semio_s_artifact_stdio_svg::SvgSnapshot;
use crate::ShootingAsset;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::ShootingCamera;
use crate::ShootingSavedCamera;
use crate::ShootingSceneLighting;
use crate::ShootingShot;

//#region 🔖️MediaExport
/// 🌉️ Builds a real `SemioDrawingSnapshot` (canvas + one named `DrawStyle` per painted primitive
/// and a single "scene" `DrawLayer`) from the active shot/asset/scene — replaces the old hand-rolled
/// SVG string builder. The shot `shape` becomes a real `Path` (a rectangle as four `Line`
/// segments, an ellipse as two `Arc` segments — `DrawNode` has no native ellipse/rect primitive,
/// `Path` is the recursive scene graph's only drawable shape, see `shooting_shape_path_segments`);
/// the emblem override (if any) becomes a real `Image` node (base64-decoded to raw bytes — the
/// drawing subset's own svg export leaf re-encodes them, this never touches SVG text directly);
/// the active asset's name becomes a `Text` node.
///
/// Honest lossy points versus the old hand-rolled SVG (queued as `stdio_gaps`, not worked around
/// here — see ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT
/// w5b report): (1) `DrawNode::Text` carries no font-size/text-anchor/font-family field, so the
/// label now renders left/top-anchored at the browser's default size instead of centered/bold
/// like before; (2) `DrawCanvas.background` is captured here for round-trip fidelity but the
/// `s.stdio.semio/v1/drawing` → svg export leaf never reads it — the background is therefore
/// painted as an explicit filled `Path` layer child instead, which the export leaf DOES lower
/// into real SVG markup.
pub(crate) fn shooting_scene_to_semio_drawing(snapshot: &ShootingSnapshot) -> (SemioDrawingSnapshot, u32, u32) {
    let shot = active_shot(snapshot);
    let asset = active_asset(snapshot);
    let (width, height) = shot.map_or((256, 256), |entry| (entry.width, entry.height));
    let shape = shot.map_or("rectangle", |entry| entry.shape.as_str());
    let shot_background = shot.and_then(|entry| entry.background.clone()).unwrap_or_else(|| snapshot.scene.background.clone());
    let label = asset.map_or("Untitled", |entry| entry.name.as_str());

    let mut children = Vec::new();
    let mut styles = vec![DrawStyle { name: "label".into(), fill: Some(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }), stroke: None, stroke_width: None, opacity: None }];
    let canvas_background = (!is_transparent_shooting_background(&shot_background)).then(|| shooting_hex_color_to_rgba(&shot_background).unwrap_or(SemioRgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }));
    children.push(DrawNode::Path { segments: shooting_shape_path_segments(shape, width as f64, height as f64), style: Some("shape".into()) });
    styles.insert(
        0,
        DrawStyle {
            name: "shape".into(),
            fill: Some(canvas_background.unwrap_or(SemioRgba { r: 0.0, g: 0.0, b: 0.0, a: 0.0 })),
            stroke: shooting_hex_color_to_rgba(&snapshot.scene.material.stroke),
            stroke_width: None,
            opacity: None,
        },
    );
    if let Some(bytes) = crate::shooting_emblem_bytes(snapshot).filter(|bytes| !bytes.is_empty()) {
        children.push(DrawNode::Image { at: SemioPoint2 { x: 0.0, y: 0.0 }, width: width as f64, height: height as f64, mime: "image/png".into(), bytes });
    }
    children.push(DrawNode::Text { value: label.to_string(), at: SemioPoint2 { x: width as f64 / 2.0, y: height as f64 * 0.92 }, style: Some("label".into()) });

    let drawing = SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: width as f64, height: height as f64, background: canvas_background },
        styles,
        layers: vec![DrawLayer { id: "scene".into(), name: "scene".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
    };
    (drawing, width, height)
}

/// ✏️ The shot `shape` field's only two real values, lowered to real path geometry: `"ellipse"`
/// draws a full ellipse via the standard two-`A`rc-command SVG technique (`M` to the right vertex,
/// two semicircular arcs back to the same point); anything else (`"rectangle"` in practice) draws
/// the four canvas edges as `L`ine segments.
fn shooting_shape_path_segments(shape: &str, width: f64, height: f64) -> Vec<PathSegment> {
    if shape == "ellipse" {
        let (cx, cy, rx, ry) = (width / 2.0, height / 2.0, width / 2.0, height / 2.0);
        vec![
            PathSegment::MoveTo { to: SemioPoint2 { x: cx + rx, y: cy } },
            PathSegment::ArcTo { rx, ry, x_rotation: 0.0, large_arc: true, sweep: false, to: SemioPoint2 { x: cx - rx, y: cy } },
            PathSegment::ArcTo { rx, ry, x_rotation: 0.0, large_arc: true, sweep: false, to: SemioPoint2 { x: cx + rx, y: cy } },
            PathSegment::Close,
        ]
    } else {
        vec![
            PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } },
            PathSegment::LineTo { to: SemioPoint2 { x: width, y: 0.0 } },
            PathSegment::LineTo { to: SemioPoint2 { x: width, y: height } },
            PathSegment::LineTo { to: SemioPoint2 { x: 0.0, y: height } },
            PathSegment::Close,
        ]
    }
}

/// 🎨️ `"#rrggbb"`/`"#rrggbbaa"` (the only two hex shapes the shooting document ever stores in
/// `scene.background`) into a `SemioRgba`. `None` for anything else (an empty string is handled
/// by the caller's own default-color fallback before this ever runs).
fn shooting_hex_color_to_rgba(hex: &str) -> Option<SemioRgba> {
    let trimmed = hex.trim().trim_start_matches('#');
    let byte = |s: &str| u8::from_str_radix(s, 16).ok().map(|v| v as f32 / 255.0);
    match trimmed.len() {
        6 => Some(SemioRgba { r: byte(&trimmed[0..2])?, g: byte(&trimmed[2..4])?, b: byte(&trimmed[4..6])?, a: 1.0 }),
        8 => Some(SemioRgba { r: byte(&trimmed[0..2])?, g: byte(&trimmed[2..4])?, b: byte(&trimmed[4..6])?, a: byte(&trimmed[6..8])? }),
        _ => None,
    }
}





/// 🖼️ Renders the active shot as a real SVG scene — shot shape as a filled background path, the
/// emblem override (if any) as an embedded raster image, and the asset name as a text label — via
/// the `s.stdio.semio/v1/drawing` → svg stdio bridge (`shooting_scene_to_semio_drawing` +
/// `shooting_drawing_to_svg_text`), never hand-rolled SVG string formatting.


/// 🌉️ `shooting_scene_svg` over an already-deserialized document `Value`.


/// 🖼️ Builds the icon-render host request JSON for `shot`/`asset` under `fixture`'s scene lighting —
/// consumed both by the icon window's `render()` and by the `exportActiveShot`/`exportAllShots` shell
/// commands (`🎮️commands/🖨️export`), two consumers. `fit` mirrors the scene window's centre-model
/// lane (`ShootingConfig::center_model`): the host re-targets the shot camera at the asset's bounding
/// sphere and backs off to frame it, so the icon shows what the centred scene shows.

//#endregion 🔖️MediaExport

/// 📄️ Parses the handcrafted DSL fixture once per call — used both for the in-plugin default document
/// and to bridge into the framework's still-JSON-only `App::example` surface, so
/// `crate::standards::v1::subsets::any::io::text::snapshot::SHOOTING_EXAMPLE_TEXT` stays the single source of truth for the
/// snapshot.
pub fn default_snapshot() -> ShootingSnapshot {
    crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(crate::standards::v1::subsets::any::io::text::snapshot::SHOOTING_EXAMPLE_TEXT).unwrap_or_else(|_| crate::empty_shooting_snapshot())
}

/// 🌉️ JSON bridge for `semio_framework_plugin`'s `App::example` override, which hardcodes
/// `serde_json::from_str` on its `document_json` parameter (shared framework machinery, out of scope
/// for this migration) — derives the JSON from the DSL fixture rather than keeping a second, redundant
/// JSON copy of it on disk.
pub fn default_snapshot_json() -> String {
    semio_framework_pack_json::to_json_string(&default_snapshot())
}

/// 🔌️ Runs the `s.stdio.semio/v1/drawing` composer registration exactly once per process —
/// idempotent (`register_composer_entries`/`register_document_codec` both overwrite on
/// re-registration, neither panics), so this is safe to call regardless of whether the hosting
/// OS/plugin runtime already ran stdio's own boot-time `plugin()` registration first.
pub(crate) fn shooting_ensure_semio_drawing_bridge_registered() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        semio_s_artifact_stdio_semio_drawing::io::register();
        (semio_s_artifact_stdio_semio_drawing_import::CONVERSION.register)();
        (semio_s_artifact_stdio_semio_drawing_export::CONVERSION.register)();
    });
}

/// 🌉️ `SemioDrawingSnapshot` → real SVG text, entirely through stdio's registered
/// `s.stdio.semio/v1/drawing` → `s.stdio.svg/1.1/*` composer entry (`io_dispatch`) — never a
/// hand-rolled SVG string. Returns the exporter's own `write_svg_xml` output (raw `<svg>…</svg>`
/// markup, no semio envelope preamble) so callers can hand it straight to
/// `rasterize_svg_to_png_base64`/embed it in an `<img>`, exactly like the old `wrap_svg` output did.
pub(crate) fn shooting_drawing_to_svg_text(drawing: &SemioDrawingSnapshot) -> Result<String, String> {
    shooting_ensure_semio_drawing_bridge_registered();
    let key = semio_framework_plugin::io::IoKey {
        artifact_kind: "s.stdio.semio".into(),
        standard: "v1".into(),
        subset: "drawing".into(),
        direction: semio_framework_plugin::io::IoDirection::Export,
        format_kind: "s.stdio.svg".into(),
        format_standard: "1.1".into(),
        format_subset: "*".into(),
    };
    let source = semio_framework_plugin::io::ErasedComposeSource {
        dialect: semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_artifact_reference::StandardId("v1"), subset: semio_framework_artifact_reference::SubsetId("drawing") },
        payload: semio_framework_plugin::io::IoPayload::Binary(<SemioDrawingSnapshot as pack::snapshot::ArtifactPack>::encode_pack(drawing)),
    };
    let composed = ::semio_framework_async::poll::resolve_ready(semio_framework_plugin::io::io_dispatch(&key, std::slice::from_ref(&source))).map_err(|error| error.message)?;
    let bytes = match composed.payload {
        semio_framework_plugin::io::IoPayload::Binary(bytes) => bytes,
        semio_framework_plugin::io::IoPayload::Text(_) => return Err("s.stdio.semio/v1/drawing -> s.stdio.svg dispatch returned Text, expected Binary (ArtifactPack)".into()),
    };
    let svg_snapshot = <SvgSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).map_err(|error| error.to_string())?;
    write_svg_xml(&svg_snapshot.doc)
}
}
pub use snapshot_wire_codec::*;

mod scene_native_codec {
use super::*;
use crate::{is_transparent_shooting_background, ShootingAsset, ShootingCamera, ShootingSavedCamera, ShootingSceneLighting, ShootingShot};
use semio_framework_pack_json::{json,Value};
pub fn shooting_scene_svg(snapshot: &ShootingSnapshot) -> Result<(String, u32, u32), String> {
    let (drawing, width, height) = shooting_scene_to_semio_drawing(snapshot);
    let svg = shooting_drawing_to_svg_text(&drawing)?;
    Ok((svg, width, height))
}

pub fn shooting_document_json_to_svg(value: &Value) -> Result<(String, u32, u32), String> {
    let dsl_value: semio_framework_value::DslValue = semio_framework_pack_json::to_dsl_value(value);
    let snapshot: ShootingSnapshot = semio_framework_value::FromValue::from_value(dsl_value).map_err(|error| error.to_string())?;
    shooting_scene_svg(&snapshot)
}

pub fn shooting_icon_render_request_json(snapshot: &ShootingSnapshot, shot: &ShootingShot, asset: &ShootingAsset, fallback_camera: &ShootingCamera, fit: bool) -> String {
    let vec3 = |v: [f64; 3]| Value::from(v.iter().map(|c| Value::from(*c)).collect::<Vec<Value>>());
    let camera = crate::shooting_resolve_shot_camera(snapshot, shot, fallback_camera);
    let scene = &snapshot.scene;
    let mut camera_value = json!({
        "position": vec3(camera.position),
        "target": vec3(camera.target),
        "zoom": camera.zoom,
        "fov": camera.fov,
        "projection": camera.projection.clone().unwrap_or_else(|| "perspective".into()),
    });
    if let (Some(object), Some(up)) = (camera_value.as_object_mut(), camera.up) {
        object.insert("up", vec3(up));
    }
    let mut value = json!({
        "assetUrl": asset.url.as_str(),
        "camera": camera_value,
        "fit": { "enabled": fit, "padding": 1.25 },
        "lights": {
            "ambientIntensity": scene.ambient.intensity,
            "ambientColor": scene.ambient.color.as_str(),
            "sunAzimuth": scene.sun.azimuth,
            "sunElevation": scene.sun.elevation,
            "sunIntensity": scene.sun.intensity,
            "sunColor": scene.sun.color.as_str(),
        },
        "width": shot.width,
        "height": shot.height,
        "format": shot.format.as_str(),
        "shape": if shot.shape == "ellipse" { "ellipse" } else { "rectangle" },
        "shadowEnabled": scene.shadow.enabled,
        "material": {
            "color": scene.material.color.as_str(),
            "metalness": scene.material.metalness,
            "roughness": scene.material.roughness,
            "emissive": scene.material.emissive.as_str(),
            "emissiveIntensity": scene.material.emissive_intensity,
            "stroke": scene.material.stroke.as_str(),
        },
    });
    if let Some(object) = value.as_object_mut() {
        let background = shot.background.clone().unwrap_or_else(|| scene.background.clone());
        if !is_transparent_shooting_background(&background) {
            object.insert("background", json!(background.as_str()));
        }
    }
    value.to_string()
}
}
pub use scene_native_codec::{shooting_scene_svg,shooting_document_json_to_svg,shooting_icon_render_request_json};
