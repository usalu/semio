//! ⚖️ Imperative app — binary command protocol surface + laws (constitutional: protocol).

use imperative_op::ImperativeOperation;
use protocol::OpBinary;

/// 📦️ Encodes an `ImperativeOperation` to its binary command form.
pub fn encode_op(operation: &ImperativeOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes an `ImperativeOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<ImperativeOperation, protocol::ProtocolError> {
    ImperativeOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use imperative::PathRef;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Remove { id: "step-1".into() } };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trip_with_applied_operation() {
        use imperative::{Dictionary, ImperativeDocument, Step};
        use std::collections::BTreeMap;

        let document = imperative_engine::default_document();
        let envelope = store::create_document_envelope::<ImperativeDocument, ImperativeOperation>("imperative.document/v1", "test", document, None);
        let mut store = store::DocumentStore::new(envelope);
        let step = Step { id: "step-x".into(), kind: "log.print".into(), params: Dictionary::new(), bodies: BTreeMap::new() };
        let operation = ImperativeOperation { path_ref: PathRef::default(), collection: protocol::CollectionOperation::Add { id: "step-x".to_string(), item: step, at: 0 } };
        store
            .dispatch(store::DocumentCommand::Apply { operations: vec![operation], description: None })
            .expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
        store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
