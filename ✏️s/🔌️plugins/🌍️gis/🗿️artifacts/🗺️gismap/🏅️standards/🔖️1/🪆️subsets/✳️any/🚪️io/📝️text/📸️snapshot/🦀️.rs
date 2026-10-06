//! 📜️ GIS map artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::GisMapSnapshot;

/// 🗺️ The bundled "reuse map" example document, handcrafted in the `.gismap` DSL.
pub const REUSE_MAP_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.gismap` DSL text into a `GisMapSnapshot`.
pub fn parse_dsl(text: &str) -> Result<GisMapSnapshot, semio_framework_diagnostic::TextError> {
    <GisMapSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `GisMapSnapshot` back to `.gismap` DSL text.
pub fn print_dsl(document: &GisMapSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type GisMapSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod pack_codec {
use crate::standards::v1::subsets::any::schema::snapshot::*;
use semio_framework_dsl_record::DslField;
use semio_framework_value::Number;
use semio_framework_value::ValueError;
use crate::standards::v1::subsets::any::io::binary::snapshot::owned_pack::*;
impl store::ArtifactDsl for GisMapSnapshot{
 const EXTENSION:&'static str="gismap";fn envelope_id()->&'static str{"gis.gismap"}
 fn print_dsl(&self)->String{let body=semio_framework_dsl_record::print(&Map::from_snapshot(self).__dsl_to_record(),&Map::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("GIS map identity");store::semio_format::wrap_text(&envelope,&body)}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|e|semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(e.to_string()).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))?;if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("GIS map native Text identity differs").to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))}let record=semio_framework_dsl_record::parse_exact(body,&Map::__dsl_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Document})?;ordinary(Map::__dsl_from_record(&record)?).map_err(positioned)}
}
}

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{gis_map_drawing_child_handle, gis_map_value_child_handle, GisMapDrawingChild, GisMapImageChild, GisMapValueChild, MapFeature};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};










}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{gis_map_drawing_child_handle, gis_map_value_child_handle, GisMapDrawingChild, GisMapImageChild, GisMapValueChild, MapFeature};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};

pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

pub(crate) fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}

pub(crate) fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}

pub(crate) fn enc_child<S>(c: &store::ArtifactChild<S>) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}

pub(crate) fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}

pub(crate) fn enc_child_opt<S>(c: &Option<store::ArtifactChild<S>>) -> String {
    match c {
        Some(c) => enc_child(c),
        None => "[]".to_string(),
    }
}

pub(crate) fn dec_child_opt<S>(s: &str) -> Result<Option<store::ArtifactChild<S>>, String> {
    if s == "[]" {
        return Ok(None);
    }
    Ok(Some(dec_child(s)?))
}

/// 🧾️ `positions`/`routes`/`regions` are structured (`Vec<MapFeature>`, already
/// `Serialize`/`Deserialize`): serialize to JSON, then hex-encode the JSON bytes — same convention
/// every other text field in this file already uses (`📐️cad`'s `enc_json`/`dec_json`).
pub(crate) fn enc_json<T: ToValue>(value: &T) -> String {
    enc_str(&semio_framework_pack_json::to_json_string(value))
}

pub(crate) fn dec_json<T: FromValue>(s: &str) -> Result<T, String> {
    semio_framework_pack_json::from_json_str(&dec_str(s)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())
}

/// 🔁️ One JSON report of carrying `dsl_text` through this subset's own codecs, for a
/// language-neutral test adapter. Same reachability wall as `gis_map_mutation_report_json`:
/// `store::ArtifactDsl`/`store::ArtifactPack` and their error types are unnameable outside this
/// crate, so the identity law's evidence has to be produced here and handed over as text.
///
/// `canonicalText` is `print_dsl` of the parsed document and `canonicalTextAgain` is `print_dsl` of
/// re-parsing that — [`store::ArtifactDsl`]'s own documented LAW is that canonical output is a
/// `parse_dsl` fixpoint (hand-written text may normalize on the way in), so the two must be
/// byte-identical while neither is required to equal the committed file. `packDecoded` comes back
/// through a SEPARATE binary codec, so agreeing on one snapshot cannot be achieved by carrying text
/// bytes across.
pub fn gis_map_identity_report_json(dsl_text: &str) -> Result<String, String> {
    let parsed = <GisMapSnapshot as store::ArtifactDsl>::parse_dsl(dsl_text).map_err(|error| error.to_string())?;
    let canonical = <GisMapSnapshot as store::ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <GisMapSnapshot as store::ArtifactDsl>::parse_dsl(&canonical).map_err(|error| error.to_string())?;
    let canonical_again = <GisMapSnapshot as store::ArtifactDsl>::print_dsl(&reparsed);
    let packed = <GisMapSnapshot as store::ArtifactPack>::encode_pack(&reparsed);
    let unpacked = <GisMapSnapshot as store::ArtifactPack>::decode_pack(&packed).map_err(|error| error.to_string())?;
    let report = semio_framework_pack_json::object([
        ("parsed".to_string(), semio_framework_pack_json::from_dsl_value(&parsed.to_value())),
        ("reparsed".to_string(), semio_framework_pack_json::from_dsl_value(&reparsed.to_value())),
        ("packDecoded".to_string(), semio_framework_pack_json::from_dsl_value(&unpacked.to_value())),
        ("canonicalText".to_string(), semio_framework_pack_json::Value::from(canonical.as_str())),
        ("canonicalTextAgain".to_string(), semio_framework_pack_json::Value::from(canonical_again.as_str())),
    ]);
    Ok(semio_framework_pack_json::to_string(&report))
}
}
pub use snapshot_wire_codec::*;

#[allow(unused_imports)]
mod snapshot_wire2_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::*;
use crate::standards::v1::subsets::any::io::text::snapshot::REUSE_MAP_EXAMPLE_TEXT;
use crate::mutations::{create_position, create_region, create_route, delete_position, delete_region, delete_route, replace_position_data, replace_region_data, replace_route_data};
use crate::standards::v1::subsets::any::schema::mutations::GisMapMutation;
use crate::{gis_map_snapshot_with_derived_children, GisMapDrawingChild, GisMapImageChild, GisMapSnapshot, GisMapValueChild, MapFeature};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_plugin::{io_dispatch,  ArtifactSerializer, ErasedComposeSource, IoDirection, IoKey, IoPayload};
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::png::v1_2::any::{circle_normal_form, compose_affine, flatten_segments, semio_transform_affine};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::svg::v1_1::any::SemioDrawingToSvg;
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};
use semio_s_artifact_stdio_svg::SvgSnapshot;
use serde_json::Value;
use std::collections::HashSet;

pub fn empty_gis_map_snapshot() -> GisMapSnapshot {
    GisMapSnapshot::default()
}

/// 🗺️ The default map document, seeded from the bundled reuse example (see
/// `crate::GisMapSnapshot`'s derive-generated `.gismap` DSL).
pub fn default_document() -> GisMapSnapshot {
    let parsed = <GisMapSnapshot as store::ArtifactDsl>::parse_dsl(REUSE_MAP_EXAMPLE_TEXT).unwrap_or_else(|_| empty_gis_map_snapshot());
    gis_map_snapshot_with_derived_children(parsed)
}

/// 🔑️ The `s.stdio.semio/v1/drawing` → `s.stdio.svg/1.1/*` `IoKey`, derived from
/// `SemioDrawingToSvg`'s own `FROM`/`INTO` dialect constants (no hardcoded coordinate strings —
/// stays correct if stdio ever renames the dialect).
pub(crate) fn drawing_to_svg_io_key() -> IoKey {
    let from = SemioDrawingToSvg::FROM;
    let into = SemioDrawingToSvg::INTO;
    IoKey {
        artifact_kind: from.artifact_kind.to_string(),
        standard: from.standard.0.to_string(),
        subset: from.subset.0.to_string(),
        direction: IoDirection::Export,
        format_kind: into.artifact_kind.to_string(),
        format_standard: into.standard.0.to_string(),
        format_subset: into.subset.0.to_string(),
    }
}

/// 🌉️ Renders a `SemioDrawingSnapshot` to real SVG text + dimensions through stdio's registered
/// `s.stdio.semio/v1/drawing` → `s.stdio.svg` bridge — the ONLY svg-producing call in this plugin
/// (no hand-rolled `<svg>` string emission left in gis).
pub(crate) fn render_drawing_to_svg(drawing: &SemioDrawingSnapshot) -> Result<(String, u32, u32), String> {
    let width = drawing.canvas.width.round().max(1.0) as u32;
    let height = drawing.canvas.height.round().max(1.0) as u32;
    let pack_bytes = <SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(drawing);
    let source = ErasedComposeSource { dialect: SemioDrawingToSvg::FROM, payload: IoPayload::Binary(pack_bytes) };
    let composed = ::semio_framework_async::poll::resolve_ready(io_dispatch(&drawing_to_svg_io_key(), std::slice::from_ref(&source))).map_err(|error| error.message)?;
    let svg_bytes = match composed.payload {
        IoPayload::Binary(bytes) => bytes,
        IoPayload::Text(_) => return Err("drawing->svg bridge returned Text, expected an ArtifactPack-encoded SvgSnapshot".into()),
    };
    let svg_snapshot = <SvgSnapshot as store::ArtifactPack>::decode_pack(&svg_bytes).map_err(|error| error.to_string())?;
    let svg_text = String::from_utf8(svg_snapshot.export_utf8()?).map_err(|error| error.to_string())?;
    Ok((svg_text, width, height))
}
}
pub use snapshot_wire2_codec::*;

#[allow(unused_imports)]
mod snapshot_wire3_codec {
use crate::standards::v1::subsets::any::schema::*;
use crate::standards::v1::subsets::any::io::text::snapshot::REUSE_MAP_EXAMPLE_TEXT;
use crate::mutations::{create_position, create_region, create_route, delete_position, delete_region, delete_route, replace_position_data, replace_region_data, replace_route_data};
use crate::standards::v1::subsets::any::schema::mutations::GisMapMutation;
use crate::{gis_map_snapshot_with_derived_children, GisMapDrawingChild, GisMapImageChild, GisMapSnapshot, GisMapValueChild, MapFeature};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_plugin::{io_dispatch,  ArtifactSerializer, ErasedComposeSource, IoDirection, IoKey, IoPayload};
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioRgba, SemioTransform};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::png::v1_2::any::{circle_normal_form, compose_affine, flatten_segments, semio_transform_affine};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::svg::v1_1::any::SemioDrawingToSvg;
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, DrawStyle, PathSegment, SemioDrawingSnapshot};
use semio_s_artifact_stdio_svg::SvgSnapshot;
use serde_json::Value;
use std::collections::HashSet;

/// 🧭️ Relocated from the artifact's `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): pure document helpers over
/// `GisMapSnapshot`/`MapFeature`, no app-state dependency — an artifact must never depend on an app.
pub(crate) fn value_to_dsl(value: &Value) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::from(value)
}

/// 📥️ Parses a `{ positions, routes, regions }` map-descriptor JSON into a `GisMapSnapshot` — each
/// array entry becomes a `MapFeature` keyed by its `id`, keeping the full object as the payload.
pub fn gis_map_document_from_descriptor_json(json: &str) -> GisMapSnapshot {
    let value: Value = serde_json::from_str(json).unwrap_or_else(|_| serde_json::json!({}));
    let features = |key: &str| -> Vec<MapFeature> {
        value
            .get(key)
            .and_then(|entry| entry.as_array())
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(|item| {
                        let id = item.get("id").and_then(|value| value.as_str())?.to_string();
                        Some(MapFeature { id, data: value_to_dsl(item) })
                    })
                    .collect()
            })
            .unwrap_or_default()
    };
    gis_map_snapshot_with_derived_children(GisMapSnapshot { positions: features("positions"), routes: features("routes"), regions: features("regions"), ..Default::default() })
}
}
pub use snapshot_wire3_codec::*;
