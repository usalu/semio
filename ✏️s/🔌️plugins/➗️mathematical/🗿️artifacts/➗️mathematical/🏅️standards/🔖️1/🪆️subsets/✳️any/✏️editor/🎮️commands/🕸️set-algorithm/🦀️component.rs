//! 🕸️ 🕸️ Mathematical play app commands command — `set-algorithm`.

use crate::editor::mathematical::config::{MathematicalConfig, MathematicalConfigMutation};
use crate::artifacts::mathematical::op::MathematicalMutation;
use crate::artifacts::mathematical::schema::mutations::replace_graph::mutation::ReplaceGraph;
use crate::artifacts::mathematical::MathematicalSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct SetAlgorithm {
    pub algorithm: String,
    pub seed: Option<String>,
}

pub async fn handle(payload: &SetAlgorithm, doc: &ArtifactView<'_, MathematicalSnapshot>, _cfg: &ConfigView<'_, MathematicalConfig>) -> Result<Emit<MathematicalMutation, MathematicalConfigMutation>, Fault> {
    let mut graph = crate::artifacts::mathematical::mathematical_graph(doc.snapshot);
    graph.algorithm = payload.algorithm.clone();
    graph.algorithm_seed = payload.seed.clone();
    Ok(Emit::commit(vec![MathematicalMutation::ReplaceGraph(ReplaceGraph { graph })], "setAlgorithm"))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::mathematical::commands::{node_graph_edit, node_graph_viewport, set_directed};
    use crate::editor::mathematical::testkit::{dispatch, math_app, MathApp};
    use crate::editor::mathematical::MathematicalCommand;
    use crate::artifacts::mathematical::{mathematical_graph, MathematicalCamera};

    async fn node_graph_edit(operation: serde_json::Value) -> MathematicalCommand {
        MathematicalCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: serde_json::to_string(&vec![operation]).unwrap() })
    }

    #[test]
    async fn set_algorithm_updates_graph_and_seed() {
        let mut app = math_app();
        dispatch(&mut app, MathematicalCommand::SetAlgorithm(SetAlgorithm { algorithm: "bfs".into(), seed: Some("a".into()) }));
        let projection = app.snapshot().expect("projection");
        assert_eq!(mathematical_graph(&projection).algorithm, "bfs");
        assert_eq!(mathematical_graph(&projection).algorithm_seed.as_deref(), Some("a"));
    }

    #[test]
    async fn set_directed_toggles_the_graph() {
        let mut app = math_app();
        dispatch(&mut app, MathematicalCommand::SetDirected(set_directed::SetDirected { directed: false }));
        assert!(!mathematical_graph(&app.snapshot().expect("projection")).directed);
    }

    #[test]
    async fn node_graph_viewport_writes_config_not_mutations() {
        let mut app: MathApp = math_app();
        let camera = MathematicalCamera { x: 5.0, y: 6.0, zoom: 2.0 };
        let result = app.dispatch_typed(MathematicalCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { camera }), &semio_framework_plugin::testkit::meta("local")).expect("viewport");
        assert!(result.mutations.is_empty(), "nodeGraphViewport must not emit a VCS operation");
    }

    #[test]
    async fn node_graph_edit_add_node_appends_a_node() {
        let mut app = math_app();
        let before = mathematical_graph(&app.snapshot().expect("projection")).nodes.len();
        dispatch(&mut app, node_graph_edit(serde_json::json!({ "operation": "addNode", "x": 1.0, "y": 2.0 })));
        assert_eq!(mathematical_graph(&app.snapshot().expect("projection")).nodes.len(), before + 1);
    }

    #[test]
    async fn node_graph_edit_move_updates_node_position() {
        let mut app = math_app();
        let node_id = mathematical_graph(&app.snapshot().expect("projection")).nodes[0].id.clone();
        dispatch(&mut app, node_graph_edit(serde_json::json!({ "operation": "move", "nodeId": node_id, "x": 42.0, "y": 43.0 })));
        let moved = mathematical_graph(&app.snapshot().expect("projection")).nodes.iter().find(|node| node.id == node_id).cloned().expect("moved node");
        assert_eq!((moved.x, moved.y), (42.0, 43.0));
    }

    #[test]
    async fn node_graph_edit_connect_appends_an_edge() {
        let mut app = math_app();
        let before = mathematical_graph(&app.snapshot().expect("projection")).edges.len();
        dispatch(&mut app, node_graph_edit(serde_json::json!({ "operation": "connect", "sourceNodeId": "a", "targetNodeId": "d" })));
        let projection = mathematical_graph(&app.snapshot().expect("projection"));
        assert_eq!(projection.edges.len(), before + 1);
        assert!(projection.edges.iter().any(|edge| edge.source == "a" && edge.target == "d"));
    }

    #[test]
    async fn node_graph_edit_delete_selection_removes_nodes_and_incident_edges() {
        let mut app = math_app();
        dispatch(&mut app, node_graph_edit(serde_json::json!({ "operation": "deleteSelection", "nodeIds": ["a"] })));
        let projection = mathematical_graph(&app.snapshot().expect("projection"));
        assert!(!projection.nodes.iter().any(|node| node.id == "a"));
        assert!(!projection.edges.iter().any(|edge| edge.source == "a" || edge.target == "a"));
    }

    #[test]
    async fn node_graph_edit_unknown_operation_and_empty_array_emit_no_operations() {
        let mut app = math_app();
        let result = app.dispatch_typed(node_graph_edit(serde_json::json!({ "operation": "unknownTag" })), &semio_framework_plugin::testkit::meta("local")).expect("no-op tag");
        assert!(result.mutations.is_empty());
        let result = app.dispatch_typed(MathematicalCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }), &semio_framework_plugin::testkit::meta("local")).expect("empty array");
        assert!(result.mutations.is_empty());
    }

    #[test]
    async fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = math_app();
        let before = mathematical_graph(&app.snapshot().expect("projection")).nodes.len();
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(&mut app, node_graph_edit(serde_json::json!({ "operation": "addNode", "x": 1.0, "y": 2.0 })), |app| mathematical_graph(&app.snapshot().expect("projection")).nodes.len(), before, before + 1);
    }

    #[test]
    async fn two_instances_converge_disjoint_edits_via_backbone() {
        semio_framework_plugin::testkit::assert_two_instances_converge::<semio_framework_plugin::EditorApp<crate::editor::mathematical::MathematicalPlayApp>, _>(
            "mem://mathematical-convergence",
            node_graph_edit(serde_json::json!({ "operation": "addNode", "x": 9.0, "y": 9.0 })),
            MathematicalCommand::SetDirected(set_directed::SetDirected { directed: false }),
            |app| {
                let projection = mathematical_graph(&app.snapshot().expect("projection"));
                (projection.nodes.len(), projection.directed)
            },
        );
    }

    #[test]
    async fn ingest_operations_is_idempotent_for_mathematical() {
        semio_framework_plugin::testkit::assert_ingest_idempotent::<semio_framework_plugin::EditorApp<crate::editor::mathematical::MathematicalPlayApp>, _>(node_graph_edit(serde_json::json!({ "operation": "addNode", "x": 3.0, "y": 4.0 })), |app| mathematical_graph(&app.snapshot().expect("projection")).nodes.len());
    }
}
//#endregion 🧪️Tests
