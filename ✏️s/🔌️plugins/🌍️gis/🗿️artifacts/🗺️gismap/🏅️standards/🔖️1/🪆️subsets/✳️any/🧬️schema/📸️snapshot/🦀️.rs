//! 🧬️ GIS map snapshot schema — artifact-lane fields only.
//!
//! P6 handcrafted `ArtifactDsl`/`ArtifactPack` (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`):
//! `GisMapSnapshot` now carries real `store::ArtifactChild<…>` handles for its composed
//! `drawing`/`image`/`value` slots, which `dsl::DslRecord`'s derive cannot represent (no `DslField`
//! impl for `ArtifactChild<S>`) — same reason `🏔️gisterrain`/`💠️lowpoly`/`📐️cad` hand-roll their own
//! codecs. Follows their exact hex/bracket convention; `positions`/`routes`/`regions` (still real
//! `Vec<MapFeature>`, gis's own domain data, see `crate::🦀️.rs`'s
//! `🔖️Composition` region) round-trip via JSON-then-hex, matching `📐️cad`'s `enc_json`/`dec_json`
//! convention for its own structured (non-child) fields.

use crate::{gis_map_drawing_child_handle, gis_map_value_child_handle, GisMapDrawingChild, GisMapImageChild, GisMapValueChild, MapFeature};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};

//#region 🔹Snapshot
/// 📸️ Persisted GIS map document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.gis.gismap")]
pub struct GisMapSnapshot {
    #[state(artifact)]
    pub positions: Vec<MapFeature>,
    #[state(artifact)]
    pub routes: Vec<MapFeature>,
    #[state(artifact)]
    pub regions: Vec<MapFeature>,
    /// 🕸️ Composed `s.stdio.semio.drawing` child — see `crate::🦀️.rs`'s
    /// `🔖️Composition` region for the full stable-member design.
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub drawing: GisMapDrawingChild,
    /// 🕸️ Composed `s.stdio.semio.image` child — always absent today (see `🔖️Composition`'s doc).
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<GisMapImageChild>,
    /// 🕸️ Composed `s.stdio.semio.value` child — the lossless `{positions,routes,regions}` mirror.
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub value: GisMapValueChild,
}

impl Default for GisMapSnapshot {
    fn default() -> Self {
        Self { positions: Vec::new(), routes: Vec::new(), regions: Vec::new(), drawing: gis_map_drawing_child_handle(), image: None, value: gis_map_value_child_handle() }
    }
}
//#endregion 🔹Snapshot

//#region 🔖️CodecPrimitives
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
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
fn enc_json<T: ToValue>(value: &T) -> String {
    enc_str(&semio_framework_pack_json::to_json_string(value))
}
fn dec_json<T: FromValue>(s: &str) -> Result<T, String> {
    semio_framework_pack_json::from_json_str(&dec_str(s)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())
}
//#endregion 🔖️CodecPrimitives

//#region 🔖️TextPrimitives


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
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

pub(crate) fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) {
    write_str_lp(out, &r.to_uri());
}
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
        Some(c) => {
            out.push(1);
            write_child(out, c);
        }
        None => out.push(0),
    }
}
pub(crate) fn read_child_opt<S>(reader: &mut store::ByteReader<'_>) -> Result<Option<store::ArtifactChild<S>>, String> {
    let presence = reader.read_u8().map_err(|e| e.to_string())?;
    if presence == 0 {
        Ok(None)
    } else {
        Ok(Some(read_child(reader)?))
    }
}



//#endregion 🔖️BinaryPrimitives

#[path="📦️pack/🦀️.rs"]
mod owned_pack;

//#region 🌉️IdentityBridge
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
//#endregion 🌉️IdentityBridge

#[path="🪶️sqlite/🦀️.rs"]
mod sqlite;

#[cfg(test)]
#[path="🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_tests;
