//! ⚡️ Wires artifact — operation enum + laws (constitutional: op).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::wires::diff::{steps_diff, MindmapWiresDiff};
use crate::artifacts::wires::engine::{entity_id, find_board_edge, find_board_node, find_relationship};
use crate::artifacts::wires::{BoardFixtureDsl, MindmapWiresDocument, WiresFixtureDsl};
use dsl::DslValue;
use protocol::Mutation;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Operations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum MindmapWiresMutation {
    AddNode { node: DslValue },
    RemoveNode { node_id: String },
    PatchNode { node_id: String, patch: BTreeMap<String, DslValue> },
    AddRelationship { edge: DslValue, relationship: DslValue },
    RemoveEdge { edge_id: String },
    ReplaceDocument { wires_fixture: DslValue, board_fixture: DslValue },
}

//#region 🔖️DslMirror
/// 🧯️ `large_enum_variant`: `ReplaceDocument`'s two nested fixture structs make it far larger than the
/// other variants, but boxing them would require the `#[derive(dsl::DslEnum)]` field-shape machinery to
/// see through `Box<T>`, which is unverified — same accepted tradeoff as
/// `procedural_3d`'s `🦀️config.rs` config-operation enum.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
enum MindmapWiresMutationDsl {
    AddNode { node: DslValue },
    RemoveNode { node_id: String },
    PatchNode { node_id: String, patch: BTreeMap<String, DslValue> },
    AddRelationship { edge: DslValue, relationship: DslValue },
    RemoveEdge { edge_id: String },
    ReplaceDocument { wires_fixture: WiresFixtureDsl, board_fixture: BoardFixtureDsl },
}





fn mindmap_wires_operation_to_dsl(op: &MindmapWiresMutation) -> MindmapWiresMutationDsl {
    match op {
        MindmapWiresMutation::AddNode { node } => MindmapWiresMutationDsl::AddNode { node: node.clone() },
        MindmapWiresMutation::RemoveNode { node_id } => MindmapWiresMutationDsl::RemoveNode { node_id: node_id.clone() },
        MindmapWiresMutation::PatchNode { node_id, patch } => MindmapWiresMutationDsl::PatchNode { node_id: node_id.clone(), patch: patch.clone() },
        MindmapWiresMutation::AddRelationship { edge, relationship } => MindmapWiresMutationDsl::AddRelationship { edge: edge.clone(), relationship: relationship.clone() },
        MindmapWiresMutation::RemoveEdge { edge_id } => MindmapWiresMutationDsl::RemoveEdge { edge_id: edge_id.clone() },
        MindmapWiresMutation::ReplaceDocument { wires_fixture, board_fixture } => MindmapWiresMutationDsl::ReplaceDocument {
            wires_fixture: dsl::from_dsl_value(wires_fixture.clone()).unwrap_or_else(|error| panic!("wires_fixture does not match the reasoning.wires.fixture schema: {error}")),
            board_fixture: dsl::from_dsl_value(board_fixture.clone()).unwrap_or_else(|error| panic!("board_fixture does not match the reasoning.mindmap.fixture schema: {error}")),
        },
    }
}

fn mindmap_wires_operation_from_dsl(parsed: MindmapWiresMutationDsl) -> Result<MindmapWiresMutation, store::TextError> {
    match parsed {
        MindmapWiresMutationDsl::AddNode { node } => Ok(MindmapWiresMutation::AddNode { node }),
        MindmapWiresMutationDsl::RemoveNode { node_id } => Ok(MindmapWiresMutation::RemoveNode { node_id }),
        MindmapWiresMutationDsl::PatchNode { node_id, patch } => Ok(MindmapWiresMutation::PatchNode { node_id, patch }),
        MindmapWiresMutationDsl::AddRelationship { edge, relationship } => Ok(MindmapWiresMutation::AddRelationship { edge, relationship }),
        MindmapWiresMutationDsl::RemoveEdge { edge_id } => Ok(MindmapWiresMutation::RemoveEdge { edge_id }),
        MindmapWiresMutationDsl::ReplaceDocument { wires_fixture, board_fixture } => {
            let wires_val = dsl::to_dsl_value(&wires_fixture).map_err(|error| store::TextError::new(format!("invalid wires fixture: {error}"), store::TextSpan::at(1, 1)))?;
            let board_val = dsl::to_dsl_value(&board_fixture).map_err(|error| store::TextError::new(format!("invalid board fixture: {error}"), store::TextSpan::at(1, 1)))?;
            Ok(MindmapWiresMutation::ReplaceDocument { wires_fixture: wires_val, board_fixture: board_val })
        }
    }
}




//#endregion 🔖️DslMirror

impl Mutation<MindmapWiresDocument> for MindmapWiresMutation {
    type Diff = MindmapWiresDiff;

    fn diff(&self, _projection: &MindmapWiresDocument) -> MindmapWiresDiff {
        match self {
            MindmapWiresMutation::AddNode { node } => steps_diff(vec![crate::artifacts::wires::diff::MindmapWiresStep::AddNode { node: node.clone() }]),
            MindmapWiresMutation::RemoveNode { node_id } => steps_diff(vec![crate::artifacts::wires::diff::MindmapWiresStep::RemoveNode { node_id: node_id.clone() }]),
            MindmapWiresMutation::PatchNode { node_id, patch } => steps_diff(vec![crate::artifacts::wires::diff::MindmapWiresStep::PatchNode { node_id: node_id.clone(), patch: patch.clone() }]),
            MindmapWiresMutation::AddRelationship { edge, relationship } => steps_diff(vec![crate::artifacts::wires::diff::MindmapWiresStep::AddEdge { edge: edge.clone(), relationship: relationship.clone() }]),
            MindmapWiresMutation::RemoveEdge { edge_id } => steps_diff(vec![crate::artifacts::wires::diff::MindmapWiresStep::RemoveEdge { edge_id: edge_id.clone() }]),
            MindmapWiresMutation::ReplaceDocument { wires_fixture, board_fixture } => {
                MindmapWiresDiff { steps: Vec::new(), replace: Some(Box::new(MindmapWiresDocument { wires_fixture: wires_fixture.clone(), board_fixture: board_fixture.clone() })) }
            }
        }
    }

    fn inverse(&self, projection: &MindmapWiresDocument) -> Vec<Self> {
        match self {
            MindmapWiresMutation::AddNode { node } => entity_id(node, "id").map(|node_id| vec![MindmapWiresMutation::RemoveNode { node_id: node_id.to_string() }]).unwrap_or_default(),
            MindmapWiresMutation::RemoveNode { node_id } => find_board_node(projection, node_id).map(|node| vec![MindmapWiresMutation::AddNode { node: node.clone() }]).unwrap_or_default(),
            MindmapWiresMutation::PatchNode { node_id, patch } => {
                let node = find_board_node(projection, node_id);
                let inverse: BTreeMap<String, DslValue> = patch
                    .keys()
                    .map(|key| {
                        let prior = node.and_then(|node| node.get(key)).cloned().unwrap_or(DslValue::Null);
                        (key.clone(), prior)
                    })
                    .collect();
                vec![MindmapWiresMutation::PatchNode { node_id: node_id.clone(), patch: inverse }]
            }
            MindmapWiresMutation::AddRelationship { edge, .. } => entity_id(edge, "id").map(|edge_id| vec![MindmapWiresMutation::RemoveEdge { edge_id: edge_id.to_string() }]).unwrap_or_default(),
            MindmapWiresMutation::RemoveEdge { edge_id } => {
                find_board_edge(projection, edge_id).map(|edge| MindmapWiresMutation::AddRelationship { edge: edge.clone(), relationship: find_relationship(projection, edge_id).cloned().unwrap_or(DslValue::Null) }).into_iter().collect()
            }
            MindmapWiresMutation::ReplaceDocument { .. } => vec![MindmapWiresMutation::ReplaceDocument { wires_fixture: projection.wires_fixture.clone(), board_fixture: projection.board_fixture.clone() }],
        }
    }
}
//#endregion 🔖️Operations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::wires::empty_mindmap_wires_document;
    use serde_json::json;
    use store::apply_mutation;

    fn node(id: &str, text: &str) -> DslValue {
        dsl::to_dsl_value(&json!({ "id": id, "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": text, "handles": [] })).unwrap()
    }

    fn round_trip(document: &MindmapWiresDocument, operation: &MindmapWiresMutation) -> MindmapWiresDocument {
        let forward = apply_mutation(document, operation);
        let mut restored = forward.clone();
        for back in operation.inverse(document) {
            restored = apply_mutation(&restored, &back);
        }
        assert_eq!(&restored, document, "backwards() must restore the pre-operation document");
        forward
    }

    #[test]
    fn add_remove_patch_node_round_trip() {
        let document = empty_mindmap_wires_document();
        let with_node = round_trip(&document, &MindmapWiresMutation::AddNode { node: node("node-1", "Alpha") });
        assert_eq!(with_node.board_fixture.get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
        let mut patch = BTreeMap::new();
        patch.insert("text".into(), dsl::to_dsl_value(&json!("Renamed")).unwrap());
        let patched = round_trip(&with_node, &MindmapWiresMutation::PatchNode { node_id: "node-1".into(), patch });
        assert_eq!(find_board_node(&patched, "node-1").and_then(|node| node.get("text")), Some(&DslValue::String("Renamed".into())));
        let removed = round_trip(&patched, &MindmapWiresMutation::RemoveNode { node_id: "node-1".into() });
        assert!(removed.board_fixture.get("nodes").and_then(|value| value.as_array()).is_some_and(|items| items.is_empty()));
    }

    #[test]
    fn add_remove_relationship_round_trip() {
        let mut document = empty_mindmap_wires_document();
        document = apply_mutation(&document, &MindmapWiresMutation::AddNode { node: node("node-1", "A") });
        document = apply_mutation(&document, &MindmapWiresMutation::AddNode { node: node("node-2", "B") });
        let edge = dsl::to_dsl_value(&json!({ "id": "edge-1", "edgeKind": "wires.owns", "source": "node-1", "target": "node-2" })).unwrap();
        let relationship = dsl::to_dsl_value(&json!({ "edgeId": "edge-1", "kind": "owns", "sourceIdentityId": 1, "targetIdentityId": 2 })).unwrap();
        let with_edge = round_trip(&document, &MindmapWiresMutation::AddRelationship { edge, relationship });
        assert_eq!(with_edge.board_fixture.get("edges").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
        assert_eq!(with_edge.wires_fixture.get("relationships").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
        let removed = round_trip(&with_edge, &MindmapWiresMutation::RemoveEdge { edge_id: "edge-1".into() });
        assert!(removed.board_fixture.get("edges").and_then(|value| value.as_array()).is_some_and(|items| items.is_empty()));
        assert!(removed.wires_fixture.get("relationships").and_then(|value| value.as_array()).is_some_and(|items| items.is_empty()));
    }

    //#region 🔖️OpTextTests
    #[test]
    fn op_text_round_trip_add_node() {
        store::test_support::assert_op_line_round_trip(&MindmapWiresMutation::AddNode { node: node("node-1", "Alpha") });
    }

    #[test]
    fn op_text_round_trip_remove_node() {
        store::test_support::assert_op_line_round_trip(&MindmapWiresMutation::RemoveNode { node_id: "node-1".into() });
    }

    #[test]
    fn op_text_round_trip_patch_node() {
        let mut patch = BTreeMap::new();
        patch.insert("text".into(), dsl::to_dsl_value(&json!("Renamed")).unwrap());
        patch.insert("x".into(), dsl::to_dsl_value(&json!(12.5)).unwrap());
        store::test_support::assert_op_line_round_trip(&MindmapWiresMutation::PatchNode { node_id: "node-1".into(), patch });
    }

    #[test]
    fn op_text_round_trip_add_relationship() {
        let edge = dsl::to_dsl_value(&json!({ "id": "edge-1", "edgeKind": "wires.owns", "source": "node-1", "target": "node-2" })).unwrap();
        let relationship = dsl::to_dsl_value(&json!({ "edgeId": "edge-1", "kind": "owns", "sourceIdentityId": 1.0, "targetIdentityId": 2.0 })).unwrap();
        store::test_support::assert_op_line_round_trip(&MindmapWiresMutation::AddRelationship { edge, relationship });
    }

    #[test]
    fn op_text_round_trip_remove_edge() {
        store::test_support::assert_op_line_round_trip(&MindmapWiresMutation::RemoveEdge { edge_id: "edge-1".into() });
    }

    #[test]
    fn op_text_round_trip_replace_document() {
        store::test_support::assert_op_line_round_trip(&MindmapWiresMutation::ReplaceDocument { wires_fixture: crate::artifacts::wires::empty_wires_fixture(), board_fixture: crate::artifacts::wires::empty_board_fixture() });
    }
    //#endregion 🔖️OpTextTests
}
//#endregion 🧪️Tests


pub fn apply_mindmap_wires_mutation(projection: &mut MindmapWiresDocument, mutation: &MindmapWiresMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
}

pub fn inverse_mindmap_wires_mutation(projection: &MindmapWiresDocument, mutation: &MindmapWiresMutation) -> Vec<MindmapWiresMutation> {
    mutation.inverse(projection)
}
