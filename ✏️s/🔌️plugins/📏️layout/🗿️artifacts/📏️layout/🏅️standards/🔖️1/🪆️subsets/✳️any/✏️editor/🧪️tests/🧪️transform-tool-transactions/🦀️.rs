//! 🛠️ Laws for the transform tool's ONE promise, through the real registry-backed app and its retained command route:
//! one gumball gesture is one `ToolTransaction` — one edit, one history row stamped with its `TransactionRef`, one
//! parametric leaf carrying the literal targets and the net parameters — streamed ticks live in the Blueprint window's
//! open transaction (previewed by that window, never history), and a cancelled gesture leaves zero trace.

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
/// recorded centroid pivot, labelled from the leaf.
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
