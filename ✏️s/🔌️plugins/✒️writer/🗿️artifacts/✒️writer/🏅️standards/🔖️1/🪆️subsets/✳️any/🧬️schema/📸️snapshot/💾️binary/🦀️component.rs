//! 📦️ Writer artifact — binary document surface + laws (constitutional: pack).

use crate::artifacts::writer::WriterSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


/// 📦️ Encodes a `WriterSnapshot` to its binary pack form.
pub fn encode(projection: &WriterSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(projection)
}

/// 📖️ Decodes a `WriterSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<WriterSnapshot, PackError> {
    <WriterSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::writer::engine;

    /// ✍️ Hand-built representative document — used across the artifact's own component tests.
    fn jack_snapshot() -> WriterSnapshot {
        WriterSnapshot { schema: "writer.document".into(), id: "jack".into(), language_id: "jack".into(), uri: "writer://jack".into(), text: "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name".into() }
    }

    #[test]
    fn writer_projection_dsl_pack_equivalence() {
        let empty = engine::empty_writer_snapshot();
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
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::writer::op::WriterMutation;
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

        let mut store: ArtifactStore<WriterSnapshot, WriterMutation> = ArtifactStore::new(create_document_envelope("writer.document", "writer", engine::empty_writer_snapshot(), None));
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

    #[test]
    fn component_protocol_semio_is_protocol_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Protocol);
        assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
        let _ = COMPONENT_PROTOCOL_PATH;
    }

    #[test]
    fn verify_protocol_bytes_against_encoded_pack() {
        let document = crate::artifacts::writer::engine::empty_writer_snapshot();
        let bytes = encode(&document);
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes pack bytes");
    }
}

