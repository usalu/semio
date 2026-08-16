//! ⚖️ En1999 app — binary command protocol surface + laws (constitutional: protocol).


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::en1999::schema::mutations::text::En1999Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub fn encode_op(mutation: &En1999Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<En1999Mutation, protocol::ProtocolError> {
    En1999Mutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1999::mutations::change_n_ed_kn;
    use crate::artifacts::en1999::En1999Snapshot;

    fn sample_mutation() -> En1999Mutation {
        En1999Mutation::ChangeNEdKn(change_n_ed_kn::mutation::ChangeNEdKn { new_n_ed_kn: 95.0 })
    }

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let mutation = sample_mutation();
        store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
        let bytes = encode_op(&mutation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), mutation);
    }

    #[test]
    fn document_text_round_trips_through_store() {
        let envelope = store::create_document_envelope("norm.en1999/v1", "en1999", En1999Snapshot::default(), None);
        let mut store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![sample_mutation()], description: None }).expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
