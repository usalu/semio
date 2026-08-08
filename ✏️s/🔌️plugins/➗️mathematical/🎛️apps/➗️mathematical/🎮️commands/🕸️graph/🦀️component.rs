//! 🕸️ Mathematical play app commands — the graph window's algorithm/direction controls, the node-graph
//! canvas edit gestures, and its viewport.

use crate::apps::mathematical::config::{MathConfig, MathConfigMutation};
use crate::artifacts::mathematical::op::MathMutation;
use crate::artifacts::mathematical::{MathCamera, MathEdge, MathNode, MathProjection};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetAlgorithm
pub mod set_algorithm {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-algorithm")]
    pub struct SetAlgorithm {
        pub algorithm: String,
        pub seed: Option<String>,
    }

    pub fn handle(payload: &SetAlgorithm, doc: &DocumentView<'_, MathProjection>, _cfg: &ConfigView<'_, MathConfig>) -> Result<Emit<MathMutation, MathConfigMutation>, Fault> {
        let mut graph = doc.projection.graph.clone();
        graph.algorithm = payload.algorithm.clone();
        graph.algorithm_seed = payload.seed.clone();
        Ok(Emit::commit(vec![MathMutation::SetGraph { graph }], "setAlgorithm"))
    }
}
//#endregion 🔖️SetAlgorithm

//#region 🔖️SetDirected
pub mod set_directed {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-directed")]
    pub struct SetDirected {
        pub directed: bool,
    }

    pub fn handle(payload: &SetDirected, doc: &DocumentView<'_, MathProjection>, _cfg: &ConfigView<'_, MathConfig>) -> Result<Emit<MathMutation, MathConfigMutation>, Fault> {
        let mut graph = doc.projection.graph.clone();
        graph.directed = payload.directed;
        Ok(Emit::mutations(vec![MathMutation::SetGraph { graph }]))
    }
}
//#endregion 🔖️SetDirected

//#region 🔖️NodeGraphEdit
pub mod node_graph_edit {
    use super::*;

    /// 🎨️ `nodeGraphActions.edit` (`"nodeGraphEdit"`) is the shared renderer-wide action id the generic
    /// node-graph canvas dispatches interactive edit gestures under (see the React node-graph surface,
    /// `dispatch(nodeGraphActions.edit, { operations: [...] })`) — renaming or splitting it here would
    /// silently strand every node-graph interaction the frontend still targets under that id. Keeps its
    /// former batched-array shape (`operations_json`, a JSON array of tagged sub-edits) verbatim rather
    /// than splitting into one typed variant per sub-edit kind.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-edit")]
    pub struct NodeGraphEdit {
        pub operations_json: String,
    }

    pub fn handle(payload: &NodeGraphEdit, doc: &DocumentView<'_, MathProjection>, _cfg: &ConfigView<'_, MathConfig>) -> Result<Emit<MathMutation, MathConfigMutation>, Fault> {
        let edit_operations: Vec<serde_json::Value> = serde_json::from_str(&payload.operations_json).unwrap_or_default();
        let mut graph = doc.projection.graph.clone();
        let mut changed = false;
        for operation in edit_operations {
            match operation.get("operation").and_then(serde_json::Value::as_str).unwrap_or("") {
                "addNode" => {
                    let x = operation.get("x").and_then(serde_json::Value::as_f64).unwrap_or(0.0);
                    let y = operation.get("y").and_then(serde_json::Value::as_f64).unwrap_or(0.0);
                    let id = format!("n{}", graph.nodes.len());
                    graph.nodes.push(MathNode { label: id.to_uppercase(), id, x, y });
                    changed = true;
                }
                "move" => {
                    if let (Some(node_id), Some(x), Some(y)) = (operation.get("nodeId").and_then(serde_json::Value::as_str), operation.get("x").and_then(serde_json::Value::as_f64), operation.get("y").and_then(serde_json::Value::as_f64)) {
                        if let Some(node) = graph.nodes.iter_mut().find(|node| node.id == node_id) {
                            node.x = x;
                            node.y = y;
                            changed = true;
                        }
                    }
                }
                "connect" => {
                    if let (Some(source), Some(target)) = (operation.get("sourceNodeId").and_then(serde_json::Value::as_str), operation.get("targetNodeId").and_then(serde_json::Value::as_str)) {
                        let id = format!("e{}", graph.edges.len());
                        graph.edges.push(MathEdge { id, source: source.into(), target: target.into() });
                        changed = true;
                    }
                }
                "deleteSelection" => {
                    if let Some(ids) = operation.get("nodeIds").and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
                        graph.nodes.retain(|node| !ids.contains(&node.id));
                        graph.edges.retain(|edge| !ids.contains(&edge.source) && !ids.contains(&edge.target));
                        changed = true;
                    }
                }
                _ => {}
            }
        }
        if changed {
            Ok(Emit::mutations(vec![MathMutation::SetGraph { graph }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️NodeGraphEdit

//#region 🔖️NodeGraphViewport
pub mod node_graph_viewport {
    use super::*;

    /// 👁️ Config-only: the node-graph viewport never touches the document — it's written into `cfg`,
    /// session-only, no VCS edit, no undo entry on the document store.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "node-graph-viewport")]
    pub struct NodeGraphViewport {
        #[dsl(block)]
        pub camera: MathCamera,
    }

    pub fn handle(payload: &NodeGraphViewport, _doc: &DocumentView<'_, MathProjection>, _cfg: &ConfigView<'_, MathConfig>) -> Result<Emit<MathMutation, MathConfigMutation>, Fault> {
        Ok(Emit::config(vec![MathConfigMutation::SetCamera { camera: payload.camera.clone() }]))
    }
}
//#endregion 🔖️NodeGraphViewport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::mathematical::testkit::{dispatch, math_app, MathApp};
    use crate::apps::mathematical::MathCommand;

    fn node_graph_edit(operation: serde_json::Value) -> MathCommand {
        MathCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: serde_json::to_string(&vec![operation]).unwrap() })
    }

    #[test]
    fn set_algorithm_updates_graph_and_seed() {
        let mut app = math_app();
        dispatch(&mut app, MathCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: "bfs".into(), seed: Some("a".into()) }));
        let projection = app.projection().expect("projection");
        assert_eq!(projection.graph.algorithm, "bfs");
        assert_eq!(projection.graph.algorithm_seed.as_deref(), Some("a"));
    }

    #[test]
    fn set_directed_toggles_the_graph() {
        let mut app = math_app();
        dispatch(&mut app, MathCommand::SetDirected(set_directed::SetDirected { directed: false }));
        assert!(!app.projection().expect("projection").graph.directed);
    }

    #[test]
    fn node_graph_viewport_writes_config_not_document_mutations() {
        let mut app: MathApp = math_app();
        let camera = MathCamera { x: 5.0, y: 6.0, zoom: 2.0 };
        let result = app.dispatch_typed(MathCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { camera }), &semio_framework_plugin::testkit::meta("local")).expect("viewport");
        assert!(result.document_mutations.is_empty(), "nodeGraphViewport must not emit a VCS operation");
    }

    #[test]
    fn node_graph_edit_add_node_appends_a_node() {
        let mut app = math_app();
        let before = app.projection().expect("projection").graph.nodes.len();
        dispatch(&mut app, node_graph_edit(serde_json::json!({ "operation": "addNode", "x": 1.0, "y": 2.0 })));
        assert_eq!(app.projection().expect("projection").graph.nodes.len(), before + 1);
    }

    #[test]
    fn node_graph_edit_move_updates_node_position() {
        let mut app = math_app();
        let node_id = app.projection().expect("projection").graph.nodes[0].id.clone();
        dispatch(&mut app, node_graph_edit(serde_json::json!({ "operation": "move", "nodeId": node_id, "x": 42.0, "y": 43.0 })));
        let moved = app.projection().expect("projection").graph.nodes.iter().find(|node| node.id == node_id).cloned().expect("moved node");
        assert_eq!((moved.x, moved.y), (42.0, 43.0));
    }

    #[test]
    fn node_graph_edit_connect_appends_an_edge() {
        let mut app = math_app();
        let before = app.projection().expect("projection").graph.edges.len();
        dispatch(&mut app, node_graph_edit(serde_json::json!({ "operation": "connect", "sourceNodeId": "a", "targetNodeId": "d" })));
        let projection = app.projection().expect("projection");
        assert_eq!(projection.graph.edges.len(), before + 1);
        assert!(projection.graph.edges.iter().any(|edge| edge.source == "a" && edge.target == "d"));
    }

    #[test]
    fn node_graph_edit_delete_selection_removes_nodes_and_incident_edges() {
        let mut app = math_app();
        dispatch(&mut app, node_graph_edit(serde_json::json!({ "operation": "deleteSelection", "nodeIds": ["a"] })));
        let projection = app.projection().expect("projection");
        assert!(!projection.graph.nodes.iter().any(|node| node.id == "a"));
        assert!(!projection.graph.edges.iter().any(|edge| edge.source == "a" || edge.target == "a"));
    }

    #[test]
    fn node_graph_edit_unknown_operation_and_empty_array_emit_no_operations() {
        let mut app = math_app();
        let result = app.dispatch_typed(node_graph_edit(serde_json::json!({ "operation": "unknownTag" })), &semio_framework_plugin::testkit::meta("local")).expect("no-op tag");
        assert!(result.document_mutations.is_empty());
        let result = app.dispatch_typed(MathCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }), &semio_framework_plugin::testkit::meta("local")).expect("empty array");
        assert!(result.document_mutations.is_empty());
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = math_app();
        let before = app.projection().expect("projection").graph.nodes.len();
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(&mut app, node_graph_edit(serde_json::json!({ "operation": "addNode", "x": 1.0, "y": 2.0 })), |app| app.projection().expect("projection").graph.nodes.len(), before, before + 1);
    }

    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        semio_framework_plugin::testkit::assert_two_instances_converge::<crate::apps::mathematical::MathematicalPlayApp, _>(
            "mem://mathematical-convergence",
            node_graph_edit(serde_json::json!({ "operation": "addNode", "x": 9.0, "y": 9.0 })),
            MathCommand::SetDirected(set_directed::SetDirected { directed: false }),
            |app| {
                let projection = app.projection().expect("projection");
                (projection.graph.nodes.len(), projection.graph.directed)
            },
        );
    }

    #[test]
    fn ingest_operations_is_idempotent_for_mathematical() {
        semio_framework_plugin::testkit::assert_ingest_idempotent::<crate::apps::mathematical::MathematicalPlayApp, _>(node_graph_edit(serde_json::json!({ "operation": "addNode", "x": 3.0, "y": 4.0 })), |app| app.projection().expect("projection").graph.nodes.len());
    }
}
//#endregion 🧪️Tests
