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
use vcs::{
    create_document_vcs_envelope, CollectionDiff, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, ItemPatch,
    Operation, OperationDiff,
};
use serde::{Deserialize, Serialize};

pub const MINDMAP_SCHEMA: &str = "reasoning.mindmap";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MindmapDocument {
    pub topics: std::collections::BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MindmapTopicPatch {
    pub label: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MindmapDiff {
    pub topics: Option<CollectionDiff<String, MindmapTopicPatch, (String, String)>>,
}

impl OperationDiff<MindmapDocument> for MindmapDiff {
    fn apply(&self, projection: &MindmapDocument) -> MindmapDocument {
        let mut next = projection.clone();
        if let Some(topics) = &self.topics {
            for id in &topics.removed {
                next.topics.remove(id);
            }
            for patch in &topics.modified {
                if let Some(entry) = next.topics.get_mut(&patch.id) {
                    if let Some(label) = &patch.patch.label {
                        *entry = label.clone();
                    }
                }
            }
            for (id, label) in &topics.added {
                next.topics.insert(id.clone(), label.clone());
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        match (&mut self.topics, other.topics) {
            (Some(a), Some(b)) => {
                a.removed.extend(b.removed);
                a.modified.extend(b.modified);
                a.added.extend(b.added);
            }
            (None, Some(b)) => self.topics = Some(b),
            _ => {}
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum MindmapOp {
    AddTopic { id: String, label: String },
    RemoveTopic { id: String },
    RenameTopic { id: String, label: String },
}

impl Operation<MindmapDocument> for MindmapOp {
    type Diff = MindmapDiff;

    fn diff(&self, _projection: &MindmapDocument) -> MindmapDiff {
        match self {
            MindmapOp::AddTopic { id, label } => MindmapDiff {
                topics: Some(CollectionDiff {
                    added: vec![(id.clone(), label.clone())],
                    ..Default::default()
                }),
            },
            MindmapOp::RemoveTopic { id } => MindmapDiff {
                topics: Some(CollectionDiff {
                    removed: vec![id.clone()],
                    ..Default::default()
                }),
            },
            MindmapOp::RenameTopic { id, label } => MindmapDiff {
                topics: Some(CollectionDiff {
                    modified: vec![ItemPatch {
                        id: id.clone(),
                        patch: MindmapTopicPatch {
                            label: Some(label.clone()),
                        },
                    }],
                    ..Default::default()
                }),
            },
        }
    }

    fn backwards(&self, projection: &MindmapDocument) -> Vec<Self> {
        match self {
            MindmapOp::AddTopic { id, .. } => vec![MindmapOp::RemoveTopic { id: id.clone() }],
            MindmapOp::RemoveTopic { id } => projection
                .topics
                .get(id)
                .map(|label| vec![MindmapOp::AddTopic { id: id.clone(), label: label.clone() }])
                .unwrap_or_default(),
            MindmapOp::RenameTopic { id, .. } => projection
                .topics
                .get(id)
                .map(|label| vec![MindmapOp::RenameTopic { id: id.clone(), label: label.clone() }])
                .unwrap_or_default(),
        }
    }
}

pub type MindmapEnvelope = DocumentVcsEnvelope<MindmapDocument, MindmapOp>;
pub type MindmapStore = DocumentVcsStore<MindmapDocument, MindmapOp>;

pub fn empty_mindmap_projection() -> MindmapDocument {
    MindmapDocument {
        topics: std::collections::BTreeMap::new(),
    }
}
// #endregion 🔖DocumentVcs

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct MindmapDocumentVcs {
        store: RefCell<MindmapStore>,
    }

    #[wasm_bindgen]
    impl MindmapDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<MindmapDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: MindmapEnvelope =
                        serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    MindmapStore::new(envelope)
                }
                None => MindmapStore::new(create_document_vcs_envelope(
                    MINDMAP_SCHEMA,
                    "mindmap",
                    empty_mindmap_projection(),
                    None,
                )),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store
                .borrow_mut()
                .dispatch_json(command_json)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .projection_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .envelope_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
//#endregion 🔖WasmBridge

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topic_is_node_id() {
        let id: TopicId = 42;
        let mut ext = DefaultMindmapExtension::default();
        ext.topics.insert(id, "Semantics".into());
        assert_eq!(ext.topic_label(id), Some("Semantics"));
    }
}
// #endregion 🔖Tests
