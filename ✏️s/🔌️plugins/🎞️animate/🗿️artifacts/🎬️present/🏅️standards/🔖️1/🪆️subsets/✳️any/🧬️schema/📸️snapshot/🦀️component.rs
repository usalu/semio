//! 🧬️ Present snapshot schema — persistent fields only.
//!
//! P6 handcrafted `ArtifactDsl`/`ArtifactPack` (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`,
//! `animate→C:presentation,animation`): `PresentSnapshot` now carries two owned composed-child
//! handles (`presentation`/`animation`) instead of the old inline `source: FigureTileSource` +
//! `tiles: Vec<FigureTileDraft>` fields — `store::ArtifactChild<S>` has no `DslField` impl, so the
//! old `dsl::DslRecord`-derived mirror (`PresentSnapshotDsl`) is gone; this file hand-rolls the
//! codec directly, following writer's/lowpoly's exact hex/bracket (text) + LEB128-length-prefixed
//! (binary) child-handle convention.

use crate::artifacts::present::{AnimationChild, PresentationChild, PRESENT_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted present document snapshot — a composed `presentation` deck (shared source figure +
/// named tile crops, see `crate::artifacts::present::presentation_snapshot_from_source_tiles`) plus a
/// composed `animation` set (currently always empty — see `crate::artifacts::present::animation_child_handle`'s
/// doc comment for the honest gap). Both slots are bare (never absent) — this artifact always
/// composes exactly one of each, matching writer's `document: WriterDocumentChild` single-`Option`-in-
/// the-diff convention rather than lowpoly's optional-slot double-`Option` shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.animate.present")]
pub struct PresentSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.presentation")]
    pub presentation: PresentationChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.animation")]
    pub animation: AnimationChild,
}

impl Default for PresentSnapshot {
    fn default() -> Self {
        default_snapshot()
    }
}

/// 🌱 Canonical default document used by the play app and examples.
pub fn default_snapshot() -> PresentSnapshot {
    crate::artifacts::present::present_snapshot_with_tiles(&crate::artifacts::present::default_figure_tile_source(), &[])
}
//#endregion 🔖️Snapshot

//#region 🔖️ChildCodecPrimitives
/// 🧪️ Real hex/bracket child-handle codec (mirrors writer's/cad's own `enc_child`/`dec_child`) — a
/// handle is exactly two strings (`child_id`, the target's `ArtifactRef` flattened via `to_uri()`),
/// never the child's own content.
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
fn print_present_snapshot_body(s: &PresentSnapshot) -> String {
    format!("schema={}\npresentation={}\nanimation={}", enc_str(&s.schema), enc_child(&s.presentation), enc_child(&s.animation))
}
fn parse_present_snapshot_body(body: &str) -> Result<PresentSnapshot, String> {
    let mut schema = None;
    let mut presentation = None;
    let mut animation = None;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("presentation=") {
            presentation = Some(dec_child(rest)?);
        } else if let Some(rest) = line.strip_prefix("animation=") {
            animation = Some(dec_child(rest)?);
        } else {
            return Err(format!("present snapshot: unknown line {line:?}"));
        }
    }
    Ok(PresentSnapshot {
        schema: schema.ok_or_else(|| "present snapshot: missing schema line".to_string())?,
        presentation: presentation.ok_or_else(|| "present snapshot: missing presentation line".to_string())?,
        animation: animation.ok_or_else(|| "present snapshot: missing animation line".to_string())?,
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
fn write_child<S>(out: &mut Vec<u8>, c: &store::ArtifactChild<S>) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
fn read_child<S>(reader: &mut store::ByteReader<'_>) -> Result<store::ArtifactChild<S>, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}

fn encode_present_snapshot_binary(s: &PresentSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &s.schema);
    write_child(&mut out, &s.presentation);
    write_child(&mut out, &s.animation);
    out
}
fn decode_present_snapshot_binary(bytes: &[u8]) -> Result<PresentSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let presentation = read_child(&mut reader)?;
    let animation = read_child(&mut reader)?;
    Ok(PresentSnapshot { schema, presentation, animation })
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for PresentSnapshot {
    const EXTENSION: &'static str = "present";
    fn envelope_id() -> &'static str {
        PRESENT_DOCUMENT_SCHEMA
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_present_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = print_present_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PresentSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_present_snapshot_binary(self);
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
        decode_present_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips() {
        let snap = PresentSnapshot::default();
        let bytes = <PresentSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <PresentSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = PresentSnapshot::default();
        let text = <PresentSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <PresentSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    #[test]
    fn populated_snapshot_pack_and_dsl_round_trip() {
        let source = crate::artifacts::present::default_figure_tile_source();
        let tiles = vec![crate::artifacts::present::FigureTileDraft {
            id: "t1".into(),
            name: "Tile One".into(),
            crop: crate::artifacts::present::FigureTileFrame { x: 0.1, y: 0.1, width: 0.2, height: 0.2 },
        }];
        let snap = crate::artifacts::present::present_snapshot_with_tiles(&source, &tiles);
        let bytes = <PresentSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <PresentSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);

        let text = <PresentSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back_text = <PresentSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back_text);
    }
}
//#endregion 🧪️Tests
