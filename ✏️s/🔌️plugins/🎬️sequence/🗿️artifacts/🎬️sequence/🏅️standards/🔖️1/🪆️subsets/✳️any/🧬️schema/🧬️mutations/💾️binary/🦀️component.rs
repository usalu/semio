//! ⚖️ Sequence artifact — state-patch-representation wire codec (constitutional: protocol). The
//! `OpText`/`OpBinary` impls for `SequenceMutation` are handcrafted in the sibling `📝️text` facet
//! (P6: derive no longer emits these traits); this facet only carries the normative binary
//! protocol doc-string and the encode/decode free-function wrappers + their round-trip tests.
//!
//! The app's typed `SequenceCommand` enum — which used to share the old `📡️protocol` crate with this
//! codec — is an APP concern, not an artifact one: it lives in `🎛️apps/🎬️sequence/🦀️component.rs`,
//! assembled from the `🎮️commands/*` payload modules by `semio_framework_plugin::app_commands!`.


//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::sequence::schema::mutations::SequenceMutation;
use protocol::OpBinary;

//#region 🔖️OpText
/// 📦️ Encodes a `SequenceMutation` to its binary state-patch form.
pub fn encode_op(mutation: &SequenceMutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a `SequenceMutation` from its binary state-patch form.
pub fn decode_op(bytes: &[u8]) -> Result<SequenceMutation, protocol::ProtocolError> {
    SequenceMutation::decode_op(bytes)
}
//#endregion 🔖️OpText

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::sequence::schema::mutations::{connect_steps, create_step, delete_step};
    use crate::artifacts::sequence::{default_snapshot, SequenceSnapshot, SequenceStep, StepParams};
    use neural_engine::{Atom, Value};

    #[test]
    fn op_binary_round_trips_and_agrees_with_text() {
        let mutation = move_step_for_test();
        store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
        let bytes = encode_op(&mutation).expect("encode");
        assert_eq!(decode_op(&bytes).expect("decode"), mutation);
    }

    fn move_step_for_test() -> SequenceMutation {
        crate::artifacts::sequence::schema::mutations::move_step("step-1".into(), 42.0, -6.5)
    }

    /// 🧪️ Whole-store round trip: applies a mutation through a real `SequenceStore`, then proves
    /// the resulting envelope survives both the text and binary document-level protocols.
    #[test]
    fn sequence_document_text_round_trips_store_with_applied_mutation() {
        let envelope = store::create_document_envelope::<SequenceSnapshot, SequenceMutation>(crate::artifacts::sequence::SEQUENCE_DOCUMENT_SCHEMA, "sequence-text-test", default_snapshot(), None);
        let mut doc_store = store::ArtifactStore::new(envelope).expect("valid artifact store fixture");
        doc_store
            .dispatch(store::ArtifactCommand::Apply {
                mutations: vec![create_step(SequenceStep { id: "step-7".into(), kind: "log.print".into(), params: StepParams::new(), x: 12.0, y: 24.0, slot: None, collapsed: false })],
                description: None,
            })
            .expect("apply");
        store::os_store::test_support::assert_document_text_round_trip(&doc_store);
        store::os_store::test_support::assert_document_pack_round_trip(&doc_store);
    }

    //#region 🔖️OpTextTests
    #[test]
    fn op_text_round_trips_create_step() {
        store::os_store::test_support::assert_op_line_round_trip(&create_step(SequenceStep {
            id: "step-99".into(),
            kind: "log.print".into(),
            params: StepParams::new().insert("message", Value::Atom(Atom::String("hi there".into()))),
            x: 5.0,
            y: -6.5,
            slot: None,
            collapsed: false,
        }));
    }

    #[test]
    fn op_text_round_trips_delete_step() {
        store::os_store::test_support::assert_op_line_round_trip(&delete_step("step-99".into()));
    }

    #[test]
    fn op_text_round_trips_move_step() {
        store::os_store::test_support::assert_op_line_round_trip(&move_step_for_test());
    }

    #[test]
    fn op_text_round_trips_connect_steps() {
        store::os_store::test_support::assert_op_line_round_trip(&connect_steps("edge-2".into(), "step-2".into(), "step-3".into()));
    }
    //#endregion 🔖️OpTextTests
}
//#endregion 🧪️Tests
