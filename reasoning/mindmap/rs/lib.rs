//! 🧠 Mindmap graph extension: topics and relationships on a property graph.

pub use infinite_cavas as cavas;
pub use infinite_board_normal_directed as graph;

// #region 🔖MindmapExtension
/// 🧠 Mindmap semantics over a property graph canvas.
pub trait MindmapExtension: graph::GraphExtension {
    fn topic_label(&self, node_id: graph::NodeId) -> Option<&str>;
}

/// 🧩 Topic is a graph node; relationship is a graph edge.
pub type TopicId = graph::NodeId;
pub type RelationshipId = graph::EdgeId;

/// 🧭 Default mindmap extension stub.
#[derive(Clone, Debug, Default)]
pub struct DefaultMindmapExtension {
    pub topics: std::collections::BTreeMap<TopicId, String>,
}

impl cavas::CanvasExtension for DefaultMindmapExtension {
    fn extension_id(&self) -> &str {
        "reasoning.mindmap/default"
    }
}

impl graph::GraphExtension for DefaultMindmapExtension {}

impl MindmapExtension for DefaultMindmapExtension {
    fn topic_label(&self, node_id: TopicId) -> Option<&str> {
        self.topics.get(&node_id).map(String::as_str)
    }
}
// #endregion 🔖MindmapExtension

// #region 🔖DocumentVcs
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use vcs::{Operation, OperationDiff};

pub const MINDMAP_WIRES_SCHEMA: &str = "reasoning.wires.fixture";
pub const MINDMAP_BOARD_SCHEMA: &str = "puzzle.2d.fixture";

// #region 🔖Document
/// 🧠 The mindmap-wires document: the semantic wires fixture (identities/relationships/kind catalogs)
/// paired with its `puzzle.2d` board fixture (nodes/edges/camera). Both are kept as opaque JSON so
/// this crate stays free of the puzzle/board schema types, while ops still address board nodes/edges
/// and wires relationships by id for mergeable, granular edits.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MindmapWiresDocument {
    pub wires_fixture: Value,
    pub board_fixture: Value,
}

pub fn empty_board_fixture() -> Value {
    serde_json::json!({
        "schema": MINDMAP_BOARD_SCHEMA,
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "nodes": [],
        "edges": [],
        "wires": []
    })
}

pub fn empty_wires_fixture() -> Value {
    serde_json::json!({
        "schema": MINDMAP_WIRES_SCHEMA,
        "identities": [],
        "relationships": [],
        "board": empty_board_fixture()
    })
}

pub fn empty_mindmap_wires_document() -> MindmapWiresDocument {
    MindmapWiresDocument { wires_fixture: empty_wires_fixture(), board_fixture: empty_board_fixture() }
}

fn array_mut<'a>(fixture: &'a mut Value, key: &str) -> &'a mut Vec<Value> {
    let object = fixture.as_object_mut().expect("mindmap fixture must be a JSON object");
    object.entry(key.to_string()).or_insert_with(|| Value::Array(Vec::new()));
    object
        .get_mut(key)
        .and_then(|value| {
            if !value.is_array() {
                *value = Value::Array(Vec::new());
            }
            value.as_array_mut()
        })
        .expect("array coerced above")
}

fn entity_id<'a>(entity: &'a Value, key: &str) -> Option<&'a str> {
    entity.get(key).and_then(|value| value.as_str())
}

pub fn find_board_node<'a>(document: &'a MindmapWiresDocument, node_id: &str) -> Option<&'a Value> {
    document
        .board_fixture
        .get("nodes")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .find(|node| entity_id(node, "id") == Some(node_id))
}

fn find_board_edge<'a>(document: &'a MindmapWiresDocument, edge_id: &str) -> Option<&'a Value> {
    document
        .board_fixture
        .get("edges")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .find(|edge| entity_id(edge, "id") == Some(edge_id))
}

fn find_relationship<'a>(document: &'a MindmapWiresDocument, edge_id: &str) -> Option<&'a Value> {
    document
        .wires_fixture
        .get("relationships")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .find(|relationship| entity_id(relationship, "edgeId") == Some(edge_id))
}
// #endregion 🔖Document

// #region 🔖Steps
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
// #endregion 🔖Steps

// #region 🔖Ops
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum MindmapWiresOp {
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

impl Operation<MindmapWiresDocument> for MindmapWiresOp {
    type Diff = MindmapWiresDiff;

    fn diff(&self, _projection: &MindmapWiresDocument) -> MindmapWiresDiff {
        match self {
            MindmapWiresOp::AddNode { node } => steps_diff(vec![MindmapWiresStep::AddNode { node: node.clone() }]),
            MindmapWiresOp::RemoveNode { node_id } => steps_diff(vec![MindmapWiresStep::RemoveNode { node_id: node_id.clone() }]),
            MindmapWiresOp::PatchNode { node_id, patch } => {
                steps_diff(vec![MindmapWiresStep::PatchNode { node_id: node_id.clone(), patch: patch.clone() }])
            }
            MindmapWiresOp::AddRelationship { edge, relationship } => {
                steps_diff(vec![MindmapWiresStep::AddEdge { edge: edge.clone(), relationship: relationship.clone() }])
            }
            MindmapWiresOp::RemoveEdge { edge_id } => steps_diff(vec![MindmapWiresStep::RemoveEdge { edge_id: edge_id.clone() }]),
            MindmapWiresOp::ReplaceDocument { wires_fixture, board_fixture } => MindmapWiresDiff {
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
            MindmapWiresOp::AddNode { node } => entity_id(node, "id")
                .map(|node_id| vec![MindmapWiresOp::RemoveNode { node_id: node_id.to_string() }])
                .unwrap_or_default(),
            MindmapWiresOp::RemoveNode { node_id } => find_board_node(projection, node_id)
                .map(|node| vec![MindmapWiresOp::AddNode { node: node.clone() }])
                .unwrap_or_default(),
            MindmapWiresOp::PatchNode { node_id, patch } => {
                let node = find_board_node(projection, node_id);
                let inverse: Map<String, Value> = patch
                    .keys()
                    .map(|key| {
                        let prior = node.and_then(|node| node.get(key)).cloned().unwrap_or(Value::Null);
                        (key.clone(), prior)
                    })
                    .collect();
                vec![MindmapWiresOp::PatchNode { node_id: node_id.clone(), patch: inverse }]
            }
            MindmapWiresOp::AddRelationship { edge, .. } => entity_id(edge, "id")
                .map(|edge_id| vec![MindmapWiresOp::RemoveEdge { edge_id: edge_id.to_string() }])
                .unwrap_or_default(),
            MindmapWiresOp::RemoveEdge { edge_id } => find_board_edge(projection, edge_id)
                .map(|edge| MindmapWiresOp::AddRelationship {
                    edge: edge.clone(),
                    relationship: find_relationship(projection, edge_id).cloned().unwrap_or(Value::Null),
                })
                .into_iter()
                .collect(),
            MindmapWiresOp::ReplaceDocument { .. } => vec![MindmapWiresOp::ReplaceDocument {
                wires_fixture: projection.wires_fixture.clone(),
                board_fixture: projection.board_fixture.clone(),
            }],
        }
    }
}

pub type MindmapWiresEnvelope = vcs::DocumentVcsEnvelope<MindmapWiresDocument, MindmapWiresOp>;
pub type MindmapWiresStore = vcs::DocumentVcsStore<MindmapWiresDocument, MindmapWiresOp>;
// #endregion 🔖Ops
// #endregion 🔖DocumentVcs

// #region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use vcs::{apply_operation, create_document_vcs_envelope, DocumentVcsCommand};

    fn node(id: &str, text: &str) -> Value {
        json!({ "id": id, "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": text, "handles": [] })
    }

    fn round_trip(document: &MindmapWiresDocument, op: &MindmapWiresOp) -> MindmapWiresDocument {
        let forward = apply_operation(document, op);
        let mut restored = forward.clone();
        for back in op.backwards(document) {
            restored = apply_operation(&restored, &back);
        }
        assert_eq!(&restored, document, "backwards() must restore the pre-op document");
        forward
    }

    #[test]
    fn add_remove_patch_node_round_trip() {
        let document = empty_mindmap_wires_document();
        let with_node = round_trip(&document, &MindmapWiresOp::AddNode { node: node("node-1", "Alpha") });
        assert_eq!(with_node.board_fixture["nodes"].as_array().unwrap().len(), 1);
        let mut patch = Map::new();
        patch.insert("text".into(), json!("Renamed"));
        let patched = round_trip(&with_node, &MindmapWiresOp::PatchNode { node_id: "node-1".into(), patch });
        assert_eq!(find_board_node(&patched, "node-1").unwrap()["text"], json!("Renamed"));
        let removed = round_trip(&patched, &MindmapWiresOp::RemoveNode { node_id: "node-1".into() });
        assert!(removed.board_fixture["nodes"].as_array().unwrap().is_empty());
    }

    #[test]
    fn add_remove_relationship_round_trip() {
        let mut document = empty_mindmap_wires_document();
        document = apply_operation(&document, &MindmapWiresOp::AddNode { node: node("node-1", "A") });
        document = apply_operation(&document, &MindmapWiresOp::AddNode { node: node("node-2", "B") });
        let edge = json!({ "id": "edge-1", "edgeKind": "wires.owns", "source": "node-1", "target": "node-2" });
        let relationship = json!({ "edgeId": "edge-1", "kind": "owns", "sourceIdentityId": 1, "targetIdentityId": 2 });
        let with_edge = round_trip(&document, &MindmapWiresOp::AddRelationship { edge, relationship });
        assert_eq!(with_edge.board_fixture["edges"].as_array().unwrap().len(), 1);
        assert_eq!(with_edge.wires_fixture["relationships"].as_array().unwrap().len(), 1);
        let removed = round_trip(&with_edge, &MindmapWiresOp::RemoveEdge { edge_id: "edge-1".into() });
        assert!(removed.board_fixture["edges"].as_array().unwrap().is_empty());
        assert!(removed.wires_fixture["relationships"].as_array().unwrap().is_empty());
    }

    #[test]
    fn store_applies_node_add() {
        let mut store = MindmapWiresStore::new(create_document_vcs_envelope(
            MINDMAP_WIRES_SCHEMA,
            "mindmap-wires",
            empty_mindmap_wires_document(),
            None,
        ));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![MindmapWiresOp::AddNode { node: node("node-1", "Alpha") }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").board_fixture["nodes"].as_array().unwrap().len(), 1);
    }
}
// #endregion 🧪Tests
