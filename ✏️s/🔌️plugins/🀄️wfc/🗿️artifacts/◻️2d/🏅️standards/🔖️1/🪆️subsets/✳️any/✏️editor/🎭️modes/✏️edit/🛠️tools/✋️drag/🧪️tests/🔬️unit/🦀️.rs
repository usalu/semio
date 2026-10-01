//! ✋️ Laws of the wfc2d drag tool: one released node drag is ONE `ToolTransaction` of relative `drag-slots` leaves —
//! one edit, one history row stamped with its `TransactionRef`, labelled from the leaf in English and German; a
//! release that moves nothing leaves zero trace; two drags are two transactions; and a drag edited in history
//! replays its downstream exactly like a fresh fold of the edited log.

use super::*;
use crate::editor::wfc2d::modes::edit::windows::graph::WFC_GRAPH_WINDOW;
use crate::editor::wfc2d::{Wfc2dEditor, Wfc2dEditorCommand, WFC_2D_GRAPH_VIEW_SCALE};
use crate::mutations::{apply_wfc2d_mutation, resize_slot};
use crate::{Wfc2dSnapshot, WFC_2D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ActionMeta, App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

fn base() -> Wfc2dSnapshot {
    crate::examples::two_room_corridor::document()
}

fn record(gesture: &str, node_ids: &[&str], dx: f64, dy: f64) -> NodeDragRecord {
    NodeDragRecord { gesture_id: gesture.into(), node_ids: node_ids.iter().map(|id| id.to_string()).collect(), dx, dy }
}

//#region 🛠️Tool
#[test]
fn a_release_is_one_transaction_of_one_relative_leaf_in_document_units() {
    let (transaction, leaves) = wfc2d_drag_tool_commit("nodeGraphEdit", "seed", &base(), Vec::new(), &[record("node-drag:1", &["room-a", "corridor"], 280.0, -70.0)], 140.0).expect("the release commits");
    assert!(transaction.id.starts_with("tx-"), "{transaction:?}");
    assert_eq!(transaction.tool, "s.wfc.wfc2d@1/*#editor#nodeGraphEdit");
    assert_eq!(leaves, vec![drag_slots(vec!["room-a".into(), "corridor".into()], 2.0, -0.5)]);
}

#[test]
fn a_release_that_moves_nothing_leaves_zero_trace() {
    assert!(wfc2d_drag_tool_commit("nodeGraphEdit", "seed", &base(), Vec::new(), &[record("node-drag:1", &["room-a"], 0.0, 0.0)], 140.0).is_none(), "a zero offset");
    assert!(wfc2d_drag_tool_commit("nodeGraphEdit", "seed", &base(), Vec::new(), &[record("node-drag:1", &["ghost"], 140.0, 0.0)], 140.0).is_none(), "no slot of this document");
    assert!(wfc2d_drag_tool_commit("nodeGraphEdit", "seed", &base(), Vec::new(), &[], 140.0).is_none(), "no record");
}

#[test]
fn two_releases_are_two_transactions() {
    let (first, _) = wfc2d_drag_tool_commit("nodeGraphEdit", "seed-one", &base(), Vec::new(), &[record("node-drag:1", &["room-a"], 140.0, 0.0)], 140.0).expect("first");
    let (second, _) = wfc2d_drag_tool_commit("nodeGraphEdit", "seed-two", &base(), Vec::new(), &[record("node-drag:2", &["room-a"], 140.0, 0.0)], 140.0).expect("second");
    assert_ne!(first.id, second.id);
}

#[test]
fn a_wire_drawn_by_the_same_gesture_rides_the_same_transaction() {
    let edge = crate::schema::snapshot::Wfc2dSlotEdge { id: "room-a-room-b".into(), from_slot_id: "room-a".into(), to_slot_id: "room-b".into(), relation: "adjacent".into() };
    let wire = crate::mutations::connect_slots(edge);
    let (_, leaves) = wfc2d_drag_tool_commit("nodeGraphEdit", "seed", &base(), vec![wire.clone()], &[record("node-drag:1", &["room-a"], 140.0, 0.0)], 140.0).expect("the release commits");
    assert_eq!(leaves.len(), 2);
    assert_eq!(leaves[0], wire, "the wire first, then the drag");
}

#[test]
fn a_seedless_view_publishes_the_leaves_plainly() {
    let emit = wfc2d_drag_tool_emit("nodeGraphEdit", "", &base(), Vec::new(), &[record("node-drag:1", &["room-a"], 140.0, 0.0)], 140.0, "Drag room-a".into());
    assert!(emit.transaction.is_none());
    assert_eq!(emit.artifact_mutations.len(), 1);
}

#[test]
fn the_leaf_labels_the_row_in_english_and_german() {
    let label = <crate::Wfc2dMutation as protocol::SemanticMutation<Wfc2dSnapshot>>::label(&drag_slots(vec!["room-a".into(), "room-b".into()], 1.5, -0.25));
    assert_eq!(label.resolve(protocol::Terminology::Native, protocol::Locale::En), "Drag 2 slots by (1.5, -0.25)");
    assert_eq!(label.resolve(protocol::Terminology::Native, protocol::Locale::De), "2 Slots um (1,5; -0,25) ziehen");
}
//#endregion 🛠️Tool

//#region 🧩️MountedApp
type Wfc2dApp = VcsArtifactApp<EditorApp<Wfc2dEditor>>;

fn block_on<F: std::future::Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => return output,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn manifest() -> App {
    App { definition: crate::editor::wfc2d::create_wfc2d_editor(), examples: Vec::new() }
}

fn app() -> Wfc2dApp {
    let mut app = block_on(semio_framework_plugin::artifact_app_laws::new_app_with_registry::<EditorApp<Wfc2dEditor>>(manifest));
    block_on(app.bind_instance_id(1));
    app
}

fn graph_meta() -> ActionMeta {
    let window_instances = vec![ViewWindowInstance { id: WFC_GRAPH_WINDOW.into(), window_kind_id: WFC_GRAPH_WINDOW.into() }];
    let view = ViewModel { window_instances, ..Default::default() }.for_window_instance(WFC_GRAPH_WINDOW).expect("the graph window is in the roster");
    ActionMeta { view_state: Some(view), ..semio_framework_plugin::artifact_app_laws::meta("local") }
}

fn settle(app: &mut Wfc2dApp, result: Result<InvocationResult, semio_framework_plugin::Fault>) -> InvocationResult {
    let mut result = result.expect("the dispatch is admitted");
    for _ in 0..1_048_576 {
        while let Some(page) = app.take_typed_operation_result_page(1) {
            assert!(page.lane != semio_framework_plugin::app::TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
            app.acknowledge_typed_operation_result(page.token).expect("the page is acknowledged");
        }
        while let Some(completion) = block_on(app.take_typed_operation_completion()).expect("the completion is taken") {
            if let Some(patch) = completion.history_patch {
                match result.history_patch.as_mut() {
                    Some(previous) => previous.upserts.extend(patch.upserts),
                    None => result.history_patch = Some(patch),
                }
            }
        }
        while app.take_typed_operation_effect().is_some() || app.take_typed_operation_event().is_some() {}
        let _ = app.take_typed_operation_ui_scope();
        if !app.has_pending_typed_operations() {
            return result;
        }
        PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("maintenance");
        block_on(app.advance_typed_operation_publication()).expect("publication advances");
    }
    panic!("the wfc2d dispatch did not settle");
}

fn drag(app: &mut Wfc2dApp, gesture: &str, node_ids: &[&str], dx: f64, dy: f64) -> InvocationResult {
    let row = record(gesture, node_ids, dx * WFC_2D_GRAPH_VIEW_SCALE, dy * WFC_2D_GRAPH_VIEW_SCALE).to_row();
    let command = Wfc2dEditorCommand::NodeGraphEdit { operations_json: dsl::json::to_json_string(&dsl::DslValue::Array(vec![row])) };
    let result = block_on(app.dispatch_typed(command, &graph_meta()));
    settle(app, result)
}

fn edit_rows(result: &InvocationResult) -> Vec<semio_framework::kernel::HistoryEntry> {
    result.history_patch.as_ref().map(|patch| patch.upserts.iter().filter(|entry| entry.applied && !entry.op_lines.is_empty()).cloned().collect()).unwrap_or_default()
}

fn close(app: &mut Wfc2dApp) {
    for _ in 0..1_048_576 {
        if app.close_terminal_is_empty() {
            return;
        }
        if PluginApp::close_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("wfc2d app close") == semio_framework_plugin::PluginCloseStep::Complete {
            break;
        }
    }
    assert!(app.close_terminal_is_empty(), "the wfc2d app did not reach terminal-empty ownership");
}

#[test]
fn one_mounted_drag_is_one_edit_one_row_and_one_transaction() {
    let mut app = app();
    let before = app.snapshot().expect("projection");
    let rows = edit_rows(&drag(&mut app, "node-drag:1", &["room-a"], 1.0, 0.0));
    assert_eq!(rows.len(), 1, "one drag, one history row: {rows:?}");
    let transaction = rows[0].transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.wfc.wfc2d@1/*#editor#nodeGraphEdit", "{transaction:?}");
    assert!(rows[0].op_lines.iter().all(|line| line.starts_with("drag-slots")), "the op is the relative leaf: {:?}", rows[0].op_lines);
    assert_eq!(rows[0].label.resolve(protocol::Terminology::Native, protocol::Locale::En), "Drag 1 slot by (1, 0)");
    assert_eq!(rows[0].label.resolve(protocol::Terminology::Native, protocol::Locale::De), "1 Slot um (1; 0) ziehen");
    let after = app.snapshot().expect("projection");
    let moved = |snapshot: &Wfc2dSnapshot| snapshot.slots.iter().find(|slot| slot.id == "room-a").map(|slot| slot.x).expect("room-a");
    assert_eq!(moved(&after), moved(&before) + 1.0, "the drag landed in document units");
    close(&mut app);
}

#[test]
fn a_mounted_release_that_moves_nothing_and_two_drags_keep_their_shape() {
    let mut app = app();
    let before = app.snapshot().expect("projection");
    assert!(edit_rows(&drag(&mut app, "node-drag:0", &["room-a"], 0.0, 0.0)).is_empty(), "a release that moved nothing is no history");
    assert_eq!(app.snapshot().expect("projection"), before, "zero trace");
    let first = edit_rows(&drag(&mut app, "node-drag:1", &["room-a"], 1.0, 0.0));
    let second = edit_rows(&drag(&mut app, "node-drag:2", &["room-b"], 0.0, 1.0));
    assert_eq!((first.len(), second.len()), (1, 1));
    assert_ne!(first[0].transaction.as_ref().expect("first ref").id, second[0].transaction.as_ref().expect("second ref").id, "two drags are two transactions");
    close(&mut app);
}
//#endregion 🧩️MountedApp

//#region ⏪️TimeTravel
/// ⏪️ Time travel edits a drag's inputs: a drag re-offset in history previews as the state before it plus the draft,
/// and its Report replay re-applies the downstream relative drag and resize onto the edited drag — exactly the fresh
/// fold of the edited log.
#[test]
fn a_drag_edited_in_history_replays_its_downstream() {
    use protocol::OpBinary;
    block_on(async {
        let mut store = store::ArtifactStore::<Wfc2dSnapshot, crate::Wfc2dMutation>::new(store::create_document_envelope::<Wfc2dSnapshot, crate::Wfc2dMutation>(WFC_2D_DOCUMENT_SCHEMA, "drag-time-travel", base(), None)).await.expect("the store opens");
        store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<Wfc2dSnapshot, crate::Wfc2dMutation>());
        let log = [drag_slots(vec!["room-a".into()], 1.0, 0.0), drag_slots(vec!["room-a".into(), "room-b".into()], 0.0, 2.0), resize_slot("room-a".into(), 3.0, 1.5)];
        for mutation in &log {
            store.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation.clone()], description: None, transaction: None }).await.expect("the edit applies");
        }
        let ids: Vec<protocol::MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
        let edited = drag_slots(vec!["room-a".into()], 3.0, -1.0);
        let drafts: std::collections::BTreeMap<protocol::MutationId, protocol::InputReplacement> = [(ids[0].clone(), protocol::InputReplacement::Input { schema: WFC_2D_DOCUMENT_SCHEMA.into(), payload: edited.encode_op().expect("the edited leaf encodes") })].into_iter().collect();
        let mut preview = store.state_before(&ids[0], &drafts).expect("the preview base folds").as_ref().clone();
        assert_eq!(preview, base(), "the preview base is the state right before the edited drag");
        apply_wfc2d_mutation(&mut preview, &edited).expect("the draft applies to its base");
        let mut replay = store.begin_report_replay(&drafts, Some(&ids[0])).expect("the replay begins at the edited drag");
        assert!(matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), store::ReplayStep::Finished(_)));
        let result = replay.finish().expect("a finished replay yields its result");
        let report = store.replay_report(&result).expect("report");
        assert!(!report.blocks_finalize(), "a re-offset drag never blocks finalizing");
        let mut fresh = base();
        for mutation in [edited, log[1].clone(), log[2].clone()] {
            apply_wfc2d_mutation(&mut fresh, &mutation).expect("the edited log folds");
        }
        assert_eq!(result.state().expect("the replay reached a state").as_ref(), &fresh, "the replay equals the fresh fold of the edited log");
        store.commit_finished_replay(result, store::HistoryFinalization::Overwrite).await.expect("overwrite commits");
        assert_eq!(store.snapshot_ref(), &fresh, "the overwritten history folds to the edited state");
        let mut disposer = semio_framework_plugin::bounded_document_store_disposer::<Wfc2dSnapshot, crate::Wfc2dMutation>();
        for _ in 0..4_096 {
            if disposer.terminal_is_empty(&store) {
                break;
            }
            disposer.close_step(&mut store, 1, 64 * 1024).expect("the store retires");
        }
        assert!(disposer.terminal_is_empty(&store), "the standalone store retires to its terminal-empty shell");
    });
}
//#endregion ⏪️TimeTravel
