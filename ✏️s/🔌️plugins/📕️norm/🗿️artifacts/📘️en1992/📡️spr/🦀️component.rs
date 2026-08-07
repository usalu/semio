//! ⚖️ EN 1992 design of concrete structures — binary command protocol surface + laws (constitutional: protocol).

use crate::artifacts::en1992::op::Operation;
use protocol::OpBinary;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

/// 📦️ Encodes an `Operation` to its binary command form.
pub fn encode_op(operation: &Operation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes an `Operation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<Operation, protocol::ProtocolError> {
    Operation::decode_op(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1992::Document;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = Operation::SetDocument { document: Document::default() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trips_through_store() {
        let next = Document { m_ed_knm: 200.0, ..Document::default() };
        let envelope = store::create_document_envelope("norm.en1992/v1", "en1992", Document::default(), None);
        let mut doc_store = store::DocumentStore::new(envelope);
        doc_store.dispatch(store::DocumentCommand::Apply { operations: vec![Operation::SetDocument { document: next }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
    }
}
