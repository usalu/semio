//! ⚖️ ISO 16757 app — binary command protocol surface + laws (constitutional: protocol).

use iso16757_op::Operation;
use protocol::OpBinary;

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
    use iso16757::Document;
    use iso16757_op::Iso16757Store;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = Operation::SetDocument { document: Document::reference_fixture() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trips_through_a_vcs_store() {
        let envelope = store::create_document_envelope(iso16757::ISO16757_EXTENSION, "iso16757.demo", Document::reference_fixture(), None);
        let mut store = Iso16757Store::new(envelope);
        let mut mutated = Document::reference_fixture();
        mutated.exchange_process = iso16757::part_5::ExchangeProcess::ProvideCatalogue;
        store.dispatch(store::DocumentCommand::Apply { operations: vec![Operation::SetDocument { document: mutated }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
    }

    #[test]
    fn document_pack_round_trips_through_a_vcs_store() {
        let envelope = store::create_document_envelope(iso16757::ISO16757_EXTENSION, "iso16757.demo", Document::reference_fixture(), None);
        let mut store = Iso16757Store::new(envelope);
        let mut mutated = Document::reference_fixture();
        mutated.exchange_process = iso16757::part_5::ExchangeProcess::ProvideCatalogue;
        store.dispatch(store::DocumentCommand::Apply { operations: vec![Operation::SetDocument { document: mutated }], description: None }).expect("apply");
        store::test_support::assert_document_pack_round_trip(&store);
    }
}
