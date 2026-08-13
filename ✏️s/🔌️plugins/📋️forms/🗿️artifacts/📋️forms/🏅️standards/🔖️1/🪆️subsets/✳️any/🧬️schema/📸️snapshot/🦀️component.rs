//! 🧬️ Forms snapshot schema — artifact-lane fields only.

use crate::artifacts::forms::{forms_snapshot_with_state, FormsResultsChild, FormsStructureChild, FORMS_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted forms document snapshot (persistent fields of the artifact). Ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`forms→C:value,table`): the inline
/// `steps: Vec<FormStep>` field is replaced by two fixed composed CHILD slots — this plugin no
/// longer defines its own bespoke document tree, it composes stdio's `value`/`table` subsets
/// instead. See `crate::artifacts::forms::🔖️Composition` (`🗿️artifacts/📋️forms/🦀️component.rs`)
/// for the converters/working-scene this slot pair is built and read through. `#[child(...)]`
/// drives `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.forms.forms")]
pub struct FormsSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub version: String,
    #[state(artifact)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.value")]
    pub structure: FormsStructureChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.table")]
    pub results: FormsResultsChild,
}

impl Default for FormsSnapshot {
    fn default() -> Self {
        forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, Vec::new())
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec (mirrors `➗️mathematical`'s/`📐️cad`'s own `enc_child`/
/// `dec_child`) — a handle is exactly two strings (`child_id`, the target's `ArtifactRef`
/// flattened via `to_uri()`), never the child's own content.
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
fn enc_opt_str(s: &Option<String>) -> String {
    match s {
        Some(v) => enc_str(v),
        None => "-".to_string(),
    }
}
fn dec_opt_str(s: &str) -> Result<Option<String>, String> {
    if s == "-" { Ok(None) } else { Ok(Some(dec_str(s)?)) }
}
fn enc_ref(r: &store::os_io::ArtifactRef) -> String {
    enc_str(&r.to_uri())
}
fn dec_ref(s: &str) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&dec_str(s)?)
}
fn enc_child<S>(c: &store::ArtifactChild<S>) -> String {
    format!("[{},{}]", enc_str(&c.child_id), enc_ref(&c.target))
}
fn dec_child<S>(s: &str) -> Result<store::ArtifactChild<S>, String> {
    let inner = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or_else(|| format!("expected [...], got {s:?}"))?;
    let parts: Vec<&str> = inner.splitn(2, ',').collect();
    let [child_id, target] = parts.as_slice() else { return Err(format!("child handle: expected 2 fields, got {}", parts.len())) };
    Ok(store::ArtifactChild::new(dec_str(child_id)?, dec_ref(target)?))
}
//#endregion 🔖️ChildCodecPrimitives

//#region 🔖️TextPrimitives
fn print_forms_snapshot_body(s: &FormsSnapshot) -> String {
    format!("schema={}\nid={}\nversion={}\ntitle={}\nstructure={}\nresults={}", enc_str(&s.schema), enc_str(&s.id), enc_str(&s.version), enc_opt_str(&s.title), enc_child(&s.structure), enc_child(&s.results))
}
fn parse_forms_snapshot_body(body: &str) -> Result<FormsSnapshot, String> {
    let mut schema = None;
    let mut id = None;
    let mut version = None;
    let mut title = None;
    let mut structure = None;
    let mut results = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("id=") {
            id = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("version=") {
            version = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("title=") {
            title = Some(dec_opt_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("structure=") {
            structure = Some(dec_child(rest)?);
        } else if let Some(rest) = line.strip_prefix("results=") {
            results = Some(dec_child(rest)?);
        } else {
            return Err(format!("forms snapshot: unknown line {line:?}"));
        }
    }
    Ok(FormsSnapshot {
        schema: schema.ok_or_else(|| "forms snapshot: missing schema line".to_string())?,
        id: id.ok_or_else(|| "forms snapshot: missing id line".to_string())?,
        version: version.ok_or_else(|| "forms snapshot: missing version line".to_string())?,
        title: title.unwrap_or(None),
        structure: structure.ok_or_else(|| "forms snapshot: missing structure line".to_string())?,
        results: results.ok_or_else(|| "forms snapshot: missing results line".to_string())?,
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
fn write_opt_str_lp(out: &mut Vec<u8>, s: &Option<String>) {
    match s {
        Some(v) => {
            out.push(1);
            write_str_lp(out, v);
        }
        None => out.push(0),
    }
}
fn read_opt_str_lp(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(read_str_lp(reader)?)),
        other => Err(format!("bad option tag {other}")),
    }
}
fn write_ref(out: &mut Vec<u8>, r: &store::os_io::ArtifactRef) {
    write_str_lp(out, &r.to_uri());
}
fn read_ref(reader: &mut store::ByteReader<'_>) -> Result<store::os_io::ArtifactRef, String> {
    store::os_io::ArtifactRef::parse_uri(&read_str_lp(reader)?)
}
fn write_child<S>(out: &mut Vec<u8>, c: &store::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}

fn encode_forms_snapshot_binary(s: &FormsSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &s.schema);
    write_str_lp(&mut out, &s.id);
    write_str_lp(&mut out, &s.version);
    write_opt_str_lp(&mut out, &s.title);
    write_child(&mut out, &s.structure);
    write_child(&mut out, &s.results);
    out
}
fn decode_forms_snapshot_binary(bytes: &[u8]) -> Result<FormsSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    Ok(FormsSnapshot {
        schema: read_str_lp(&mut reader)?,
        id: read_str_lp(&mut reader)?,
        version: read_str_lp(&mut reader)?,
        title: read_opt_str_lp(&mut reader)?,
        structure: read_child(&mut reader)?,
        results: read_child(&mut reader)?,
    })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ Real hex/bracket text + LEB128 binary primitives, hand-rolled directly on `FormsSnapshot` —
/// the previous codec bridged through the shared `flow::playbook::PlaybookSpec` grammar (whose
/// `steps` field mapped 1:1 onto this struct's old bare `steps` field); that bridge cannot express
/// a composed child slot (no `dsl::DslField` impl reachable from this crate for `ArtifactChild<S>`),
/// so this upgrade drops it in favor of the same `enc_child`/`dec_child` pattern `➗️mathematical`/
/// `📐️cad`/`✒️writer` established once their own snapshot gained a real child slot.
impl store::ArtifactDsl for FormsSnapshot {
    const EXTENSION: &'static str = "forms";
    fn envelope_id() -> &'static str {
        FORMS_DOCUMENT_SCHEMA
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_forms_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_forms_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for FormsSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_forms_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
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
        decode_forms_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::forms::{forms_children_from_steps, FormStep};

    #[test]
    fn snapshot_dsl_round_trips_with_composed_children() {
        let steps = vec![FormStep { id: "s1".into(), title: "Step".into(), description: None, blocks: Vec::new() }];
        let (structure, results) = forms_children_from_steps(&steps);
        let snapshot = FormsSnapshot { schema: FORMS_DOCUMENT_SCHEMA.into(), id: "forms".into(), version: "1".into(), title: Some("T".into()), structure, results };
        let printed = store::ArtifactDsl::print_dsl(&snapshot);
        let parsed = <FormsSnapshot as store::ArtifactDsl>::parse_dsl(&printed).expect("parses");
        assert_eq!(parsed, snapshot);
    }

    #[test]
    fn snapshot_pack_round_trips_with_composed_children() {
        let steps = vec![FormStep { id: "s1".into(), title: "Step".into(), description: None, blocks: Vec::new() }];
        let (structure, results) = forms_children_from_steps(&steps);
        let snapshot = FormsSnapshot { schema: FORMS_DOCUMENT_SCHEMA.into(), id: "forms".into(), version: "1".into(), title: None, structure, results };
        let encoded = store::ArtifactPack::encode_pack(&snapshot);
        let decoded = <FormsSnapshot as store::ArtifactPack>::decode_pack(&encoded).expect("decodes");
        assert_eq!(decoded, snapshot);
    }
}
//#endregion 🧪️Tests
