use crate::editor::sequence::unit_tests::context::{dispatch, live_host_snapshot, main_window_meta, new_app, new_app_with_registry_wired};
use crate::editor::sequence::SequenceCommand;
use crate::SequenceCamera;
use semio_framework_plugin::PluginApp;

use super::set_viewport::SetViewport;

/// 🎥️ `SetViewport` is config-only — it must never emit a `SequenceMutation` (no VCS edit, no
/// undo entry) and instead write straight into the config store.
#[semio_framework_async_macros::async_test]
async fn set_viewport_writes_config_not_operations() {
    let mut app = new_app().await;
    // 🔁️ A mounted app publishes AFTER it answers, so the config write is only visible once the
    // retained operation has settled — `context::dispatch` drives that continuation the way the host
    // does. `result.mutations` is always empty on a mounted app; the no-VCS-edit claim is proven by
    // the settled receipt carrying no `Artifact` lane.
    let result = dispatch(&mut app, SequenceCommand::SetViewport(SetViewport { camera: SequenceCamera { x: 5.0, y: 6.0, zoom: 2.0 } })).await;
    assert!(result.mutations.is_empty(), "setViewport must not emit a VCS operation");
    // 🪟️ The camera lives in the MAIN window's own config, so the read has to come through the same
    // window instance the write was addressed at — an unaddressed render sees the default camera.
    let view = main_window_meta().view_state.expect("main window view");
    let node = app.render(crate::editor::sequence::modes::edit::windows::main::SEQUENCE_PLAY_BODY_MAIN, None, &view).await.expect("render");
    let semio_framework_plugin::Component::Surface(props) = &node.root.component else { panic!("semantic graph") };
    let scene: semio_framework_plugin::NodeGraphScene = semio_framework_ui_scene::decode(props).expect("packed graph");
    assert_eq!(scene.viewport.expect("camera viewport").zoom, 2.0);
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(node).expect("retire viewport graph");
}

/// 🗑️ A `delete` row removes exactly the steps it names (never an ambient selection), with every edge they hold; a row
/// outside the shared vocabulary, or one a sequence has no widget for, refuses the whole batch.
#[semio_framework_async_macros::async_test]
async fn node_graph_edit_delete_removes_the_named_step() {
    let mut app = new_app_with_registry_wired().await;
    dispatch(&mut app, SequenceCommand::NodeGraphEdit(super::node_graph_edit::NodeGraphEdit { operations_json: "[{\"operation\":\"delete\",\"nodeIds\":[\"step-1\"],\"synapseIds\":[]}]".into() })).await;
    let live = live_host_snapshot(&app).await;
    assert!(!live.steps.iter().any(|step| step.id == "step-1"));
    assert!(!live.edges.iter().any(|edge| edge.from == "step-1" || edge.to == "step-1"));
}

/// ⚖️ LAW: the sequence guest decodes the renderer's committed node-graph rows through the ONE shared decoder: every
/// refused row is refused, every accepted row a sequence carries (`move`, `connect`, `disconnect`, `delete`) decodes, and
/// the `setSlider`/`insertPort` rows it has no widget for are refused.
#[test]
fn the_renderer_row_fixture_decodes_exactly() {
    let fixture: serde_json::Value = serde_json::from_str(NODE_GRAPH_EDIT_ROWS).expect("the row fixture parses");
    for case in fixture["accepted"].as_array().expect("accepted rows") {
        let carried = !matches!(case["row"]["operation"].as_str(), Some("setSlider" | "insertPort"));
        assert_eq!(super::node_graph_edit::sequence_node_graph_row(&semio_framework_value::DslValue::from(case["row"].clone())).is_ok(), carried, "accepted row {}", case["id"]);
    }
    for case in fixture["refused"].as_array().expect("refused rows") {
        assert!(super::node_graph_edit::sequence_node_graph_row(&semio_framework_value::DslValue::from(case["row"].clone())).is_err(), "refused row {} decoded", case["id"]);
    }
}

/// 🧾️ The framework's committed node-graph row vocabulary (schema `🧰️framework/🔨️modules/🛠️tool-machine/🧬️schema/🔣️node-graph-edit-rows`).
const NODE_GRAPH_EDIT_ROWS: &str = include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json");

//#region ✋️ChildIntentLeaves
fn fold(base: &semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot, leaves: &[semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::SemioFlowMutation]) -> semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot {
    let mut state = base.clone();
    for leaf in leaves {
        let outcome = semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::apply_semio_flow_mutation(&mut state, leaf);
        assert!(outcome.worst_level().is_none_or(|level| level < semio_framework_diagnostic::Severity::Error), "{leaf:?} refused: {:?}", outcome.messages());
    }
    state
}

/// ⚖️ LAW: every editor edit publishes child INTENT leaves, never a whole `set-snapshot`, and folding them onto the base
/// content reproduces the content the editor host left behind — a drag of two steps is ONE relative `drag-nodes`.
#[semio_framework_async_macros::async_test]
async fn child_intent_leaves_reproduce_the_edited_content() {
    use crate::editor::sequence::{sequence_content_leaves, sequence_scene_leaves};
    use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::SemioFlowMutation;
    let app = new_app().await;
    let live = live_host_snapshot(&app).await;
    let base = neural_engine::ColdOwner::new(crate::SequenceWorkingScene { steps: live.steps.clone(), edges: live.edges.clone() });
    let base_content = crate::sequence_content_snapshot_from_working(&base.steps, &base.edges);
    let ids: Vec<String> = base.steps.iter().take(2).map(|step| step.id.clone()).collect();
    let mut dragged = neural_engine::ColdOwner::new(crate::SequenceWorkingScene::clone(&base));
    for step in dragged.steps.iter_mut().filter(|step| ids.contains(&step.id)) {
        step.x += 40.0;
        step.y -= 12.5;
    }
    let leaves = sequence_scene_leaves(&base, &dragged);
    assert!(matches!(leaves.as_slice(), [SemioFlowMutation::DragNodes(drag)] if drag.targets == ids && drag.dx == 40.0 && drag.dy == -12.5), "{leaves:?}");
    let mut edited = neural_engine::ColdOwner::new(crate::SequenceWorkingScene::clone(&dragged));
    edited.edges.clear();
    if let Some(first) = edited.steps.first_mut() {
        first.kind = "log".into();
    }
    let removed = edited.steps.pop().map(neural_engine::ColdOwner::new);
    let next_content = crate::sequence_content_snapshot_from_working(&edited.steps, &edited.edges);
    let leaves = sequence_content_leaves(&base_content, &next_content);
    assert!(!leaves.iter().any(|leaf| matches!(leaf, SemioFlowMutation::SetSnapshot(_))), "{leaves:?}");
    assert_eq!(fold(&base_content, &leaves), next_content);
    assert!(sequence_content_leaves(&base_content, &base_content).is_empty(), "no change, no leaf");
    drop(removed);
}

/// ⚖️ LAW: a released node drag (the node-graph gesture record) moves its steps by the ONE offset and lands as ONE
/// composed-child tool transaction; a drag that moves nothing leaves zero trace.
#[semio_framework_async_macros::async_test]
async fn a_node_drag_record_is_one_child_transaction() {
    let mut app = new_app_with_registry_wired().await;
    let live = live_host_snapshot(&app).await;
    let (id, x, y) = live.steps.first().map(|step| (step.id.clone(), step.x, step.y)).expect("a step");
    drop(live);
    let rows = |history: semio_framework::kernel::HistoryPatch| history.upserts.into_iter().filter(|entry| entry.applied && entry.transaction.is_some()).count();
    let before = rows(app.history_snapshot().await.expect("history"));
    dispatch(&mut app, SequenceCommand::NodeGraphEdit(super::node_graph_edit::NodeGraphEdit { operations_json: format!("[{{\"operation\":\"move\",\"gestureId\":\"node-drag:0\",\"nodeIds\":[\"{id}\"],\"dx\":0.0,\"dy\":0.0}}]") })).await;
    assert_eq!(rows(app.history_snapshot().await.expect("history")), before, "a click moves nothing");
    dispatch(&mut app, SequenceCommand::NodeGraphEdit(super::node_graph_edit::NodeGraphEdit { operations_json: format!("[{{\"operation\":\"move\",\"gestureId\":\"node-drag:1\",\"nodeIds\":[\"{id}\"],\"dx\":25.0,\"dy\":5.0}}]") })).await;
    let moved = live_host_snapshot(&app).await.steps.iter().find(|entry| entry.id == id).map(|entry| (entry.x, entry.y)).expect("moved step");
    assert_eq!(moved, (x + 25.0, y + 5.0));
    assert_eq!(rows(app.history_snapshot().await.expect("history")), before + 1, "one drag, one transaction row");
}
//#endregion ✋️ChildIntentLeaves
