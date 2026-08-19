//! 🧬️ Cad snapshot schema — artifact-lane fields only.

use crate::artifacts::cad::{empty_cad_snapshot, CadDrawingChild, CadModelChild, CadNode, CadReferenceList};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Snapshot
/// 📸️ Persisted cad document snapshot (persistent fields of the artifact). Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: the four per-pane object/geometry field
/// pairs that used to duplicate `SemioBrepSnapshot`'s topology inline (`CadObject`/`CadGeometry` at
/// `crate::artifacts::cad::🦀️component.rs`) are replaced by four fixed composed
/// `s.stdio.semio.model` CHILD slots — one per `CadPaneId` — plus a forward `drawings` composition
/// slot per the design map's `cad | engineering assembly | model, drawing` row. `#[child(...)]`
/// drives `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.cad.cad")]
pub struct CadSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.model")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.model")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub building_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.model")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub energy_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.model")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structure_classic_model: Option<CadModelChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.drawing")]
    #[serde(default)]
    pub drawings: Vec<CadDrawingChild>,
    #[serde(default)]
    #[state(artifact)]
    pub references_by_model_definition_id: BTreeMap<String, CadReferenceList>,
    #[serde(default)]
    #[state(artifact)]
    pub nodes: Vec<CadNode>,
    #[serde(default = "default_model_definition_id")]
    #[state(artifact)]
    pub active_model_definition_id: String,
}

async fn default_model_definition_id() -> String {
    "spatial.shape".into()
}

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec (mirrors `✳️object`/`✳️kit`'s own — the working reference
/// for a composite subset's `enc_child`/`dec_child` helpers) — a handle is exactly two strings
/// (`child_id`, the target's `ArtifactRef` flattened via `to_uri()`), never the child's own content.
async fn hex_encode(bytes: &[u8]) -> String { bytes.iter().map(|b| format!("{b:02x}")).collect() }
async fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 { return Err(format!("odd hex length: {s:?}")); }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) async fn enc_str(s: &str) -> String { hex_encode(s.as_bytes()) }
pub(crate) async fn dec_str(s: &str) -> Result<String, String> { String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string()) }
pub(crate) async fn enc_ref(r: &store::os_io::ArtifactRef) -> String { enc_str(&r.to_uri()) }
pub(crate) async fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> { store::os_io::ArtifactRef::parse_uri(&dec_str(s)?) }

/// 🔧️ Local `split_top_level`/`strip_brackets` (bracket-depth-aware split, `[...]` unwrap) — same
/// shape as stdio's own `engine::triples` helpers, duplicated rather than imported since that
/// module is private to the stdio crate.
async fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
async fn split_top_level(s: &str, sep: char) -> Vec<&str> {
    if s.is_empty() { return Vec::new(); }
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (i, c) in s.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            c if c == sep && depth == 0 => { out.push(&s[start..i]); start = i + c.len_utf8(); }
            _ => {}
        }
    }
    out.push(&s[start..]);
    out
}

pub(crate) async fn enc_child<S>(c: &store::ArtifactChild<S>) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
pub(crate) async fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
pub(crate) async fn enc_child_opt<S>(c: &Option<store::ArtifactChild<S>>) -> String {
    match c { Some(c) => enc_child(c), None => "[]".to_string() }
}
pub(crate) async fn dec_child_opt<S>(s: &str) -> Result<Option<store::ArtifactChild<S>>, String> {
    if s == "[]" { return Ok(None); }
    Ok(Some(dec_child(s)?))
}
pub(crate) async fn enc_child_list<S>(items: &[store::ArtifactChild<S>]) -> String {
    format!("[{}]", items.iter().map(enc_child).collect::<Vec<_>>().join(","))
}
pub(crate) async fn dec_child_list<S>(s: &str) -> Result<Vec<store::ArtifactChild<S>>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_child).collect()
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️JsonFieldPrimitives
/// 🧾️ `nodes`/`references_by_model_definition_id` are structured (`Vec<CadNode>` /
/// `BTreeMap<String, Vec<CadReference>>`), already `Serialize`/`Deserialize`. Round 1's schema
/// restructuring added both fields to `CadSnapshot` but never wired them into
/// `print_cad_snapshot_body`/`parse_cad_snapshot_body` — confirmed by a real
/// `assert_document_text_round_trip`/`assert_document_pack_round_trip` failure (both silently
/// dropped every reload), not a hypothetical gap. Fixed the same way `enc_str`/`dec_str` already
/// hex-encode every other text field in this file: serialize to JSON, then hex-encode the JSON
/// bytes — one more line-oriented field, no new wire primitive.
async fn enc_json<T: Serialize>(value: &T) -> String {
    enc_str(&serde_json::to_string(value).expect("CadSnapshot structured fields are always JSON-serializable"))
}
async fn dec_json<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, String> {
    serde_json::from_str(&dec_str(s)?).map_err(|e| e.to_string())
}
//#endregion 🔖️JsonFieldPrimitives

//#region 🔖️TextPrimitives
async fn print_cad_snapshot_body(s: &CadSnapshot) -> String {
    format!(
        "schema={}\nid={}\nshapeModel={}\nbuildingModel={}\nenergyModel={}\nstructureClassicModel={}\ndrawings={}\nreferencesByModelDefinitionId={}\nnodes={}\nactiveModelDefinitionId={}",
        enc_str(&s.schema), enc_str(&s.id),
        enc_child_opt(&s.shape_model), enc_child_opt(&s.building_model), enc_child_opt(&s.energy_model), enc_child_opt(&s.structure_classic_model),
        enc_child_list(&s.drawings),
        enc_json(&s.references_by_model_definition_id),
        enc_json(&s.nodes),
        enc_str(&s.active_model_definition_id),
    )
}
async fn parse_cad_snapshot_body(body: &str) -> Result<CadSnapshot, String> {
    let mut snapshot = empty_cad_snapshot();
    let mut saw_schema = false;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        if let Some(rest) = line.strip_prefix("schema=") { snapshot.schema = dec_str(rest)?; saw_schema = true; }
        else if let Some(rest) = line.strip_prefix("id=") { snapshot.id = dec_str(rest)?; }
        else if let Some(rest) = line.strip_prefix("shapeModel=") { snapshot.shape_model = dec_child_opt(rest)?; }
        else if let Some(rest) = line.strip_prefix("buildingModel=") { snapshot.building_model = dec_child_opt(rest)?; }
        else if let Some(rest) = line.strip_prefix("energyModel=") { snapshot.energy_model = dec_child_opt(rest)?; }
        else if let Some(rest) = line.strip_prefix("structureClassicModel=") { snapshot.structure_classic_model = dec_child_opt(rest)?; }
        else if let Some(rest) = line.strip_prefix("drawings=") { snapshot.drawings = dec_child_list(rest)?; }
        else if let Some(rest) = line.strip_prefix("referencesByModelDefinitionId=") { snapshot.references_by_model_definition_id = dec_json(rest)?; }
        else if let Some(rest) = line.strip_prefix("nodes=") { snapshot.nodes = dec_json(rest)?; }
        else if let Some(rest) = line.strip_prefix("activeModelDefinitionId=") { snapshot.active_model_definition_id = dec_str(rest)?; }
        else { return Err(format!("cad snapshot: unknown line {line:?}")); }
    }
    if !saw_schema { return Err("cad snapshot: missing schema line".to_string()); }
    Ok(snapshot)
}
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
async fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
async fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
async fn write_str_lp(out: &mut Vec<u8>, s: &str) { write_bytes_lp(out, s.as_bytes()); }
async fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> { String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string()) }
async fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) { write_str_lp(out, &r.to_uri()); }
async fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> { store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?) }
async fn write_child<S>(out: &mut Vec<u8>, c: &store::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
async fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}
async fn write_child_opt<S>(out: &mut Vec<u8>, c: &Option<store::ArtifactChild<S>>) {
    match c {
        Some(c) => { out.push(1); write_child(out, c); }
        None => out.push(0),
    }
}
async fn read_child_opt<S>(reader: &mut store::ByteReader<'_>) -> Result<Option<store::ArtifactChild<S>>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? { 0 => Ok(None), _ => Ok(Some(read_child(reader)?)) }
}
async fn write_child_list<S>(out: &mut Vec<u8>, items: &[store::ArtifactChild<S>]) {
    store::pack_rt::write_varint_u64(out, items.len() as u64);
    for item in items { write_child(out, item); }
}
async fn read_child_list<S>(reader: &mut store::ByteReader<'_>) -> Result<Vec<store::ArtifactChild<S>>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut items = Vec::with_capacity(count as usize);
    for _ in 0..count { items.push(read_child(reader)?); }
    Ok(items)
}

async fn encode_cad_snapshot_binary(s: &CadSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &s.schema);
    write_str_lp(&mut out, &s.id);
    write_child_opt(&mut out, &s.shape_model);
    write_child_opt(&mut out, &s.building_model);
    write_child_opt(&mut out, &s.energy_model);
    write_child_opt(&mut out, &s.structure_classic_model);
    write_child_list(&mut out, &s.drawings);
    write_str_lp(&mut out, &serde_json::to_string(&s.references_by_model_definition_id).expect("CadReference map is always JSON-serializable"));
    write_str_lp(&mut out, &serde_json::to_string(&s.nodes).expect("CadNode list is always JSON-serializable"));
    write_str_lp(&mut out, &s.active_model_definition_id);
    out
}
async fn decode_cad_snapshot_binary(bytes: &[u8]) -> Result<CadSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT { return Err(format!("unsupported pack format {format}")); }
    let mut snapshot = empty_cad_snapshot();
    snapshot.schema = read_str_lp(&mut reader)?;
    snapshot.id = read_str_lp(&mut reader)?;
    snapshot.shape_model = read_child_opt(&mut reader)?;
    snapshot.building_model = read_child_opt(&mut reader)?;
    snapshot.energy_model = read_child_opt(&mut reader)?;
    snapshot.structure_classic_model = read_child_opt(&mut reader)?;
    snapshot.drawings = read_child_list(&mut reader)?;
    snapshot.references_by_model_definition_id = serde_json::from_str(&read_str_lp(&mut reader)?).map_err(|e| e.to_string())?;
    snapshot.nodes = serde_json::from_str(&read_str_lp(&mut reader)?).map_err(|e| e.to_string())?;
    snapshot.active_model_definition_id = read_str_lp(&mut reader)?;
    Ok(snapshot)
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack, real hex/bracket text + LEB128 binary primitives —
/// same upgrade `✳️object`/`✳️kit` made when they gained real `ArtifactChild<S>` slots (the old
/// `dsl::DslRecord`-derive-driven `Self::__dsl_spec()` path cannot express a composed child slot,
/// which has no `dsl::DslField` impl reachable from this crate).
impl store::ArtifactDsl for CadSnapshot {
    const EXTENSION: &'static str = "cad";
    async fn envelope_id() -> &'static str { "cad.cad" }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_cad_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    async fn print_dsl(&self) -> String {
        let body = print_cad_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for CadSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_cad_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
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
        decode_cad_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️Snapshot

