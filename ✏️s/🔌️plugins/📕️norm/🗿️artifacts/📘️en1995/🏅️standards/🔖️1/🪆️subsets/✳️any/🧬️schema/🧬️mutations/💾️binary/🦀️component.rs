//! ⚖️ En1995 app — binary command protocol surface + laws (constitutional: protocol).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::en1995::schema::mutations::text::En1995Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub async fn encode_op(mutation: &En1995Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub async fn decode_op(bytes: &[u8]) -> Result<En1995Mutation, protocol::ProtocolError> {
    En1995Mutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::en1995::mutations::set_snapshot;
    use crate::artifacts::en1995::En1995Snapshot;

    async fn sample_mutation() -> En1995Mutation {
        En1995Mutation::ChangeAnnex(set_snapshot::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En })
    }

    #[test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let mutation = sample_mutation();
        store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
        let bytes = encode_op(&mutation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), mutation);
    }

    #[test]
    async fn document_text_round_trips_through_store() {
        let envelope = store::create_document_envelope("norm.en1995/v1", "en1995", En1995Snapshot::default(), None);
        let mut store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![sample_mutation()], description: None }).expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&store);
        store::os_store::test_support::assert_document_pack_round_trip(&store);
    }
}
//#endregion 🧪️Tests
