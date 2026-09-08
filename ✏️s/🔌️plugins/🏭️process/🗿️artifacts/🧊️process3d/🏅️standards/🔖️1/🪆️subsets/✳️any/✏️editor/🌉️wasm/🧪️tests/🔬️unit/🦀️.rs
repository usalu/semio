
use super::*;
use crate::mutations::change_cursor::ChangeCursor;
use crate::mutations::change_step_enabled::ChangeStepEnabled;
use crate::mutations::change_step_origin::ChangeStepOrigin;
use crate::mutations::change_stock_label::ChangeStockLabel;
use crate::mutations::create_step::CreateStep;
use crate::mutations::delete_step::DeleteStep;
use crate::mutations::replace_stock_solid::ReplaceStockSolid;
use crate::op::Process3dMutation;
use crate::{PROCESS_3D_SCHEMA, Pose, ProcessMeasure, ProcessStep, StepOrigin, WorkingSolid, brep_child_handle, brep_snapshot_for_working_solid, empty_process3d_snapshot};
use store::{ArtifactCommand, Author, create_document_envelope};

fn cut_step(id: &str) -> ProcessStep {
    ProcessStep { id: id.into(), label: "Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: Pose::default() } }
}

fn drill_step(id: &str) -> ProcessStep {
    ProcessStep {
        id: id.into(),
        label: "Drill".into(),
        enabled: true,
        origin: Some(StepOrigin { machine_id: "circularSaw".into(), capability_id: "crosscut".into() }),
        measure: ProcessMeasure::Drill { radius: 0.02, depth: 0.3, pose: Pose::default() },
    }
}

async fn new_store() -> Process3dStore {
    Process3dStore::new(create_document_envelope(PROCESS_3D_SCHEMA, "process3d", empty_process3d_snapshot(), None)).await.expect("new store")
}

/// ↩️ Ticket `26/09/01/PROCESS-END-TO-END`: `step_payloads` is the durable, inline timeline
/// record now — `CreateStep`/`ChangeStepEnabled`/`ChangeStepOrigin`/`DeleteStep` are real
/// mutations against it, so this dispatches each through the wasm-facing store and asserts the
/// observed effect, then confirms undo restores the full pre-delete step (not merely its id).
#[semio_framework_async_macros::async_test]
async fn step_mutations_dispatch_real_effects() {
    let mut store = new_store().await;
    let empty = store.snapshot().expect("snapshot");

    store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("cut-1") })], description: None }).await.expect("dispatch create");
    let after_create = store.snapshot().expect("snapshot");
    assert_ne!(after_create, empty, "CreateStep must change the persisted document");
    assert!(after_create.step_payloads.iter().any(|step| step.id == "cut-1"));

    store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: "cut-1".into(), new_enabled: false })], description: None }).await.expect("dispatch enabled change");
    assert!(!store.snapshot().expect("snapshot").step_payloads.iter().find(|step| step.id == "cut-1").expect("cut-1 present").enabled);

    let origin = StepOrigin { machine_id: "circularSaw".into(), capability_id: "crosscut".into() };
    store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::ChangeStepOrigin(ChangeStepOrigin { id: "cut-1".into(), new_origin: Some(origin.clone()) })], description: None }).await.expect("dispatch origin change");
    assert_eq!(store.snapshot().expect("snapshot").step_payloads.iter().find(|step| step.id == "cut-1").expect("cut-1 present").origin, Some(origin.clone()));

    store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::DeleteStep(DeleteStep { id: "cut-1".into() })], description: None }).await.expect("dispatch delete");
    assert_eq!(store.snapshot().expect("snapshot"), empty, "DeleteStep must restore the pre-create document");

    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    let restored = store.snapshot().expect("snapshot");
    let restored_step = restored.step_payloads.iter().find(|step| step.id == "cut-1").expect("undo of DeleteStep must restore cut-1");
    assert!(!restored_step.enabled, "undo must restore the disabled flag, not just the step's presence");
    assert_eq!(restored_step.origin, Some(origin), "undo must restore the full pre-delete step, including origin");
}

#[semio_framework_async_macros::async_test]
async fn moves_cursor_and_undo_restores_it() {
    let mut store = new_store().await;
    store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(2) })], description: None }).await.expect("move cursor");
    assert_eq!(store.snapshot().expect("snapshot").resolved_up_to, Some(2));

    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    assert_eq!(store.snapshot().expect("snapshot").resolved_up_to, None);
}

/// 🧬️ `Stock`'s `id` has no semantic mutation of its own (it is a fixed singleton-facet key, never
/// a user-addressed identity field) — only `solid`/`label`/`pose` each carry their own mutation
/// now (`ReplaceStockSolid`/`ChangeStockLabel`/`MoveStock`), so these two tests compose the fields
/// that actually change instead of replacing the whole `Stock` record. `ReplaceStockSolid` stays a
/// real, fully-working mutation (a handle SWAP, never needing to read prior child content).
#[semio_framework_async_macros::async_test]
async fn sets_stock_and_backwards_restores() {
    let mut store = new_store().await;
    let original_solid = store.snapshot().expect("snapshot").stock_solid;
    let new_handle = brep_child_handle("stock", &brep_snapshot_for_working_solid(&WorkingSolid::Cylinder { radius: 0.2, height: 2.0 }));
    store
        .dispatch(ArtifactCommand::Apply {
            mutations: vec![Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: new_handle.clone() }), Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() })],
            description: None,
        })
        .await
        .expect("set stock");
    let updated = store.snapshot().expect("snapshot");
    assert_eq!(updated.stock_solid, new_handle);
    assert_eq!(updated.stock_label, "Beam");

    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    assert_eq!(store.snapshot().expect("snapshot").stock_solid, original_solid);
}

#[semio_framework_async_macros::async_test]
async fn sets_stock_to_imported_solid_and_backwards_restores() {
    let mut store = new_store().await;
    let original_solid = store.snapshot().expect("snapshot").stock_solid;
    let imported_handle = brep_child_handle("stock", &brep_snapshot_for_working_solid(&WorkingSolid::ImportedSolid { solid_handle: "solid-7".into() }));
    store
        .dispatch(ArtifactCommand::Apply {
            mutations: vec![Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: imported_handle.clone() }), Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Imported STEP".into() })],
            description: None,
        })
        .await
        .expect("set imported stock");
    let updated = store.snapshot().expect("snapshot");
    assert_eq!(updated.stock_solid, imported_handle);
    assert_eq!(updated.stock_label, "Imported STEP");

    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    assert_eq!(store.snapshot().expect("snapshot").stock_solid, original_solid);
}

//#region 🔖️DocumentTextTests
#[semio_framework_async_macros::async_test]
async fn process3d_document_text_round_trips_after_apply_and_checkpoint() {
    let envelope = create_document_envelope(PROCESS_3D_SCHEMA, "process3d", empty_process3d_snapshot(), None);
    let mut store = Process3dStore::new(envelope).await.expect("new store");
    store
        .dispatch(ArtifactCommand::Apply {
            mutations: vec![
                Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: brep_child_handle("stock", &brep_snapshot_for_working_solid(&WorkingSolid::Box { width: 2.4, depth: 0.12, height: 0.24 })) }),
                Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Timber Beam".into() }),
                Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("cut-1") }),
                Process3dMutation::CreateStep(CreateStep { index: 1, step: drill_step("drill-1") }),
                Process3dMutation::ChangeCursor(ChangeCursor { new_resolved_up_to: Some(1) }),
            ],
            description: Some("build timeline".into()),
        })
        .await
        .expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }] }).await.expect("commit");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
}
//#endregion 🔖️DocumentTextTests
