//! ⚡️ Wires artifact — operation enum + laws (constitutional: op).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::wires::diff::{
    board_after_add_node, board_after_patch_node, board_after_remove_node, diff_board_fixture, diff_set_snapshot, diff_wires_and_board, fixtures_after_add_edge, fixtures_after_remove_edge,
};
use crate::artifacts::wires::diff::WiresDiff;
use crate::artifacts::wires::engine::{entity_id, find_board_edge, find_board_node, find_relationship};
use crate::artifacts::wires::WiresSnapshot;
use dsl::DslValue;
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Operations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum WiresMutation {
    AddNode { node: DslValue },
    RemoveNode { node_id: String },
    PatchNode { node_id: String, patch: BTreeMap<String, DslValue> },
    AddRelationship { edge: DslValue, relationship: DslValue },
    RemoveEdge { edge_id: String },
    SetSnapshot { snapshot: WiresSnapshot },
}

//#region 🔖️DslMirror
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
enum WiresMutationDsl {
    AddNode { node: DslValue },
    RemoveNode { node_id: String },
    PatchNode { node_id: String, patch: BTreeMap<String, DslValue> },
    AddRelationship { edge: DslValue, relationship: DslValue },
    RemoveEdge { edge_id: String },
    SetSnapshot {
        #[dsl(key = "wires", block)]
        wires_fixture: crate::artifacts::wires::WiresFixtureDsl,
        #[dsl(key = "board", block)]
        board_fixture: crate::artifacts::wires::BoardFixtureDsl,
    },
}

fn wires_operation_to_dsl(op: &WiresMutation) -> WiresMutationDsl {
    match op {
        WiresMutation::AddNode { node } => WiresMutationDsl::AddNode { node: node.clone() },
        WiresMutation::RemoveNode { node_id } => WiresMutationDsl::RemoveNode { node_id: node_id.clone() },
        WiresMutation::PatchNode { node_id, patch } => WiresMutationDsl::PatchNode { node_id: node_id.clone(), patch: patch.clone() },
        WiresMutation::AddRelationship { edge, relationship } => WiresMutationDsl::AddRelationship { edge: edge.clone(), relationship: relationship.clone() },
        WiresMutation::RemoveEdge { edge_id } => WiresMutationDsl::RemoveEdge { edge_id: edge_id.clone() },
        WiresMutation::SetSnapshot { snapshot } => WiresMutationDsl::SetSnapshot {
            wires_fixture: dsl::from_dsl_value(snapshot.wires_fixture.clone()).unwrap_or_else(|error| panic!("wires_fixture does not match the reasoning.wires.fixture schema: {error}")),
            board_fixture: dsl::from_dsl_value(snapshot.board_fixture.clone()).unwrap_or_else(|error| panic!("board_fixture does not match the reasoning.mindmap.fixture schema: {error}")),
        },
    }
}

fn wires_operation_from_dsl(parsed: WiresMutationDsl) -> Result<WiresMutation, store::TextError> {
    match parsed {
        WiresMutationDsl::AddNode { node } => Ok(WiresMutation::AddNode { node }),
        WiresMutationDsl::RemoveNode { node_id } => Ok(WiresMutation::RemoveNode { node_id }),
        WiresMutationDsl::PatchNode { node_id, patch } => Ok(WiresMutation::PatchNode { node_id, patch }),
        WiresMutationDsl::AddRelationship { edge, relationship } => Ok(WiresMutation::AddRelationship { edge, relationship }),
        WiresMutationDsl::RemoveEdge { edge_id } => Ok(WiresMutation::RemoveEdge { edge_id }),
        WiresMutationDsl::SetSnapshot { wires_fixture, board_fixture } => {
            let wires_val = dsl::to_dsl_value(&wires_fixture).map_err(|error| store::TextError::new(format!("invalid wires fixture: {error}"), store::TextSpan::at(1, 1)))?;
            let board_val = dsl::to_dsl_value(&board_fixture).map_err(|error| store::TextError::new(format!("invalid board fixture: {error}"), store::TextSpan::at(1, 1)))?;
            Ok(WiresMutation::SetSnapshot { snapshot: WiresSnapshot { wires_fixture: wires_val, board_fixture: board_val } })
        }
    }
}

impl dsl::DslVariants for WiresMutation {
    fn variants() -> Vec<(String, fn() -> dsl::RecordSpec)> {
        <WiresMutationDsl as dsl::DslVariants>::variants()
    }
    fn from_named_record(keyword: &str, record: &dsl::RecordValue) -> Result<Self, store::TextError> {
        wires_operation_from_dsl(<WiresMutationDsl as dsl::DslVariants>::from_named_record(keyword, record)?)
    }
    fn to_named_record(&self) -> (String, dsl::RecordValue) {
        <WiresMutationDsl as dsl::DslVariants>::to_named_record(&wires_operation_to_dsl(self))
    }
}
//#endregion 🔖️DslMirror

impl Mutation<WiresSnapshot> for WiresMutation {
    type Diff = WiresDiff;

    fn diff(&self, snapshot: &WiresSnapshot) -> WiresDiff {
        match self {
            WiresMutation::AddNode { node } => diff_board_fixture(board_after_add_node(snapshot, node)),
            WiresMutation::RemoveNode { node_id } => diff_board_fixture(board_after_remove_node(snapshot, node_id)),
            WiresMutation::PatchNode { node_id, patch } => diff_board_fixture(board_after_patch_node(snapshot, node_id, patch)),
            WiresMutation::AddRelationship { edge, relationship } => {
                let (wires, board) = fixtures_after_add_edge(snapshot, edge, relationship);
                diff_wires_and_board(wires, board)
            }
            WiresMutation::RemoveEdge { edge_id } => {
                let (wires, board) = fixtures_after_remove_edge(snapshot, edge_id);
                diff_wires_and_board(wires, board)
            }
            WiresMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot),
        }
    }

    fn inverse(&self, snapshot: &WiresSnapshot) -> Vec<Self> {
        match self {
            WiresMutation::AddNode { node } => entity_id(node, "id").map(|node_id| vec![WiresMutation::RemoveNode { node_id: node_id.to_string() }]).unwrap_or_default(),
            WiresMutation::RemoveNode { node_id } => find_board_node(snapshot, node_id).map(|node| vec![WiresMutation::AddNode { node: node.clone() }]).unwrap_or_default(),
            WiresMutation::PatchNode { node_id, patch } => {
                let node = find_board_node(snapshot, node_id);
                let inverse: BTreeMap<String, DslValue> = patch
                    .keys()
                    .map(|key| {
                        let prior = node.and_then(|node| node.get(key)).cloned().unwrap_or(DslValue::Null);
                        (key.clone(), prior)
                    })
                    .collect();
                vec![WiresMutation::PatchNode { node_id: node_id.clone(), patch: inverse }]
            }
            WiresMutation::AddRelationship { edge, .. } => entity_id(edge, "id").map(|edge_id| vec![WiresMutation::RemoveEdge { edge_id: edge_id.to_string() }]).unwrap_or_default(),
            WiresMutation::RemoveEdge { edge_id } => {
                find_board_edge(snapshot, edge_id)
                    .map(|edge| WiresMutation::AddRelationship { edge: edge.clone(), relationship: find_relationship(snapshot, edge_id).cloned().unwrap_or(DslValue::Null) })
                    .into_iter()
                    .collect()
            }
            WiresMutation::SetSnapshot { .. } => vec![WiresMutation::SetSnapshot { snapshot: snapshot.clone() }],
        }
    }
}
//#endregion 🔖️Operations

pub fn apply_wires_mutation(snapshot: &mut WiresSnapshot, mutation: &WiresMutation) {
    *snapshot = store::apply_mutation(snapshot, mutation);
}

pub fn inverse_wires_mutation(snapshot: &WiresSnapshot, mutation: &WiresMutation) -> Vec<WiresMutation> {
    mutation.inverse(snapshot)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::wires::empty_wires_snapshot;
    use serde_json::json;
    use store::os_store::test_support::assert_op_line_round_trip;
    use store::apply_mutation;

    fn node(id: &str, text: &str) -> DslValue {
        dsl::to_dsl_value(&json!({ "id": id, "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": text, "handles": [] })).unwrap()
    }

    fn round_trip(snapshot: &WiresSnapshot, operation: &WiresMutation) -> WiresSnapshot {
        let forward = apply_mutation(snapshot, operation);
        let mut restored = forward.clone();
        for back in operation.inverse(snapshot) {
            restored = apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, snapshot, "backwards() must restore the pre-operation snapshot");
        forward
    }

    #[test]
    fn add_remove_patch_node_round_trip() {
        let snapshot = empty_wires_snapshot();
        let with_node = round_trip(&snapshot, &WiresMutation::AddNode { node: node("node-1", "Alpha") });
        assert_eq!(with_node.board_fixture.get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
        let mut patch = BTreeMap::new();
        patch.insert("text".into(), dsl::to_dsl_value(&json!("Renamed")).unwrap());
        let patched = round_trip(&with_node, &WiresMutation::PatchNode { node_id: "node-1".into(), patch });
        assert_eq!(find_board_node(&patched, "node-1").and_then(|node| node.get("text")), Some(&DslValue::String("Renamed".into())));
        let removed = round_trip(&patched, &WiresMutation::RemoveNode { node_id: "node-1".into() });
        assert!(removed.board_fixture.get("nodes").and_then(|value| value.as_array()).is_some_and(|items| items.is_empty()));
    }

    #[test]
    fn add_remove_relationship_round_trip() {
        let mut snapshot = empty_wires_snapshot();
        snapshot = apply_mutation(&snapshot, &WiresMutation::AddNode { node: node("node-1", "A") });
        snapshot = apply_mutation(&snapshot, &WiresMutation::AddNode { node: node("node-2", "B") });
        let edge = dsl::to_dsl_value(&json!({ "id": "edge-1", "edgeKind": "wires.owns", "source": "node-1", "target": "node-2" })).unwrap();
        let relationship = dsl::to_dsl_value(&json!({ "edgeId": "edge-1", "kind": "owns", "sourceIdentityId": 1, "targetIdentityId": 2 })).unwrap();
        let with_edge = round_trip(&snapshot, &WiresMutation::AddRelationship { edge, relationship });
        assert_eq!(with_edge.board_fixture.get("edges").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
        assert_eq!(with_edge.wires_fixture.get("relationships").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
        let removed = round_trip(&with_edge, &WiresMutation::RemoveEdge { edge_id: "edge-1".into() });
        assert!(removed.board_fixture.get("edges").and_then(|value| value.as_array()).is_some_and(|items| items.is_empty()));
        assert!(removed.wires_fixture.get("relationships").and_then(|value| value.as_array()).is_some_and(|items| items.is_empty()));
    }

    #[test]
    fn op_text_round_trip_add_node() {
        assert_op_line_round_trip(&WiresMutation::AddNode { node: node("node-1", "Alpha") });
    }

    #[test]
    fn op_text_round_trip_set_snapshot() {
        assert_op_line_round_trip(&WiresMutation::SetSnapshot { snapshot: empty_wires_snapshot() });
    }
}
//#endregion 🧪️Tests
