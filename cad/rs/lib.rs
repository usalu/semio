//! 📐 CAD scene document + Rust-backed VCS on `framework_vcs`.

use framework_vcs::JsonDocumentStore;
use serde_json::{json, Value};

pub const CAD_DOCUMENT_SCHEMA: &str = "cad.scene/v1";

pub fn empty_cad_projection() -> Value {
    json!({ "schema": CAD_DOCUMENT_SCHEMA, "id": "cad", "nodes": [] })
}

#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct CadDocumentVcs {
        store: RefCell<JsonDocumentStore>,
    }

    #[wasm_bindgen]
    impl CadDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<CadDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => JsonDocumentStore::from_envelope_json(&json).map_err(|e| JsValue::from_str(&e.to_string()))?,
                None => JsonDocumentStore::new(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection()),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_json(command_json).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cad_projection_defaults() {
        let store = JsonDocumentStore::new(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_projection());
        assert!(store.projection_json().expect("projection").contains("cad"));
    }
}
