use super::*;
use crate::editor::sequence::unit_tests::context::{dispatch, new_app_with_registry_wired, SequenceApp};
use crate::editor::sequence::SequenceCommand;
use crate::schema::mutations::{connect_steps, create_step, delete_step};
use crate::{SequenceStep, StepParams};
use neural_engine::{Atom, Value};
use semio_framework_plugin::PluginApp;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
use store::{ArtifactPack, SpaceMember};

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

/// 🧸️ The live `content` member's own document, decoded off the child store — the only surface that
/// holds this artifact's steps and edges (the parent envelope carries the opaque handle alone).
async fn content_snapshot(app: &SequenceApp) -> SemioFlowSnapshot {
    let parent = app.snapshot().expect("Sequence parent snapshot");
    let bytes = app.child_store("content", &parent.content.child_id).await.expect("Sequence content child").document_pack_bytes().await.expect("Sequence content child pack");
    SemioFlowSnapshot::decode_pack(&bytes).expect("Sequence content child snapshot")
}

async fn drive_document_archive_load(app: &mut SequenceApp, operation: u64) -> protocol::DocumentArchiveLoadStatus {
    for _ in 0..1_000_000 {
        let status = Box::pin(PluginApp::poll_document_archive_load(&mut app.0, operation)).await.expect("Sequence document archive status");
        if matches!(status.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
            return status;
        }
        let _ = PluginApp::maintenance_step(&mut app.0, 1, store::OWNED_SCHEMA_DECODE_PAGE_BYTES).expect("Sequence document archive maintenance");
    }
    panic!("Sequence document archive exceeded its public maintenance authority")
}

/// 🗄️ Whole-document round trip for a COMPOSED artifact. This used to call
/// `store::os_store::test_support::assert_document_text_round_trip` over a bare `SequenceStore`, but
/// that law cannot hold here by construction: the parent's DSL/pack text carries the `content` child
/// as an opaque `[child_id,target]` HANDLE only (`🚪️io/📸️snapshot/📝️text`'s `enc_child`), so the
/// re-parsed snapshot has no local scene owner and the first replayed op faults in
/// `sequence_working_scene` ("sequence child scene must be materialized before use") — the document's
/// content lives in a separate member envelope, not in the parent text. The law that actually covers
/// a composed document is the DOCUMENT ARCHIVE round trip (parent + its exact member closure),
/// exactly as the sibling composed pilot proves it in
/// `🌊️flow/…/🧬️mutations/💾️binary/🧪️tests/🔬️unit`'s `assert_document_archive_round_trip`. It proves
/// strictly more than the old call did: that an applied mutation survives the real save/load
/// protocol together with its child document, byte for byte.
#[semio_framework_async_macros::async_test]
async fn sequence_document_archive_round_trips_app_with_applied_mutation() {
    let mut app = new_app_with_registry_wired().await;
    let content_before = app.snapshot().expect("Sequence parent before add").content.child_id.clone();
    let steps_before = Box::pin(content_snapshot(&app)).await.nodes.len();
    dispatch(&mut app, SequenceCommand::AddStep(crate::editor::sequence::commands::step::add_step::AddStep { kind: "log.print".into(), x: 560.0, y: 0.0 })).await;

    let expected_parent = app.snapshot().expect("Sequence parent after add");
    // 🧸️ A step edit is published on the CHILD lane alone — the parent envelope keeps the very same
    // composed member (the exact-window law asserts the parent's bytes do not move either), so the
    // proof that the mutation landed is the member document's own node count, not a new handle.
    assert_eq!(expected_parent.content.child_id, content_before, "a step mutation must retain the exact composed content member");
    let expected_content = Box::pin(content_snapshot(&app)).await;
    assert_eq!(expected_content.nodes.len(), steps_before + 1, "the applied step must be in the composed member document");
    let expected = Box::pin(PluginApp::document_archive(&app.0)).await.expect("Sequence document archive");
    assert_eq!(expected.members.len(), 1, "Sequence archive must carry its exact content child closure");
    let member = &expected.members[0];
    assert_eq!(member.owner.slot, "content");
    assert_eq!(member.owner.child_id, expected_parent.content.child_id);
    assert_eq!(member.reference.artifact_id, expected_parent.content.child_id);

    let mut restored = new_app_with_registry_wired().await;
    PluginApp::begin_document_archive_load(&mut restored.0, 71, expected.clone()).expect("Sequence document archive admission");
    let status = Box::pin(drive_document_archive_load(&mut restored, 71)).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "Sequence composed archive load fault: {:?}", status.fault);
    PluginApp::acknowledge_document_archive_load(&mut restored.0, 71).expect("Sequence document archive acknowledgement");
    assert_eq!(restored.snapshot().expect("restored Sequence parent snapshot"), expected_parent);
    assert_eq!(Box::pin(content_snapshot(&restored)).await, expected_content);
    assert_eq!(Box::pin(PluginApp::document_archive(&restored.0)).await.expect("restored Sequence document archive"), expected);
}

//#region 🔖️OpTextTests
#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_create_step() {
    // 🧊️ The payload owns a non-empty `StepParams` dictionary on BOTH sides of the round trip, so the
    // source is held in a cold boundary and the parsed twin is retired rather than dropped.
    let mutation = neural_engine::ColdOwner::new(create_step(SequenceStep {
        id: "step-99".into(),
        kind: "log.print".into(),
        params: StepParams::new().insert("message", Value::Atom(Atom::String("hi there".into()))),
        x: 5.0,
        y: -6.5,
        slot: None,
        collapsed: false,
    }));
    store::os_store::test_support::assert_op_line_round_trip_cold(&*mutation, |parsed: SequenceMutation| neural_engine::ColdRetire::retire_cold(parsed));
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
