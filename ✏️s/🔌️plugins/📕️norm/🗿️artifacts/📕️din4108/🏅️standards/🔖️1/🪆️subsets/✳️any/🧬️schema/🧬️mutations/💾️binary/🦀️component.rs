//! ⚖️ DIN 4108 app — binary command protocol surface + laws (constitutional: protocol).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::din4108::schema::mutations::text::Din4108Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub fn encode_op(mutation: &Din4108Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<Din4108Mutation, protocol::ProtocolError> {
    Din4108Mutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::din4108::Din4108Snapshot;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let mutation = Din4108Mutation::SetSnapshot { snapshot: Din4108Snapshot::default() };
        store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
        let bytes = encode_op(&mutation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), mutation);
    }

    #[test]
    fn document_text_round_trips_through_store() {
        let envelope = store::create_document_envelope("norm.din4108/v1", "din4108", Din4108Snapshot::default(), None);
        let mut store = store::ArtifactStore::new(envelope);
        let next = Din4108Snapshot { airtightness_n50: 1.2, ..Din4108Snapshot::default() };
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![Din4108Mutation::SetSnapshot { snapshot: next }], description: None }).expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
