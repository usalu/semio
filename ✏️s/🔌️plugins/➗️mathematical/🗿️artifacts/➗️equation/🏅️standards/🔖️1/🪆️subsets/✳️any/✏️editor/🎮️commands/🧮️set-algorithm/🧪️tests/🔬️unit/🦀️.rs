use super::*;
use crate::editor::equation::commands::add_node::AddNode;
use crate::editor::equation::commands::{node_graph_edit, node_graph_viewport, set_directed};
use crate::editor::equation::unit_tests::context::{dispatch, math_app, MathApp};
use crate::editor::equation::EquationCommand;
use crate::EquationCamera;
use semio_framework_pack_json::Value;
use semio_framework::kernel::HistoryEntry;
use semio_framework_plugin::PluginApp;

fn node_graph_edit(operation: Value) -> EquationCommand {
    EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: semio_framework_pack_json::to_string(&semio_framework_pack_json::array([operation])) })
}

#[semio_framework_async_macros::async_test]
async fn set_algorithm_updates_graph_and_seed() {
    let mut app = math_app().await;
    dispatch(&mut app, EquationCommand::SetAlgorithm(SetAlgorithm { algorithm: "bfs".into(), seed: Some("a".into()) })).await;
    let projection = app.snapshot().expect("projection");
    assert_eq!(projection.graph.algorithm, "bfs");
    assert_eq!(projection.graph.algorithm_seed.as_deref(), Some("a"));
}

#[semio_framework_async_macros::async_test]
async fn set_directed_toggles_the_graph() {
    let mut app = math_app().await;
    dispatch(&mut app, EquationCommand::SetDirected(set_directed::SetDirected { directed: false })).await;
    assert!(!app.snapshot().expect("projection").graph.directed);
}

#[semio_framework_async_macros::async_test]
async fn node_graph_viewport_writes_config_not_mutations() {
    let mut app = math_app().await;
    let camera = EquationCamera { x: 5.0, y: 6.0, zoom: 2.0 };
    let view = semio_framework_plugin::ViewModel {
        window_id: Some("equation-graph-test".into()),
        window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: "equation-graph-test".into(), window_kind_id: crate::editor::equation::modes::edit::windows::graph::MATH_PLAY_WINDOW_GRAPH.into() }],
        ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let result = app
        .dispatch_typed(
            EquationCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d { x: camera.x, y: camera.y, zoom: camera.zoom } }),
            &semio_framework_plugin::ActionMeta { view_state: Some(view), ..semio_framework_plugin::artifact_app_laws::meta("local") },
        )
        .await
        .expect("viewport");
    assert!(result.mutations.is_empty(), "nodeGraphViewport must not emit a VCS operation");
}

#[semio_framework_async_macros::async_test]
async fn the_add_node_verb_appends_a_node() {
    let mut app = math_app().await;
    let before = app.snapshot().expect("projection").graph.nodes.len();
    dispatch(&mut app, EquationCommand::AddNode(AddNode { x: 1.0, y: 2.0 })).await;
    assert_eq!(app.snapshot().expect("projection").graph.nodes.len(), before + 1);
}

#[semio_framework_async_macros::async_test]
async fn node_graph_edit_connect_appends_an_edge() {
    let mut app = math_app().await;
    let before = app.snapshot().expect("projection").graph.edges.len();
    dispatch(&mut app, node_graph_edit(semio_framework_pack_json::object([("operation".to_string(), Value::from("connect")), ("sourceNodeId".to_string(), Value::from("a")), ("sourcePortId".to_string(), Value::from("")), ("targetNodeId".to_string(), Value::from("d")), ("targetPortId".to_string(), Value::from(""))]))).await;
    let projection = (app.snapshot().expect("projection")).graph.clone();
    assert_eq!(projection.edges.len(), before + 1);
    assert!(projection.edges.iter().any(|edge| edge.source == "a" && edge.target == "d"));
}

#[semio_framework_async_macros::async_test]
async fn node_graph_edit_delete_removes_nodes_and_incident_edges() {
    let mut app = math_app().await;
    dispatch(&mut app, node_graph_edit(semio_framework_pack_json::object([("operation".to_string(), Value::from("delete")), ("nodeIds".to_string(), semio_framework_pack_json::array([Value::from("a")])), ("synapseIds".to_string(), semio_framework_pack_json::array([]))]))).await;
    let projection = (app.snapshot().expect("projection")).graph.clone();
    assert!(!projection.nodes.iter().any(|node| node.id == "a"));
    assert!(!projection.edges.iter().any(|edge| edge.source == "a" || edge.target == "a"));
}

#[semio_framework_async_macros::async_test]
async fn node_graph_edit_refuses_an_unknown_operation_and_an_empty_array_emits_nothing() {
    let mut app = math_app().await;
    assert!(app.dispatch_typed(node_graph_edit(semio_framework_pack_json::object([("operation".to_string(), Value::from("unknownTag"))])), &semio_framework_plugin::artifact_app_laws::meta("local")).await.is_err(), "a row outside the shared vocabulary refuses the batch");
    let result = app.dispatch_typed(EquationCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("empty array");
    assert!(result.mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trip_through_the_wrapper() {
    let mut app = math_app().await;
    let before = app.snapshot().expect("projection").graph.nodes.len();
    semio_framework_plugin::artifact_app_laws::assert_undo_redo_round_trip(
        &mut app,
        EquationCommand::AddNode(AddNode { x: 1.0, y: 2.0 }),
        |app| app.snapshot().expect("projection").graph.nodes.len(),
        before,
        before + 1,
    )
    .await;
}

/// 🧹️ The REGISTERED pair: equation publishes bounded tool proofs, so a registry-less `paired_apps`
/// instance faults in the `interactive-job.catalog-authority` proof join before any edit lands.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    semio_framework_plugin::artifact_app_laws::assert_two_registered_instances_converge_with_members::<semio_framework_plugin::EditorApp<crate::editor::equation::EquationPlayApp>, semio_s_artifact_stdio_semio::SemioMembers, _, _, _>(
        "mem://equation-convergence",
        || async { crate::editor::equation::unit_tests::context::equation_app_manifest_for_tests() },
        EquationCommand::AddNode(AddNode { x: 9.0, y: 9.0 }),
        EquationCommand::SetDirected(set_directed::SetDirected { directed: false }),
        |app| {
            let projection = (app.snapshot().expect("projection")).graph.clone();
            (projection.nodes.len(), projection.directed)
        },
    )
    .await;
}

/// 🔁️ The REGISTERED idempotency law — same reason as the registered convergence pair above.
#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent_for_equation() {
    semio_framework_plugin::artifact_app_laws::assert_registered_ingest_idempotent_with_members::<semio_framework_plugin::EditorApp<crate::editor::equation::EquationPlayApp>, semio_s_artifact_stdio_semio::SemioMembers, _, _, _>(
        || async { crate::editor::equation::unit_tests::context::equation_app_manifest_for_tests() },
        EquationCommand::AddNode(AddNode { x: 3.0, y: 4.0 }),
        |app| app.snapshot().expect("projection").graph.nodes.len(),
    )
    .await;
}

//#region ✋️GestureLaws
/// 🧾️ Every applied history row that carries document operations, oldest first.
async fn edit_rows(app: &mut MathApp) -> Vec<HistoryEntry> {
    let mut rows: Vec<HistoryEntry> = PluginApp::history_snapshot(app).await.expect("history").upserts.into_iter().filter(|entry| entry.applied && !entry.op_lines.is_empty()).collect();
    rows.sort_by_key(|entry| entry.seq);
    rows
}

fn drag(gesture: &str, ids: &[&str], dx: f64, dy: f64) -> EquationCommand {
    node_graph_edit(semio_framework_pack_json::object([
        ("operation".to_string(), Value::from("move")),
        ("gestureId".to_string(), Value::from(gesture)),
        ("nodeIds".to_string(), semio_framework_pack_json::array(ids.iter().map(|id| Value::from(*id)))),
        ("dx".to_string(), Value::from(dx)),
        ("dy".to_string(), Value::from(dy)),
    ]))
}

/// ⚖️ LAW: a released node drag (the node-graph gesture record) is ONE edit, one row stamped with its transaction (tool
/// `<appId>#nodeGraphEdit`), whose op is the RELATIVE `move-nodes` leaf — never a whole-graph `replace-graph`; a drag that
/// moves nothing leaves zero trace; two drags are two transactions.
#[semio_framework_async_macros::async_test]
async fn a_node_drag_is_one_transaction_of_one_relative_move() {
    let mut app = math_app().await;
    let base = (app.snapshot().expect("projection")).graph.clone();
    let ids: Vec<String> = base.nodes.iter().take(2).map(|node| node.id.clone()).collect();
    let before = edit_rows(&mut app).await.len();
    dispatch(&mut app, drag("node-drag:0", &[ids[0].as_str()], 0.0, 0.0)).await;
    dispatch(&mut app, drag("node-drag:0", &["ghost"], 5.0, 0.0)).await;
    assert_eq!(edit_rows(&mut app).await.len(), before, "nothing moved, nothing recorded");
    dispatch(&mut app, drag("node-drag:1", &[ids[0].as_str(), ids[1].as_str()], 40.0, -12.5)).await;
    dispatch(&mut app, drag("node-drag:2", &[ids[0].as_str()], 1.0, 1.0)).await;
    let graph = (app.snapshot().expect("projection")).graph.clone();
    for (index, id) in ids.iter().enumerate() {
        let (was, is) = (base.nodes.iter().find(|node| &node.id == id).expect("base"), graph.nodes.iter().find(|node| &node.id == id).expect("moved"));
        let extra = if index == 0 { 1.0 } else { 0.0 };
        assert_eq!((is.x, is.y), (was.x + 40.0 + extra, was.y - 12.5 + extra));
    }
    let rows = edit_rows(&mut app).await;
    let rows = &rows[before..];
    assert_eq!(rows.len(), 2, "{rows:?}");
    assert!(rows.iter().all(|row| row.op_lines.iter().all(|line| line.starts_with("move-nodes"))), "{rows:?}");
    let first = rows[0].transaction.as_ref().expect("a drag is a tool transaction");
    assert!(first.id.starts_with("tx-") && first.tool == "s.mathematical.equation@1/*#editor#nodeGraphEdit", "{first:?}");
    assert_ne!(first.id, rows[1].transaction.as_ref().expect("second drag").id, "two drags, two transactions");
    assert_eq!(rows[0].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Move 2 node(s) by (40, -12.5)");
    assert_eq!(rows[0].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "2 Knoten um (40; -12,5) verschieben");
}

/// ⚖️ LAW: every node-graph and algorithm verb lands intent leaves only — never `replace-graph` — and a deleted selection
/// is disconnects followed by deletes, each row point-invertible.
#[semio_framework_async_macros::async_test]
async fn graph_verbs_land_intent_leaves_only() {
    let mut app = math_app().await;
    let before = edit_rows(&mut app).await.len();
    dispatch(&mut app, EquationCommand::AddNode(AddNode { x: 1.0, y: 2.0 })).await;
    dispatch(&mut app, node_graph_edit(semio_framework_pack_json::object([("operation".to_string(), Value::from("connect")), ("sourceNodeId".to_string(), Value::from("a")), ("sourcePortId".to_string(), Value::from("")), ("targetNodeId".to_string(), Value::from("d")), ("targetPortId".to_string(), Value::from(""))]))).await;
    dispatch(&mut app, node_graph_edit(semio_framework_pack_json::object([("operation".to_string(), Value::from("delete")), ("nodeIds".to_string(), semio_framework_pack_json::array([Value::from("a")])), ("synapseIds".to_string(), semio_framework_pack_json::array([]))]))).await;
    dispatch(&mut app, EquationCommand::SetAlgorithm(SetAlgorithm { algorithm: "dfs".into(), seed: None })).await;
    dispatch(&mut app, EquationCommand::SetDirected(set_directed::SetDirected { directed: false })).await;
    let lines: Vec<String> = edit_rows(&mut app).await[before..].iter().flat_map(|row| row.op_lines.clone()).collect();
    assert!(!lines.iter().any(|line| line.starts_with("replace-graph")), "{lines:?}");
    assert!(lines[0].starts_with("create-node") && lines[1].starts_with("connect-nodes"), "{lines:?}");
    let deletes: Vec<&String> = lines.iter().filter(|line| line.starts_with("disconnect-nodes") || line.starts_with("delete-node")).collect();
    assert!(deletes.last().is_some_and(|line| line.starts_with("delete-node")), "edges are disconnected before the node goes: {lines:?}");
    assert!(lines.iter().any(|line| line.starts_with("update-graph-algorithm")) && lines.iter().any(|line| line.starts_with("change-graph-directed")), "{lines:?}");
    let graph = (app.snapshot().expect("projection")).graph.clone();
    assert!(!graph.nodes.iter().any(|node| node.id == "a") && !graph.edges.iter().any(|edge| edge.source == "a" || edge.target == "a"));
    assert_eq!((graph.algorithm.as_str(), graph.directed), ("dfs", false));
}
//#endregion ✋️GestureLaws
