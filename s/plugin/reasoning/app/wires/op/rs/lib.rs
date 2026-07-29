//! ⚡ Reasoning wires app — operation enum + laws (constitutional: op).

use protocol::{Operation, OperationDiff};
use reasoning_wires::MindmapWiresDocument;
use reasoning_wires_engine::{array_mut, entity_id, find_board_edge, find_board_node, find_relationship};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

//#region 🔖Steps
/// 🧩 One atomic, absorb-concatenatable board/wires mutation — the building block of {@link MindmapWiresDiff}.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "step", rename_all = "camelCase")]
pub enum MindmapWiresStep {
    AddNode { node: Value },
    RemoveNode { node_id: String },
    PatchNode { node_id: String, patch: Map<String, Value> },
    AddEdge { edge: Value, relationship: Value },
    RemoveEdge { edge_id: String },
}

fn apply_step(wires: &mut Value, board: &mut Value, step: &MindmapWiresStep) {
    match step {
        MindmapWiresStep::AddNode { node } => array_mut(board, "nodes").push(node.clone()),
        MindmapWiresStep::RemoveNode { node_id } => {
            array_mut(board, "nodes").retain(|node| entity_id(node, "id") != Some(node_id.as_str()));
        }
        MindmapWiresStep::PatchNode { node_id, patch } => {
            if let Some(node) = array_mut(board, "nodes")
                .iter_mut()
                .find(|node| entity_id(node, "id") == Some(node_id.as_str()))
            {
                if let Some(object) = node.as_object_mut() {
                    for (key, value) in patch {
                        object.insert(key.clone(), value.clone());
                    }
                }
            }
        }
        MindmapWiresStep::AddEdge { edge, relationship } => {
            array_mut(board, "edges").push(edge.clone());
            if !relationship.is_null() {
                array_mut(wires, "relationships").push(relationship.clone());
            }
        }
        MindmapWiresStep::RemoveEdge { edge_id } => {
            array_mut(board, "edges").retain(|edge| entity_id(edge, "id") != Some(edge_id.as_str()));
            array_mut(wires, "relationships").retain(|relationship| entity_id(relationship, "edgeId") != Some(edge_id.as_str()));
        }
    }
}
//#endregion 🔖Steps

//#region 🔖Operations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum MindmapWiresOperation {
    AddNode { node: Value },
    RemoveNode { node_id: String },
    PatchNode { node_id: String, patch: Map<String, Value> },
    AddRelationship { edge: Value, relationship: Value },
    RemoveEdge { edge_id: String },
    ReplaceDocument { wires_fixture: Value, board_fixture: Value },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MindmapWiresDiff {
    pub steps: Vec<MindmapWiresStep>,
    pub replace: Option<Box<MindmapWiresDocument>>,
}

impl OperationDiff<MindmapWiresDocument> for MindmapWiresDiff {
    fn apply(&self, projection: &MindmapWiresDocument) -> MindmapWiresDocument {
        let base = self.replace.as_ref().map(|document| (**document).clone()).unwrap_or_else(|| projection.clone());
        let mut wires = base.wires_fixture;
        let mut board = base.board_fixture;
        for step in &self.steps {
            apply_step(&mut wires, &mut board, step);
        }
        MindmapWiresDocument { wires_fixture: wires, board_fixture: board }
    }

    fn absorb(&mut self, other: Self) {
        if let Some(replace) = other.replace {
            self.replace = Some(replace);
            self.steps.clear();
        }
        self.steps.extend(other.steps);
    }
}

fn steps_diff(steps: Vec<MindmapWiresStep>) -> MindmapWiresDiff {
    MindmapWiresDiff { steps, replace: None }
}

impl Operation<MindmapWiresDocument> for MindmapWiresOperation {
    type Diff = MindmapWiresDiff;

    fn diff(&self, _projection: &MindmapWiresDocument) -> MindmapWiresDiff {
        match self {
            MindmapWiresOperation::AddNode { node } => steps_diff(vec![MindmapWiresStep::AddNode { node: node.clone() }]),
            MindmapWiresOperation::RemoveNode { node_id } => steps_diff(vec![MindmapWiresStep::RemoveNode { node_id: node_id.clone() }]),
            MindmapWiresOperation::PatchNode { node_id, patch } => {
                steps_diff(vec![MindmapWiresStep::PatchNode { node_id: node_id.clone(), patch: patch.clone() }])
            }
            MindmapWiresOperation::AddRelationship { edge, relationship } => {
                steps_diff(vec![MindmapWiresStep::AddEdge { edge: edge.clone(), relationship: relationship.clone() }])
            }
            MindmapWiresOperation::RemoveEdge { edge_id } => steps_diff(vec![MindmapWiresStep::RemoveEdge { edge_id: edge_id.clone() }]),
            MindmapWiresOperation::ReplaceDocument { wires_fixture, board_fixture } => MindmapWiresDiff {
                steps: Vec::new(),
                replace: Some(Box::new(MindmapWiresDocument {
                    wires_fixture: wires_fixture.clone(),
                    board_fixture: board_fixture.clone(),
                })),
            },
        }
    }

    fn backwards(&self, projection: &MindmapWiresDocument) -> Vec<Self> {
        match self {
            MindmapWiresOperation::AddNode { node } => entity_id(node, "id")
                .map(|node_id| vec![MindmapWiresOperation::RemoveNode { node_id: node_id.to_string() }])
                .unwrap_or_default(),
            MindmapWiresOperation::RemoveNode { node_id } => find_board_node(projection, node_id)
                .map(|node| vec![MindmapWiresOperation::AddNode { node: node.clone() }])
                .unwrap_or_default(),
            MindmapWiresOperation::PatchNode { node_id, patch } => {
                let node = find_board_node(projection, node_id);
                let inverse: Map<String, Value> = patch
                    .keys()
                    .map(|key| {
                        let prior = node.and_then(|node| node.get(key)).cloned().unwrap_or(Value::Null);
                        (key.clone(), prior)
                    })
                    .collect();
                vec![MindmapWiresOperation::PatchNode { node_id: node_id.clone(), patch: inverse }]
            }
            MindmapWiresOperation::AddRelationship { edge, .. } => entity_id(edge, "id")
                .map(|edge_id| vec![MindmapWiresOperation::RemoveEdge { edge_id: edge_id.to_string() }])
                .unwrap_or_default(),
            MindmapWiresOperation::RemoveEdge { edge_id } => find_board_edge(projection, edge_id)
                .map(|edge| MindmapWiresOperation::AddRelationship {
                    edge: edge.clone(),
                    relationship: find_relationship(projection, edge_id).cloned().unwrap_or(Value::Null),
                })
                .into_iter()
                .collect(),
            MindmapWiresOperation::ReplaceDocument { .. } => vec![MindmapWiresOperation::ReplaceDocument {
                wires_fixture: projection.wires_fixture.clone(),
                board_fixture: projection.board_fixture.clone(),
            }],
        }
    }
}
//#endregion 🔖Operations

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use reasoning_wires_engine::empty_mindmap_wires_document;
    use serde_json::json;
    use store::apply_operation;

    fn node(id: &str, text: &str) -> Value {
        json!({ "id": id, "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": text, "handles": [] })
    }

    fn round_trip(document: &MindmapWiresDocument, operation: &MindmapWiresOperation) -> MindmapWiresDocument {
        let forward = apply_operation(document, operation);
        let mut restored = forward.clone();
        for back in operation.backwards(document) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(&restored, document, "backwards() must restore the pre-operation document");
        forward
    }

    #[test]
    fn add_remove_patch_node_round_trip() {
        let document = empty_mindmap_wires_document();
        let with_node = round_trip(&document, &MindmapWiresOperation::AddNode { node: node("node-1", "Alpha") });
        assert_eq!(with_node.board_fixture["nodes"].as_array().unwrap().len(), 1);
        let mut patch = Map::new();
        patch.insert("text".into(), json!("Renamed"));
        let patched = round_trip(&with_node, &MindmapWiresOperation::PatchNode { node_id: "node-1".into(), patch });
        assert_eq!(find_board_node(&patched, "node-1").unwrap()["text"], json!("Renamed"));
        let removed = round_trip(&patched, &MindmapWiresOperation::RemoveNode { node_id: "node-1".into() });
        assert!(removed.board_fixture["nodes"].as_array().unwrap().is_empty());
    }

    #[test]
    fn add_remove_relationship_round_trip() {
        let mut document = empty_mindmap_wires_document();
        document = apply_operation(&document, &MindmapWiresOperation::AddNode { node: node("node-1", "A") });
        document = apply_operation(&document, &MindmapWiresOperation::AddNode { node: node("node-2", "B") });
        let edge = json!({ "id": "edge-1", "edgeKind": "wires.owns", "source": "node-1", "target": "node-2" });
        let relationship = json!({ "edgeId": "edge-1", "kind": "owns", "sourceIdentityId": 1, "targetIdentityId": 2 });
        let with_edge = round_trip(&document, &MindmapWiresOperation::AddRelationship { edge, relationship });
        assert_eq!(with_edge.board_fixture["edges"].as_array().unwrap().len(), 1);
        assert_eq!(with_edge.wires_fixture["relationships"].as_array().unwrap().len(), 1);
        let removed = round_trip(&with_edge, &MindmapWiresOperation::RemoveEdge { edge_id: "edge-1".into() });
        assert!(removed.board_fixture["edges"].as_array().unwrap().is_empty());
        assert!(removed.wires_fixture["relationships"].as_array().unwrap().is_empty());
    }

    //#region 🔖OpTextTests
    #[test]
    fn op_text_round_trip_add_node() {
        store::test_support::assert_op_line_round_trip(&MindmapWiresOperation::AddNode { node: node("node-1", "Alpha") });
    }

    #[test]
    fn op_text_round_trip_remove_node() {
        store::test_support::assert_op_line_round_trip(&MindmapWiresOperation::RemoveNode { node_id: "node-1".into() });
    }

    #[test]
    fn op_text_round_trip_patch_node() {
        let mut patch = Map::new();
        patch.insert("text".into(), json!("Renamed"));
        patch.insert("x".into(), json!(12.5));
        store::test_support::assert_op_line_round_trip(&MindmapWiresOperation::PatchNode { node_id: "node-1".into(), patch });
    }

    #[test]
    fn op_text_round_trip_add_relationship() {
        let edge = json!({ "id": "edge-1", "edgeKind": "wires.owns", "source": "node-1", "target": "node-2" });
        // 🌱 Number literals must be floats: op-text binds `Value` fields through `dsl`'s schema-less
        // `Shape::Value` escape hatch, whose `DslValue::Number` is a single `f64` (see `dsl/rs/lib.rs`)
        // — an integer JSON literal here would never compare equal to its own round-tripped value.
        let relationship = json!({ "edgeId": "edge-1", "kind": "owns", "sourceIdentityId": 1.0, "targetIdentityId": 2.0 });
        store::test_support::assert_op_line_round_trip(&MindmapWiresOperation::AddRelationship { edge, relationship });
    }

    #[test]
    fn op_text_round_trip_remove_edge() {
        store::test_support::assert_op_line_round_trip(&MindmapWiresOperation::RemoveEdge { edge_id: "edge-1".into() });
    }

    #[test]
    fn op_text_round_trip_replace_document() {
        store::test_support::assert_op_line_round_trip(&MindmapWiresOperation::ReplaceDocument {
            wires_fixture: reasoning_wires_engine::empty_wires_fixture(),
            board_fixture: reasoning_wires_engine::empty_board_fixture(),
        });
    }
    //#endregion 🔖OpTextTests
}
//#endregion 🧪Tests
