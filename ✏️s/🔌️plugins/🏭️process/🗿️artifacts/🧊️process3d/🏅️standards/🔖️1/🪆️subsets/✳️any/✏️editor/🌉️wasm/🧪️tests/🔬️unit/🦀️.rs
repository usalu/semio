use super::*;
use crate::mutations::change_step_enabled::ChangeStepEnabled;
use crate::mutations::change_step_origin::ChangeStepOrigin;
use crate::mutations::change_stock_label::ChangeStockLabel;
use crate::mutations::create_step::CreateStep;
use crate::mutations::delete_step::DeleteStep;
use crate::mutations::replace_stock_solid::ReplaceStockSolid;
use crate::standards::v1::subsets::any::schema::mutations::Process3dMutation;
use crate::{brep_child_handle, brep_snapshot_for_working_solid, empty_process3d_snapshot, Pose, ProcessMeasure, ProcessStep, StepOrigin, WorkingSolid, PROCESS_3D_SCHEMA};
use store::{create_document_envelope, ArtifactCommand, Author};

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

/// 🔐️ A bare `ArtifactStore::new` installs NO owner catalog, and `reserve_edit_history_slot` refuses
/// every `Apply` without one — `edit history insertion requires its exact mutation retirement
/// factory`. These fixtures therefore install the app's OWN exact document-store owners, the same
/// catalog `Process3dPlayApp::build_document_store_owners` hands the runtime, which in turn obliges
/// every fixture to finish through [`close_store`].
async fn new_store() -> Process3dStore {
    let mut store = Process3dStore::new(create_document_envelope(PROCESS_3D_SCHEMA, "process3d", empty_process3d_snapshot(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("new store");
    store.install_document_store_owners_exact(crate::host::owned::process3d_document_store_owners());
    store
}

/// 🧹️ Drains an owners-installed fixture store to the terminal-empty shallow shell `ArtifactStore`'s
/// own `Drop` asserts — one item and one page per turn, exactly the way the host retires a closing
/// document.
fn close_store(mut store: Process3dStore) {
    use semio_framework_plugin::ArtifactOwnedDisposer;
    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<Process3dSnapshot, Process3dMutation>::new();
    for _ in 0..1_048_576 {
        let grant = semio_framework_plugin::app::artifact_close_release_grant(store.next_close_byte_demand(), store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Process3d fixture admits exact close allocation");
        match disposer.close_step(&mut store, 1, grant).expect("Process3d fixture store close step") {
            semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= grant);
            }
            semio_framework_plugin::PluginCloseStep::AwaitingInput { reason } => panic!("Process3d fixture store close awaits input: {reason}"),
            semio_framework_plugin::PluginCloseStep::Blocked { reason } => {
                assert!(store.next_close_byte_demand() > grant, "Process3d fixture store close blocked without a newly retained physical demand: {reason}");
            }
            semio_framework_plugin::PluginCloseStep::Complete => {
                assert!(disposer.terminal_is_empty(&store));
                return;
            }
        }
    }
    panic!("Process3d fixture store did not reach its terminal-empty witness")
}

/// ↩️ Ticket `26/09/01/PROCESS-END-TO-END`: `step_payloads` is the durable, inline timeline
/// record now — `CreateStep`/`ChangeStepEnabled`/`ChangeStepOrigin`/`DeleteStep` are real
/// mutations against it, so this dispatches each through the wasm-facing store and asserts the
/// observed effect, then confirms undo restores the full pre-delete step (not merely its id).
#[semio_framework_async_macros::async_test]
async fn step_mutations_dispatch_real_effects() {
    let mut store = new_store().await;
    let empty = store.snapshot().expect("snapshot");

    store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("cut-1") })], transaction: None }).await.expect("dispatch create");
    let after_create = store.snapshot().expect("snapshot");
    assert_ne!(after_create, empty, "CreateStep must change the persisted document");
    assert!(after_create.step_payloads.iter().any(|step| step.id == "cut-1"));

    store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::ChangeStepEnabled(ChangeStepEnabled { id: "cut-1".into(), new_enabled: false })], transaction: None }).await.expect("dispatch enabled change");
    assert!(!store.snapshot().expect("snapshot").step_payloads.iter().find(|step| step.id == "cut-1").expect("cut-1 present").enabled);

    let origin = StepOrigin { machine_id: "circularSaw".into(), capability_id: "crosscut".into() };
    store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::ChangeStepOrigin(ChangeStepOrigin { id: "cut-1".into(), new_origin: Some(origin.clone()) })], transaction: None }).await.expect("dispatch origin change");
    assert_eq!(store.snapshot().expect("snapshot").step_payloads.iter().find(|step| step.id == "cut-1").expect("cut-1 present").origin, Some(origin.clone()));

    store.dispatch(ArtifactCommand::Apply { mutations: vec![Process3dMutation::DeleteStep(DeleteStep { id: "cut-1".into() })], transaction: None }).await.expect("dispatch delete");
    assert_eq!(store.snapshot().expect("snapshot"), empty, "DeleteStep must restore the pre-create document");

    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    let restored = store.snapshot().expect("snapshot");
    let restored_step = restored.step_payloads.iter().find(|step| step.id == "cut-1").expect("undo of DeleteStep must restore cut-1");
    assert!(!restored_step.enabled, "undo must restore the disabled flag, not just the step's presence");
    assert_eq!(restored_step.origin, Some(origin), "undo must restore the full pre-delete step, including origin");
    close_store(store);
}

/// ⏪️ Time travel edits a placed step's inputs, never the world gesture that placed it: a `create-step` superseded
/// with a deeper drill previews as the state before it plus the draft, and its Report replay re-applies every
/// downstream mutation onto the edited step — exactly the fresh fold of the edited log.
#[semio_framework_async_macros::async_test]
async fn a_placed_step_edited_in_history_replays_its_downstream() {
    use protocol::OpBinary;
    fn fold(base: &Process3dSnapshot, mutations: &[Process3dMutation]) -> Process3dSnapshot {
        mutations.iter().fold(base.clone(), |state, mutation| protocol::MutationDiff::apply(protocol::Mutation::diff(mutation, &state).diff(), &state).expect("the edited log folds"))
    }
    let mut store = new_store().await;
    let log = [
        Process3dMutation::CreateStep(CreateStep { index: 0, step: drill_step("drill-1") }),
        Process3dMutation::CreateStep(CreateStep { index: 1, step: cut_step("cut-1") }),
        Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Beam".into() }),
    ];
    for mutation in &log {
        store.dispatch(ArtifactCommand::Apply { mutations: vec![mutation.clone()], transaction: None }).await.expect("a placed step applies");
    }
    let ids: Vec<protocol::MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
    let mut deeper = drill_step("drill-1");
    deeper.measure = ProcessMeasure::Drill { radius: 0.02, depth: 0.6, pose: Pose::default() };
    let edited = Process3dMutation::CreateStep(CreateStep { index: 0, step: deeper });
    let drafts: std::collections::BTreeMap<protocol::MutationId, protocol::InputReplacement> = [(ids[0].clone(), protocol::InputReplacement::Input { schema: PROCESS_3D_SCHEMA.into(), payload: edited.encode_op().expect("the edited leaf encodes") })].into_iter().collect();
    let empty = empty_process3d_snapshot();
    assert_eq!(store.state_before(&ids[0], &drafts).expect("the preview base folds").as_ref(), &empty, "the preview base is the state right before the edited step");
    let mut replay = store.begin_report_replay(&drafts, Some(&ids[0])).expect("the replay begins at the edited step");
    assert!(matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), store::ReplayStep::Finished(_)));
    let result = replay.finish().expect("a finished replay yields its result");
    assert!(!store.replay_report(&result).expect("report").blocks_finalize(), "a deeper drill never blocks finalizing");
    let fresh = fold(&empty, &[edited, log[1].clone(), log[2].clone()]);
    assert_eq!(result.state().expect("the replay reached a state").as_ref(), &fresh, "the replay equals the fresh fold of the edited log");
    store.commit_finished_replay(result, store::HistoryFinalization::Overwrite).await.expect("overwrite commits");
    assert_eq!(store.snapshot().expect("snapshot"), fresh, "the overwritten history folds to the edited state");
    close_store(store);
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
            transaction: None,
        })
        .await
        .expect("set stock");
    let updated = store.snapshot().expect("snapshot");
    assert_eq!(updated.stock_solid, new_handle);
    assert_eq!(updated.stock_label, "Beam");

    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    assert_eq!(store.snapshot().expect("snapshot").stock_solid, original_solid);
    close_store(store);
}

#[semio_framework_async_macros::async_test]
async fn sets_stock_to_imported_solid_and_backwards_restores() {
    let mut store = new_store().await;
    let original_solid = store.snapshot().expect("snapshot").stock_solid;
    let imported_handle = brep_child_handle("stock", &brep_snapshot_for_working_solid(&WorkingSolid::ImportedSolid { solid_handle: "solid-7".into() }));
    store
        .dispatch(ArtifactCommand::Apply {
            mutations: vec![Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: imported_handle.clone() }), Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Imported STEP".into() })],
            transaction: None,
        })
        .await
        .expect("set imported stock");
    let updated = store.snapshot().expect("snapshot");
    assert_eq!(updated.stock_solid, imported_handle);
    assert_eq!(updated.stock_label, "Imported STEP");

    store.dispatch(ArtifactCommand::Undo).await.expect("undo");
    assert_eq!(store.snapshot().expect("snapshot").stock_solid, original_solid);
    close_store(store);
}

//#region 🔖️DocumentTextTests
#[semio_framework_async_macros::async_test]
async fn process3d_document_text_round_trips_after_apply_and_checkpoint() {
    let envelope = create_document_envelope(PROCESS_3D_SCHEMA, "process3d", empty_process3d_snapshot(), None);
    let mut store = Process3dStore::new(envelope, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("new store");
    store.install_document_store_owners_exact(crate::host::owned::process3d_document_store_owners());
    store
        .dispatch(ArtifactCommand::Apply {
            mutations: vec![
                Process3dMutation::ReplaceStockSolid(ReplaceStockSolid { new_solid: brep_child_handle("stock", &brep_snapshot_for_working_solid(&WorkingSolid::Box { width: 2.4, depth: 0.12, height: 0.24 })) }),
                Process3dMutation::ChangeStockLabel(ChangeStockLabel { new_label: "Timber Beam".into() }),
                Process3dMutation::CreateStep(CreateStep { index: 0, step: cut_step("cut-1") }),
                Process3dMutation::CreateStep(CreateStep { index: 1, step: drill_step("drill-1") }),
            ],
            transaction: None,
        })
        .await
        .expect("apply");
    store.dispatch(ArtifactCommand::CommitCheckpoint { message: Some("c1".into()), authors: vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }] }).await.expect("commit");
    store::os_store::test_support::assert_document_text_round_trip(&store).await;
    store::os_store::test_support::assert_document_pack_round_trip(&store).await;
    close_store(store);
}
//#endregion 🔖️DocumentTextTests
