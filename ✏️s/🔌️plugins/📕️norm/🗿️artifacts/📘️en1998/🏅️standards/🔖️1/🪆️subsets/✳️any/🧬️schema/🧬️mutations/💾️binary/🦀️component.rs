//! ⚖️ EN 1998 app — binary command protocol surface + laws (constitutional: protocol).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::en1998::schema::mutations::text::En1998Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub fn encode_op(mutation: &En1998Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<En1998Mutation, protocol::ProtocolError> {
    En1998Mutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1998::En1998Snapshot;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let mutation = En1998Mutation::SetSnapshot { snapshot: En1998Snapshot::default() };
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
        let bytes = encode_op(&mutation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), mutation);
    }

    #[test]
    fn document_text_round_trips_through_store() {
        let envelope = store::create_document_envelope("norm.en1998/v1", "en1998", En1998Snapshot::default(), None);
        let mut store = store::ArtifactStore::new(envelope);
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![En1998Mutation::SetSnapshot { snapshot: En1998Snapshot::default() }], description: None }).expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
