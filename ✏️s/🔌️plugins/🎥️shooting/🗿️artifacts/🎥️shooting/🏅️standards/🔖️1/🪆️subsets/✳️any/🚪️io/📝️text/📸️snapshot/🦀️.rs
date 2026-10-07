//! 📜️ Shooting artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! `store::ArtifactDsl for ShootingSnapshot` is implemented directly on the artifact type (see
//! `🗿️artifacts/🎥️shooting/🦀️.rs`'s doc comment for why). This component only adds the thin
//! artifact-facing `parse_dsl`/`print_dsl` wrappers plus the canonical example-fixture constant and its
//! round-trip law.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::ShootingSnapshot;

/// 🗄️ The base-icon example snapshot, handcrafted in `shooting`'s DSL (`store::ArtifactDsl`).
pub const SHOOTING_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.shooting` DSL text into a `ShootingSnapshot`.
pub fn parse_dsl(text: &str) -> Result<ShootingSnapshot, semio_framework_diagnostic::TextError> {
    <ShootingSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `ShootingSnapshot` back to `.shooting` DSL text.
pub fn print_dsl(snapshot: &ShootingSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
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
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::ShootingDiff;
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

impl store::ArtifactDsl for ShootingSnapshot{
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
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::*;
use crate::{ShootingEmblemChild, ShootingSnapshot};
use semio_framework_pack_json::json;
use semio_framework_pack_json::Value;
use schema::ArtifactSchema;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::io::text::snapshot::write_svg_xml;
use semio_s_artifact_stdio_svg::SvgSnapshot;
use crate::ShootingAsset;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::ShootingCamera;
use crate::ShootingSavedCamera;
use crate::ShootingSceneLighting;
use crate::ShootingShot;

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
    ONCE.call_once(semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::register);
}

/// 🌉️ `SemioDrawingSnapshot` → real SVG text, entirely through stdio's registered
/// `s.stdio.semio/v1/drawing` → `s.stdio.svg/1.1/*` composer entry (`io_dispatch`) — never a
/// hand-rolled SVG string. Returns the exporter's own `write_svg_xml` output (raw `<svg>…</svg>`
/// markup, no semio envelope preamble) so callers can hand it straight to
/// `rasterize_svg_to_png_base64`/embed it in an `<img>`, exactly like the old `wrap_svg` output did.
pub(crate) fn shooting_drawing_to_svg_text(drawing: &SemioDrawingSnapshot) -> Result<String, String> {
    shooting_ensure_semio_drawing_bridge_registered();
    let key = semio_framework_plugin::IoKey {
        artifact_kind: "s.stdio.semio".into(),
        standard: "v1".into(),
        subset: "drawing".into(),
        direction: semio_framework_plugin::IoDirection::Export,
        format_kind: "s.stdio.svg".into(),
        format_standard: "1.1".into(),
        format_subset: "*".into(),
    };
    let source = semio_framework_plugin::ErasedComposeSource {
        dialect: semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.semio", standard: semio_framework_artifact_reference::StandardId("v1"), subset: semio_framework_artifact_reference::SubsetId("drawing") },
        payload: semio_framework_plugin::IoPayload::Binary(<SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(drawing)),
    };
    let composed = ::semio_framework_async::poll::resolve_ready(semio_framework_plugin::io_dispatch(&key, std::slice::from_ref(&source))).map_err(|error| error.message)?;
    let bytes = match composed.payload {
        semio_framework_plugin::IoPayload::Binary(bytes) => bytes,
        semio_framework_plugin::IoPayload::Text(_) => return Err("s.stdio.semio/v1/drawing -> s.stdio.svg dispatch returned Text, expected Binary (ArtifactPack)".into()),
    };
    let svg_snapshot = <SvgSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| error.to_string())?;
    write_svg_xml(&svg_snapshot.doc)
}
}
pub use snapshot_wire_codec::*;

mod scene_native_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::*;
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
