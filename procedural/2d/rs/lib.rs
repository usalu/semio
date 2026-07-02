//! 📏 Procedural 2d document VCS on `vcs`.

use vcs::{
    create_document_vcs_envelope, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff,
};
use serde::{Deserialize, Serialize};

pub const PROCEDURAL_2D_SCHEMA: &str = "procedural.2d";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural2dDocument {
    pub revision: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural2dDiff {
    pub revision: Option<i64>,
}

impl OperationDiff<Procedural2dDocument> for Procedural2dDiff {
    fn apply(&self, projection: &Procedural2dDocument) -> Procedural2dDocument {
        Procedural2dDocument {
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
pub enum Procedural2dOp {
    SetRevision { revision: i64 },
}

impl Operation<Procedural2dDocument> for Procedural2dOp {
    type Diff = Procedural2dDiff;

    fn diff(&self, _projection: &Procedural2dDocument) -> Procedural2dDiff {
        match self {
            Procedural2dOp::SetRevision { revision } => Procedural2dDiff {
                revision: Some(*revision),
            },
        }
    }

    fn backwards(&self, projection: &Procedural2dDocument) -> Vec<Self> {
        vec![Procedural2dOp::SetRevision {
            revision: projection.revision,
        }]
    }
}

pub type Procedural2dEnvelope = DocumentVcsEnvelope<Procedural2dDocument, Procedural2dOp>;
pub type Procedural2dStore = DocumentVcsStore<Procedural2dDocument, Procedural2dOp>;

pub fn empty_procedural2d_projection() -> Procedural2dDocument {
    Procedural2dDocument { revision: 0 }
}

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Procedural2dDocumentVcs {
        store: RefCell<Procedural2dStore>,
    }

    #[wasm_bindgen]
    impl Procedural2dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Procedural2dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Procedural2dEnvelope =
                        serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Procedural2dStore::new(envelope)
                }
                None => Procedural2dStore::new(create_document_vcs_envelope(
                    PROCEDURAL_2D_SCHEMA,
                    "procedural2d",
                    empty_procedural2d_projection(),
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
    fn procedural2d_document_vcs_replays_ops() {
        let mut store = Procedural2dStore::new(create_document_vcs_envelope(
            PROCEDURAL_2D_SCHEMA,
            "procedural2d",
            empty_procedural2d_projection(),
            None,
        ));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![Procedural2dOp::SetRevision { revision: 2 }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").revision, 2);
    }
}
