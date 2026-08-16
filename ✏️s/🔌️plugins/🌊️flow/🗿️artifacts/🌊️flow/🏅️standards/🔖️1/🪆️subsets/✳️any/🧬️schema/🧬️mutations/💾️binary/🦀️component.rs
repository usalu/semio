//! ⚖️ Flow artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! `protocol::OpBinary for FlowMutation` is implemented directly in the flow kernel crate (`flow`);
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


use crate::artifacts::flow::op::FlowMutation;
use protocol::OpBinary;

/// 📦️ Encodes a `FlowMutation` to its binary state-patch form.
pub fn encode_op(operation: &FlowMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖️ Decodes a `FlowMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<FlowMutation, protocol::ProtocolError> {
    FlowMutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::flow::FlowSnapshot;

    fn sample_move_widgets_operation() -> FlowMutation {
        FlowMutation::MoveWidgets(crate::artifacts::flow::schema::mutations::move_widgets::mutation::MoveWidgets {
            entries: vec![flow::FlowLayoutEntry { id: "slider".into(), layout: Some(flow::WidgetLayout { x: 1.0, y: 2.0 }) }],
        })
    }

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let operation = sample_move_widgets_operation();
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[test]
    fn flow_document_text_round_trips_store_with_applied_operation() {
        let envelope = store::create_document_envelope::<FlowSnapshot, FlowMutation>("flow.fixture", "doc-text-test", FlowSnapshot::default(), None);
        let mut doc_store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![sample_move_widgets_operation()], description: None }).expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&doc_store);
        store::os_store::test_support::assert_document_pack_round_trip(&doc_store);
    }
}
//#endregion 🧪️Tests
