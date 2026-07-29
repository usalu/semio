//! ⚖️ Note app — binary command protocol surface + laws (constitutional: protocol).

use note_op::NoteOperation;
use protocol::OpBinary;

/// 📦 Encodes a `NoteOperation` to its binary command form.
pub fn encode_op(operation: &NoteOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes a `NoteOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<NoteOperation, protocol::ProtocolError> {
    NoteOperation::decode_op(bytes)
}

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let document = note_engine::empty_note_document();
        let operation = NoteOperation::SetCamera { camera: document.camera };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn note_document_text_round_trips_store_with_applied_operation() {
        use note::NoteDocument;

        let envelope = store::create_document_envelope::<NoteDocument, NoteOperation>(
            "note.document",
            "doc-text-test",
            note_engine::empty_note_document(),
            None,
        );
        let mut doc_store = store::DocumentStore::new(envelope);
        doc_store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![NoteOperation::SetGridSpacing { spacing: Some(48.0) }],
                description: None,
            })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
    }
}
//#endregion 🧪Tests
