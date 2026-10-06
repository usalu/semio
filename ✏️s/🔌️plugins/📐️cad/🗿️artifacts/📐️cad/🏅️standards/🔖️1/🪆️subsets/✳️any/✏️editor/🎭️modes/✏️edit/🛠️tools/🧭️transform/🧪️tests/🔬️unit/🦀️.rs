//! 🧭️ Laws for the transform tool's ONE promise: one object gesture — a gumball translate, rotate or scale, an
//! inspector origin delta, the commit of a `spatial.interaction` — is one `ToolTransaction` of child-lane leaves on the
//! panes' composed `s.stdio.semio@v1/model` children (design §12, §20.15): one relative leaf per touched pane carrying the
//! literal targets and parameters, every member edit stamped with the gesture's `TransactionRef`, and zero trace when
//! nothing moves. The leaves stay editable: a history edit of the drag's offset replays onto any base.

use super::*;
use crate::editor::cad::engine::interaction::apply_event;
use crate::editor::cad::unit_tests::context::{close, member_rows, meta, new_app, pane_objects, settle, CadFixtureApp, TEST_INSTANCE};
use crate::editor::cad::{start_interaction_session, try_commit_session_entries, CadCommand, CadPlayRuntime};
use crate::sample_scene_fixture::sample_object;
use crate::standards::v1::subsets::any::io::geometry_import::{objects_from_model_snapshot, semio_model_snapshot_from_objects};
use protocol::{Mutation, MutationDiff};
use semio_framework_plugin::app::ChildEmitPreparationStep;
use semio_framework_plugin::{AppOperationContext, HistoryView, PluginApp};

/// 🪆️ Two panes' composed models: the shape pane holds `object-a` and `object-b`, the building pane `object-c`, so one
/// selection can span two panes.
fn models() -> CadPaneModels {
    CadPaneModels(vec![
        (CadPaneId::Shape, "shape-model-test".into(), semio_model_snapshot_from_objects(&[sample_object("object-a", [0.0, 0.0, 0.0]), sample_object("object-b", [4.0, 0.0, 0.0])])),
        (CadPaneId::Building, "building-model-test".into(), semio_model_snapshot_from_objects(&[sample_object("object-c", [1.0, 1.0, 0.0])])),
    ])
}

fn ids(values: &[&str]) -> Vec<String> {
    values.iter().map(|id| id.to_string()).collect()
}

/// 🧮️ `model` after `leaves`, folded the way the child store folds them.
fn apply(model: &SemioModelSnapshot, leaves: &[SemioModelMutation]) -> SemioModelSnapshot {
    leaves.iter().fold(model.clone(), |state, leaf| leaf.diff(&state).diff().apply(&state).expect("leaf applies"))
}

fn origin(model: &SemioModelSnapshot, id: &str) -> [f64; 3] {
    objects_from_model_snapshot(model).into_iter().find(|object| object.id == id).expect("object survives").origin
}

/// 📦️ `emit` with every owned-child preparation driven to its ready wire group, as the runtime's bounded driver does.
fn prepared(mut emit: Emit<CadMutation, CadConfigMutation>) -> Emit<CadMutation, CadConfigMutation> {
    for _ in 0..4096 {
        match emit.prepare_child_one(1, 65_536).expect("bounded child preparation") {
            ChildEmitPreparationStep::Ready => return emit,
            ChildEmitPreparationStep::Pending => {}
            ChildEmitPreparationStep::Refused(fault) => panic!("child preparation refused: {}", fault.message),
        }
    }
    panic!("child preparation must complete within its authored bound");
}

/// ▶️ A translate is ONE transaction: one relative leaf on the pane holding the targets, the targets deduplicated in
/// first-seen order, under a ref the tool minted as `<appId>#translateSelection`.
#[test]
fn a_translate_is_one_transaction_with_its_relative_leaf() {
    let models = models();
    let (transaction, leaves) = cad_transform_tool_commit("translateSelection", "seed-a", &models, vec![CadToolEntry::Transform(CadTransformRecord::drag(ids(&["object-a", "object-b", "object-a"]), [1.5, -2.0, 0.5]))]).expect("the drag commits");
    assert!(transaction.id.starts_with("tx-"), "{transaction:?}");
    assert_eq!(transaction.tool, "s.cad.cad@1/*#editor#translateSelection");
    assert_eq!(leaves, vec![CadToolLeaf { pane: CadPaneId::Shape, leaf: SemioModelMutation::DragElements(DragElements { targets: ids(&["object-a", "object-b"]), offset: [1.5, -2.0, 0.5] }) }], "one leaf, targets deduplicated in first-seen order");
    let after = apply(models.model(CadPaneId::Shape).expect("shape model"), &[leaves[0].leaf.clone()]);
    assert_eq!(origin(&after, "object-b"), [5.5, -2.0, 0.5]);
}

/// 📮️ The committed transaction publishes ONE composite group: one ready child edit per touched pane, each on the pane's
/// own slot and child id and labelled from its leaf, all under the one `TransactionRef` — and never a parent mutation.
#[test]
fn a_committed_gesture_is_one_child_edit_per_touched_pane_under_one_transaction() {
    let models = models();
    let (transaction, leaves) = cad_transform_tool_commit("rotateSelection", "seed-b", &models, vec![CadToolEntry::Transform(CadTransformRecord::rotate(ids(&["object-c", "object-a"]), [0.0, 0.0, 1.0], 0.5))]).expect("the turn commits");
    assert_eq!(transaction.tool, "s.cad.cad@1/*#editor#rotateSelection");
    assert_eq!(
        leaves,
        vec![
            CadToolLeaf { pane: CadPaneId::Shape, leaf: SemioModelMutation::RotateElements(RotateElements { targets: ids(&["object-a"]), axis: [0.0, 0.0, 1.0], angle: 0.5 }) },
            CadToolLeaf { pane: CadPaneId::Building, leaf: SemioModelMutation::RotateElements(RotateElements { targets: ids(&["object-c"]), axis: [0.0, 0.0, 1.0], angle: 0.5 }) },
        ]
    );
    let emit = prepared(cad_child_leaves_emit(&models, Some(transaction.clone()), leaves));
    assert_eq!(emit.transaction.as_ref(), Some(&transaction));
    assert!(emit.artifact_mutations.is_empty() && emit.config_mutations.is_empty(), "an object gesture never edits the parent or the config");
    let groups: Vec<(&str, &str, usize)> = emit.child_emits.iter().map(|child| (child.slot.as_str(), child.child_id.as_str(), child.ops.len())).collect();
    assert_eq!(groups, vec![("shapeModel", "shape-model-test", 1), ("buildingModel", "building-model-test", 1)]);
    assert_eq!(emit.child_emits[0].labels[0].resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Rotate 1 element by 28.65°");
}

/// 👻️ A stranger id, an identity motion, a pane without a model child and an empty request leave zero trace: no leaf,
/// no transaction, no child edit.
#[test]
fn nothing_to_move_leaves_zero_trace() {
    let models = models();
    let history = HistoryView::empty();
    let document = crate::empty_cad_snapshot();
    let doc = ArtifactView::with_operation(&document, &history, AppOperationContext { app_instance_id: 1, parent_document_id: "cad-test-document".into(), operation_id: 1, generation: 1, canonical_base_revision: [0; 32], authoring_seed: "seed-c".into() });
    for entries in [
        vec![CadToolEntry::Transform(CadTransformRecord::drag(ids(&["ghost"]), [1.0, 0.0, 0.0]))],
        vec![CadToolEntry::Transform(CadTransformRecord::drag(ids(&["object-a"]), [0.0, 0.0, 0.0]))],
        vec![CadToolEntry::Transform(CadTransformRecord::scale(ids(&["object-a"]), [1.0, 1.0, 1.0]))],
        vec![CadToolEntry::Transform(CadTransformRecord::rotate(ids(&["object-a"]), [0.0, 0.0, 0.0], 1.0))],
        vec![CadToolEntry::Create { pane: CadPaneId::Energy, element: crate::standards::v1::subsets::any::io::geometry_import::model_element_from_cad_object(&sample_object("object-e", [0.0; 3])) }],
        Vec::new(),
    ] {
        assert!(cad_transform_tool_commit("translateSelection", "seed-c", &models, entries.clone()).is_none(), "{entries:?} must commit nothing");
        let emit = cad_transform_tool_emit(&doc, "translateSelection", entries.clone());
        assert!(emit.child_preparations.is_empty() && emit.child_emits.is_empty() && emit.artifact_mutations.is_empty() && emit.transaction.is_none(), "{entries:?} must leave zero trace");
    }
}

/// 🔀️ Two gestures are two transactions: each admission's seed mints its own ref.
#[test]
fn two_gestures_are_two_transactions() {
    let models = models();
    let entries = || vec![CadToolEntry::Transform(CadTransformRecord::drag(ids(&["object-a"]), [1.0, 0.0, 0.0]))];
    let (first, _) = cad_transform_tool_commit("translateSelection", "seed-first", &models, entries()).expect("first commits");
    let (second, _) = cad_transform_tool_commit("translateSelection", "seed-second", &models, entries()).expect("second commits");
    assert_ne!(first.id, second.id, "two admissions never share a transaction");
}

/// 🪪️ Leaves published without a transaction — a view without command authority, a plain inspector edit — are a plain
/// child edit: no ref to stamp, the same ready group.
#[test]
fn leaves_without_a_transaction_are_a_plain_child_edit() {
    let models = models();
    let leaves = vec![CadToolLeaf { pane: CadPaneId::Shape, leaf: CadTransformRecord::scale(ids(&["object-a"]), [2.0, 2.0, 2.0]).leaf(ids(&["object-a"])) }];
    let emit = prepared(cad_child_leaves_emit(&models, None, leaves));
    assert!(emit.transaction.is_none());
    assert_eq!(emit.child_emits.len(), 1);
    assert_eq!((emit.child_emits[0].slot.as_str(), emit.child_emits[0].ops.len()), ("shapeModel", 1));
}

/// 🧱️ A construction's created element and a later transform of it are yielded in order on the pane's running model: the
/// second entry sees the first, and a duplicate id is not yielded twice.
#[test]
fn created_elements_fold_onto_the_running_model() {
    let models = models();
    let element = crate::standards::v1::subsets::any::io::geometry_import::model_element_from_cad_object(&sample_object("object-new", [2.0, 2.0, 0.0]));
    let entries = vec![
        CadToolEntry::Create { pane: CadPaneId::Shape, element: element.clone() },
        CadToolEntry::Transform(CadTransformRecord::drag(ids(&["object-new"]), [1.0, 0.0, 0.0])),
        CadToolEntry::Create { pane: CadPaneId::Shape, element },
    ];
    let yields = cad_tool_yields(&models, &entries);
    assert_eq!(yields.iter().map(|(key, _)| key.as_str()).collect::<Vec<_>>(), vec!["create:0", "transform:1:spatial.shape"], "the repeated id is refused by the leaf and never yielded");
    let leaves: Vec<SemioModelMutation> = yields.into_iter().map(|(_, leaf)| leaf.leaf).collect();
    assert_eq!(origin(&apply(models.model(CadPaneId::Shape).expect("shape model"), &leaves), "object-new"), [3.0, 2.0, 0.0]);
}

/// ⏪️ History edits the yielded leaf, never the tool: the drag's offset, rewritten through the leaf's own payload
/// value (what time travel's `historyEditInput` does), replays onto the BASE of the gesture — and onto a base where
/// the object already sits elsewhere, relative to there.
#[test]
fn an_edited_drag_replays_relative_to_its_base() {
    let models = models();
    let shape = models.model(CadPaneId::Shape).expect("shape model");
    let (_, leaves) = cad_transform_tool_commit("translateSelection", "seed-d", &models, vec![CadToolEntry::Transform(CadTransformRecord::drag(ids(&["object-a"]), [1.0, 0.0, 0.0]))]).expect("drag commits");
    let leaf = &leaves[0].leaf;
    let mut payload = serde_json::Value::from(leaf.payload_value());
    payload["offset"] = serde_json::json!([5.0, 0.0, 0.0]);
    let edited = leaf.with_payload_value(semio_framework_value::DslValue::from(payload)).expect("the edited offset decodes as the same leaf");
    assert!(matches!(&edited, SemioModelMutation::DragElements(drag) if drag.offset == [5.0, 0.0, 0.0]), "{edited:?}");
    assert_eq!(origin(&apply(shape, std::slice::from_ref(&edited)), "object-a"), [5.0, 0.0, 0.0]);
    let elsewhere = semio_model_snapshot_from_objects(&[sample_object("object-a", [10.0, 0.0, 0.0])]);
    assert_eq!(origin(&apply(&elsewhere, std::slice::from_ref(&edited)), "object-a"), [15.0, 0.0, 0.0]);
}

/// 🕹️ The commit of a `transform.move` interaction is the transform tool's relative drag, never an absolute pose.
#[test]
fn a_move_interaction_commits_a_relative_drag() {
    let models = models();
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
    let entries = try_commit_session_entries(&models, &mut runtime, CadPaneId::Shape, &snapshot);
    assert_eq!(entries, vec![CadToolEntry::Transform(CadTransformRecord::drag(ids(&["object-a"]), [3.0, 4.0, 0.0]))]);
    let (transaction, leaves) = cad_transform_tool_commit("transform.move", "seed-e", &models, entries).expect("the interaction commits");
    assert_eq!(transaction.tool, "s.cad.cad@1/*#editor#transform.move");
    assert_eq!(leaves, vec![CadToolLeaf { pane: CadPaneId::Shape, leaf: SemioModelMutation::DragElements(DragElements { targets: ids(&["object-a"]), offset: [3.0, 4.0, 0.0] }) }]);
    assert!(runtime.engagement_session.is_none(), "the committed session is closed");
}

/// 🧱️ Adds one box to the mounted app's shape pane and answers its id.
async fn place_box(app: &mut CadFixtureApp) -> String {
    use crate::editor::cad::commands::object::add_object::AddObject;
    app.dispatch_typed(CadCommand::AddObject(AddObject { typology: Some("spatial.shape.primitive.box".into()) }), &meta("local")).await.expect("addObject is admitted");
    settle(app).await;
    pane_objects(app, CadPaneId::Shape).await.last().expect("the box is placed").id.clone()
}

async fn placed_origin(app: &CadFixtureApp, id: &str) -> [f64; 3] {
    pane_objects(app, CadPaneId::Shape).await.into_iter().find(|object| object.id == id).expect("the box survives").origin
}

/// 🪟️ The full runtime: a gumball translate of a placed box is ONE history row, keyed by its tool transaction, naming
/// the shape pane's member store, editable, and labelled from its child leaf in English and German — never the op text,
/// never one row per tick, never a parent edit.
#[semio_framework_async_macros::async_test]
async fn a_mounted_translate_is_one_history_row_keyed_by_its_transaction() {
    use crate::editor::cad::commands::transform::translate_selection::TranslateSelection;
    let mut app = new_app().await;
    let parent = app.snapshot().expect("snapshot");
    let store = format!("{}/{}", cad_pane_model_slot(CadPaneId::Shape), cad_pane_model(&parent, CadPaneId::Shape).expect("shape child").child_id);
    let placed = place_box(&mut app).await;
    let origin_before = placed_origin(&app, &placed).await;
    let rows_before = member_rows(&mut app).await.len();
    app.dispatch_typed(CadCommand::TranslateSelection(TranslateSelection { object_ids: vec![placed.clone()], dx: 1.0, dy: 0.0, dz: 0.0 }), &meta("local")).await.expect("the translate is admitted");
    settle(&mut app).await;
    let rows = member_rows(&mut app).await;
    assert_eq!(rows.len(), rows_before + 1, "one gesture, one row: {rows:?}");
    let row = rows.last().expect("the gesture row");
    let transaction = row.transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.cad.cad@1/*#editor#translateSelection", "{transaction:?}");
    assert!(row.op_lines.iter().any(|line| line.starts_with("drag-elements")), "the op is the relative child leaf: {:?}", row.op_lines);
    assert_eq!(row.mutations.len(), 1, "one leaf: {:?}", row.mutations);
    assert_eq!(row.mutations[0].store.as_deref(), Some(store.as_str()));
    assert!(row.mutations[0].editable, "a drag's inputs are editable");
    assert_eq!(row.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Drag 1 element by (1, 0, 0)");
    assert_eq!(row.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "1 Element um (1; 0; 0) ziehen");
    assert_eq!(placed_origin(&app, &placed).await[0], origin_before[0] + 1.0, "the leaf moved the box");
    assert_eq!(app.snapshot().expect("snapshot"), parent, "an object gesture never moves the parent document");
    close(&mut app);
}

/// 🎚️ One streamed gumball dispatch of the press `gesture` (design §13.1, §17.4): the cumulative offset from the press, the
/// release adding `commit`, a cancel naming its `abort` reason — settled like the host settles it.
async fn stream_translate(app: &mut CadFixtureApp, target: &str, gesture: &str, dx: f64, phase: Option<(&str, semio_framework_value::DslValue)>) {
    let mut args = vec![
        ("objectIds".to_string(), semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::String(target.into())])),
        ("dx".to_string(), semio_framework_value::DslValue::float(dx)),
        ("dy".to_string(), semio_framework_value::DslValue::float(0.0)),
        ("dz".to_string(), semio_framework_value::DslValue::float(0.0)),
        ("gesture".to_string(), semio_framework_value::DslValue::String(gesture.into())),
    ];
    args.extend(phase.map(|(key, value)| (key.to_string(), value)));
    app.handle_action("translateSelection", Some(&semio_framework_value::DslValue::Object(args)), &meta("local")).await.expect("the streamed translate is admitted");
    let _ = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, TEST_INSTANCE).await;
}

/// 🪟️ The streamed gumball (design §17.4): every tick of one press stays provisional — the pane's child and the history
/// stay untouched — the release lands ONE `drag-elements` edit of the release's cumulative offset stamped with the
/// press's transaction, and a press the host cancels leaves zero trace.
#[semio_framework_async_macros::async_test]
async fn a_streamed_gumball_press_is_one_transaction_and_a_cancel_leaves_zero_trace() {
    let mut app = new_app().await;
    let placed = place_box(&mut app).await;
    let start = placed_origin(&app, &placed).await[0];
    let rows = member_rows(&mut app).await.len();
    stream_translate(&mut app, &placed, "gumball:1", 0.5, None).await;
    stream_translate(&mut app, &placed, "gumball:1", 1.25, None).await;
    assert_eq!(member_rows(&mut app).await.len(), rows, "ticks list no history row");
    assert_eq!(placed_origin(&app, &placed).await[0], start, "the committed box never moves during the press");
    stream_translate(&mut app, &placed, "gumball:1", 2.0, Some(("commit", semio_framework_value::DslValue::Bool(true)))).await;
    let after = member_rows(&mut app).await;
    assert_eq!(after.len(), rows + 1, "the release is ONE row");
    assert!(after.last().and_then(|row| row.transaction.as_ref()).is_some_and(|transaction| transaction.tool == "s.cad.cad@1/*#editor#translateSelection"), "{after:?}");
    assert_eq!(placed_origin(&app, &placed).await[0], start + 2.0, "the release lands the cumulative offset");
    stream_translate(&mut app, &placed, "gumball:2", 5.0, None).await;
    stream_translate(&mut app, &placed, "gumball:2", 0.0, Some(("abort", semio_framework_value::DslValue::String("captureLost".into())))).await;
    assert_eq!(member_rows(&mut app).await.len(), rows + 1, "a cancelled press leaves zero trace");
    assert_eq!(placed_origin(&app, &placed).await[0], start + 2.0, "the cancelled press never landed");
    close(&mut app);
}
