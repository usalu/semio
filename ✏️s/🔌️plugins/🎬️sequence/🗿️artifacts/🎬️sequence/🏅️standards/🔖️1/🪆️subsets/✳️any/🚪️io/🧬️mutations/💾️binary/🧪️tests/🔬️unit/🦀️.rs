use super::*;
use crate::schema::mutations::{connect_steps, create_step, delete_step};
use crate::{default_snapshot, SequenceSnapshot, SequenceStep, StepParams};
use neural_engine::{Atom, Value};

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let mutation = move_step_for_test();
    store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
    let bytes = encode_op(&mutation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), mutation);
}

fn move_step_for_test() -> SequenceMutation {
    crate::schema::mutations::move_step("step-1".into(), 42.0, -6.5)
}

/// 🧪️ Whole-store round trip: applies a mutation through a real `SequenceStore`, then proves
/// the resulting envelope survives both the text and binary document-level protocols.
#[semio_framework_async_macros::async_test]
async fn sequence_document_text_round_trips_store_with_applied_mutation() {
    let envelope = store::create_document_envelope::<SequenceSnapshot, SequenceMutation>(crate::SEQUENCE_DOCUMENT_SCHEMA, "sequence-text-test", default_snapshot(), None);
    let mut doc_store = store::ArtifactStore::new(envelope).await.expect("valid artifact store fixture");
    doc_store
        .dispatch(store::ArtifactCommand::Apply { mutations: vec![create_step(SequenceStep { id: "step-7".into(), kind: "log.print".into(), params: StepParams::new(), x: 12.0, y: 24.0, slot: None, collapsed: false })], description: None })
        .await
        .expect("apply");
    store::os_store::test_support::assert_document_text_round_trip(&doc_store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&doc_store).await;
}

//#region 🔖️OpTextTests
#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_create_step() {
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

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_delete_step() {
    store::os_store::test_support::assert_op_line_round_trip(&delete_step("step-99".into()));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_move_step() {
    store::os_store::test_support::assert_op_line_round_trip(&move_step_for_test());
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_connect_steps() {
    store::os_store::test_support::assert_op_line_round_trip(&connect_steps("edge-2".into(), "step-2".into(), "step-3".into()));
}
//#endregion 🔖️OpTextTests
