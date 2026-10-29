#![cfg(test)]

//! 🛠️ Laws for the transform tool's ONE promise, through the real registry-backed app and its retained command route:
//! one gumball gesture is one `ToolTransaction` — one edit, one history row stamped with its `TransactionRef`, one
//! parametric leaf carrying the literal targets and the net parameters — streamed ticks live in the Blueprint window's
//! open transaction (previewed by that window, never history), and a cancelled gesture leaves zero trace. Time travel edits
//! the yielded leaf's inputs (offset, pivot, targets) through the real `historyEdit*` verbs and replays the downstream.

extern crate semio_framework_os_kernel as protocol;

use protocol::{ColdMutation as _, ColdMutationDiff as _, ColdDiffAlgebra as _};
use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::LAYOUT_PLAY_BODY_BLUEPRINT;
use semio_s_artifact_layout_layout_editor::{LayoutCommand, LayoutPlayApp, layout_entity_label};
use semio_s_artifact_layout_layout_editor_commands::layout_select_action_args;
use semio_s_artifact_layout_layout_editor::unit_tests::context::{layout_app_with_registry, scene, window_meta, LayoutApp, INSTANCE, WINDOW};
use semio_framework::kernel::HistoryEntry;
use semio_framework_plugin::{artifact_app_laws, ArtifactEditor, DslValue, Fault, InvocationResult, PluginApp, INTERACTION_SELECT_ACTION_ID};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_s_artifact_layout_layout_model::Frame;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use serde_json::Value;

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
async fn settle<E: std::fmt::Debug>(app: &mut LayoutApp, result: Result<InvocationResult, E>) -> InvocationResult {
    let mut result = result.unwrap_or_else(|fault| panic!("the dispatch is admitted: {fault:?}"));
    let mut fault = None;
    for _ in 0..1_048_576 {
        if !app.0.has_pending_typed_operations() && app.0.maintenance_terminal_is_empty() {
            drain(app, &mut result, &mut fault).await.expect("drain");
            assert!(fault.is_none(), "the operation faulted: {fault:?}");
            return result;
        }
        PluginApp::maintenance_step(&mut app.0, app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("maintenance");
        app.0.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("publication");
        drain(app, &mut result, &mut fault).await.expect("drain");
    }
    panic!("the layout operation did not settle")
}

async fn run(app: &mut LayoutApp, command: LayoutCommand) -> InvocationResult {
    let result = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &command, &window_meta(WINDOW), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    settle(app, result).await
}

/// 🎟️ One framework verb through the borrowed action endpoint under the app's original caller, its pending turns driven to the admitted result.
async fn act(app: &mut LayoutApp, action: &str, args: Option<DslValue>) -> Result<InvocationResult, semio_framework_job::OriginalTurnFailure> {
    artifact_app_laws::dispatch_fixture_action(&mut app.0, action, args, &window_meta(WINDOW), &mut artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await
}

/// 🕰️ A framework-reserved verb (`undo`, `hostEvent`, …) from the Blueprint window.
async fn reserved(app: &mut LayoutApp, action: &str, args: Option<Value>) -> InvocationResult {
    let args = args.map(|value| DslValue::from(&value));
    let admitted = act(app, action, args).await.unwrap_or_else(|fault| panic!("{action} is admitted: {fault:?}"));
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
    LayoutCommand::TranslateSelection(semio_s_artifact_layout_layout_editor_commands::gumball::TranslateSelection { ids: vec!["frame-1".into()], dx, dy, phase: Some("stream".into()), reason: None })
}

fn phase(phase: &str, reason: Option<&str>) -> LayoutCommand {
    LayoutCommand::TranslateSelection(semio_s_artifact_layout_layout_editor_commands::gumball::TranslateSelection { ids: vec!["frame-1".into()], dx: 0.0, dy: 0.0, phase: Some(phase.into()), reason: reason.map(str::to_string) })
}

fn label(entry: &HistoryEntry, locale: semio_framework_ui_locale::Locale) -> String {
    entry.label.resolve(semio_framework_ui_locale::Terminology::Native, locale).to_string()
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
    assert_eq!(label(&rows[0], semio_framework_ui_locale::Locale::En), "Drag 1 frame by (30, -6)");
    assert_eq!(label(&rows[0], semio_framework_ui_locale::Locale::De), "1 Rahmen um (30; -6) ziehen");
    assert_eq!(origin(&app, "frame-1"), (before.0 + 30.0, before.1 - 6.0));
    assert_eq!(painted(&mut app, "frame-1").await, (before.0 + 30.0, before.1 - 6.0), "the window paints the committed document");
    reserved(&mut app, "undo", None).await;
    assert_eq!(origin(&app, "frame-1"), before, "one undo takes the whole gesture back");
    app.close_fixture();
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
    app.close_fixture();
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
    app.close_fixture();
}

/// 🔃️ One-shot turn and scaling verbs (palette, agent) are each one transaction yielding their parametric leaf about the
/// recorded centroid pivot, labelled from the leaf — for one frame and for several, whose inverse restores two absolute
/// rows per frame inside the leaf's declared fold footprint; one undo takes each back exactly.
#[semio_framework_async_macros::async_test]
async fn one_shot_turns_and_scalings_are_one_transaction_each() {
    let mut app = layout_app_with_registry().await;
    let turned = run(&mut app, LayoutCommand::RotateSelection(semio_s_artifact_layout_layout_editor_commands::gumball::RotateSelection { ids: vec!["frame-1".into()], angle: std::f64::consts::FRAC_PI_2, phase: None, reason: None })).await;
    let rows = edit_rows(&turned);
    assert_eq!(rows.len(), 1);
    assert!(rows[0].transaction.as_ref().is_some_and(|transaction| transaction.tool.ends_with("#rotateSelection")));
    assert_eq!(label(&rows[0], semio_framework_ui_locale::Locale::En), "Rotate 1 frame by 90°");
    let scaled = run(&mut app, LayoutCommand::ScaleSelection(semio_s_artifact_layout_layout_editor_commands::gumball::ScaleSelection { ids: vec!["frame-1".into()], sx: 2.0, sy: 2.0, phase: None, reason: None })).await;
    let rows = edit_rows(&scaled);
    assert_eq!(rows.len(), 1);
    assert_eq!(label(&rows[0], semio_framework_ui_locale::Locale::De), "1 Rahmen um (2; 2) skalieren");
    let before = app.0.snapshot().expect("the head before the multi-frame transforms");
    let pair = vec!["frame-1".to_string(), "frame-text-1".to_string()];
    let turned = run(&mut app, LayoutCommand::RotateSelection(semio_s_artifact_layout_layout_editor_commands::gumball::RotateSelection { ids: pair.clone(), angle: std::f64::consts::FRAC_PI_2, phase: None, reason: None })).await;
    assert_eq!(edit_rows(&turned).iter().map(|row| label(row, semio_framework_ui_locale::Locale::En)).collect::<Vec<_>>(), vec!["Rotate 2 frames by 90°".to_string()], "a two-frame turn is one row");
    let scaled = run(&mut app, LayoutCommand::ScaleSelection(semio_s_artifact_layout_layout_editor_commands::gumball::ScaleSelection { ids: pair, sx: 0.5, sy: 3.0, phase: None, reason: None })).await;
    assert_eq!(edit_rows(&scaled).iter().map(|row| label(row, semio_framework_ui_locale::Locale::De)).collect::<Vec<_>>(), vec!["2 Rahmen um (0,5; 3) skalieren".to_string()], "a two-frame scaling is one row");
    assert_ne!(app.0.snapshot().expect("the transformed head"), before);
    reserved(&mut app, "undo", None).await;
    reserved(&mut app, "undo", None).await;
    assert_eq!(app.0.snapshot().expect("the undone head"), before, "two undos restore both frames exactly through their multi-row inverses");
    app.close_fixture();
}

/// 🧲️ A streamed gumball drag of two frames is one transaction holding ONE `drag-frames` leaf over both targets — one row,
/// both frames moved by the net offset, and one undo restores both.
#[semio_framework_async_macros::async_test]
async fn a_streamed_drag_of_two_frames_is_one_transaction() {
    let mut app = layout_app_with_registry().await;
    let before = (origin(&app, "frame-1"), origin(&app, "frame-text-1"));
    let tick = |dx: f64, phase: &str| LayoutCommand::TranslateSelection(semio_s_artifact_layout_layout_editor_commands::gumball::TranslateSelection { ids: vec!["frame-1".into(), "frame-text-1".into()], dx, dy: 1.0, phase: Some(phase.into()), reason: None });
    run(&mut app, tick(4.0, "stream")).await;
    run(&mut app, tick(4.0, "stream")).await;
    let rows = edit_rows(&run(&mut app, tick(2.0, "commit")).await);
    assert_eq!(rows.iter().map(|row| label(row, semio_framework_ui_locale::Locale::En)).collect::<Vec<_>>(), vec!["Drag 2 frames by (10, 3)".to_string()], "one gesture, one row, the net leaf");
    assert!(rows[0].transaction.is_some(), "the row is the gesture's transaction");
    assert_eq!((origin(&app, "frame-1"), origin(&app, "frame-text-1")), ((before.0 .0 + 10.0, before.0 .1 + 3.0), (before.1 .0 + 10.0, before.1 .1 + 3.0)));
    reserved(&mut app, "undo", None).await;
    assert_eq!((origin(&app, "frame-1"), origin(&app, "frame-text-1")), before, "one undo restores both frames");
    app.close_fixture();
}

/// 📐️ A document edit landing under an open gesture aborts it (`baseMoved`): the commit that finds it publishes
/// nothing, and only the other edit is history.
#[semio_framework_async_macros::async_test]
async fn a_document_moved_under_an_open_gesture_aborts_it() {
    let mut app = layout_app_with_registry().await;
    let before = origin(&app, "frame-1");
    run(&mut app, stream(20.0, 0.0)).await;
    let other = run(&mut app, LayoutCommand::PatchFrame(semio_s_artifact_layout_layout_editor_commands::patch_frame::PatchFrame { frame_id: "frame-text-1".into(), page_id: Some("page-1".into()), field: "y".into(), value: "77".into() })).await;
    assert_eq!(edit_rows(&other).len(), 1, "the other edit lands");
    let late = run(&mut app, phase("commit", None)).await;
    assert!(edit_rows(&late).is_empty(), "the gesture opened on the old base publishes nothing");
    assert_eq!(origin(&app, "frame-1"), before);
    assert_eq!(origin(&app, "frame-text-1").1, 77.0);
    app.close_fixture();
}
/// 🪢️ A real mounted follower retains its original caller across preview, cancelled drain and paid close.
#[semio_framework_async_macros::async_test]
async fn mounted_original_follower_settles_and_closes_on_the_same_cancelled_source() {
    let row: Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let mut app = layout_app_with_registry().await;
    let before = origin(&app, "frame-1");
    let dx = row["tick"][0].as_f64().unwrap();
    let dy = row["tick"][1].as_f64().unwrap();
    let result = run(&mut app, stream(dx, dy)).await;
    assert_eq!(edit_rows(&result).len(), row["historyRows"].as_u64().unwrap() as usize);
    assert!(app.0.maintenance_terminal_is_empty());
    assert!(app.0.maintenance_source_terminal_is_empty());
    let committed=origin(&app,"frame-1");
    assert_eq!((committed.0-before.0,committed.1-before.1),(row["committed"][0].as_f64().unwrap(),row["committed"][1].as_f64().unwrap()));
    let preview = painted(&mut app, "frame-1").await;
    assert_eq!((preview.0-before.0,preview.1-before.1),(row["preview"][0].as_f64().unwrap(),row["preview"][1].as_f64().unwrap()));
    let aborted = run(&mut app, phase("abort", Some("captureLost"))).await;
    assert_zero_trace(&mut app, &aborted, before, "original follower abort").await;
    let mut result = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &stream(dx,dy), &window_meta(WINDOW), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap();
    let policy = semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
    let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items:1, ..policy.maintenance };
    let mut fault=None;
    for _ in 0..1_048_576 {
        if !app.0.maintenance_source_terminal_is_empty() { break; }
        app.0.maintenance_step(app.1.cx(),grant).unwrap();
        app.0.advance_typed_operation_publication(&mut artifact_app_laws::fixture_identity(),app.1.cx()).await.unwrap();
        drain(&mut app,&mut result,&mut fault).await.unwrap();
    }
    assert!(fault.is_none());
    assert!(!app.0.maintenance_source_terminal_is_empty(), "the actual follower must be retained before cancellation");
    let source=app.1.cx().original_recipient_identity();
    let decoder=app.1.cx().native_receiving_identity();
    assert_ne!(source,0);
    assert!(decoder.is_some());
    let operation=app.1.cx().operation();
    let generation=app.1.cx().generation();
    let before_receipt=app.1.cx().retained_progress();
    let denied=app.0.maintenance_source_close_step(app.1.cx(),Default::default()).unwrap();
    assert_eq!(denied.progress().unwrap_or_default(),Default::default());
    assert_eq!(app.1.cx().retained_progress(),before_receipt);
    assert!(!app.0.maintenance_source_terminal_is_empty());
    app.1.cx().original_source_parent().unwrap().cancellation().unwrap().cancel_now();
    assert!(app.1.cx().is_cancelled());
    for _ in 0..1_048_576 {
        if app.0.maintenance_source_terminal_is_empty() { break; }
        app.1.cx().with_original_retirement(|same|app.0.maintenance_source_close_step(same,policy.close).map_err(|failure|failure.into())).unwrap();
        assert_eq!(app.1.cx().original_recipient_identity(),source);
        assert_eq!(app.1.cx().native_receiving_identity(),decoder);
        assert_eq!(app.1.cx().operation(),operation);
        assert_eq!(app.1.cx().generation(),generation);
    }
    assert_eq!(app.0.maintenance_source_terminal_is_empty(),row["sourceClosed"].as_bool().unwrap());
    let cancelled=origin(&app,"frame-1");
    assert_eq!(cancelled,before);
    app.1.cx().with_original_retirement(|same|artifact_app_laws::close_registered_fixture_app(&mut app.0,policy,same).map_err(|failure|failure.into())).unwrap();
    assert!(app.0.close_terminal_is_empty());
    println!("original-follower-lifecycle={}",serde_json::json!({"preview":[preview.0-before.0,preview.1-before.1],"committed":[committed.0-before.0,committed.1-before.1],"cancelled":[cancelled.0-before.0,cancelled.1-before.1],"historyRows":edit_rows(&result).len(),"sourceClosed":app.0.maintenance_source_terminal_is_empty()}));
}

//#endregion 🌊️Gestures

//#region ⏪️TimeTravel
/// ⏪️ One `historyEdit*` verb from the Blueprint window, refused verbs failing the law.
async fn history_edit(app: &mut LayoutApp, verb: &str, args: Value) {
    let args = DslValue::from(&args);
    let result = act(app, verb, Some(args)).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
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
        app.0.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("a driver turn");
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
    use protocol::{ColdMutation, Mutation, MutationDiff};
    log.iter().fold(semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document(), |document, mutation| protocol::apply_diff_cold(mutation.diff(&document).diff(), &document).expect("the leaf applies"))
}

fn drag_leaf(dx: f64) -> LayoutMutation {
    LayoutMutation::DragFrames(semio_s_artifact_layout_layout_mutations_frames::drag_frames::DragFrames { page_id: "page-1".into(), targets: vec!["frame-1".into()], dx, dy: 0.0 })
}

fn turn_leaf(pivot_x: f64, pivot_y: f64) -> LayoutMutation {
    LayoutMutation::RotateFrames(semio_s_artifact_layout_layout_mutations_frames::rotate_frames::RotateFrames { page_id: "page-1".into(), targets: vec!["frame-1".into()], pivot_x, pivot_y, angle: std::f64::consts::FRAC_PI_2 })
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
    run(&mut app, LayoutCommand::RotateSelection(semio_s_artifact_layout_layout_editor_commands::gumball::RotateSelection { ids: vec!["frame-1".into()], angle: std::f64::consts::FRAC_PI_2, phase: None, reason: None })).await;
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
    app.close_fixture();
}

/// 🎯️ Selects `ids` in the elements domain, as a canvas click does — interaction verbs keep working during time travel.
async fn select(app: &mut LayoutApp, ids: &[&str]) {
    let ids: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
    let args = layout_select_action_args(&ids, "replace");
    let admitted = act(app, INTERACTION_SELECT_ACTION_ID, Some(args)).await.unwrap_or_else(|fault| panic!("the selection is admitted: {fault:?}"));
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
    app.close_fixture();
}

/// 🪧️ The editor's reference chips name a frame by its kind, its content and its page in every locale — what "Use
/// selection" fills into a frame-target input reads as the frame, never its id (gap N3); other kinds and unknown ids are
/// left to the framework's generic chip.
#[test]
fn reference_chips_name_frames_by_kind_content_and_page() {
    use semio_framework_ui_locale::Locale;
    use semio_framework_ui_locale::Terminology;
    let document = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
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

/// 🧺️ The public original settlement retains partial lanes and restores commit order into the same funded backings.
#[test]
fn original_press_settlement_preserves_lane_backings_under_denial_commit_and_cancellation(){
    use semio_framework_plugin_tool_run::tool_jobs_tool_machine::{OriginalPressSettlement,PressLeaf,ScrubTag};
    use semio_framework_tool_machine::{ScrubLedger,ScrubPhase,OriginalScrubLedgerClose};
    use semio_framework_value::retirement::controlled::ControlledRetirement;
    use semio_framework_job::close_original_retained_step;
    type A=semio_framework_plugin::EditorApp<LayoutPlayApp>;
    type L=PressLeaf<LayoutMutation,semio_framework_plugin::NoConfigMutation>;
    let fixture:Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let row=&fixture["settlement"];
    for cancel in [false,true]{
        let mut caller=artifact_app_laws::OriginalFixtureScope::admit();
        let mut ledger=ScrubLedger::<L>::default();
        let mut emit=semio_framework_plugin::Emit::<LayoutMutation,semio_framework_plugin::NoConfigMutation,semio_framework_plugin::NoDraftMutation>::mutations(row["members"].as_array().unwrap().iter().map(|name|LayoutMutation::RenameLayout(semio_s_artifact_layout_layout_mutations_content::rename_layout::RenameLayout{new_name:name.as_str().unwrap().into()})).collect());
        emit.child_emits.reserve_exact(2);emit.config_mutations.reserve_exact(2);emit.window_config_mutations.reserve_exact(2);
        let pointers=[emit.artifact_mutations.as_ptr()as usize,emit.child_emits.as_ptr()as usize,emit.config_mutations.as_ptr()as usize,emit.window_config_mutations.as_ptr()as usize];
        let mut tag=Some(ScrubTag{window:WINDOW.into(),tool:"s.layout.layout#renameLayout".into(),phase:ScrubPhase::Commit{gesture:"fixture-press".into()}});
        let base=[7u8;32];let actor="fixture-actor";let mut owner=None;
        let denied=caller.cx().with_retained_loan(Default::default(),|same|OriginalPressSettlement::<A>::admit(&base,actor,&mut emit,0,||(tag.take().unwrap(),semio_framework_tool_machine::authoring_clock(1)),same,&mut owner)).unwrap();
        assert!(!denied);assert!(tag.is_some());assert!(owner.is_none());assert_eq!(emit.artifact_mutations.len(),2);
        assert!(OriginalPressSettlement::<A>::admit(&base,actor,&mut emit,0,||(tag.take().unwrap(),semio_framework_tool_machine::authoring_clock(1)),caller.cx(),&mut owner).unwrap());
        assert!(tag.is_none());let before=caller.cx().retained_progress();
        caller.cx().with_retained_loan(Default::default(),|same|owner.as_mut().unwrap().advance(&mut ledger,&base,actor,&mut emit,same)).unwrap();assert_eq!(caller.cx().retained_progress(),before);assert_eq!(emit.artifact_mutations.len(),2);
        for _ in 0..1_048_576{if emit.artifact_mutations.len()==1{break}let turn=semio_framework_value::RetainedCloneGrant{maximum_items:row["turnItems"].as_u64().unwrap()as usize,..caller.cx().retained_grant()};caller.cx().with_retained_loan(turn,|same|owner.as_mut().unwrap().advance(&mut ledger,&base,actor,&mut emit,same)).unwrap()}
        assert_eq!(emit.artifact_mutations.len(),1,"accepted source must be genuinely partially transferred");
        if cancel{caller.cx().original_source_parent().unwrap().cancellation().unwrap().cancel_now();assert!(caller.cx().is_cancelled());for _ in 0..1_048_576{if owner.as_ref().unwrap().terminal_is_empty(){break}caller.cx().with_original_retirement(|same|owner.as_mut().unwrap().close_step(&mut ledger,same)).unwrap();}}
        else{for _ in 0..1_048_576{if owner.as_ref().unwrap().terminal_is_empty(){break}let turn=semio_framework_value::RetainedCloneGrant{maximum_items:row["turnItems"].as_u64().unwrap()as usize,..caller.cx().retained_grant()};caller.cx().with_retained_loan(turn,|same|owner.as_mut().unwrap().advance(&mut ledger,&base,actor,&mut emit,same)).unwrap();}}
        assert!(owner.as_ref().unwrap().terminal_is_empty());assert_eq!(owner.as_ref().unwrap().result(),!cancel);
        assert_eq!([emit.artifact_mutations.as_ptr()as usize,emit.child_emits.as_ptr()as usize,emit.config_mutations.as_ptr()as usize,emit.window_config_mutations.as_ptr()as usize],pointers);
        let actual:Vec<_>=emit.artifact_mutations.iter().map(|mutation|match mutation{LayoutMutation::RenameLayout(value)=>value.new_name.as_str(),_=>panic!("actual source leaf changed kind")}).collect();
        assert_eq!(serde_json::to_value(&actual).unwrap(),row[if cancel{"cancelRemaining"}else{"committed"}]);assert_eq!(emit.transaction.is_some(),!cancel);
        println!("original-press-settlement={}",serde_json::json!({"cancelled":cancel,"members":actual,"restored":emit.transaction.is_some(),"ownerClosed":owner.as_ref().unwrap().terminal_is_empty()}));
        let demand=owner.as_ref().unwrap().shell_close_demands();caller.cx().with_original_retirement(|same|close_original_retained_step(same,demand,|_|{owner.take();Ok(semio_framework_value::RetainedCloneStep::Complete(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))})).unwrap();assert!(owner.is_none());
        let mut close=None;for _ in 0..1_048_576{if close.is_some(){break}caller.cx().with_original_retirement(|same|OriginalScrubLedgerClose::admit(&mut ledger,&mut close,same)).unwrap();}assert!(close.is_some());
        for _ in 0..1_048_576{if close.as_ref().unwrap().terminal_is_empty(){break}caller.cx().with_original_retirement(|same|close.as_mut().unwrap().close_step(&mut ledger,same)).unwrap();}assert!(close.as_ref().unwrap().terminal_is_empty());let demand=close.as_ref().unwrap().shell_close_demands();caller.cx().with_original_retirement(|same|close_original_retained_step(same,demand,|_|{close.take();Ok(semio_framework_value::RetainedCloneStep::Complete(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))})).unwrap();
        let mut emitted=ControlledRetirement::retain(emit);for _ in 0..1_048_576{if emitted.terminal_is_empty(){break}let demand=emitted.next_demand(caller.cx().retained_grant().maximum_copy_bytes).unwrap();caller.cx().with_original_retirement(|same|close_original_retained_step(same,demand,|grant|emitted.step(grant))).unwrap();}assert!(emitted.terminal_is_empty());caller.retire();
    }
}

/// 🏷️ Original tag metadata keeps UTF-8 and its actual source loan through denial and partial cancellation.
#[test]
fn original_tool_tag_capture_preserves_exact_metadata_and_closes_partial_text(){
 use semio_framework_plugin_tool_run::tool_jobs_tool_machine::{OriginalToolTagCapture,OriginalToolTagSource,OriginalToolTagKind,OriginalToolWindow,ToolTag};
 use semio_framework_tool_machine::{ScrubPhase,ToolAbortReason};
 use semio_framework_job::close_original_retained_step;
 use semio_framework_value::retirement::controlled::ControlledRetirement;
 let fixture:Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in fixture["tagCases"].as_array().unwrap(){for cancel in [false,true]{
  let mut caller=artifact_app_laws::OriginalFixtureScope::admit();let mut owner=None;let mut tag=None;
  let kind=match row["kind"].as_str().unwrap(){"tick"=>OriginalToolTagKind::ScrubTick,"commit"=>OriginalToolTagKind::ScrubCommit,"abort"=>OriginalToolTagKind::ScrubAbort(ToolAbortReason::parse(row["reason"].as_str().unwrap()).unwrap()),_=>OriginalToolTagKind::TypingEdit};
  let source=OriginalToolTagSource{window:match row["window"].as_str(){Some(value)=>OriginalToolWindow::Window(value),None=>OriginalToolWindow::Global},app:row["app"].as_str().unwrap(),action:row["action"].as_str().unwrap(),value:row["value"].as_str().unwrap(),kind};
  assert!(!caller.cx().with_retained_loan(Default::default(),|same|OriginalToolTagCapture::admit(source,same,&mut owner)).unwrap());assert!(owner.is_none());assert!(tag.is_none());
  assert!(OriginalToolTagCapture::admit(source,caller.cx(),&mut owner).unwrap());let before=caller.cx().retained_progress();assert!(!caller.cx().with_retained_loan(Default::default(),|same|owner.as_mut().unwrap().advance(source,&mut tag,same)).unwrap());assert_eq!(caller.cx().retained_progress(),before);assert_eq!(owner.as_ref().unwrap().captured_bytes(),0);
  for _ in 0..1_048_576{if owner.as_ref().unwrap().captured_bytes()>0{break}owner.as_mut().unwrap().advance(source,&mut tag,caller.cx()).unwrap();}assert!(owner.as_ref().unwrap().captured_bytes()>0);assert!(tag.is_none());
  if cancel{caller.cx().original_source_parent().unwrap().cancellation().unwrap().cancel_now();assert!(caller.cx().is_cancelled());}
  else{for _ in 0..1_048_576{if owner.as_ref().unwrap().complete(){break}owner.as_mut().unwrap().advance(source,&mut tag,caller.cx()).unwrap();}assert!(owner.as_ref().unwrap().complete());let actual=match tag.as_ref().unwrap(){ToolTag::Scrub(value)=>{let phase=match value.phase{ScrubPhase::Tick{..}=>"tick",ScrubPhase::Commit{..}=>"commit",ScrubPhase::Abort{..}=>"abort"};serde_json::json!({"window":value.window,"tool":value.tool,"value":value.phase.gesture(),"kind":phase})},ToolTag::Typing(value)=>serde_json::json!({"window":value.window,"tool":value.tool,"value":value.buffer,"kind":"typing"})};let expected=serde_json::json!({"window":row["window"].as_str().unwrap_or(""),"tool":format!("{}#{}",source.app,source.action),"value":row["value"],"kind":row["kind"]});assert_eq!(actual,expected);println!("original-tool-tag={actual}");}
  for _ in 0..1_048_576{if owner.as_ref().unwrap().terminal_is_empty(){break}caller.cx().with_original_retirement(|same|owner.as_mut().unwrap().close_step(same)).unwrap();}assert!(owner.as_ref().unwrap().terminal_is_empty());let demand=owner.as_ref().unwrap().shell_close_demands();caller.cx().with_original_retirement(|same|close_original_retained_step(same,demand,|_|{owner.take();Ok(semio_framework_value::RetainedCloneStep::Complete(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))})).unwrap();
  if !cancel{let mut returned=None;let operation=caller.cx().operation().0;assert!(!caller.cx().with_retained_loan(Default::default(),|same|semio_framework_plugin_scrub_host::OriginalToolTagRetirement::admit_owned(operation,&mut tag,same,&mut returned)).unwrap());assert!(tag.is_some());assert!(returned.is_none());assert!(semio_framework_plugin_scrub_host::OriginalToolTagRetirement::admit_owned(operation,&mut tag,caller.cx(),&mut returned).unwrap());assert!(tag.is_none());for _ in 0..1_048_576{if returned.as_ref().unwrap().terminal_is_empty(){break}caller.cx().with_original_retirement(|same|returned.as_mut().unwrap().close_step(same)).unwrap();}assert!(returned.as_ref().unwrap().terminal_is_empty());let demand=returned.as_ref().unwrap().shell_close_demands();caller.cx().with_original_retirement(|same|close_original_retained_step(same,demand,|_|{returned.take();Ok(semio_framework_value::RetainedCloneStep::Complete(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))})).unwrap();}
  let mut delivered=ControlledRetirement::retain(tag);for _ in 0..1_048_576{if delivered.terminal_is_empty(){break}let demand=delivered.next_demand(caller.cx().retained_grant().maximum_copy_bytes).unwrap();caller.cx().with_original_retirement(|same|close_original_retained_step(same,demand,|grant|delivered.step(grant))).unwrap();}assert!(delivered.terminal_is_empty());caller.retire();
 }}
}

/// ⏳️ Expired semantic copying returns the exact original metadata through its unchanged financed source.
#[test]
fn original_metadata_copy_restores_after_expiry_without_renewing_clock_or_stock(){
 use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,close_original_retained_step};
 use semio_framework_plugin_decl_emission::{ActionMeta,OriginalActionMetaCopy};
 use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
 use semio_framework_value::retirement::controlled::ControlledRetirement;
 use std::sync::atomic::{AtomicU64,Ordering};
 static CLOCK:AtomicU64=AtomicU64::new(0);fn now()->Option<u64>{Some(CLOCK.load(Ordering::SeqCst))}
 let fixture:Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let row=&fixture["metadataExpiry"];
 let policy:Value=serde_json::from_str(include_str!("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();let g=&policy["grant"];
 let grant=RetainedCloneGrant{maximum_items:g["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:g["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:g["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:g["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:g["maximumDepth"].as_u64().unwrap()as usize};let deadline=policy["deadlineUs"].as_u64().unwrap();
 CLOCK.store(policy["clockUs"].as_u64().unwrap(),Ordering::SeqCst);let mut caller=OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(policy["operation"].as_u64().unwrap()),semio_framework_job::Generation(policy["generation"].as_u64().unwrap()),grant,now,deadline,policy["fuel"].as_u64().unwrap());
 let mut decoder=OriginalJobDecodeOwner::empty();let mut owner=None;let mut metadata=Some(ActionMeta{actor:row["actor"].as_str().unwrap().into(),instance_id:row["instance"].as_u64().unwrap()as u32,view_state:None});let pointer=metadata.as_ref().unwrap().actor.as_ptr();let clock=caller.original_clock_binding();
 caller.turn_with_decoder(grant,&mut decoder,|cx|OriginalActionMetaCopy::admit(cx,&mut owner)).unwrap();assert!(owner.is_some());
 caller.turn_with_decoder(grant,&mut decoder,|cx|owner.as_mut().unwrap().advance(&mut metadata,cx)).unwrap();assert!(metadata.is_none(),"the exact original must occupy its admitted copy source before expiry");
 CLOCK.store(row["expiredClockUs"].as_u64().unwrap(),Ordering::SeqCst);let before=caller.cumulative_receipt();
 caller.turn_with_decoder(grant,&mut decoder,|cx|{assert!(cx.should_yield());assert_eq!(cx.deadline_us(),deadline);assert_eq!(cx.original_clock_binding(),Some(clock));owner.as_mut().unwrap().advance(&mut metadata,cx)}).unwrap();assert_eq!(caller.cumulative_receipt(),before);assert!(metadata.is_none());
 for _ in 0..32768{let done=caller.turn_with_decoder(grant,&mut decoder,|cx|{assert!(cx.should_yield());assert_eq!(cx.deadline_us(),deadline);assert_eq!(cx.original_clock_binding(),Some(clock));cx.with_original_retirement(|same|owner.as_mut().unwrap().close_step(&mut metadata,same))}).unwrap();if done{break}}
 assert!(owner.as_ref().unwrap().terminal_is_empty());assert_eq!(metadata.as_ref().unwrap().actor.as_ptr(),pointer);assert_eq!(metadata.as_ref().unwrap().actor,row["actor"].as_str().unwrap());
 println!("original-metadata-expiry={}",serde_json::json!({"semanticBlocked":true,"restored":metadata.is_some(),"sameActorPointer":metadata.as_ref().unwrap().actor.as_ptr()==pointer,"closed":owner.as_ref().unwrap().terminal_is_empty()}));
 let mut demand=OriginalActionMetaCopy::release_demand();demand.copy_bytes=std::mem::size_of_val(&owner);caller.turn_with_decoder(grant,&mut decoder,|cx|cx.with_original_retirement(|same|close_original_retained_step(same,demand,|_|{owner.take();Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))}))).unwrap();
 let mut restored=ControlledRetirement::retain(metadata);for _ in 0..32768{if restored.terminal_is_empty(){break}caller.turn_with_decoder(grant,&mut decoder,|cx|{let demand=restored.next_demand(cx.retained_grant().maximum_copy_bytes)?;cx.with_original_retirement(|same|close_original_retained_step(same,demand,|g|restored.step(g)))}).unwrap();}assert!(restored.terminal_is_empty());
 for _ in 0..32768{if decoder.terminal_is_empty(){break}caller.turn(grant,|cx|{let demand=decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;cx.with_original_retirement(|same|close_original_retained_step(same,demand,|g|decoder.close_step(g)))}).unwrap();}assert!(decoder.terminal_is_empty());caller.retire(grant);
}

/// 🧶️ The genuine typing dispatcher retains one metadata owner until paid restoration or cancellation.
#[test]
fn original_typing_dispatch_retains_metadata_until_atomic_finish_or_cancel(){
 use semio_framework_plugin_instance_typed_ops::original_typing::OriginalTypingDispatch;
 use semio_framework_plugin_decl_emission::ActionMeta;
 use semio_framework_tool_machine::TypingLedger;
 use semio_framework_job::close_original_retained_step;
 use semio_framework_value::retirement::controlled::ControlledRetirement;
 type A=semio_framework_plugin::EditorApp<LayoutPlayApp>;
 let fixture:Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let row=&fixture["typingMetadata"];
 for cancel in [false,true]{
  let mut caller=artifact_app_laws::OriginalFixtureScope::admit();let ledger=TypingLedger::<LayoutMutation>::default();let mut owner=OriginalTypingDispatch::<A>::default();
  let mut metadata=Some(ActionMeta{actor:row["actor"].as_str().unwrap().into(),instance_id:row["instance"].as_u64().unwrap()as u32,view_state:None});let pointer=metadata.as_ref().unwrap().actor.as_ptr();let mut ready=false;let mut decision=None;
  assert!(!caller.cx().with_retained_loan(Default::default(),|same|owner.admit(&ledger,true,true,&mut metadata,same)).unwrap());assert!(!owner.pending());assert_eq!(metadata.as_ref().unwrap().actor.as_ptr(),pointer);
  assert!(owner.admit(&ledger,true,true,&mut metadata,caller.cx()).unwrap());assert!(metadata.is_none());assert_eq!(owner.original_metadata_for_testing().unwrap().actor.as_ptr(),pointer);
  let before=caller.cx().retained_progress();assert!(!caller.cx().with_retained_loan(Default::default(),|same|owner.capture_clock(||row["clockMs"].as_u64().unwrap(),same)).unwrap());assert_eq!(caller.cx().retained_progress(),before);
  assert!(owner.capture_clock(||row["clockMs"].as_u64().unwrap(),caller.cx()).unwrap());for _ in 0..32768{if owner.admit_parent(caller.cx()).unwrap(){break}}
  assert!(owner.complete_empty_ledger(&ledger,caller.cx()).unwrap());assert!(owner.complete());assert!(metadata.is_none());assert!(!ready);assert!(decision.is_none());
  let mut replacement=Some(ActionMeta{actor:row["replacement"].as_str().unwrap().into(),instance_id:row["instance"].as_u64().unwrap()as u32,view_state:None});let replacement_pointer=replacement.as_ref().unwrap().actor.as_ptr();let before=caller.cx().retained_progress();
  assert!(owner.finish_into(&mut ready,&mut decision,&mut replacement,caller.cx()).is_err());assert_eq!(caller.cx().retained_progress(),before);assert_eq!(replacement.as_ref().unwrap().actor.as_ptr(),replacement_pointer);assert_eq!(owner.original_metadata_for_testing().unwrap().actor.as_ptr(),pointer);
  if cancel{caller.cx().original_source_parent().unwrap().cancellation().unwrap().cancel_now();for _ in 0..32768{if owner.terminal_is_empty(){break}caller.cx().with_original_retirement(|same|owner.close_step(same)).unwrap();}assert!(!ready);assert!(decision.is_none());assert!(metadata.is_none());}
  else{let before=caller.cx().retained_progress();assert!(!caller.cx().with_retained_loan(Default::default(),|same|owner.finish_into(&mut ready,&mut decision,&mut metadata,same)).unwrap());assert_eq!(caller.cx().retained_progress(),before);assert!(!ready);assert!(metadata.is_none());for _ in 0..32768{if owner.finish_into(&mut ready,&mut decision,&mut metadata,caller.cx()).unwrap(){break}}assert!(ready);assert_eq!(decision,Some(true));assert_eq!(metadata.as_ref().unwrap().actor.as_ptr(),pointer);}
  assert!(owner.terminal_is_empty());assert!(ledger.is_empty());let record=serde_json::json!({"cancelled":cancel,"externalAbsentWhilePending":true,"replacementRefused":true,"restored":metadata.is_some(),"sameActorPointer":metadata.as_ref().is_some_and(|value|value.actor.as_ptr()==pointer),"ready":ready,"suppressed":decision==Some(true),"closed":owner.terminal_is_empty()});println!("original-typing-metadata={record}");
  let mut delivered=ControlledRetirement::retain((metadata,replacement));for _ in 0..32768{if delivered.terminal_is_empty(){break}let demand=delivered.next_demand(caller.cx().retained_grant().maximum_copy_bytes).unwrap();caller.cx().with_original_retirement(|same|close_original_retained_step(same,demand,|grant|delivered.step(grant))).unwrap();}assert!(delivered.terminal_is_empty());caller.retire();
 }
}

/// 🚧️ Direct intents qualify the actual declared action before typing control can suppress its verb.
#[semio_framework_async_macros::async_test]
async fn direct_intent_typing_commit_requires_original_catalog_qualification(){
 use semio_framework_job::close_original_retained_step;
 use semio_framework_value::retirement::controlled::ControlledRetirement;
 let fixture:Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let law=&fixture["directIntentQualification"];
 for row in law["cases"].as_array().unwrap(){
  let mut app=layout_app_with_registry().await;let before=app.0.snapshot().unwrap();
  let intent:semio_framework_ui_contract::UiIntent=serde_json::from_value(serde_json::json!({"surface":"qualification-law","revision":1,"node":1,"nodeKey":"typing-control","trigger":"commit","action":{"scope":law["scope"],"name":row["action"],"version":1},"seq":1})).unwrap();
  let mut args=Some(DslValue::from(&serde_json::json!({"typing":law["typing"],"typingCommit":law["typingCommit"]})));let args_pointer=args.as_ref().unwrap()as*const _;let mut meta=Some(window_meta(WINDOW));let mut fault=None;let mut resolved=None;let mut rejected=false;
  let receipt=app.1.cx().retained_progress();let denied=app.1.cx().with_retained_loan(Default::default(),|same|semio_framework_async::poll::resolve_ready(app.0.handle_intent_frame(&intent,&mut args,&mut fault,&mut meta,&mut artifact_app_laws::fixture_identity(),same))).unwrap();assert!(denied.is_none());assert_eq!(app.1.cx().retained_progress(),receipt);assert_eq!(args.as_ref().unwrap()as*const _,args_pointer);
  for _ in 0..1_048_576{match app.0.handle_intent_frame(&intent,&mut args,&mut fault,&mut meta,&mut artifact_app_laws::fixture_identity(),app.1.cx()).await{Ok(Some(result))=>{resolved=Some(result);break},Ok(None)=>{},Err(_)=>{rejected=true;break}}}
  assert_eq!(rejected,!row["qualified"].as_bool().unwrap());assert_eq!(resolved.is_some(),row["qualified"].as_bool().unwrap());assert_eq!(app.0.snapshot().unwrap(),before);
  if let Some(result)=resolved{let result=settle(&mut app,Ok::<_,Fault>(result)).await;assert!(edit_rows(&result).is_empty());}
  let policy=semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
  for _ in 0..1_048_576{if app.0.maintenance_source_terminal_is_empty(){break}app.1.cx().with_original_retirement(|same|app.0.maintenance_source_close_step(same,policy.close).map_err(|failure|failure.into())).unwrap();}assert!(app.0.maintenance_source_terminal_is_empty());
  println!("direct-intent-qualification={}",serde_json::json!({"action":row["action"],"qualified":!rejected,"suppressed":row["qualified"],"unchanged":app.0.snapshot().unwrap()==before,"closed":app.0.maintenance_source_terminal_is_empty()}));
  let mut sources=ControlledRetirement::retain((args,meta,fault));for _ in 0..1_048_576{if sources.terminal_is_empty(){break}let d=sources.next_demand(app.1.cx().retained_grant().maximum_copy_bytes).unwrap();app.1.cx().with_original_retirement(|same|close_original_retained_step(same,d,|g|sources.step(g))).unwrap();}assert!(sources.terminal_is_empty());app.close_fixture();
 }
}

/// 🪦️ A cancelled direct qualification returns its original catalog lease before any typing control runs.
#[semio_framework_async_macros::async_test]
async fn direct_intent_cancelled_qualification_closes_before_typing_control(){
 use semio_framework_job::close_original_retained_step;
 use semio_framework_value::retirement::controlled::ControlledRetirement;
 let fixture:Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let law=&fixture["directIntentQualification"];let row=&law["cases"][1];let mut app=layout_app_with_registry().await;let before=app.0.snapshot().unwrap();
 let intent:semio_framework_ui_contract::UiIntent=serde_json::from_value(serde_json::json!({"surface":"qualification-law","revision":1,"node":1,"nodeKey":"typing-control","trigger":"commit","action":{"scope":law["scope"],"name":row["action"],"version":1},"seq":1})).unwrap();
 let mut args=Some(DslValue::from(&serde_json::json!({"typing":law["typing"],"typingCommit":law["typingCommit"]})));let mut meta=Some(window_meta(WINDOW));let pointer=meta.as_ref().unwrap().actor.as_ptr();let mut fault=None;
 for _ in 0..3{assert!(app.0.handle_intent_frame(&intent,&mut args,&mut fault,&mut meta,&mut artifact_app_laws::fixture_identity(),app.1.cx()).await.unwrap().is_none());}
 assert!(!app.0.maintenance_source_terminal_is_empty());assert_eq!(meta.as_ref().unwrap().actor.as_ptr(),pointer);assert!(args.is_some());
 app.1.cx().original_source_parent().unwrap().cancellation().unwrap().cancel_now();let policy=semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
 for _ in 0..1_048_576{if app.0.maintenance_source_terminal_is_empty(){break}app.1.cx().with_original_retirement(|same|app.0.maintenance_source_close_step(same,policy.close).map_err(|failure|failure.into())).unwrap();}assert!(app.0.maintenance_source_terminal_is_empty());assert_eq!(app.0.snapshot().unwrap(),before);assert_eq!(meta.as_ref().unwrap().actor.as_ptr(),pointer);
 println!("direct-intent-qualification-cancel={}",serde_json::json!({"unchanged":true,"metadataRetained":true,"argsRetained":args.is_some(),"closed":app.0.maintenance_source_terminal_is_empty()}));
 let mut sources=ControlledRetirement::retain((args,meta,fault));for _ in 0..1_048_576{if sources.terminal_is_empty(){break}let d=sources.next_demand(app.1.cx().retained_grant().maximum_copy_bytes).unwrap();app.1.cx().with_original_retirement(|same|close_original_retained_step(same,d,|g|sources.step(g))).unwrap();}assert!(sources.terminal_is_empty());app.1.cx().with_original_retirement(|same|artifact_app_laws::close_registered_fixture_app(&mut app.0,policy,same).map_err(|failure|failure.into())).unwrap();assert!(app.0.close_terminal_is_empty());
}

/// 🔏️ Rewriting the same inline intent-name bytes cannot reuse an already qualified typing control.
#[semio_framework_async_macros::async_test]
async fn direct_intent_qualified_action_rewrite_is_refused_without_control_effects(){
 use semio_framework_job::close_original_retained_step;
 use semio_framework_value::retirement::controlled::ControlledRetirement;
 let fixture:Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let law=&fixture["directIntentQualification"];let mut app=layout_app_with_registry().await;let before=app.0.snapshot().unwrap();
 let mut intent:semio_framework_ui_contract::UiIntent=serde_json::from_value(serde_json::json!({"surface":"qualification-law","revision":1,"node":1,"nodeKey":"typing-control","trigger":"commit","action":{"scope":law["scope"],"name":law["cases"][1]["action"],"version":1},"seq":1})).unwrap();
 let mut args=Some(DslValue::from(&serde_json::json!({"typing":law["typing"],"typingCommit":law["typingCommit"]})));let mut meta=Some(window_meta(WINDOW));let mut fault=None;
 for _ in 0..1_048_576{if meta.is_none(){break}assert!(app.0.handle_intent_frame(&intent,&mut args,&mut fault,&mut meta,&mut artifact_app_laws::fixture_identity(),app.1.cx()).await.unwrap().is_none());}
 assert!(meta.is_none(),"control must be pending after actual catalog qualification");let pointer=intent.action.name.as_str().as_ptr();let length=intent.action.name.as_str().len();intent.action.name=semio_framework_ui_contract::UiText::try_from_str(law["changedAction"].as_str().unwrap()).unwrap();assert_eq!(intent.action.name.as_str().as_ptr(),pointer);assert_eq!(intent.action.name.as_str().len(),length);
 let receipt=app.1.cx().retained_progress();assert!(app.0.handle_intent_frame(&intent,&mut args,&mut fault,&mut meta,&mut artifact_app_laws::fixture_identity(),app.1.cx()).await.is_err());assert_eq!(app.1.cx().retained_progress(),receipt);assert!(meta.is_none());assert_eq!(app.0.snapshot().unwrap(),before);
 app.1.cx().original_source_parent().unwrap().cancellation().unwrap().cancel_now();let policy=semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
 for _ in 0..1_048_576{if app.0.maintenance_source_terminal_is_empty(){break}app.1.cx().with_original_retirement(|same|app.0.maintenance_source_close_step(same,policy.close).map_err(|failure|failure.into())).unwrap();}assert!(app.0.maintenance_source_terminal_is_empty());
 println!("direct-intent-qualification-rewrite={}",serde_json::json!({"samePointer":true,"sameLength":true,"refused":true,"unchanged":app.0.snapshot().unwrap()==before,"closed":app.0.maintenance_source_terminal_is_empty()}));
 let mut sources=ControlledRetirement::retain((args,meta,fault));for _ in 0..1_048_576{if sources.terminal_is_empty(){break}let d=sources.next_demand(app.1.cx().retained_grant().maximum_copy_bytes).unwrap();app.1.cx().with_original_retirement(|same|close_original_retained_step(same,d,|g|sources.step(g))).unwrap();}assert!(sources.terminal_is_empty());app.1.cx().with_original_retirement(|same|artifact_app_laws::close_registered_fixture_app(&mut app.0,policy,same).map_err(|failure|failure.into())).unwrap();assert!(app.0.close_terminal_is_empty());
}
