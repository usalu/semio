//! ⚖️ Lowpoly artifact — binary command wire codec (constitutional: spr).
//!
//! `protocol::OpBinary for LowpolyMutation` is implemented directly in `../📝️text/🦀️component.rs`
//! (JSON-body encoding, see that file's doc comment). This component only adds the thin
//! artifact-facing `encode_op`/`decode_op` wrappers plus the op text/binary equivalence law and a
//! whole-store round trip.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::lowpoly::schema::mutations::text::LowpolyMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `LowpolyMutation` to its binary command form.
pub async fn encode_op(operation: &LowpolyMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `LowpolyMutation` from its binary command form.
pub async fn decode_op(bytes: &[u8]) -> Result<LowpolyMutation, protocol::ProtocolError> {
    LowpolyMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::lowpoly::schema::default_snapshot;
    use crate::artifacts::lowpoly::mutations::rename_object;
    use crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA;

    #[semio_framework_async_macros::async_test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let projection = default_snapshot();
        let object_id = projection.objects[0].id.clone();
        let operation = LowpolyMutation::RenameObject(rename_object::mutation::RenameObject { id: object_id, new_name: "Renamed".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[semio_framework_async_macros::async_test]
    async fn document_text_round_trip_after_applying_an_operation() {
        let projection = default_snapshot();
        let object_id = projection.objects[0].id.clone();
        let envelope = store::create_document_envelope::<crate::artifacts::lowpoly::LowpolySnapshot, LowpolyMutation>(LOWPOLY_DOCUMENT_SCHEMA, "test-doc", projection, None);
        let mut doc_store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        let operation = LowpolyMutation::RenameObject(rename_object::mutation::RenameObject { id: object_id, new_name: "Renamed Layer".into() });
        doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![operation], description: None }).expect("apply");
        semio_framework_os_kernel::os_store::test_support::assert_document_text_round_trip(&doc_store);
        semio_framework_os_kernel::os_store::test_support::assert_document_pack_round_trip(&doc_store);
    }
}
//#endregion 🧪️Tests
