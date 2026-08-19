//! 📦️ Writer artifact — binary document surface + laws (constitutional: pack). Owns the REAL
//! `store::ArtifactPack` impl for `WriterSnapshot` (design.md §1 CORRECTION: the native codec is
//! one bidirectional thing, unsplit, so it lives here rather than mirrored under import/export).

use crate::artifacts::writer::{WriterDocumentChild, WriterSnapshot};
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

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
async fn write_child(out: &mut Vec<u8>, c: &WriterDocumentChild) {
    write_str_lp(out, &c.child_id);
    write_ref(out, &c.target);
}
async fn read_child(reader: &mut store::ByteReader<'_>) -> Result<WriterDocumentChild, String> {
    let child_id = read_str_lp(reader)?;
    let target = read_ref(reader)?;
    Ok(store::ArtifactChild::new(child_id, target))
}

async fn encode_writer_snapshot_binary(s: &WriterSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = vec![PACK_BINARY_FORMAT];
    write_str_lp(&mut out, &s.schema);
    write_str_lp(&mut out, &s.id);
    write_str_lp(&mut out, &s.language_id);
    write_str_lp(&mut out, &s.uri);
    write_child(&mut out, &s.document);
    out
}
async fn decode_writer_snapshot_binary(bytes: &[u8]) -> Result<WriterSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT { return Err(format!("unsupported pack format {format}")); }
    let mut snapshot = WriterSnapshot::default();
    snapshot.schema = read_str_lp(&mut reader)?;
    snapshot.id = read_str_lp(&mut reader)?;
    snapshot.language_id = read_str_lp(&mut reader)?;
    snapshot.uri = read_str_lp(&mut reader)?;
    snapshot.document = read_child(&mut reader)?;
    Ok(snapshot)
}
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactPack
impl store::ArtifactPack for WriterSnapshot {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let _ = options;
        let raw = encode_writer_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        decode_writer_snapshot_binary(&inner).map_err(PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactPack

/// 📦️ Encodes a `WriterSnapshot` to its binary pack form.
pub async fn encode(projection: &WriterSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(projection)
}

/// 📖️ Decodes a `WriterSnapshot` from its binary pack form.
pub async fn decode(bytes: &[u8]) -> Result<WriterSnapshot, PackError> {
    <WriterSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::writer::schema;

    /// ✍️ Hand-built representative document — used across the artifact's own component tests.
    async fn jack_snapshot() -> WriterSnapshot {
        crate::artifacts::writer::writer_snapshot_with_text(
            "writer.document",
            "jack",
            "jack",
            "writer://jack",
            "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name",
        )
    }

    #[semio_framework_async_macros::async_test]
    async fn writer_projection_dsl_pack_equivalence() {
        let empty = schema::empty_writer_snapshot();
        store::os_store::test_support::assert_dsl_pack_equivalence(&empty);
        let bytes = encode(&empty);
        assert_eq!(decode(&bytes).expect("decode"), empty);

        let jack = jack_snapshot();
        store::os_store::test_support::assert_dsl_pack_equivalence(&jack);
        let bytes = encode(&jack);
        assert_eq!(decode(&bytes).expect("decode"), jack);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `WriterMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
    /// existing dsl/pack round-trip law.
    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::writer::op::WriterMutation;
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

        let mut store: ArtifactStore<WriterSnapshot, WriterMutation> = ArtifactStore::new(create_document_envelope("writer.document", "writer", schema::empty_writer_snapshot(), None)).expect("valid artifact store fixture");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![WriterMutation::EditText(crate::artifacts::writer::schema::mutations::EditText { text: "hello".into() })], description: None }).expect("apply");
        let edit: &Edit<WriterMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<WriterSnapshot, WriterMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semio_protocol_conformance {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn component_protocol_semio_is_protocol_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Protocol);
        assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
        let _ = COMPONENT_PROTOCOL_PATH;
    }

    #[semio_framework_async_macros::async_test]
    async fn verify_protocol_bytes_against_encoded_pack() {
        let document = crate::artifacts::writer::schema::empty_writer_snapshot();
        let bytes = encode(&document);
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes pack bytes");
    }
}
