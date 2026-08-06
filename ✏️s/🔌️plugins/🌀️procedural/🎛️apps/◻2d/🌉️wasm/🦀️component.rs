//! 🕸️ Procedural2d play app — the `wasm32` JS bridge (`Procedural2dDocumentVcs`).

#![cfg(target_arch = "wasm32")]

use crate::artifacts::procedural2d::engine::empty_procedural2d_projection;
use crate::artifacts::procedural2d::op::{Procedural2dEnvelope, Procedural2dStore};
use crate::artifacts::procedural2d::PROCEDURAL_2D_SCHEMA;
use std::cell::RefCell;
use store::create_document_envelope;
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
                let envelope: Procedural2dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                Procedural2dStore::new(envelope)
            }
            None => Procedural2dStore::new(create_document_envelope(PROCEDURAL_2D_SCHEMA, "procedural2d", empty_procedural2d_projection(), None)),
        };
        Ok(Self { store: RefCell::new(store) })
    }

    #[wasm_bindgen(js_name = dispatchText)]
    pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
        self.store.borrow_mut().dispatch_text(command_text).map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = dispatchBinary)]
    pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
        self.store.borrow_mut().dispatch_binary(command_bytes).map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = projectionJson)]
    pub fn projection_json(&self) -> Result<String, JsValue> {
        self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = envelopeJson)]
    pub fn envelope_json(&self) -> Result<String, JsValue> {
        self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = generation)]
    pub fn generation(&self) -> u32 {
        self.store.borrow().generation() as u32
    }
}

