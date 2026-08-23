//! 🕸️ Animate present app — the WASM VCS bridge (constitutional: was part of `ui`/`protocol`). Only
//! compiled under `target_arch = "wasm32"`; the native build never sees this module's contents.

#![cfg(target_arch = "wasm32")]

use crate::artifacts::present::spr::PresentStore;
use crate::artifacts::present::PRESENT_DOCUMENT_SCHEMA;
use std::cell::RefCell;
use store::create_document_envelope;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PresentArtifactVcs {
    store: RefCell<PresentStore>,
}

#[wasm_bindgen]
impl PresentArtifactVcs {
    #[wasm_bindgen(js_name = create)]
    pub async fn create() -> Result<PresentArtifactVcs, JsValue> {
        let store = PresentStore::new(create_document_envelope(PRESENT_DOCUMENT_SCHEMA, "animate-present", crate::artifacts::present::schema::empty_present_snapshot(), None)).await.map_err(|error| JsValue::from_str(&error.to_string()))?;
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

    #[wasm_bindgen(js_name = projectionJson)]
    pub async fn projection_json(&self) -> Result<String, JsValue> {
        self.store.borrow().snapshot_json().await.map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
