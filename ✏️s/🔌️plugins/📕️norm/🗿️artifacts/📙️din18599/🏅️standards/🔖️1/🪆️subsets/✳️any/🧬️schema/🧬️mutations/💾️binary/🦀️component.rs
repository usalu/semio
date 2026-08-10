//! ⚖️ DIN V 18599 app — binary command protocol surface + laws (constitutional: protocol).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::din18599::schema::mutations::text::Din18599Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub fn encode_op(mutation: &Din18599Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<Din18599Mutation, protocol::ProtocolError> {
    Din18599Mutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::din18599::Din18599Snapshot;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let mutation = Din18599Mutation::SetSnapshot { snapshot: Din18599Snapshot::default() };
        store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
        let bytes = encode_op(&mutation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), mutation);
    }

    #[test]
    fn document_text_round_trips_through_store() {
        let envelope = store::create_document_envelope("norm.din18599/v1", "din18599", Din18599Snapshot::default(), None);
        let mut store = store::ArtifactStore::new(envelope);
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![Din18599Mutation::SetSnapshot { snapshot: Din18599Snapshot::default() }], description: None }).expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
