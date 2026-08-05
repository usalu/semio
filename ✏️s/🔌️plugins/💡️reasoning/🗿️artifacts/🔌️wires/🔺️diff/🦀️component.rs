//! 🔺️ Wires artifact — diff structs + `OperationDiff` impl (constitutional: diff, extracted from op).

use crate::artifacts::wires::engine::{array_mut, entity_id};
use crate::artifacts::wires::MindmapWiresDocument;
use dsl::DslValue;
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Steps
/// 🧩️ One atomic, absorb-concatenatable board/wires mutation — the building block of {@link MindmapWiresDiff}.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "step", rename_all = "camelCase")]
pub enum MindmapWiresStep {
    AddNode { node: DslValue },
    RemoveNode { node_id: String },
    PatchNode { node_id: String, patch: BTreeMap<String, DslValue> },
    AddEdge { edge: DslValue, relationship: DslValue },
    RemoveEdge { edge_id: String },
}

pub fn apply_step(wires: &mut DslValue, board: &mut DslValue, step: &MindmapWiresStep) {
    match step {
        MindmapWiresStep::AddNode { node } => array_mut(board, "nodes").push(node.clone()),
        MindmapWiresStep::RemoveNode { node_id } => {
            array_mut(board, "nodes").retain(|node| entity_id(node, "id") != Some(node_id.as_str()));
        }
        MindmapWiresStep::PatchNode { node_id, patch } => {
            if let Some(node) = array_mut(board, "nodes").iter_mut().find(|node| entity_id(node, "id") == Some(node_id.as_str())) {
                if let DslValue::Object(entries) = node {
                    for (key, value) in patch {
                        if let Some((_, slot)) = entries.iter_mut().find(|(entry_key, _)| entry_key == key) {
                            *slot = value.clone();
                        } else {
                            entries.push((key.clone(), value.clone()));
                        }
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
//#endregion 🔖️Steps

//#region 🔖️Diff
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

pub fn steps_diff(steps: Vec<MindmapWiresStep>) -> MindmapWiresDiff {
    MindmapWiresDiff { steps, replace: None }
}
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::wires::empty_mindmap_wires_document;
    use serde_json::json;

    fn node(id: &str, text: &str) -> DslValue {
        dsl::to_dsl_value(&json!({ "id": id, "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": text, "handles": [] })).unwrap()
    }

    #[test]
    fn apply_step_add_node_appends_to_board_nodes() {
        let document = empty_mindmap_wires_document();
        let diff = steps_diff(vec![MindmapWiresStep::AddNode { node: node("node-1", "Alpha") }]);
        let after = diff.apply(&document);
        assert_eq!(after.board_fixture.get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
    }

    #[test]
    fn absorb_concatenates_steps_and_replace_wins() {
        let mut diff = steps_diff(vec![MindmapWiresStep::AddNode { node: node("node-1", "Alpha") }]);
        let replacement = empty_mindmap_wires_document();
        diff.absorb(MindmapWiresDiff { steps: Vec::new(), replace: Some(Box::new(replacement.clone())) });
        assert!(diff.steps.is_empty());
        assert_eq!(diff.replace, Some(Box::new(replacement)));
    }
}
//#endregion 🧪️Tests
