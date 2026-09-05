//! ⚖️ Flow artifact — state-patch-representation wire codec + laws (was: constitutional `protocol`).
//!
//! `protocol::OpBinary for FlowMutation` is implemented directly in the flow kernel crate (`flow`);
//! see `🗿️artifacts/🌊️flow/🦀️.rs` for why. This component only adds the thin artifact-facing
//! `encode_op`/`decode_op` wrappers plus the op text↔binary equivalence law and a whole-store round trip.
//!
//! The app's typed `FlowCommand` enum — which used to share the old `📡️protocol` crate with this codec —
//! is an APP concern, not an artifact one: it now lives in `🎛️apps/🌊️flow/🦀️.rs`, assembled from
//! the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
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
    use protocol::Identified;

    fn sample_move_widgets_operation() -> FlowMutation {
        FlowMutation::MoveWidgets(crate::artifacts::flow::schema::mutations::move_widgets::MoveWidgets { entries: vec![flow::FlowLayoutEntry { id: "slider".into(), layout: Some(flow::WidgetLayout { x: 1.0, y: 2.0 }) }] })
    }

    #[semio_framework_async_macros::async_test]
    async fn op_binary_round_trips_and_agrees_with_text() {
        let operation = sample_move_widgets_operation();
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        let bytes = encode_op(&operation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), operation);
    }

    #[semio_framework_async_macros::async_test]
    async fn flow_document_text_round_trips_store_with_applied_operation() {
        let envelope = store::create_document_envelope::<FlowSnapshot, FlowMutation>("flow.fixture", "doc-text-test", FlowSnapshot::default(), None);
        let mut doc_store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
        doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![sample_move_widgets_operation()], description: None }).await.expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&doc_store).await;
        store::os_store::test_support::assert_document_pack_round_trip(&doc_store).await;
    }

    /// 🌉️ The composite pilot's own op text/binary law, plus proof it survives the REAL
    /// `ArtifactStore::dispatch` path — `replay_mutations` calls `Op::encode_op()` on every applied
    /// mutation before it even reaches history, so a composite whose codec only worked in isolation
    /// would still fail here.
    #[semio_framework_async_macros::async_test]
    async fn duplicate_widget_composite_round_trips_through_op_codecs_and_a_real_store_dispatch() {
        let widget = flow::Widget::InputNote { id: "note-1".into(), text: "hello".into() };
        let create = FlowMutation::CreateWidget(crate::artifacts::flow::schema::mutations::create_widget::CreateWidget { index: 0, widget });
        let duplicate = FlowMutation::DuplicateWidget(crate::artifacts::flow::schema::mutations::duplicate_widget::mutation::DuplicateWidget {
            source_id: "note-1".into(),
            new_id: "note-2".into(),
            synapse_id: "note-1-to-note-2".into(),
            from_port: "out".into(),
            to_port: "in".into(),
        });
        store::os_store::test_support::assert_op_text_binary_equivalence(&duplicate);
        let bytes = encode_op(&duplicate).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), duplicate);

        let envelope = store::create_document_envelope::<FlowSnapshot, FlowMutation>("flow.fixture", "doc-composite-test", FlowSnapshot::default(), None);
        let mut doc_store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
        doc_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![create, duplicate], description: None }).await.expect("apply composite through the real store");
        let live = doc_store.snapshot().expect("snapshot").to_fixture();
        assert!(live.widgets.iter().any(|widget| widget.id() == "note-2"));
        assert!(live.synapses.iter().any(|synapse| synapse.id == "note-1-to-note-2"));
        store::os_store::test_support::assert_document_text_round_trip(&doc_store).await;
        store::os_store::test_support::assert_document_pack_round_trip(&doc_store).await;
    }
}
//#endregion 🧪️Tests
