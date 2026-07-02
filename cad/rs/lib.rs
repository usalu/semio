//! 📐 CAD scene document + typed VCS on `vcs`.

use vcs::{
    create_document_vcs_envelope, CollectionDiff, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore,
    ItemPatch, Operation, OperationDiff,
};
use serde::{Deserialize, Serialize};

pub const CAD_DOCUMENT_SCHEMA: &str = "cad.scene";

//#region 🔖Domain
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadNode {
    pub id: String,
    pub label: String,
    pub kind: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadScene {
    pub schema: String,
    pub id: String,
    pub nodes: Vec<CadNode>,
}

pub type CadEnvelope = DocumentVcsEnvelope<CadScene, CadOp>;
pub type CadStore = DocumentVcsStore<CadScene, CadOp>;

pub fn empty_cad_projection() -> CadScene {
    CadScene {
        schema: CAD_DOCUMENT_SCHEMA.into(),
        id: "cad".into(),
        nodes: Vec::new(),
    }
}
//#endregion 🔖Domain

//#region 🔖Ops
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum CadOp {
    AddNode {
        node: CadNode,
    },
    RemoveNode {
        node_id: String,
    },
    RenameNode {
        node_id: String,
        label: String,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadNodePatch {
    pub label: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadDiff {
    pub nodes: Option<CollectionDiff<String, CadNodePatch, CadNode>>,
}

impl OperationDiff<CadScene> for CadDiff {
    fn apply(&self, projection: &CadScene) -> CadScene {
        let mut next = projection.clone();
        if let Some(nodes) = &self.nodes {
            for id in &nodes.removed {
                next.nodes.retain(|node| node.id != *id);
            }
            for patch in &nodes.modified {
                for node in &mut next.nodes {
                    if node.id == patch.id {
                        if let Some(label) = &patch.patch.label {
                            node.label = label.clone();
                        }
                    }
                }
            }
            for added in &nodes.added {
                next.nodes.push(added.clone());
            }
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        match (&mut self.nodes, other.nodes) {
            (Some(a), Some(b)) => {
                a.removed.extend(b.removed);
                a.modified.extend(b.modified);
                a.added.extend(b.added);
            }
            (None, Some(b)) => self.nodes = Some(b),
            _ => {}
        }
    }
}

impl Operation<CadScene> for CadOp {
    type Diff = CadDiff;

    fn diff(&self, _projection: &CadScene) -> CadDiff {
        match self {
            CadOp::AddNode { node } => CadDiff {
                nodes: Some(CollectionDiff {
                    added: vec![node.clone()],
                    ..Default::default()
                }),
            },
            CadOp::RemoveNode { node_id } => CadDiff {
                nodes: Some(CollectionDiff {
                    removed: vec![node_id.clone()],
                    ..Default::default()
                }),
            },
            CadOp::RenameNode { node_id, label } => CadDiff {
                nodes: Some(CollectionDiff {
                    modified: vec![ItemPatch {
                        id: node_id.clone(),
                        patch: CadNodePatch {
                            label: Some(label.clone()),
                        },
                    }],
                    ..Default::default()
                }),
            },
        }
    }

    fn backwards(&self, projection: &CadScene) -> Vec<Self> {
        match self {
            CadOp::AddNode { node } => vec![CadOp::RemoveNode {
                node_id: node.id.clone(),
            }],
            CadOp::RemoveNode { node_id } => projection
                .nodes
                .iter()
                .find(|n| n.id == *node_id)
                .map(|node| vec![CadOp::AddNode { node: node.clone() }])
                .unwrap_or_default(),
            CadOp::RenameNode { node_id, .. } => projection
                .nodes
                .iter()
                .find(|n| n.id == *node_id)
                .map(|node| {
                    vec![CadOp::RenameNode {
                        node_id: node_id.clone(),
                        label: node.label.clone(),
                    }]
                })
                .unwrap_or_default(),
        }
    }
}
//#endregion 🔖Ops

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct CadDocumentVcs {
        store: RefCell<CadStore>,
    }

    #[wasm_bindgen]
    impl CadDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<CadDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: CadEnvelope =
                        serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    CadStore::new(envelope)
                }
                None => CadStore::new(create_document_vcs_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection(), None)),
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
    }
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cad_projection_defaults() {
        let store = CadStore::new(create_document_vcs_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection(), None));
        assert_eq!(store.projection().expect("projection").id, "cad");
    }
}
//#endregion 🧪Tests
