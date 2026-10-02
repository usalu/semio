//! 🛠️ Laws for the transform tool's ONE promise, through the real registry-backed app and its retained command route:
//! one gumball gesture is one `ToolTransaction` — one edit, one history row stamped with its `TransactionRef`, one
//! parametric leaf carrying the literal targets and the net parameters — streamed ticks live in the Blueprint window's
//! open transaction (previewed by that window, never history), and a cancelled gesture leaves zero trace. Time travel edits
//! the yielded leaf's inputs (offset, pivot, targets) through the real `historyEdit*` verbs and replays the downstream.

use super::*;
use crate::editor::layout::unit_tests::context::{layout_app_with_registry, scene, window_meta, LayoutApp, INSTANCE, WINDOW};
use semio_framework::kernel::HistoryEntry;
use semio_framework_plugin::{artifact_app_laws, InvocationResult, PluginApp};

//#region 🔁️Settle
/// 📄️ Drains everything one settle turn can leave behind — presented pages (ACKed), effects, events, the UI scope,
/// composed results, local-interaction replies, completions and the history patch they carry — into `result`.
async fn drain(app: &mut LayoutApp, result: &mut InvocationResult, fault: &mut Option<Fault>) -> Result<(), Fault> {
    while let Some(page) = app.0.take_typed_operation_result_page(INSTANCE) {
        if page.lane == semio_framework_plugin::app::TypedOperationResultLane::Fault {
            fault.get_or_insert_with(|| Fault::from(String::from_utf8_lossy(page.bytes()).into_owned()));
        }
        app.0.acknowledge_typed_operation_result(page.token)?;
    }
    while let Some(effect) = app.0.take_typed_operation_effect() {
        result.requested_effects.push(effect);
    }
    while let Some(event) = app.0.take_typed_operation_event() {
        result.events.push(event);
    }
    while let Some(scope) = app.0.take_typed_operation_ui_scope() {
        result.ui_scope = scope;
    }
    while app.0.take_typed_operation_composed_result().is_some() {}
    while let Some(reply) = app.0.take_local_interaction_query_reply() {
        if let protocol::LocalInteractionQueryReply::Page { page } = reply {
            let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
            app.0.acknowledge_local_interaction_query(&token);
        }
    }
    while let Some(completion) = app.0.take_typed_operation_completion().await? {
        if let Some(patch) = completion.history_patch {
            match result.history_patch.as_mut() {
                Some(previous) => {
                    previous.upserts.extend(patch.upserts);
                    previous.can_undo = patch.can_undo;
                    previous.can_redo = patch.can_redo;
                }
                None => result.history_patch = Some(patch),
            }
        }
    }
    Ok(())
}

/// 🔁️ Drives the dispatch's typed operation to quiescence, collecting what it published.
async fn settle(app: &mut LayoutApp, result: Result<InvocationResult, Fault>) -> InvocationResult {
    let mut result = result.unwrap_or_else(|fault| panic!("the dispatch is admitted: {fault:?}"));
    let mut fault = None;
    for _ in 0..1_048_576 {
        if !app.0.has_pending_typed_operations() {
            drain(app, &mut result, &mut fault).await.expect("drain");
            assert!(fault.is_none(), "the operation faulted: {fault:?}");
            return result;
        }
        PluginApp::maintenance_step(&mut app.0, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("maintenance");
        app.0.advance_typed_operation_publication().await.expect("publication");
        drain(app, &mut result, &mut fault).await.expect("drain");
    }
    panic!("the layout operation did not settle")
}

async fn run(app: &mut LayoutApp, command: LayoutCommand) -> InvocationResult {
    let result = app.0.dispatch_typed(command, &window_meta(WINDOW)).await;
    settle(app, result).await
}

/// 🕰️ A framework-reserved verb (`undo`, `hostEvent`, …) from the Blueprint window.
async fn reserved(app: &mut LayoutApp, action: &str, args: Option<Value>) -> InvocationResult {
    let meta = window_meta(WINDOW);
    let args = args.map(|value| DslValue::from(&value));
    let admitted = app.0.handle_action(action, args.as_ref(), &meta).await.unwrap_or_else(|fault| panic!("{action} is admitted: {fault:?}"));
    let result = semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted).await;
    settle(app, result).await
}
//#endregion 🔁️Settle

//#region 🔎️Probes
/// 🧾️ The applied history rows one dispatch upserted that carry document ops.
fn edit_rows(result: &InvocationResult) -> Vec<HistoryEntry> {
    result.history_patch.as_ref().map(|patch| patch.upserts.iter().filter(|entry| entry.applied && !entry.op_lines.is_empty()).cloned().collect()).unwrap_or_default()
}

fn origin(app: &LayoutApp, id: &str) -> (f64, f64) {
    let document = app.0.snapshot().expect("projection");
    let bounds = document.pages[0].frames.iter().find(|frame| frame.id() == id).expect("demo frame").bounds().clone();
    (bounds.x, bounds.y)
}

/// 👁️ Where the Blueprint window PAINTS a rect frame's origin — its first path point, which previews the open gesture.
async fn painted(app: &mut LayoutApp, id: &str) -> (f64, f64) {
    let layers: Value = serde_json::from_str(&scene(app, LAYOUT_PLAY_BODY_BLUEPRINT).await.layers_json).expect("layer JSON");
    let layer = layers.as_array().expect("layers").iter().find(|layer| layer["id"] == id).unwrap_or_else(|| panic!("painted layer {id}"));
    let to = &layer["segments"][0]["to"];
    (to[0].as_f64().expect("x"), to[1].as_f64().expect("y"))
}

fn stream(dx: f64, dy: f64) -> LayoutCommand {
    LayoutCommand::TranslateSelection(gumball::TranslateSelection { ids: vec!["frame-1".into()], dx, dy, phase: Some("stream".into()), reason: None })
}

fn phase(phase: &str, reason: Option<&str>) -> LayoutCommand {
    LayoutCommand::TranslateSelection(gumball::TranslateSelection { ids: vec!["frame-1".into()], dx: 0.0, dy: 0.0, phase: Some(phase.into()), reason: reason.map(str::to_string) })
}

fn label(entry: &HistoryEntry, locale: protocol::Locale) -> String {
    entry.label.resolve(protocol::Terminology::Native, locale).to_string()
}

/// 🧯️ Zero trace: no edit, no row, the document untouched and the window painting the document again.
async fn assert_zero_trace(app: &mut LayoutApp, result: &InvocationResult, before: (f64, f64), what: &str) {
    assert!(edit_rows(result).is_empty(), "{what}: no history row");
    assert_eq!(origin(app, "frame-1"), before, "{what}: the document is untouched");
    assert_eq!(painted(app, "frame-1").await, before, "{what}: the window paints the document again");
}
//#endregion 🔎️Probes

//#region 🌊️Gestures
/// 🌊️ One gumball drag streamed over several dispatches: every tick lands in the window's ONE open transaction — the
/// window previews it, the document never moves, no row appears — and the commit publishes ONE edit, one row stamped
/// with the transform tool's transaction, whose op is the `drag-frames` leaf with the net offset and whose label is the
/// leaf's, in English and German; one undo takes the whole gesture back.
#[semio_framework_async_macros::async_test]
async fn one_gumball_drag_is_one_edit_one_row_and_one_transaction() {
    let mut app = layout_app_with_registry().await;
    let before = origin(&app, "frame-1");
    for tick in 1..=3 {
        let result = run(&mut app, stream(10.0, -2.0)).await;
        assert!(edit_rows(&result).is_empty(), "tick {tick} is no history row");
        assert_eq!(origin(&app, "frame-1"), before, "tick {tick} never moves the document");
        assert_eq!(painted(&mut app, "frame-1").await, (before.0 + 10.0 * f64::from(tick), before.1 - 2.0 * f64::from(tick)), "tick {tick} is previewed in the window");
    }
    let result = run(&mut app, phase("commit", None)).await;
    let rows = edit_rows(&result);
    assert_eq!(rows.len(), 1, "one gesture, one history row: {rows:?}");
    let transaction = rows[0].transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.layout.layout@1/*#editor#translateSelection", "{transaction:?}");
    assert!(rows[0].op_lines.iter().any(|line| line.contains("DragFrames") && line.contains("frame-1") && line.contains("30")), "the op is the parametric leaf with the net offset: {:?}", rows[0].op_lines);
    assert_eq!(label(&rows[0], protocol::Locale::En), "Drag 1 frame by (30, -6)");
    assert_eq!(label(&rows[0], protocol::Locale::De), "1 Rahmen um (30; -6) ziehen");
    assert_eq!(origin(&app, "frame-1"), (before.0 + 30.0, before.1 - 6.0));
    assert_eq!(painted(&mut app, "frame-1").await, (before.0 + 30.0, before.1 - 6.0), "the window paints the committed document");
    reserved(&mut app, "undo", None).await;
    assert_eq!(origin(&app, "frame-1"), before, "one undo takes the whole gesture back");
    artifact_app_laws::close_registered_fixture_app(&mut app.0);
}

/// 🧯️ A cancelled gesture leaves zero trace — the overlay's own `abort` phase and every window fact the host forwards
/// (`hostEvent` blur, lost capture, closing window) — and a late commit finds nothing.
#[semio_framework_async_macros::async_test]
async fn a_cancelled_gesture_leaves_zero_trace() {
    let mut app = layout_app_with_registry().await;
    let before = origin(&app, "frame-1");
    run(&mut app, stream(12.0, 0.0)).await;
    assert_eq!(painted(&mut app, "frame-1").await, (before.0 + 12.0, before.1), "the gesture is open");
    let aborted = run(&mut app, phase("abort", Some("captureLost"))).await;
    assert_zero_trace(&mut app, &aborted, before, "abort phase").await;
    for kind in ["blur", "captureLost", "retiring"] {
        run(&mut app, stream(7.0, 3.0)).await;
        assert_eq!(painted(&mut app, "frame-1").await, (before.0 + 7.0, before.1 + 3.0), "{kind}: the gesture is open");
        let result = reserved(&mut app, "hostEvent", Some(serde_json::json!({ "windowId": WINDOW, "kind": kind }))).await;
        assert_zero_trace(&mut app, &result, before, kind).await;
        let late = run(&mut app, phase("commit", None)).await;
        assert_zero_trace(&mut app, &late, before, &format!("a commit after {kind}")).await;
    }
    artifact_app_laws::close_registered_fixture_app(&mut app.0);
}

/// ✌️ Two gestures are two transactions: two edits, two rows, two distinct transaction ids — never merged into one
/// undo step the way a static coalesce key merged consecutive drags.
#[semio_framework_async_macros::async_test]
async fn two_gestures_are_two_transactions() {
    let mut app = layout_app_with_registry().await;
    let before = origin(&app, "frame-1");
    let mut ids = Vec::new();
    for dx in [5.0, 8.0] {
        run(&mut app, stream(dx, 0.0)).await;
        let rows = edit_rows(&run(&mut app, phase("commit", None)).await);
        assert_eq!(rows.len(), 1, "each gesture is one row: {rows:?}");
        ids.push(rows[0].transaction.clone().expect("a tool transaction").id);
    }
    assert_ne!(ids[0], ids[1], "two gestures never share a transaction");
    assert_eq!(origin(&app, "frame-1"), (before.0 + 13.0, before.1));
    reserved(&mut app, "undo", None).await;
    assert_eq!(origin(&app, "frame-1"), (before.0 + 5.0, before.1), "one undo takes back exactly the second gesture");
    artifact_app_laws::close_registered_fixture_app(&mut app.0);
}

/// 🔃️ One-shot turn and scaling verbs (palette, agent) are each one transaction yielding their parametric leaf about the
/// recorded centroid pivot, labelled from the leaf — for one frame and for several, whose inverse restores two absolute
/// rows per frame inside the leaf's declared fold footprint; one undo takes each back exactly.
#[semio_framework_async_macros::async_test]
async fn one_shot_turns_and_scalings_are_one_transaction_each() {
    let mut app = layout_app_with_registry().await;
    let turned = run(&mut app, LayoutCommand::RotateSelection(rotate_selection::RotateSelection { ids: vec!["frame-1".into()], angle: std::f64::consts::FRAC_PI_2, phase: None, reason: None })).await;
    let rows = edit_rows(&turned);
    assert_eq!(rows.len(), 1);
    assert!(rows[0].transaction.as_ref().is_some_and(|transaction| transaction.tool.ends_with("#rotateSelection")));
    assert_eq!(label(&rows[0], protocol::Locale::En), "Rotate 1 frame by 90°");
    let scaled = run(&mut app, LayoutCommand::ScaleSelection(scale_selection::ScaleSelection { ids: vec!["frame-1".into()], sx: 2.0, sy: 2.0, phase: None, reason: None })).await;
    let rows = edit_rows(&scaled);
    assert_eq!(rows.len(), 1);
    assert_eq!(label(&rows[0], protocol::Locale::De), "1 Rahmen um (2; 2) skalieren");
    let before = app.0.snapshot().expect("the head before the multi-frame transforms");
    let pair = vec!["frame-1".to_string(), "frame-text-1".to_string()];
    let turned = run(&mut app, LayoutCommand::RotateSelection(rotate_selection::RotateSelection { ids: pair.clone(), angle: std::f64::consts::FRAC_PI_2, phase: None, reason: None })).await;
    assert_eq!(edit_rows(&turned).iter().map(|row| label(row, protocol::Locale::En)).collect::<Vec<_>>(), vec!["Rotate 2 frames by 90°".to_string()], "a two-frame turn is one row");
    let scaled = run(&mut app, LayoutCommand::ScaleSelection(scale_selection::ScaleSelection { ids: pair, sx: 0.5, sy: 3.0, phase: None, reason: None })).await;
    assert_eq!(edit_rows(&scaled).iter().map(|row| label(row, protocol::Locale::De)).collect::<Vec<_>>(), vec!["2 Rahmen um (0,5; 3) skalieren".to_string()], "a two-frame scaling is one row");
    assert_ne!(app.0.snapshot().expect("the transformed head"), before);
    reserved(&mut app, "undo", None).await;
    reserved(&mut app, "undo", None).await;
    assert_eq!(app.0.snapshot().expect("the undone head"), before, "two undos restore both frames exactly through their multi-row inverses");
    artifact_app_laws::close_registered_fixture_app(&mut app.0);
}

/// 🧲️ A streamed gumball drag of two frames is one transaction holding ONE `drag-frames` leaf over both targets — one row,
/// both frames moved by the net offset, and one undo restores both.
#[semio_framework_async_macros::async_test]
async fn a_streamed_drag_of_two_frames_is_one_transaction() {
    let mut app = layout_app_with_registry().await;
    let before = (origin(&app, "frame-1"), origin(&app, "frame-text-1"));
    let tick = |dx: f64, phase: &str| LayoutCommand::TranslateSelection(gumball::TranslateSelection { ids: vec!["frame-1".into(), "frame-text-1".into()], dx, dy: 1.0, phase: Some(phase.into()), reason: None });
    run(&mut app, tick(4.0, "stream")).await;
    run(&mut app, tick(4.0, "stream")).await;
    let rows = edit_rows(&run(&mut app, tick(2.0, "commit")).await);
    assert_eq!(rows.iter().map(|row| label(row, protocol::Locale::En)).collect::<Vec<_>>(), vec!["Drag 2 frames by (10, 3)".to_string()], "one gesture, one row, the net leaf");
    assert!(rows[0].transaction.is_some(), "the row is the gesture's transaction");
    assert_eq!((origin(&app, "frame-1"), origin(&app, "frame-text-1")), ((before.0 .0 + 10.0, before.0 .1 + 3.0), (before.1 .0 + 10.0, before.1 .1 + 3.0)));
    reserved(&mut app, "undo", None).await;
    assert_eq!((origin(&app, "frame-1"), origin(&app, "frame-text-1")), before, "one undo restores both frames");
    artifact_app_laws::close_registered_fixture_app(&mut app.0);
}

/// 📐️ A document edit landing under an open gesture aborts it (`baseMoved`): the commit that finds it publishes
/// nothing, and only the other edit is history.
#[semio_framework_async_macros::async_test]
async fn a_document_moved_under_an_open_gesture_aborts_it() {
    let mut app = layout_app_with_registry().await;
    let before = origin(&app, "frame-1");
    run(&mut app, stream(20.0, 0.0)).await;
    let other = run(&mut app, LayoutCommand::PatchFrame(patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "y".into(), value: "77".into() })).await;
    assert_eq!(edit_rows(&other).len(), 1, "the other edit lands");
    let late = run(&mut app, phase("commit", None)).await;
    assert!(edit_rows(&late).is_empty(), "the gesture opened on the old base publishes nothing");
    assert_eq!(origin(&app, "frame-1"), before);
    assert_eq!(origin(&app, "frame-text-1").1, 77.0);
    artifact_app_laws::close_registered_fixture_app(&mut app.0);
}
//#endregion 🌊️Gestures

//#region ⏪️TimeTravel
/// ⏪️ One `historyEdit*` verb from the Blueprint window, refused verbs failing the law.
async fn history_edit(app: &mut LayoutApp, verb: &str, args: Value) {
    let args = DslValue::from(&args);
    let result = app.0.handle_action(verb, Some(&args), &window_meta(WINDOW)).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
    assert!(result.output.get("rejected").is_none(), "{verb} was refused: {:?}", result.output);
}

async fn time_travel(app: &mut LayoutApp) -> Option<semio_framework::kernel::HistoryTimeTravel> {
    app.0.history_snapshot().await.expect("history").time_travel
}

/// ⏳️ Drives the session's replay job until `done` holds for its stage (absent = time travel closed).
async fn pump(app: &mut LayoutApp, done: impl Fn(Option<semio_framework::kernel::HistoryTimeTravelStage>) -> bool) {
    for _ in 0..10_000 {
        if done(time_travel(app).await.map(|status| status.stage)) {
            return;
        }
        app.0.advance_typed_operation_publication().await.expect("a driver turn");
        while app.0.take_typed_operation_ui_progress().is_some() {}
    }
    panic!("the history edit never settled: {:?}", time_travel(app).await);
}

/// 🧾️ The history rows that carry a document mutation, oldest first.
async fn document_rows(app: &mut LayoutApp) -> Vec<HistoryEntry> {
    let mut rows: Vec<_> = app.0.history_snapshot().await.expect("history").upserts.into_iter().filter(|entry| entry.edit_id.is_some() && !entry.mutations.is_empty()).collect();
    rows.sort_by_key(|entry| entry.seq);
    rows
}

/// ✋️ One gumball drag of `frame-1` by `(dx, 0)`, streamed and committed — one row.
async fn gumball_drag(app: &mut LayoutApp, dx: f64) {
    run(app, stream(dx, 0.0)).await;
    assert_eq!(edit_rows(&run(app, phase("commit", None)).await).len(), 1, "the drag is one row");
}

/// 🧮️ The fresh fold of a log of frame-selection leaves on the demo document.
fn folded(log: &[LayoutMutation]) -> LayoutSnapshot {
    use protocol::{Mutation, MutationDiff};
    log.iter().fold(crate::standards::v1::subsets::any::schema::default_document(), |document, mutation| mutation.diff(&document).diff().apply(&document).expect("the leaf applies"))
}

fn drag_leaf(dx: f64) -> LayoutMutation {
    LayoutMutation::DragFrames(crate::mutations::drag_frames::DragFrames { page_id: "page-1".into(), targets: vec!["frame-1".into()], dx, dy: 0.0 })
}

fn turn_leaf(pivot_x: f64, pivot_y: f64) -> LayoutMutation {
    LayoutMutation::RotateFrames(crate::mutations::rotate_frames::RotateFrames { page_id: "page-1".into(), targets: vec!["frame-1".into()], pivot_x, pivot_y, angle: std::f64::consts::FRAC_PI_2 })
}

/// ⏪️ Time travel edits the gesture's yielded leaf, never the gesture: the gumball drag's offset and then the downstream
/// turn's pivot are edited through the real `historyEdit*` verbs. Reviewing never touches the committed document; every
/// overwrite replays the downstream turn onto the edited frame and the head equals the fresh fold of the edited log;
/// both edits keep their mutation ids and rows.
#[semio_framework_async_macros::async_test]
async fn a_gumball_drag_and_its_downstream_turn_edited_in_time_travel_replay_exactly() {
    let mut app = layout_app_with_registry().await;
    let rows_before = document_rows(&mut app).await.len();
    gumball_drag(&mut app, 10.0).await;
    run(&mut app, LayoutCommand::RotateSelection(rotate_selection::RotateSelection { ids: vec!["frame-1".into()], angle: std::f64::consts::FRAC_PI_2, phase: None, reason: None })).await;
    let rows = document_rows(&mut app).await;
    assert_eq!(rows.len(), rows_before + 2, "two gestures, two rows");
    assert!(rows[rows_before..].iter().all(|row| row.transaction.is_some() && row.mutations.iter().all(|mutation| mutation.editable)), "both rows are tool transactions of editable leaves");
    assert_eq!(app.0.snapshot().expect("head"), folded(&[drag_leaf(10.0), turn_leaf(40.0, 30.0)]), "the turn recorded the centroid pivot of the dragged frame");
    let (drag, turn) = (rows[rows_before].mutations[0].mutation_id.clone(), rows[rows_before + 1].mutations[0].mutation_id.clone());
    let committed = app.0.snapshot().expect("committed head");

    history_edit(&mut app, "historyEditBegin", serde_json::json!({ "mutationId": drag })).await;
    history_edit(&mut app, "historyEditInput", serde_json::json!({ "path": "/dx", "value": -5.0 })).await;
    history_edit(&mut app, "historyEditAccept", serde_json::json!({})).await;
    pump(&mut app, |stage| stage != Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;
    let status = time_travel(&mut app).await.expect("a live session");
    assert_eq!((status.stage, status.blocking), (semio_framework::kernel::HistoryTimeTravelStage::Reviewing, false), "a clean replay reviews: {status:?}");
    assert_eq!(app.0.snapshot().expect("committed head"), committed, "reviewing never touches the committed document");
    history_edit(&mut app, "historyEditFinalize", serde_json::json!({})).await;
    history_edit(&mut app, "historyEditCommit", serde_json::json!({ "choice": "overwrite" })).await;
    pump(&mut app, |stage| stage.is_none()).await;
    assert_eq!(app.0.snapshot().expect("edited head"), folded(&[drag_leaf(-5.0), turn_leaf(40.0, 30.0)]), "the downstream turn replays about its recorded pivot onto the edited drag");

    history_edit(&mut app, "historyEditBegin", serde_json::json!({ "mutationId": turn })).await;
    history_edit(&mut app, "historyEditInput", serde_json::json!({ "path": "/pivotX", "value": 0.0 })).await;
    history_edit(&mut app, "historyEditAccept", serde_json::json!({})).await;
    pump(&mut app, |stage| stage != Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;
    history_edit(&mut app, "historyEditFinalize", serde_json::json!({})).await;
    history_edit(&mut app, "historyEditCommit", serde_json::json!({ "choice": "overwrite" })).await;
    pump(&mut app, |stage| stage.is_none()).await;
    assert_eq!(app.0.snapshot().expect("edited head"), folded(&[drag_leaf(-5.0), turn_leaf(0.0, 30.0)]), "an edited pivot replays exactly");
    let ids: Vec<_> = document_rows(&mut app).await[rows_before..].iter().map(|row| row.mutations[0].mutation_id.clone()).collect();
    assert_eq!(ids, vec![drag, turn], "editing never re-mints the gestures' mutations or rows");
    artifact_app_laws::close_registered_fixture_app(&mut app.0);
}

/// 🎯️ Selects `ids` in the elements domain, as a canvas click does — interaction verbs keep working during time travel.
async fn select(app: &mut LayoutApp, ids: &[&str]) {
    let ids: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
    let args = layout_select_action_args(&ids, "replace");
    let admitted = app.0.handle_action(INTERACTION_SELECT_ACTION_ID, Some(&args), &window_meta(WINDOW)).await.unwrap_or_else(|fault| panic!("the selection is admitted: {fault:?}"));
    let result = semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted).await;
    settle(app, result).await;
}

/// 🚨️ The fatal-resolution loop: a drag whose targets are edited to a frame that does not exist reports
/// `mutation.target-missing` (Error) and blocks finalizing; re-targeting it through "use selection" (the `targets`
/// input's `Reference` reads the elements selection) clears the block, and Exit discards the whole session with zero
/// trace.
#[semio_framework_async_macros::async_test]
async fn a_drag_edited_onto_a_missing_frame_blocks_finalize_until_its_targets_are_fixed() {
    let mut app = layout_app_with_registry().await;
    let rows_before = document_rows(&mut app).await.len();
    gumball_drag(&mut app, 10.0).await;
    let drag = document_rows(&mut app).await[rows_before].mutations[0].mutation_id.clone();
    let committed = app.0.snapshot().expect("committed head");
    history_edit(&mut app, "historyEditBegin", serde_json::json!({ "mutationId": drag })).await;
    history_edit(&mut app, "historyEditInput", serde_json::json!({ "path": "/targets", "value": ["frame-missing"] })).await;
    history_edit(&mut app, "historyEditAccept", serde_json::json!({})).await;
    pump(&mut app, |stage| stage != Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;
    let status = time_travel(&mut app).await.expect("a live session");
    assert!(status.blocking && status.worst == Some(semio_framework::kernel::Severity::Error), "a missing target is an Error that blocks finalizing: {status:?}");
    let rows = document_rows(&mut app).await;
    let outcome = &rows[rows_before].mutations[0];
    assert!(outcome.messages.iter().any(|message| message.code == "mutation.target-missing"), "the drag reports target-missing: {outcome:?}");
    select(&mut app, &["frame-text-1"]).await;
    history_edit(&mut app, "historyEditBegin", serde_json::json!({ "mutationId": drag })).await;
    history_edit(&mut app, "historyEditUseSelection", serde_json::json!({ "path": "/targets" })).await;
    history_edit(&mut app, "historyEditAccept", serde_json::json!({})).await;
    pump(&mut app, |stage| stage != Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;
    let status = time_travel(&mut app).await.expect("a live session");
    assert!(!status.blocking, "the selection's frame clears the block: {status:?}");
    history_edit(&mut app, "historyEditExit", serde_json::json!({})).await;
    pump(&mut app, |stage| stage.is_none()).await;
    assert_eq!(app.0.snapshot().expect("head"), committed, "exit leaves zero trace");
    assert_eq!(document_rows(&mut app).await.len(), rows_before + 1, "no row was added or removed");
    artifact_app_laws::close_registered_fixture_app(&mut app.0);
}

/// 🪧️ The editor's reference chips name a frame by its kind, its content and its page in every locale — what "Use
/// selection" fills into a frame-target input reads as the frame, never its id (gap N3); other kinds and unknown ids are
/// left to the framework's generic chip.
#[test]
fn reference_chips_name_frames_by_kind_content_and_page() {
    use semio_framework_ui_locale::Locale;
    use semio_framework_ui_locale::Terminology;
    let document = crate::standards::v1::subsets::any::schema::default_document();
    let frame = ["frame".to_string()];
    let chip = |kinds: &[String], id: &str, locale| <LayoutPlayApp as ArtifactEditor>::entity_label(&document, kinds, id).map(|label| label.resolve(Terminology::Native, locale).to_string());
    assert_eq!(chip(&frame, "frame-text-1", Locale::En).as_deref(), Some("Text Frame \u{201c}Hello layout\u{201d} \u{b7} Page 1"));
    assert_eq!(chip(&frame, "frame-text-1", Locale::De).as_deref(), Some("Textrahmen \u{201e}Hello layout\u{201c} \u{b7} Page 1"));
    assert_eq!(chip(&frame, "frame-image-1", Locale::En).as_deref(), Some("Image Frame \u{201c}missing.png\u{201d} \u{b7} Page 1"));
    assert_eq!(chip(&frame, "frame-1", Locale::De).as_deref(), Some("Rechteck \u{b7} Page 1"));
    assert_eq!(chip(&[], "frame-inherited", Locale::En).as_deref(), Some("Rectangle \u{b7} Master"));
    assert_eq!(chip(&frame, "frame-missing", Locale::En), None);
    assert_eq!(chip(&["page".to_string()], "page-1", Locale::En), None, "a page names itself through the generic walk");
    let mut crowded = document.clone();
    let twin = crowded.pages[0].frames.iter().find(|candidate| candidate.id() == "frame-1").cloned().map(|mut twin| {
        if let Frame::Rect { id, .. } = &mut twin {
            *id = "frame-2".into();
        }
        twin
    });
    crowded.pages[0].frames.extend(twin);
    let crowded_chip = |id: &str| layout_entity_label(&crowded, &frame, id).map(|label| label.resolve(Terminology::Native, Locale::En).to_string());
    assert_eq!((crowded_chip("frame-1").as_deref(), crowded_chip("frame-2").as_deref()), (Some("Rectangle 1 \u{b7} Page 1"), Some("Rectangle 2 \u{b7} Page 1")), "several frames of one kind on a page are numbered");
}
//#endregion ⏪️TimeTravel
