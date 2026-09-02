//! ⚖️ DIN EN 16798 app — binary command protocol surface + laws (constitutional: protocol).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::din16798::schema::mutations::text::Din16798Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub fn encode_op(mutation: &Din16798Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<Din16798Mutation, protocol::ProtocolError> {
    Din16798Mutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::din16798::mutations::change_t_op_c;
    use crate::artifacts::din16798::Din16798Snapshot;

    fn sample_mutation() -> Din16798Mutation {
        Din16798Mutation::ChangeTOpC(change_t_op_c::ChangeTOpC { new_t_op_c: 23.0 })
    }

    #[semio_framework_async_macros::async_test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let mutation = sample_mutation();
        store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
        let bytes = encode_op(&mutation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), mutation);
    }

    #[semio_framework_async_macros::async_test]
    fn document_text_round_trips_through_store() {
        let envelope = store::create_document_envelope("norm.din16798/v1", "din16798", Din16798Snapshot::default(), None);
        let mut store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![sample_mutation()], description: None }).expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
