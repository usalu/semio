//! 👯 Puzzle 5d document VCS on `vcs`.

use vcs::{
    create_document_vcs_envelope, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff,
};
use serde::{Deserialize, Serialize};

pub const PUZZLE_5D_SCHEMA: &str = "puzzle.5d";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dDocument {
    pub revision: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dDiff {
    pub revision: Option<i64>,
}

impl OperationDiff<Puzzle5dDocument> for Puzzle5dDiff {
    fn apply(&self, projection: &Puzzle5dDocument) -> Puzzle5dDocument {
        Puzzle5dDocument {
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
pub enum Puzzle5dOp {
    SetRevision { revision: i64 },
}

impl Operation<Puzzle5dDocument> for Puzzle5dOp {
    type Diff = Puzzle5dDiff;

    fn diff(&self, _projection: &Puzzle5dDocument) -> Puzzle5dDiff {
        match self {
            Puzzle5dOp::SetRevision { revision } => Puzzle5dDiff {
                revision: Some(*revision),
            },
        }
    }

    fn backwards(&self, projection: &Puzzle5dDocument) -> Vec<Self> {
        vec![Puzzle5dOp::SetRevision {
            revision: projection.revision,
        }]
    }
}

pub type Puzzle5dEnvelope = DocumentVcsEnvelope<Puzzle5dDocument, Puzzle5dOp>;
pub type Puzzle5dStore = DocumentVcsStore<Puzzle5dDocument, Puzzle5dOp>;

pub fn empty_puzzle5d_projection() -> Puzzle5dDocument {
    Puzzle5dDocument { revision: 0 }
}

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Puzzle5dDocumentVcs {
        store: RefCell<Puzzle5dStore>,
    }

    #[wasm_bindgen]
    impl Puzzle5dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Puzzle5dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Puzzle5dEnvelope =
                        serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Puzzle5dStore::new(envelope)
                }
                None => Puzzle5dStore::new(create_document_vcs_envelope(
                    PUZZLE_5D_SCHEMA,
                    "puzzle5d",
                    empty_puzzle5d_projection(),
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
    fn puzzle5d_document_vcs_replays_ops() {
        let mut store = Puzzle5dStore::new(create_document_vcs_envelope(
            PUZZLE_5D_SCHEMA,
            "puzzle5d",
            empty_puzzle5d_projection(),
            None,
        ));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![Puzzle5dOp::SetRevision { revision: 5 }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").revision, 5);
    }
}
