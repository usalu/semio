//! ⚖️ VDI 3805 app — binary command protocol surface + laws (constitutional: protocol).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use protocol::OpBinary;
use crate::artifacts::vdi3805::op::Operation;

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
    use crate::artifacts::vdi3805::op::Vdi3805Store;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = Operation::SetDocument { document: crate::artifacts::vdi3805::reference_fixture() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn document_text_round_trips_through_a_vcs_store() {
        let envelope = store::create_document_envelope(crate::artifacts::vdi3805::VDI3805_EXTENSION, "vdi3805.demo", crate::artifacts::vdi3805::reference_fixture(), None);
        let mut store = Vdi3805Store::new(envelope);
        let mut mutated = crate::artifacts::vdi3805::reference_fixture();
        mutated.strict_mode = true;
        store.dispatch(store::DocumentCommand::Apply { operations: vec![Operation::SetDocument { document: mutated }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&store);
    }

    #[test]
    fn document_pack_round_trips_through_a_vcs_store() {
        let envelope = store::create_document_envelope(crate::artifacts::vdi3805::VDI3805_EXTENSION, "vdi3805.demo", crate::artifacts::vdi3805::reference_fixture(), None);
        let mut store = Vdi3805Store::new(envelope);
        let mut mutated = crate::artifacts::vdi3805::reference_fixture();
        mutated.strict_mode = true;
        store.dispatch(store::DocumentCommand::Apply { operations: vec![Operation::SetDocument { document: mutated }], description: None }).expect("apply");
        store::test_support::assert_document_pack_round_trip(&store);
    }
}
