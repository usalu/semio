//! 🧬️ Process3d snapshot schema — persistent fields only.
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `stock`/`steps` are no longer
//! inline (`Stock`/`Vec<ProcessStep>`, duplicating `SolidSpec` geometry) — they compose real
//! `s.stdio.semio.brep`/`s.stdio.semio.flow` CHILD HANDLES. `#[derive(dsl::DslRecord)]` is dropped
//! (an `ArtifactChild<S>` field has no `dsl::DslField` impl reachable from this crate, same wall
//! `📐️cad`/`✳️object`/`✳️kit` hit) in favor of a hand-rolled `ArtifactDsl`/`ArtifactPack` — see
//! `🔖️HandcraftedArtifactCodecs` below, matching `📐️cad`'s own snapshot facet exactly.

use crate::artifacts::process3d::{Pose, Workshop};
use schema::ArtifactSchema;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted process3d document snapshot (persistent fields of the artifact). `stock_solid`/
/// `steps`/`tool_solids` are composed CHILD slots — `#[child(...)]` drives
/// `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written. Children must sit directly
/// on this struct (not nested inside a helper record) for the derive to see them — confirmed against
/// `🧬️schema/✨️derive/🦀️component.rs`'s field-walk, which only iterates a struct's own direct fields.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.process.process3d")]
pub struct Process3dSnapshot {
    #[state(artifact)]
    pub workshop: Workshop,
    #[state(artifact)]
    pub stock_id: String,
    #[state(artifact)]
    pub stock_label: String,
    #[state(artifact)]
    pub stock_pose: Pose,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.brep")]
    pub stock_solid: store::ArtifactChild<SemioBrepSnapshot>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub steps: store::ArtifactChild<SemioFlowSnapshot>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.brep")]
    #[serde(default)]
    pub tool_solids: Vec<store::ArtifactChild<SemioBrepSnapshot>>,
    #[state(artifact)]
    #[serde(default)]
    pub resolved_up_to: Option<usize>,
}

impl Default for Process3dSnapshot {
    fn default() -> Self {
        crate::artifacts::process3d::empty_process3d_snapshot()
    }
}

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec (mirrors `📐️cad`/`✳️object`/`✳️kit`'s own) — a handle is
/// exactly two strings (`child_id`, the target's `ArtifactRef` flattened via `to_uri()`), never the
/// child's own content.
fn hex_encode(bytes: &[u8]) -> String { bytes.iter().map(|b| format!("{b:02x}")).collect() }
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 { return Err(format!("odd hex length: {s:?}")); }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) fn enc_str(s: &str) -> String { hex_encode(s.as_bytes()) }
pub(crate) fn dec_str(s: &str) -> Result<String, String> { String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string()) }
pub(crate) fn enc_ref(r: &store::os_io::ArtifactRef) -> String { enc_str(&r.to_uri()) }
pub(crate) fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> { store::os_io::ArtifactRef::parse_uri(&dec_str(s)?) }

fn strip_brackets(s: &str) -> Result<&str, String> {
    s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))
}
fn split_top_level(s: &str, sep: char) -> Vec<&str> {
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

pub(crate) fn enc_child<S>(c: &store::ArtifactChild<S>) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
pub(crate) fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
pub(crate) fn enc_child_list<S>(items: &[store::ArtifactChild<S>]) -> String {
    format!("[{}]", items.iter().map(enc_child).collect::<Vec<_>>().join(","))
}
pub(crate) fn dec_child_list<S>(s: &str) -> Result<Vec<store::ArtifactChild<S>>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_child).collect()
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️JsonFieldPrimitives
/// 🧾️ `workshop`/`stock_pose` are structured but child-free — JSON-serialize then hex-encode
/// through the shared `enc_str`/`dec_str`, matching `📐️cad`'s established `enc_json`/`dec_json`
/// convention for structured fields that don't need a bespoke wire shape.
fn enc_json<T: Serialize>(value: &T) -> String {
    enc_str(&serde_json::to_string(value).expect("process3d structured fields are always JSON-serializable"))
}
fn dec_json<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, String> {
    serde_json::from_str(&dec_str(s)?).map_err(|e| e.to_string())
}
//#endregion 🔖️JsonFieldPrimitives

//#region 🔖️TextPrimitives
fn print_process3d_snapshot_body(s: &Process3dSnapshot) -> String {
    format!(
        "workshop={}\nstockId={}\nstockLabel={}\nstockPose={}\nstockSolid={}\nsteps={}\ntoolSolids={}\nresolvedUpTo={}",
        enc_json(&s.workshop),
        enc_str(&s.stock_id),
        enc_str(&s.stock_label),
        enc_json(&s.stock_pose),
        enc_child(&s.stock_solid),
        enc_child(&s.steps),
        enc_child_list(&s.tool_solids),
        enc_json(&s.resolved_up_to),
    )
}
fn parse_process3d_snapshot_body(body: &str) -> Result<Process3dSnapshot, String> {
    let mut snapshot = crate::artifacts::process3d::empty_process3d_snapshot();
    let mut saw_workshop = false;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        if let Some(rest) = line.strip_prefix("workshop=") { snapshot.workshop = dec_json(rest)?; saw_workshop = true; }
        else if let Some(rest) = line.strip_prefix("stockId=") { snapshot.stock_id = dec_str(rest)?; }
        else if let Some(rest) = line.strip_prefix("stockLabel=") { snapshot.stock_label = dec_str(rest)?; }
        else if let Some(rest) = line.strip_prefix("stockPose=") { snapshot.stock_pose = dec_json(rest)?; }
        else if let Some(rest) = line.strip_prefix("stockSolid=") { snapshot.stock_solid = dec_child(rest)?; }
        else if let Some(rest) = line.strip_prefix("steps=") { snapshot.steps = dec_child(rest)?; }
        else if let Some(rest) = line.strip_prefix("toolSolids=") { snapshot.tool_solids = dec_child_list(rest)?; }
        else if let Some(rest) = line.strip_prefix("resolvedUpTo=") { snapshot.resolved_up_to = dec_json(rest)?; }
        else { return Err(format!("process3d snapshot: unknown line {line:?}")); }
    }
    if !saw_workshop { return Err("process3d snapshot: missing workshop line".to_string()); }
    Ok(snapshot)
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
fn write_str_lp(out: &mut Vec<u8>, s: &str) { write_bytes_lp(out, s.as_bytes()); }
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> { String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string()) }
fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) { write_str_lp(out, &r.to_uri()); }
fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> { store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?) }
fn write_child<S>(out: &mut Vec<u8>, c: &store::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}
fn write_child_list<S>(out: &mut Vec<u8>, items: &[store::ArtifactChild<S>]) {
    store::pack_rt::write_varint_u64(out, items.len() as u64);
    for item in items { write_child(out, item); }
}
fn read_child_list<S>(reader: &mut store::ByteReader<'_>) -> Result<Vec<store::ArtifactChild<S>>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut items = Vec::with_capacity(count as usize);
    for _ in 0..count { items.push(read_child(reader)?); }
    Ok(items)
}

fn encode_process3d_snapshot_binary(s: &Process3dSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &serde_json::to_string(&s.workshop).expect("Workshop is always JSON-serializable"));
    write_str_lp(&mut out, &s.stock_id);
    write_str_lp(&mut out, &s.stock_label);
    write_str_lp(&mut out, &serde_json::to_string(&s.stock_pose).expect("Pose is always JSON-serializable"));
    write_child(&mut out, &s.stock_solid);
    write_child(&mut out, &s.steps);
    write_child_list(&mut out, &s.tool_solids);
    write_str_lp(&mut out, &serde_json::to_string(&s.resolved_up_to).expect("Option<usize> is always JSON-serializable"));
    out
}
fn decode_process3d_snapshot_binary(bytes: &[u8]) -> Result<Process3dSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT { return Err(format!("unsupported pack format {format}")); }
    let mut snapshot = crate::artifacts::process3d::empty_process3d_snapshot();
    snapshot.workshop = serde_json::from_str(&read_str_lp(&mut reader)?).map_err(|e| e.to_string())?;
    snapshot.stock_id = read_str_lp(&mut reader)?;
    snapshot.stock_label = read_str_lp(&mut reader)?;
    snapshot.stock_pose = serde_json::from_str(&read_str_lp(&mut reader)?).map_err(|e| e.to_string())?;
    snapshot.stock_solid = read_child(&mut reader)?;
    snapshot.steps = read_child(&mut reader)?;
    snapshot.tool_solids = read_child_list(&mut reader)?;
    snapshot.resolved_up_to = serde_json::from_str(&read_str_lp(&mut reader)?).map_err(|e| e.to_string())?;
    Ok(snapshot)
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ Real hex/bracket text + LEB128 binary primitives — same upgrade `✳️object`/`✳️kit`/`📐️cad`
/// made when they gained real `ArtifactChild<S>` slots (the old `dsl::DslRecord`-derive-driven
/// `Self::__dsl_spec()` path cannot express a composed child slot).
impl store::ArtifactDsl for Process3dSnapshot {
    const EXTENSION: &'static str = "process3d";
    fn envelope_id() -> &'static str { "process.process3d" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_process3d_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_process3d_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Process3dSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_process3d_snapshot_binary(self);
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
        decode_process3d_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️Snapshot
