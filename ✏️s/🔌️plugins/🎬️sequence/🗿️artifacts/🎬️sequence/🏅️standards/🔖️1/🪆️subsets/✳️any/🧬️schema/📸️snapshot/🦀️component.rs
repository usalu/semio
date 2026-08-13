//! 🧬️ Sequence snapshot schema — artifact-lane fields only.

use crate::artifacts::sequence::{sequence_content_child_handle_and_cache, sequence_working_scene, SequenceContentChild, SequenceEdge, SequenceStep, SEQUENCE_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted sequence document snapshot. Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`
/// (`sequence→C:flow`): the inline `steps: Vec<SequenceStep>` / `edges: Vec<SequenceEdge>` content
/// fields are replaced by a fixed composed `s.stdio.semio.flow` CHILD slot — the sequence plugin no
/// longer defines its own step-DAG content model, it composes stdio's `flow` subset instead.
/// `#[child(...)]` drives `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.sequence.sequence")]
pub struct SequenceSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub content: SequenceContentChild,
}

impl Default for SequenceSnapshot {
    fn default() -> Self {
        default_snapshot()
    }
}

/// 🌱 Canonical default document used by the play app and examples.
pub fn default_snapshot() -> SequenceSnapshot {
    SequenceSnapshot::from_fixture(SequenceFixture {
        schema: SEQUENCE_DOCUMENT_SCHEMA.into(),
        steps: vec![
            SequenceStep {
                id: "step-1".into(),
                kind: "state.set".into(),
                params: crate::artifacts::sequence::StepParams::new()
                    .insert("key", neural_engine::Value::Atom(neural_engine::Atom::String("counter".into())))
                    .insert("value", neural_engine::Value::Atom(neural_engine::Atom::Integer(0))),
                x: 0.0,
                y: 0.0,
                slot: None,
                collapsed: false,
            },
            SequenceStep {
                id: "step-2".into(),
                kind: "log.print".into(),
                params: crate::artifacts::sequence::StepParams::new().insert("message", neural_engine::Value::Atom(neural_engine::Atom::String("hello sequence".into()))),
                x: 280.0,
                y: 0.0,
                slot: None,
                collapsed: false,
            },
        ],
        edges: vec![SequenceEdge { id: "edge-1".into(), from: "step-1".into(), to: "step-2".into() }],
    })
}
//#endregion 🔖️Snapshot

//#region 🔖️Fixture
/// 🌊️ The plain pre-migration document shape (`{schema, steps, edges}`) — this plugin's own
/// analog of `flow::FlowFixture`: the live editing representation `SequenceHost` and the WASM
/// bridge operate on, and the JSON wire contract `SequenceHost::to_json`/`load_json` still speak.
/// Bridges to/from the composed-child `SequenceSnapshot` via `to_fixture`/`from_fixture` below.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceFixture {
    pub schema: String,
    pub steps: Vec<SequenceStep>,
    pub edges: Vec<SequenceEdge>,
}

impl SequenceSnapshot {
    /// 🌱 Builds a persisted snapshot from a plain fixture — mints and caches a fresh
    /// content-addressed handle for the fixture's steps/edges.
    pub fn from_fixture(fixture: SequenceFixture) -> Self {
        Self { schema: fixture.schema, content: sequence_content_child_handle_and_cache(fixture.steps, fixture.edges) }
    }

    /// 🌱 Converts this snapshot into the plain fixture shape — reads the live steps/edges off the
    /// working-scene cache (see `sequence_working_scene`'s doc comment for the staleness gap this
    /// bridges).
    pub fn to_fixture(&self) -> SequenceFixture {
        let scene = sequence_working_scene(self);
        SequenceFixture { schema: self.schema.clone(), steps: scene.steps, edges: scene.edges }
    }
}
//#endregion 🔖️Fixture

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec (mirrors `📐️cad`/`✒️writer`'s own `enc_child`/`dec_child`)
/// — a handle is exactly two strings (`child_id`, the target's `ArtifactRef` flattened via
/// `to_uri()`), never the child's own content. `SequenceSnapshot` no longer derives
/// `dsl::DslRecord` (the composed child has no reachable `DslField` impl from this crate) — this
/// facet hand-rolls the whole `ArtifactDsl`/`ArtifactPack` codec instead.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}
fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}
fn enc_child(c: &SequenceContentChild) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
fn dec_child(s: &str) -> Result<SequenceContentChild, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️TextPrimitives
fn print_sequence_snapshot_body(s: &SequenceSnapshot) -> String {
    format!("schema={}\ncontent={}", enc_str(&s.schema), enc_child(&s.content))
}
fn parse_sequence_snapshot_body(body: &str) -> Result<SequenceSnapshot, String> {
    let mut schema = None;
    let mut content = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("content=") {
            content = Some(dec_child(rest)?);
        } else {
            return Err(format!("sequence snapshot: unknown line {line:?}"));
        }
    }
    Ok(SequenceSnapshot {
        schema: schema.ok_or_else(|| "sequence snapshot: missing schema line".to_string())?,
        content: content.ok_or_else(|| "sequence snapshot: missing content line".to_string())?,
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
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) {
    write_str_lp(out, &r.to_uri());
}
fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}
fn write_child(out: &mut Vec<u8>, c: &SequenceContentChild) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
fn read_child(reader: &mut store::ByteReader<'_>) -> Result<SequenceContentChild, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}

fn encode_sequence_snapshot_binary(s: &SequenceSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &s.schema);
    write_child(&mut out, &s.content);
    out
}
fn decode_sequence_snapshot_binary(bytes: &[u8]) -> Result<SequenceSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let content = read_child(&mut reader)?;
    Ok(SequenceSnapshot { schema, content })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for SequenceSnapshot {
    const EXTENSION: &'static str = "sequence";
    fn envelope_id() -> &'static str {
        "sequence.sequence"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_sequence_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_sequence_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SequenceSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_sequence_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        decode_sequence_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
