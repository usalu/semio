//! 🧬️ GIS terrain snapshot schema — persistent fields only.
//!
//! P6 handcrafted `ArtifactDsl`/`ArtifactPack` (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`):
//! `GisTerrainSnapshot` now carries a real `store::ArtifactChild<SemioMeshSnapshot>` handle for its
//! `mesh` slot, which `dsl::DslRecord`'s derive cannot represent (no `DslField` impl for
//! `ArtifactChild<S>`) — the same reason `✳️object`/`✳️kit` (stdio) and `💠️lowpoly`/`📐️cad` hand-roll
//! their own codecs rather than deriving. This file follows their exact hex/bracket convention, never
//! a hand-written slot list — `#[derive(ArtifactSchema)]` still emits `field_states()`/the `#[child(…)]`
//! slot table for the top-level facets.

use crate::artifacts::gisterrain::{gis_terrain_mesh_child_handle, gis_terrain_mesh_content_key};
use schema::ArtifactSchema;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔹Snapshot
/// 📸️ Persisted GIS terrain document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.gis.gisterrain")]
pub struct GisTerrainSnapshot {
    #[state(persistent)]
    pub exaggeration: f64,
    /// 🔌️ `map:in`'s insertion point — last-imported `2d.map` descriptor JSON.
    #[state(persistent)]
    pub imported_features_json: String,
    /// 🕸️ Owned CHILD handle for this terrain's composed mesh representation (`s.stdio.semio.mesh`,
    /// ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`). Content-addressed off
    /// `(exaggeration, imported_features_json)` — the only two persisted fields — via
    /// `gis_terrain_mesh_child_handle`/`gis_terrain_mesh_content_key`; every constructor of a
    /// `GisTerrainSnapshot` (this file's `Default`, `apply_gis_terrain_mutation`,
    /// `GisTerrainDiff::apply`) re-derives it so the handle never drifts from what
    /// `gis_terrain_mesh_from_snapshot` would actually build. Replaces the placeholder-only
    /// `3d.mesh` `ArtifactKindSpec` this artifact used to re-declare (see
    /// `crate::artifacts::gisterrain::🦀️component.rs`'s removal comment).
    #[state(persistent)]
    #[child(kind = "s.stdio.semio.mesh")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>,
}

impl Default for GisTerrainSnapshot {
    fn default() -> Self {
        let mesh = Some(gis_terrain_mesh_child_handle(&gis_terrain_mesh_content_key(0.0, "")));
        Self { exaggeration: 0.0, imported_features_json: String::new(), mesh }
    }
}
//#endregion 🔹Snapshot

//#region 🔖️CodecPrimitives
fn hex_encode(bytes: &[u8]) -> String { bytes.iter().map(|b| format!("{b:02x}")).collect() }
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 { return Err(format!("odd hex length: {s:?}")); }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) fn enc_str(s: &str) -> String { hex_encode(s.as_bytes()) }
pub(crate) fn dec_str(s: &str) -> Result<String, String> { String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string()) }

pub(crate) fn enc_ref(r: &store::os_io::ArtifactRef) -> String { enc_str(&r.to_uri()) }
pub(crate) fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> { store::os_io::ArtifactRef::parse_uri(&dec_str(s)?) }

/// 🪪️ `[<hex child_id>,<hex target-uri>]` — the two-string handle, real and complete, never content.
pub(crate) fn enc_child<S>(c: &store::ArtifactChild<S>) -> String { format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target)) }
pub(crate) fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
pub(crate) fn enc_child_opt<S>(c: &Option<store::ArtifactChild<S>>) -> String {
    match c { Some(c) => enc_child(c), None => "[]".to_string() }
}
pub(crate) fn dec_child_opt<S>(s: &str) -> Result<Option<store::ArtifactChild<S>>, String> {
    if s == "[]" { return Ok(None); }
    Ok(Some(dec_child(s)?))
}
//#endregion 🔖️CodecPrimitives

//#region 🔖️TextPrimitives
fn print_gis_terrain_snapshot_body(s: &GisTerrainSnapshot) -> String {
    format!("exaggeration={}\nimportedFeaturesJson={}\nmesh={}", s.exaggeration, enc_str(&s.imported_features_json), enc_child_opt(&s.mesh))
}
fn parse_gis_terrain_snapshot_body(body: &str) -> Result<GisTerrainSnapshot, String> {
    let mut exaggeration = None;
    let mut imported_features_json = None;
    let mut mesh = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        if let Some(rest) = line.strip_prefix("exaggeration=") { exaggeration = Some(rest.trim().parse::<f64>().map_err(|e| e.to_string())?); }
        else if let Some(rest) = line.strip_prefix("importedFeaturesJson=") { imported_features_json = Some(dec_str(rest)?); }
        else if let Some(rest) = line.strip_prefix("mesh=") { mesh = dec_child_opt(rest)?; }
        // 🏔️ Any other line (`origin lon=… lat=…`, `position id=… …`) is the fixture's own
        // human-readable scenery sidecar data — real content, but NOT a `GisTerrainSnapshot` field;
        // `terrain_fixture_text::parse_descriptor` (`💡️inferences/🦀️component.rs`) reads it directly
        // off the same bundled `.gisterrain` file. Silently skipped here, exactly like the
        // derive-generated grammar this codec replaced.
        else { continue; }
    }
    Ok(GisTerrainSnapshot {
        exaggeration: exaggeration.ok_or_else(|| "gis terrain snapshot: missing exaggeration line".to_string())?,
        imported_features_json: imported_features_json.unwrap_or_default(),
        mesh,
    })
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) { write_bytes_lp(out, s.as_bytes()); }
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> { String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string()) }

pub(crate) fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) { write_str_lp(out, &r.to_uri()); }
pub(crate) fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}
pub(crate) fn write_child<S>(out: &mut Vec<u8>, c: &store::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
pub(crate) fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}
pub(crate) fn write_child_opt<S>(out: &mut Vec<u8>, c: &Option<store::ArtifactChild<S>>) {
    match c {
        Some(c) => { out.push(1); write_child(out, c); }
        None => out.push(0),
    }
}
pub(crate) fn read_child_opt<S>(reader: &mut store::ByteReader<'_>) -> Result<Option<store::ArtifactChild<S>>, String> {
    let presence = reader.read_u8().map_err(|e| e.to_string())?;
    if presence == 0 { Ok(None) } else { Ok(Some(read_child(reader)?)) }
}

fn encode_gis_terrain_snapshot_binary(s: &GisTerrainSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    out.extend_from_slice(&s.exaggeration.to_le_bytes());
    write_str_lp(&mut out, &s.imported_features_json);
    write_child_opt(&mut out, &s.mesh);
    out
}
fn decode_gis_terrain_snapshot_binary(bytes: &[u8]) -> Result<GisTerrainSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT { return Err(format!("unsupported pack format {format}")); }
    let exaggeration = f64::from_le_bytes(reader.read_bytes(8).map_err(|e| e.to_string())?.try_into().map_err(|_| "exaggeration: short read".to_string())?);
    let imported_features_json = read_str_lp(&mut reader)?;
    let mesh = read_child_opt(&mut reader)?;
    Ok(GisTerrainSnapshot { exaggeration, imported_features_json, mesh })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔹HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits — see this file's
/// module doc comment).
impl store::ArtifactDsl for GisTerrainSnapshot {
    const EXTENSION: &'static str = "gisterrain";
    fn envelope_id() -> &'static str { "gis.gisterrain" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_gis_terrain_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_gis_terrain_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for GisTerrainSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_gis_terrain_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        decode_gis_terrain_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔹HandcraftedArtifactCodecs
