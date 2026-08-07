//! ⚖️ Flow artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! `protocol::OpBinary for FlowOperation` is implemented directly in the flow kernel crate (`flow`);
//! see `🗿️artifacts/🌊️flow/🦀️component.rs` for why. This component only adds the thin artifact-facing
//! `encode_op`/`decode_op` wrappers plus the op text↔binary equivalence law and a whole-store round trip.
//!
//! The app's typed `FlowCommand` enum — which used to share the old `📡️protocol` crate with this codec —
//! is an APP concern, not an artifact one: it now lives in `🎛️apps/🌊️flow/🦀️component.rs`, assembled from
//! the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::flow::op::FlowOperation;
use protocol::OpBinary;

/// 📦️ Encodes a `FlowOperation` to its binary state-patch form.
pub fn encode_op(operation: &FlowOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `FlowOperation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<FlowOperation, protocol::ProtocolError> {
    FlowOperation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::flow::FlowFixture;

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = FlowOperation::SetLayout { entries: Vec::new() };
        store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn flow_document_text_round_trips_store_with_applied_operation() {
        let envelope = store::create_document_envelope::<FlowFixture, FlowOperation>("flow.fixture", "doc-text-test", FlowFixture::default(), None);
        let mut doc_store = store::DocumentStore::new(envelope);
        doc_store.dispatch(store::DocumentCommand::Apply { operations: vec![FlowOperation::SetLayout { entries: Vec::new() }], description: None }).expect("apply");
        store::test_support::assert_document_text_round_trip(&doc_store);
        store::test_support::assert_document_pack_round_trip(&doc_store);
    }
}
//#endregion 🧪️Tests
