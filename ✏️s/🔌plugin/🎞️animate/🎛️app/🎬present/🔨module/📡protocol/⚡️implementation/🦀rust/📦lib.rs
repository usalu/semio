//! ⚖️ Animate present app — binary command protocol surface + laws (constitutional: protocol). Also
//! hosts the `PresentEnvelope`/`PresentStore` type aliases and the WASM VCS bridge — both need
//! `PresentOperation` (from `present_op`) alongside `PresentDeck` (from `present`), so this is the
//! first constitutional crate in the stack where that pairing is available.

use present::{PresentDeck, PRESENT_DECK_SCHEMA};
use present_engine::empty_present_deck;
use present_op::PresentOperation;
use protocol::OpBinary;
use store::{create_document_envelope, materialize_document_projection, DocumentEnvelope, DocumentStore};

/// 📦 Encodes a `PresentOperation` to its binary command form.
pub fn encode_op(operation: &PresentOperation) -> Result<Vec<u8>, protocol::ProtocolError> {
    operation.encode_op()
}

/// 📖 Decodes a `PresentOperation` from its binary command form.
pub fn decode_op(bytes: &[u8]) -> Result<PresentOperation, protocol::ProtocolError> {
    PresentOperation::decode_op(bytes)
}

//#region 🔖Store
pub type PresentEnvelope = DocumentEnvelope<PresentDeck, PresentOperation>;
pub type PresentStore = DocumentStore<PresentDeck, PresentOperation>;
//#endregion 🔖Store

//#region 🔖VcsEnvelope
/// @emoji 📦 Creates an empty typed VCS envelope for a presentation deck document.
pub fn create_present_envelope(id: &str) -> PresentEnvelope {
    create_document_envelope(PRESENT_DECK_SCHEMA, id, empty_present_deck(), None)
}

/// @emoji 📐 Replays every stored edit in `envelope_json` and returns the materialized deck projection.
pub fn materialize_present_projection_json(envelope_json: &str) -> Result<PresentDeck, present_engine::PresentError> {
    let envelope: PresentEnvelope = serde_json::from_str(envelope_json)?;
    let edit_ids: Vec<String> = envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect();
    Ok(materialize_document_projection(&envelope, &edit_ids)?)
}
//#endregion 🔖VcsEnvelope

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct PresentDocumentVcs {
        store: RefCell<PresentStore>,
    }

    #[wasm_bindgen(js_name = createPresentEnvelopeJson)]
    pub fn create_present_envelope_json(id: &str) -> Result<String, JsValue> {
        serde_json::to_string(&create_present_envelope(id)).map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen(js_name = materializePresentProjectionJson)]
    pub fn materialize_present_projection_json_wasm(envelope_json: &str) -> Result<String, JsValue> {
        let deck = materialize_present_projection_json(envelope_json).map_err(|error| JsValue::from_str(&error.to_string()))?;
        serde_json::to_string(&deck).map_err(|error| JsValue::from_str(&error.to_string()))
    }

    #[wasm_bindgen]
    impl PresentDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<PresentDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: PresentEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    PresentStore::new(envelope)
                }
                None => PresentStore::new(create_document_envelope(PRESENT_DECK_SCHEMA, "animate-present", empty_present_deck(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use present::default_present_deck;
    use protocol::CollectionOperation;
    use store::{test_support, DocumentCommand};

    #[test]
    fn envelope_helpers_round_trip() {
        let envelope = create_present_envelope("deck-1");
        let json = serde_json::to_string(&envelope).expect("serialize");
        let deck = materialize_present_projection_json(&json).expect("materialize");
        assert_eq!(deck.schema, PRESENT_DECK_SCHEMA);
        assert!(deck.tiles.is_empty());
    }

    #[test]
    fn present_deck_materializes() {
        let mut store = PresentStore::new(create_document_envelope(PRESENT_DECK_SCHEMA, "animate-present", empty_present_deck(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![PresentOperation::Tiles(CollectionOperation::Add {
                    id: "t1".into(),
                    item: present::FigureTileDraft { id: "t1".into(), name: "A".into(), crop: present::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } },
                    at: 0,
                })],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").tiles.len(), 1);
    }

    //#region 🔖DocumentTextTests
    #[test]
    fn document_text_round_trip_with_operation_applied() {
        let mut store = PresentStore::new(create_document_envelope(PRESENT_DECK_SCHEMA, "animate-present", default_present_deck(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![PresentOperation::Tiles(CollectionOperation::Add {
                    id: "t1".into(),
                    item: present::FigureTileDraft { id: "t1".into(), name: "A".into(), crop: present::FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 } },
                    at: 0,
                })],
                description: None,
            })
            .expect("apply");
        test_support::assert_document_text_round_trip(&store);
        test_support::assert_document_pack_round_trip(&store);
    }
    //#endregion 🔖DocumentTextTests
}
//#endregion 🧪Tests
