//! ⚡️ Reasoning wires app — operation enum + laws (constitutional: op).

use dsl::DslValue;
use protocol::{Operation, OperationDiff};
use reasoning_wires::MindmapWiresDocument;
use reasoning_wires_engine::{array_mut, entity_id, find_board_edge, find_board_node, find_relationship};
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

fn apply_step(wires: &mut DslValue, board: &mut DslValue, step: &MindmapWiresStep) {
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

//#region 🔖️Operations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum MindmapWiresOperation {
    AddNode { node: DslValue },
    RemoveNode { node_id: String },
    PatchNode { node_id: String, patch: BTreeMap<String, DslValue> },
    AddRelationship { edge: DslValue, relationship: DslValue },
    RemoveEdge { edge_id: String },
    ReplaceDocument { wires_fixture: DslValue, board_fixture: DslValue },
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
            MindmapWiresOperation::PatchNode { node_id, patch } => steps_diff(vec![MindmapWiresStep::PatchNode { node_id: node_id.clone(), patch: patch.clone() }]),
            MindmapWiresOperation::AddRelationship { edge, relationship } => steps_diff(vec![MindmapWiresStep::AddEdge { edge: edge.clone(), relationship: relationship.clone() }]),
            MindmapWiresOperation::RemoveEdge { edge_id } => steps_diff(vec![MindmapWiresStep::RemoveEdge { edge_id: edge_id.clone() }]),
            MindmapWiresOperation::ReplaceDocument { wires_fixture, board_fixture } => {
                MindmapWiresDiff { steps: Vec::new(), replace: Some(Box::new(MindmapWiresDocument { wires_fixture: wires_fixture.clone(), board_fixture: board_fixture.clone() })) }
            }
        }
    }

    fn backwards(&self, projection: &MindmapWiresDocument) -> Vec<Self> {
        match self {
            MindmapWiresOperation::AddNode { node } => entity_id(node, "id").map(|node_id| vec![MindmapWiresOperation::RemoveNode { node_id: node_id.to_string() }]).unwrap_or_default(),
            MindmapWiresOperation::RemoveNode { node_id } => find_board_node(projection, node_id).map(|node| vec![MindmapWiresOperation::AddNode { node: node.clone() }]).unwrap_or_default(),
            MindmapWiresOperation::PatchNode { node_id, patch } => {
                let node = find_board_node(projection, node_id);
                let inverse: BTreeMap<String, DslValue> = patch
                    .keys()
                    .map(|key| {
                        let prior = node.and_then(|node| node.get(key)).cloned().unwrap_or(DslValue::Null);
                        (key.clone(), prior)
                    })
                    .collect();
                vec![MindmapWiresOperation::PatchNode { node_id: node_id.clone(), patch: inverse }]
            }
            MindmapWiresOperation::AddRelationship { edge, .. } => entity_id(edge, "id").map(|edge_id| vec![MindmapWiresOperation::RemoveEdge { edge_id: edge_id.to_string() }]).unwrap_or_default(),
            MindmapWiresOperation::RemoveEdge { edge_id } => {
                find_board_edge(projection, edge_id).map(|edge| MindmapWiresOperation::AddRelationship { edge: edge.clone(), relationship: find_relationship(projection, edge_id).cloned().unwrap_or(DslValue::Null) }).into_iter().collect()
            }
            MindmapWiresOperation::ReplaceDocument { .. } => vec![MindmapWiresOperation::ReplaceDocument { wires_fixture: projection.wires_fixture.clone(), board_fixture: projection.board_fixture.clone() }],
        }
    }
}
//#endregion 🔖️Operations

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `reasoning_wires_engine::WiresConfig`'s operation enum — one variant per settled
/// interaction (mirrors the pre-B1 `WiresPlayRuntime` field writes), plus a generic `Snapshot` every
/// variant's `backwards()` returns — mirrors `shooting_op::ShootingConfigOperation`'s identical
/// "undo is the whole-config snapshot from just before this tick" shape: since a config-only dispatch
/// is a plain `Apply` (not an `AmendLast`, except when explicitly coalesced via `Emit::amend`/
/// `Emit::amend_config` — see `ReasoningWiresPlayApp::handle`), each tick is its own distinct, real
/// config edit, and the simplest correct inverse needs no per-field reverse-patch bookkeeping.
/// `Operation::Diff` is the WHOLE `WiresConfig` (not a granular patch type): `diff()` returns "the
/// full config after this op", and `OperationDiff<WiresConfig>::apply` for `WiresConfig` itself (in
/// `reasoning_wires_engine`) just returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum WiresConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: reasoning_wires_engine::WiresConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "drag")]
    SetDrag { node_id: Option<String>, last_x: f64, last_y: f64 },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<reasoning_wires_engine::WiresConfig> for WiresConfigOperation {
    type Diff = reasoning_wires_engine::WiresConfig;

    fn diff(&self, base: &reasoning_wires_engine::WiresConfig) -> reasoning_wires_engine::WiresConfig {
        let mut next = base.clone();
        match self {
            WiresConfigOperation::Snapshot { config } => return config.clone(),
            WiresConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            WiresConfigOperation::SetDrag { node_id, last_x, last_y } => {
                next.drag_node_id = node_id.clone();
                next.drag_last_x = *last_x;
                next.drag_last_y = *last_y;
            }
            WiresConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &reasoning_wires_engine::WiresConfig) -> Vec<Self> {
        vec![WiresConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use reasoning_wires_engine::empty_mindmap_wires_document;
    use serde_json::json;
    use store::apply_operation;

    fn node(id: &str, text: &str) -> DslValue {
        dsl::to_dsl_value(&json!({ "id": id, "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": text, "handles": [] })).unwrap()
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
        assert_eq!(with_node.board_fixture.get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
        let mut patch = BTreeMap::new();
        patch.insert("text".into(), dsl::to_dsl_value(&json!("Renamed")).unwrap());
        let patched = round_trip(&with_node, &MindmapWiresOperation::PatchNode { node_id: "node-1".into(), patch });
        assert_eq!(find_board_node(&patched, "node-1").and_then(|node| node.get("text")), Some(&DslValue::String("Renamed".into())));
        let removed = round_trip(&patched, &MindmapWiresOperation::RemoveNode { node_id: "node-1".into() });
        assert!(removed.board_fixture.get("nodes").and_then(|value| value.as_array()).is_some_and(|items| items.is_empty()));
    }

    #[test]
    fn add_remove_relationship_round_trip() {
        let mut document = empty_mindmap_wires_document();
        document = apply_operation(&document, &MindmapWiresOperation::AddNode { node: node("node-1", "A") });
        document = apply_operation(&document, &MindmapWiresOperation::AddNode { node: node("node-2", "B") });
        let edge = dsl::to_dsl_value(&json!({ "id": "edge-1", "edgeKind": "wires.owns", "source": "node-1", "target": "node-2" })).unwrap();
        let relationship = dsl::to_dsl_value(&json!({ "edgeId": "edge-1", "kind": "owns", "sourceIdentityId": 1, "targetIdentityId": 2 })).unwrap();
        let with_edge = round_trip(&document, &MindmapWiresOperation::AddRelationship { edge, relationship });
        assert_eq!(with_edge.board_fixture.get("edges").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
        assert_eq!(with_edge.wires_fixture.get("relationships").and_then(|value| value.as_array()).map(|items| items.len()), Some(1));
        let removed = round_trip(&with_edge, &MindmapWiresOperation::RemoveEdge { edge_id: "edge-1".into() });
        assert!(removed.board_fixture.get("edges").and_then(|value| value.as_array()).is_some_and(|items| items.is_empty()));
        assert!(removed.wires_fixture.get("relationships").and_then(|value| value.as_array()).is_some_and(|items| items.is_empty()));
    }

    //#region 🔖️OpTextTests
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
        let mut patch = BTreeMap::new();
        patch.insert("text".into(), dsl::to_dsl_value(&json!("Renamed")).unwrap());
        patch.insert("x".into(), dsl::to_dsl_value(&json!(12.5)).unwrap());
        store::test_support::assert_op_line_round_trip(&MindmapWiresOperation::PatchNode { node_id: "node-1".into(), patch });
    }

    #[test]
    fn op_text_round_trip_add_relationship() {
        let edge = dsl::to_dsl_value(&json!({ "id": "edge-1", "edgeKind": "wires.owns", "source": "node-1", "target": "node-2" })).unwrap();
        let relationship = dsl::to_dsl_value(&json!({ "edgeId": "edge-1", "kind": "owns", "sourceIdentityId": 1.0, "targetIdentityId": 2.0 })).unwrap();
        store::test_support::assert_op_line_round_trip(&MindmapWiresOperation::AddRelationship { edge, relationship });
    }

    #[test]
    fn op_text_round_trip_remove_edge() {
        store::test_support::assert_op_line_round_trip(&MindmapWiresOperation::RemoveEdge { edge_id: "edge-1".into() });
    }

    #[test]
    fn op_text_round_trip_replace_document() {
        store::test_support::assert_op_line_round_trip(&MindmapWiresOperation::ReplaceDocument { wires_fixture: reasoning_wires_engine::empty_wires_fixture(), board_fixture: reasoning_wires_engine::empty_board_fixture() });
    }
    //#endregion 🔖️OpTextTests

    //#region 🔖️ConfigOperationTests
    #[test]
    fn config_snapshot_and_selection_op_text_round_trip() {
        store::test_support::assert_op_line_round_trip(&WiresConfigOperation::Snapshot { config: reasoning_wires_engine::WiresConfig::default() });
        store::test_support::assert_op_line_round_trip(&WiresConfigOperation::SetSelection { ids: vec!["node-1".into(), "edge-1".into()] });
    }

    #[test]
    fn config_drag_op_text_round_trip() {
        store::test_support::assert_op_line_round_trip(&WiresConfigOperation::SetDrag { node_id: Some("node-1".into()), last_x: 12.5, last_y: -7.25 });
        store::test_support::assert_op_line_round_trip(&WiresConfigOperation::SetDrag { node_id: None, last_x: 0.0, last_y: 0.0 });
    }

    #[test]
    fn config_locale_op_text_round_trip() {
        store::test_support::assert_op_line_round_trip(&WiresConfigOperation::SetLocale { value: "de-DE".into() });
    }

    /// ⏪️ `backwards()` always returns a single whole-config `Snapshot` of the pre-op state, regardless
    /// of which field the forward op touched — the same "undo restores the prior snapshot" law
    /// `shooting_op::ShootingConfigOperation` establishes.
    #[test]
    fn config_backwards_always_snapshots_the_base() {
        let base = reasoning_wires_engine::WiresConfig { selected_ids: vec!["node-1".into()], ..Default::default() };
        let forward = WiresConfigOperation::SetSelection { ids: vec!["node-2".into()] };
        let inverse = forward.backwards(&base);
        assert_eq!(inverse, vec![WiresConfigOperation::Snapshot { config: base.clone() }]);
        assert_eq!(forward.diff(&base), reasoning_wires_engine::WiresConfig { selected_ids: vec!["node-2".into()], ..base });
    }
    //#endregion 🔖️ConfigOperationTests
}
//#endregion 🧪️Tests
