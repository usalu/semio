//! 📦️ Writer artifact — binary document surface + laws (constitutional: pack).

use crate::artifacts::writer::WriterProjection;
use store::PackError;

/// 📦️ Encodes a `WriterProjection` to its binary pack form.
pub fn encode(projection: &WriterProjection) -> Vec<u8> {
    store::DocumentPack::encode_pack(projection)
}

/// 📖️ Decodes a `WriterProjection` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<WriterProjection, PackError> {
    <WriterProjection as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::writer::engine;

    /// ✍️ Hand-built representative document — used across the artifact's own component tests.
    fn jack_projection() -> WriterProjection {
        WriterProjection { schema: "writer.document".into(), id: "jack".into(), language_id: "jack".into(), uri: "writer://jack".into(), text: "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name".into() }
    }

    #[test]
    fn writer_projection_dsl_pack_equivalence() {
        let empty = engine::empty_writer_projection();
        store::test_support::assert_dsl_pack_equivalence(&empty);
        let bytes = encode(&empty);
        assert_eq!(decode(&bytes).expect("decode"), empty);

        let jack = jack_projection();
        store::test_support::assert_dsl_pack_equivalence(&jack);
        let bytes = encode(&jack);
        assert_eq!(decode(&bytes).expect("decode"), jack);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `WriterOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this file's
    /// existing dsl/pack round-trip law.
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::writer::op::WriterOperation;
        use protocol::{DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand, DocumentStore};

        let mut store: DocumentStore<WriterProjection, WriterOperation> = DocumentStore::new(create_document_envelope("writer.document", "writer", engine::empty_writer_projection(), None));
        store.dispatch(DocumentCommand::Apply { operations: vec![WriterOperation::SetText { text: "hello".into() }], description: None }).expect("apply");
        let edit: &Edit<WriterOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<WriterProjection, WriterOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
