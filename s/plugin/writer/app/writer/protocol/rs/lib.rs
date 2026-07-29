//! ⚖️ Writer app — binary command protocol surface + laws (constitutional: protocol).

use protocol::OpBinary;
use writer_op::WriterOperation;

/// 📦 Encodes a `WriterOperation` to its binary command form.
pub fn encode_op(operation: &WriterOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes a `WriterOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<WriterOperation, protocol::ProtocolError> {
    WriterOperation::decode_op(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use store::{create_document_envelope, DocumentCommand, DocumentStore};
    use writer::{WriterCamera, WriterProjection};

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = WriterOperation::SetText { text: "hello".into() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    /// ✍️ Hand-built representative document — verbatim from the original file's `🔖DslAndOpText`
    /// test region (duplicated per-crate since each constitutional crate's tests compile independently).
    fn jack_projection() -> WriterProjection {
        WriterProjection {
            schema: "writer.document".into(),
            id: "jack".into(),
            language_id: "jack".into(),
            uri: "writer://jack".into(),
            text: "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name".into(),
            camera: WriterCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        }
    }

    #[test]
    fn writer_document_text_round_trips_through_the_store() {
        let mut store = DocumentStore::<WriterProjection, WriterOperation>::new(create_document_envelope(
            "writer.document",
            "writer",
            writer_engine::empty_writer_projection(),
            None,
        ));
        store
            .dispatch(DocumentCommand::Apply { operations: vec![WriterOperation::SetDocument { document: jack_projection() }], description: None })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪Tests
