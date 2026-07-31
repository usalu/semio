//! ⚖️ EN 1997 app — binary command protocol surface + laws (constitutional: protocol).

use en1997_op::Operation;
use protocol::OpBinary;

/// 📦️ Encodes an `Operation` to its binary command form.
pub fn encode_op(operation: &Operation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes an `Operation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Operation, protocol::ProtocolError> {
    Operation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use en1997::Document;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = Operation::SetDocument { document: Document::default() };
        store::test_support::assert_op_line_round_trip(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trips_through_store() {
        let envelope = store::create_document_envelope("norm.en1997/v1", "en1997", Document::default(), None);
        let mut store = store::DocumentStore::new(envelope);
        store
            .dispatch(store::DocumentCommand::Apply {
                operations: vec![Operation::SetDocument { document: Document::default() }],
                description: None,
            })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
