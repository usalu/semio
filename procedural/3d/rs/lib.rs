//! 📐 Procedural 3d document VCS on `vcs`.

use vcs::{
    create_document_vcs_envelope, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff,
};
use serde::{Deserialize, Serialize};

pub const PROCEDURAL_3D_SCHEMA: &str = "procedural.3d/v1";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dDocument {
    pub revision: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dDiff {
    pub revision: Option<i64>,
}

impl OperationDiff<Procedural3dDocument> for Procedural3dDiff {
    fn apply(&self, projection: &Procedural3dDocument) -> Procedural3dDocument {
        Procedural3dDocument {
            revision: self.revision.unwrap_or(projection.revision),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.revision.is_some() {
            self.revision = other.revision;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum Procedural3dOp {
    SetRevision { revision: i64 },
}

impl Operation<Procedural3dDocument> for Procedural3dOp {
    type Diff = Procedural3dDiff;

    fn diff(&self, _projection: &Procedural3dDocument) -> Procedural3dDiff {
        match self {
            Procedural3dOp::SetRevision { revision } => Procedural3dDiff {
                revision: Some(*revision),
            },
        }
    }

    fn backwards(&self, projection: &Procedural3dDocument) -> Vec<Self> {
        vec![Procedural3dOp::SetRevision {
            revision: projection.revision,
        }]
    }
}

pub type Procedural3dEnvelope = DocumentVcsEnvelope<Procedural3dDocument, Procedural3dOp>;
pub type Procedural3dStore = DocumentVcsStore<Procedural3dDocument, Procedural3dOp>;

pub fn empty_procedural3d_projection() -> Procedural3dDocument {
    Procedural3dDocument { revision: 0 }
}

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Procedural3dDocumentVcs {
        store: RefCell<Procedural3dStore>,
    }

    #[wasm_bindgen]
    impl Procedural3dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Procedural3dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Procedural3dEnvelope =
                        serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Procedural3dStore::new(envelope)
                }
                None => Procedural3dStore::new(create_document_vcs_envelope(
                    PROCEDURAL_3D_SCHEMA,
                    "procedural3d",
                    empty_procedural3d_projection(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn procedural3d_document_vcs_replays_ops() {
        let mut store = Procedural3dStore::new(create_document_vcs_envelope(
            PROCEDURAL_3D_SCHEMA,
            "procedural3d",
            empty_procedural3d_projection(),
            None,
        ));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![Procedural3dOp::SetRevision { revision: 4 }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").revision, 4);
    }
}
