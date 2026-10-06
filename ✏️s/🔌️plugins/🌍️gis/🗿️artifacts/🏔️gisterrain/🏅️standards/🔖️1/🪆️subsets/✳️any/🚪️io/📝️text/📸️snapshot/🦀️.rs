//! 📜️ GIS terrain artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::GisTerrainSnapshot;

/// 🏔️ The bundled "reuse terrain" example document, handcrafted in the `.gisterrain` DSL.
pub const REUSE_TERRAIN_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.gisterrain` DSL text into a `GisTerrainSnapshot`.
pub fn parse_dsl(text: &str) -> Result<GisTerrainSnapshot, semio_framework_diagnostic::TextError> {
    <GisTerrainSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `GisTerrainSnapshot` back to `.gisterrain` DSL text.
pub fn print_dsl(document: &GisTerrainSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type GisTerrainSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod pack_codec {
use crate::standards::v1::subsets::any::schema::snapshot::GisTerrainSnapshot;
use crate::standards::v1::subsets::any::io::binary::snapshot::owned_pack::*;
impl store::ArtifactDsl for GisTerrainSnapshot{
 const EXTENSION:&'static str="gisterrain";
 fn envelope_id()->&'static str{"gis.gisterrain"}
 fn print_dsl(&self)->String{let body=semio_framework_dsl_record::print(&Terrain::from_snapshot(self).__dsl_to_record(),&Terrain::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id(Self::envelope_id(),store::semio_format::Component::Dsl,1).expect("terrain envelope");store::semio_format::wrap_text(&envelope,&body)}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{let(envelope,body)=store::semio_format::split_text_preamble(text).map_err(|e|semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;if !envelope.matches_identity(Self::envelope_id(),store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "terrain native text identity differs", semio_framework_diagnostic::TextSpan::at(1, 1)))}let record=semio_framework_dsl_record::parse_exact(body,&Terrain::__dsl_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Document})?;let snapshot=Terrain::__dsl_from_record(&record)?.into_snapshot();if let Some(map)=&snapshot.imported_map{map.validate().map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message, semio_framework_diagnostic::TextSpan::at(1, 1)))?;}Ok(snapshot)}
}
}

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{gis_terrain_mesh_child_handle, gis_terrain_mesh_content_key};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;










}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{gis_terrain_mesh_child_handle, gis_terrain_mesh_content_key};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

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

/// 🪪️ `[<hex child_id>,<hex target-uri>]` — the two-string handle, real and complete, never content.
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

/// 🔁️ One JSON report of carrying `dsl_text` through this subset's own codecs, for a
/// language-neutral test adapter. Same reachability wall as `gis_terrain_mutation_report_json`:
/// `store::ArtifactDsl`/`store::ArtifactPack` and their error types are unnameable outside this
/// crate, so the identity law's evidence has to be produced here and handed over as text.
///
/// `canonicalText` is `print_dsl` of the parsed document and `canonicalTextAgain` is `print_dsl` of
/// re-parsing that — [`store::ArtifactDsl`]'s own documented LAW is that canonical output is a
/// `parse_dsl` fixpoint (hand-written text may normalize on the way in), so the two must be
/// byte-identical while neither is required to equal the committed file. `packDecoded` comes back
/// through a SEPARATE binary codec, so agreeing on one snapshot cannot be achieved by carrying text
/// bytes across.
pub fn gis_terrain_identity_report_json(dsl_text: &str) -> Result<String, String> {
    let parsed = <GisTerrainSnapshot as store::ArtifactDsl>::parse_dsl(dsl_text).map_err(|error| error.to_string())?;
    let canonical = <GisTerrainSnapshot as store::ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <GisTerrainSnapshot as store::ArtifactDsl>::parse_dsl(&canonical).map_err(|error| error.to_string())?;
    let canonical_again = <GisTerrainSnapshot as store::ArtifactDsl>::print_dsl(&reparsed);
    let packed = <GisTerrainSnapshot as store::ArtifactPack>::encode_pack(&reparsed);
    let unpacked = <GisTerrainSnapshot as store::ArtifactPack>::decode_pack(&packed).map_err(|error| error.to_string())?;
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
use crate::standards::v1::subsets::any::io::text::snapshot::REUSE_TERRAIN_EXAMPLE_TEXT;
use crate::{gis_terrain_mesh_child_handle, gis_terrain_mesh_content_key, GisTerrainSnapshot};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_surface::terrain::tiles;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use imported_map::{ImportedMap,ImportedProperty};

/// 🧭️ Relocated from the artifact's `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): pure document helpers over
/// `GisTerrainSnapshot`, no app-state dependency — an artifact must never depend on an app.
pub fn empty_gis_terrain_snapshot() -> GisTerrainSnapshot {
    let exaggeration = 1.0;
    let imported_map = None;
    let mesh = Some(gis_terrain_mesh_child_handle(&gis_terrain_mesh_content_key(exaggeration, imported_map.as_ref())));
    GisTerrainSnapshot { exaggeration, imported_map, mesh }
}

/// 🗺️ The default terrain document, seeded from the bundled reuse example's `gisterrain
/// exaggeration=...` header (see `crate::GisTerrainSnapshot`'s
/// derive-generated `.gisterrain` DSL).
pub fn default_terrain_document() -> GisTerrainSnapshot {
    <GisTerrainSnapshot as store::ArtifactDsl>::parse_dsl(REUSE_TERRAIN_EXAMPLE_TEXT).unwrap_or_else(|_| empty_gis_terrain_snapshot())
}

#[derive(ToValue)]
#[value(rename_all = "camelCase")]
pub(crate) struct TerrainSceneStyleJson<'a> {
    tile_url_template: &'a str,
    project_origin_lon: f64,
    project_origin_lat: f64,
    exaggeration: f64,
    color_ramp: &'a str,
    min_zoom: u32,
    max_zoom: u32,
}

/// 🏔️ Builds the `World3dScene.terrain_json` payload for a descriptor — the one place gis needs to
/// reach into `semio_framework_surface::terrain` beyond the wasm session itself (for the generic engine's
/// tile zoom bounds).
pub fn build_terrain_scene_json(descriptor: &TerrainDescriptorJson) -> String {
    let style = TerrainSceneStyleJson {
        tile_url_template: GIS_3D_TERRAIN_TILE_URL_TEMPLATE,
        project_origin_lon: descriptor.project_origin.lon,
        project_origin_lat: descriptor.project_origin.lat,
        exaggeration: descriptor.exaggeration,
        color_ramp: "hypsometric",
        min_zoom: tiles::TERRAIN_TILE_MIN_ZOOM,
        max_zoom: tiles::TERRAIN_TILE_MAX_ZOOM,
    };
    semio_framework_pack_json::to_json_string(&style)
}
}
pub use snapshot_wire2_codec::*;
