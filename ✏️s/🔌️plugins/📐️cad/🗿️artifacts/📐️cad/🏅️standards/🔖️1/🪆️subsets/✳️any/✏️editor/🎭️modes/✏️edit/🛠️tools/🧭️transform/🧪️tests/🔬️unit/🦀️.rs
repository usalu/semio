//! 🧭️ Laws for the transform tool's ONE promise: one object gesture — a gumball translate, rotate or scale, an
//! inspector origin delta, the commit of a `spatial.interaction` — is one `ToolTransaction`: one document edit whose
//! every op carries the gesture's `TransactionRef`, parametric leaves carrying the literal targets and parameters, and
//! zero trace when nothing moves. The leaves stay editable: a history edit of the drag's offset replays onto any base.

use super::*;
use crate::editor::cad::engine::interaction::apply_event;
use crate::editor::cad::{start_interaction_session, try_commit_session_entries, CadPlayRuntime};
use crate::sample_scene_fixture::{materialized_objects, materialized_shape_scene, sample_object};
use protocol::{Mutation, MutationDiff};
use semio_framework_plugin::{AppOperationContext, HistoryView};

fn base() -> CadSnapshot {
    materialized_shape_scene(vec![sample_object("object-a", [0.0, 0.0, 0.0]), sample_object("object-b", [4.0, 0.0, 0.0])])
}

/// 🏢️ `base` plus a building pane materializing `object-c`, so one selection spans two panes.
fn two_pane_base() -> CadSnapshot {
    let mut scene = base();
    let working = crate::CadWorkingScene { building_objects: vec![sample_object("object-c", [1.0, 1.0, 0.0])], ..Default::default() };
    scene.building_model = crate::cad_pane_rematerialized_child(&working, CadPaneId::Building, vec![sample_object("object-c", [1.0, 1.0, 0.0])]);
    scene
}

fn operation(authoring_seed: &str) -> AppOperationContext {
    AppOperationContext { app_instance_id: 1, parent_document_id: "cad-test-document".into(), operation_id: 1, generation: 1, canonical_base_revision: [0; 32], authoring_seed: authoring_seed.into() }
}

fn ids(values: &[&str]) -> Vec<String> {
    values.iter().map(|id| id.to_string()).collect()
}

fn origin(document: &CadSnapshot, pane: CadPaneId, id: &str) -> [f64; 3] {
    materialized_objects(document, pane).into_iter().find(|object| object.id == id).expect("object survives").origin
}

fn apply(document: &CadSnapshot, mutations: &[CadMutation]) -> CadSnapshot {
    mutations.iter().fold(document.clone(), |state, mutation| mutation.diff(&state).diff().apply(&state).expect("leaf applies"))
}

/// ▶️ A translate under command authority is ONE transaction: one parametric leaf, stamped with a ref the tool minted
/// as `<appId>#translateSelection`, and no coalesce key — one gesture, one edit, one history row.
#[test]
fn a_translate_is_one_transaction_with_its_parametric_leaf() {
    let base = base();
    let history = HistoryView::empty();
    let doc = ArtifactView::with_operation(&base, &history, operation("seed-a"));
    let emit = cad_transform_tool_emit(&doc, "translateSelection", vec![CadToolEntry::Transform(CadTransformRecord::drag(ids(&["object-a", "object-b", "object-a"]), [1.5, -2.0, 0.5]))]);
    let transaction = emit.transaction.as_ref().expect("a committed gesture carries its TransactionRef");
    assert!(transaction.id.starts_with("tx-"), "{transaction:?}");
    assert_eq!(transaction.tool, "s.cad.cad@1/*#editor#translateSelection");
    assert_eq!(emit.artifact_mutations, vec![CadMutation::DragSelection(DragSelection { pane: CadPaneId::Shape, targets: ids(&["object-a", "object-b"]), offset: [1.5, -2.0, 0.5] })], "one leaf, targets deduplicated in first-seen order");
    let after = apply(&base, &emit.artifact_mutations);
    assert_eq!(origin(&after, CadPaneId::Shape, "object-b"), [5.5, -2.0, 0.5]);
}

/// 🗂️ A selection spanning two panes yields one leaf per pane, both in the SAME transaction.
#[test]
fn a_two_pane_selection_is_one_transaction_with_a_leaf_per_pane() {
    let base = two_pane_base();
    let (transaction, mutations) = cad_transform_tool_commit("rotateSelection", "seed-b", &base, vec![CadToolEntry::Transform(CadTransformRecord::rotate(ids(&["object-c", "object-a"]), [0.0, 0.0, 1.0], 0.5))]).expect("the turn commits");
    assert_eq!(transaction.tool, "s.cad.cad@1/*#editor#rotateSelection");
    assert_eq!(
        mutations,
        vec![
            CadMutation::RotateSelection(RotateSelection { pane: CadPaneId::Shape, targets: ids(&["object-a"]), axis: [0.0, 0.0, 1.0], angle: 0.5 }),
            CadMutation::RotateSelection(RotateSelection { pane: CadPaneId::Building, targets: ids(&["object-c"]), axis: [0.0, 0.0, 1.0], angle: 0.5 }),
        ]
    );
}

/// 👻️ A stranger id, an identity motion and an empty request leave zero trace: no leaf, no transaction.
#[test]
fn nothing_to_move_leaves_zero_trace() {
    let base = base();
    let history = HistoryView::empty();
    let doc = ArtifactView::with_operation(&base, &history, operation("seed-c"));
    for entries in [
        vec![CadToolEntry::Transform(CadTransformRecord::drag(ids(&["ghost"]), [1.0, 0.0, 0.0]))],
        vec![CadToolEntry::Transform(CadTransformRecord::drag(ids(&["object-a"]), [0.0, 0.0, 0.0]))],
        vec![CadToolEntry::Transform(CadTransformRecord::scale(ids(&["object-a"]), [1.0, 1.0, 1.0]))],
        vec![CadToolEntry::Transform(CadTransformRecord::rotate(ids(&["object-a"]), [0.0, 0.0, 0.0], 1.0))],
        Vec::new(),
    ] {
        let emit = cad_transform_tool_emit(&doc, "translateSelection", entries.clone());
        assert!(emit.artifact_mutations.is_empty() && emit.transaction.is_none(), "{entries:?} must leave zero trace, got {:?}", emit.artifact_mutations);
    }
}

/// 🔀️ Two gestures are two transactions: each admission's seed mints its own ref.
#[test]
fn two_gestures_are_two_transactions() {
    let base = base();
    let entries = || vec![CadToolEntry::Transform(CadTransformRecord::drag(ids(&["object-a"]), [1.0, 0.0, 0.0]))];
    let (first, _) = cad_transform_tool_commit("translateSelection", "seed-first", &base, entries()).expect("first commits");
    let (second, _) = cad_transform_tool_commit("translateSelection", "seed-second", &base, entries()).expect("second commits");
    assert_ne!(first.id, second.id, "two admissions never share a transaction");
}

/// 🪪️ A view without command authority publishes the yielded leaves plainly — no ref to stamp.
#[test]
fn a_seedless_view_publishes_the_leaves_plainly() {
    let base = base();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&base, &history);
    let emit = cad_transform_tool_emit(&doc, "scaleSelection", vec![CadToolEntry::Transform(CadTransformRecord::scale(ids(&["object-a"]), [2.0, 2.0, 2.0]))]);
    assert!(emit.transaction.is_none());
    assert_eq!(emit.artifact_mutations.len(), 1);
}

/// ⏪️ History edits the yielded leaf, never the tool: the drag's offset, rewritten through the leaf's own payload
/// value (what time travel's `historyEditInput` does), replays onto the BASE of the gesture — and onto a base where
/// the object already sits elsewhere, relative to there.
#[test]
fn an_edited_drag_replays_relative_to_its_base() {
    let base = base();
    let (_, mutations) = cad_transform_tool_commit("translateSelection", "seed-d", &base, vec![CadToolEntry::Transform(CadTransformRecord::drag(ids(&["object-a"]), [1.0, 0.0, 0.0]))]).expect("drag commits");
    let leaf = &mutations[0];
    let mut payload = serde_json::Value::from(leaf.payload_value());
    payload["offset"] = serde_json::json!([5.0, 0.0, 0.0]);
    let edited = leaf.with_payload_value(semio_framework_value::DslValue::from(payload)).expect("the edited offset decodes as the same leaf");
    assert!(matches!(&edited, CadMutation::DragSelection(drag) if drag.offset == [5.0, 0.0, 0.0]), "{edited:?}");
    assert_eq!(origin(&apply(&base, std::slice::from_ref(&edited)), CadPaneId::Shape, "object-a"), [5.0, 0.0, 0.0]);
    let elsewhere = materialized_shape_scene(vec![sample_object("object-a", [10.0, 0.0, 0.0])]);
    assert_eq!(origin(&apply(&elsewhere, std::slice::from_ref(&edited)), CadPaneId::Shape, "object-a"), [15.0, 0.0, 0.0]);
}

/// 🕹️ The commit of a `transform.move` interaction is the transform tool's parametric drag, never an absolute pose.
#[test]
fn a_move_interaction_commits_a_parametric_drag() {
    let document = base();
    let mut runtime = CadPlayRuntime::default();
    assert!(start_interaction_session(&mut runtime, CadPaneId::Shape, "transform.move"));
    let session = runtime.engagement_session.as_mut().expect("the move session starts");
    let targets = semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::object([("id".to_string(), semio_framework_value::DslValue::String("object-a".into())), ("kind".to_string(), semio_framework_value::DslValue::String("object".into()))])]);
    assert!(apply_event(session, "selection.changed", Some(&semio_framework_value::DslValue::object([("targets".to_string(), targets)]))));
    assert!(apply_event(session, "confirm", None));
    let point = |x: f64, y: f64| semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::float(x), semio_framework_value::DslValue::float(y), semio_framework_value::DslValue::float(0.0)]);
    assert!(apply_event(session, "pointer.down", Some(&point(0.0, 0.0))));
    assert!(apply_event(session, "pointer.down", Some(&point(3.0, 4.0))));
    let snapshot = session.clone();
    let entries = try_commit_session_entries(&document, &mut runtime, CadPaneId::Shape, &snapshot);
    assert_eq!(entries, vec![CadToolEntry::Transform(CadTransformRecord::drag(ids(&["object-a"]), [3.0, 4.0, 0.0]))]);
    let (transaction, mutations) = cad_transform_tool_commit("transform.move", "seed-e", &document, entries).expect("the interaction commits");
    assert_eq!(transaction.tool, "s.cad.cad@1/*#editor#transform.move");
    assert_eq!(mutations, vec![CadMutation::DragSelection(DragSelection { pane: CadPaneId::Shape, targets: ids(&["object-a"]), offset: [3.0, 4.0, 0.0] })]);
    assert!(runtime.engagement_session.is_none(), "the committed session is closed");
}

/// 🧾️ Drives one mounted dispatch to quiescence exactly as the host does and answers the history rows its completions
/// upserted — the framework fixture's own settle loop, keeping the `HistoryPatch` it would drop.
async fn dispatch_rows(app: &mut crate::editor::cad::unit_tests::context::CadFixtureApp, command: crate::editor::cad::CadCommand) -> Vec<semio_framework::kernel::HistoryEntry> {
    use semio_framework_plugin::PluginApp;
    let meta = crate::editor::cad::unit_tests::context::meta("local");
    app.dispatch_typed(command, &meta).await.expect("the command is admitted");
    let mut rows = Vec::new();
    for _ in 0..1_048_576 {
        if !app.has_pending_typed_operations() {
            break;
        }
        PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("maintenance step");
        app.advance_typed_operation_publication().await.expect("publication");
        while let Some(page) = app.take_typed_operation_result_page(crate::editor::cad::unit_tests::context::TEST_INSTANCE) {
            assert_ne!(page.lane, semio_framework_plugin::app::TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
            assert!(app.acknowledge_typed_operation_result(page.token).expect("acknowledge"));
        }
        while app.take_typed_operation_effect().is_some() {}
        while app.take_typed_operation_event().is_some() {}
        while app.take_typed_operation_ui_scope().is_some() {}
        while let Some(completion) = app.take_typed_operation_completion().await.expect("completion") {
            rows.extend(completion.history_patch.into_iter().flat_map(|patch| patch.upserts));
        }
    }
    rows
}

/// 🪟️ The full runtime: a gumball translate of a placed box is ONE history row, keyed by its tool transaction and
/// labelled from its leaf in English and German — never the op text, never one row per tick.
#[semio_framework_async_macros::async_test]
async fn a_mounted_translate_is_one_history_row_keyed_by_its_transaction() {
    use crate::editor::cad::commands::object::add_object::AddObject;
    use crate::editor::cad::commands::transform::translate_selection::TranslateSelection;
    let mut app = crate::editor::cad::unit_tests::context::new_app().await;
    dispatch_rows(&mut app, crate::editor::cad::CadCommand::AddObject(AddObject { typology: Some("spatial.shape.primitive.box".into()) })).await;
    let placed = crate::editor::cad::cad_pane_objects(&app.snapshot().expect("snapshot"), CadPaneId::Shape).last().expect("the box is placed").id.clone();
    let rows: Vec<_> = dispatch_rows(&mut app, crate::editor::cad::CadCommand::TranslateSelection(TranslateSelection { object_ids: vec![placed.clone()], dx: 1.0, dy: 0.0, dz: 0.0 })).await.into_iter().filter(|row| row.applied && !row.op_lines.is_empty()).collect();
    assert_eq!(rows.len(), 1, "one gesture, one row: {rows:?}");
    let transaction = rows[0].transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.cad.cad@1/*#editor#translateSelection", "{transaction:?}");
    assert!(rows[0].op_lines.iter().any(|line| line.starts_with("drag-selection")), "the op is the parametric leaf: {:?}", rows[0].op_lines);
    assert_eq!(rows[0].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Drag 1 object by (1, 0, 0)");
    assert_eq!(rows[0].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "1 Objekt um (1; 0; 0) ziehen");
    assert_eq!(origin(&app.snapshot().expect("snapshot"), CadPaneId::Shape, &placed)[0], 1.0, "the leaf moved the box");
    crate::editor::cad::unit_tests::context::close(&mut app);
}

/// 🎚️ One streamed gumball dispatch of the press `gesture` (design §13.1, §17.4): the cumulative offset from the press, the
/// release adding `commit`, a cancel naming its `abort` reason — settled like the host settles it.
async fn stream_translate(app: &mut crate::editor::cad::unit_tests::context::CadFixtureApp, target: &str, gesture: &str, dx: f64, phase: Option<(&str, semio_framework_value::DslValue)>) {
    use semio_framework_plugin::PluginApp;
    let mut args = vec![
        ("objectIds".to_string(), semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::String(target.into())])),
        ("dx".to_string(), semio_framework_value::DslValue::float(dx)),
        ("dy".to_string(), semio_framework_value::DslValue::float(0.0)),
        ("dz".to_string(), semio_framework_value::DslValue::float(0.0)),
        ("gesture".to_string(), semio_framework_value::DslValue::String(gesture.into())),
    ];
    args.extend(phase.map(|(key, value)| (key.to_string(), value)));
    app.handle_action("translateSelection", Some(&semio_framework_value::DslValue::Object(args)), &crate::editor::cad::unit_tests::context::meta("local")).await.expect("the streamed translate is admitted");
    let _ = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, crate::editor::cad::unit_tests::context::TEST_INSTANCE).await;
}

/// 🪟️ The streamed gumball (design §17.4): every tick of one press stays provisional — the render reads committed ⊕ the
/// press's leaf while the document and the history stay untouched — the release lands ONE `drag-selection` edit of the
/// release's cumulative offset stamped with the press's transaction, and a press the host cancels leaves zero trace.
#[semio_framework_async_macros::async_test]
async fn a_streamed_gumball_press_is_one_transaction_and_a_cancel_leaves_zero_trace() {
    use crate::editor::cad::commands::object::add_object::AddObject;
    let mut app = crate::editor::cad::unit_tests::context::new_app().await;
    dispatch_rows(&mut app, crate::editor::cad::CadCommand::AddObject(AddObject { typology: Some("spatial.shape.primitive.box".into()) })).await;
    let placed = crate::editor::cad::cad_pane_objects(&app.snapshot().expect("snapshot"), CadPaneId::Shape).last().expect("the box is placed").id.clone();
    let edits = app.edit_transactions().len();
    stream_translate(&mut app, &placed, "gumball:1", 0.5, None).await;
    stream_translate(&mut app, &placed, "gumball:1", 1.25, None).await;
    assert_eq!(app.edit_transactions().len(), edits, "ticks publish no edit");
    assert_eq!(origin(&app.snapshot().expect("snapshot"), CadPaneId::Shape, &placed)[0], 0.0, "the committed box never moves during the press");
    assert_eq!(origin(&app.rendered_snapshot(), CadPaneId::Shape, &placed)[0], 1.25, "the render previews the press's cumulative offset");
    stream_translate(&mut app, &placed, "gumball:1", 2.0, Some(("commit", semio_framework_value::DslValue::Bool(true)))).await;
    let transactions = app.edit_transactions();
    assert_eq!(transactions.len(), edits + 1, "the release is ONE edit");
    assert!(transactions.last().and_then(Option::as_ref).is_some_and(|transaction| transaction.tool == "s.cad.cad@1/*#editor#translateSelection"), "{transactions:?}");
    assert_eq!(origin(&app.snapshot().expect("snapshot"), CadPaneId::Shape, &placed)[0], 2.0, "the release lands the cumulative offset");
    stream_translate(&mut app, &placed, "gumball:2", 5.0, None).await;
    stream_translate(&mut app, &placed, "gumball:2", 0.0, Some(("abort", semio_framework_value::DslValue::String("captureLost".into())))).await;
    assert_eq!(app.edit_transactions().len(), edits + 1, "a cancelled press leaves zero trace");
    assert_eq!(origin(&app.rendered_snapshot(), CadPaneId::Shape, &placed)[0], 2.0, "the cancelled preview is gone");
    crate::editor::cad::unit_tests::context::close(&mut app);
}
