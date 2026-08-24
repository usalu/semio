//! 🕸️ Procedural3d play app — the `wasm32` JS bridge (`Procedural3dSnapshotVcs`).

#![cfg(target_arch = "wasm32")]

use crate::artifacts::procedural3d::mutations::{Procedural3dEnvelope, Procedural3dStore};
use crate::artifacts::procedural3d::schema::empty_procedural3d_snapshot;
use crate::artifacts::procedural3d::PROCEDURAL_3D_SCHEMA;
use std::cell::RefCell;
use store::create_document_envelope;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Procedural3dSnapshotVcs {
    store: RefCell<Procedural3dStore>,
}

#[wasm_bindgen]
impl Procedural3dSnapshotVcs {
    #[wasm_bindgen(constructor)]
    pub async fn new(envelope_json: Option<String>) -> Result<Procedural3dSnapshotVcs, JsValue> {
        let store = match envelope_json {
            Some(json) => {
                let envelope: Procedural3dEnvelope = store::reject_whole_buffer_artifact_envelope_ingress(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                Procedural3dStore::new(envelope).await.map_err(|e| JsValue::from_str(&e.to_string()))?
            }
            None => Procedural3dStore::new(create_document_envelope(PROCEDURAL_3D_SCHEMA, "procedural3d", empty_procedural3d_snapshot(), None)).await.map_err(|e| JsValue::from_str(&e.to_string()))?,
        };
        Ok(Self { store: RefCell::new(store) })
    }

    #[wasm_bindgen(js_name = dispatchText)]
    pub async fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
        self.store.borrow_mut().dispatch_text(command_text).await.map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = dispatchBinary)]
    pub async fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
        self.store.borrow_mut().dispatch_binary(command_bytes).await.map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = snapshotJson)]
    pub async fn snapshot_json(&self) -> Result<String, JsValue> {
        self.store.borrow().snapshot_json().await.map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = envelopeJson)]
    pub async fn envelope_json(&self) -> Result<String, JsValue> {
        self.store.borrow().envelope_json().await.map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = generation)]
    pub async fn generation(&self) -> u32 {
        self.store.borrow().generation().await as u32
    }
}
