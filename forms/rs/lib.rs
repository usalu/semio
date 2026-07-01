//! 📋 Forms document domain + Rust-backed VCS on `framework_vcs`.

use framework_vcs::JsonDocumentStore;
use serde_json::{json, Value};

pub const FORMS_DOCUMENT_SCHEMA: &str = "forms.form/v1";

pub fn empty_forms_projection() -> Value {
    json!({
        "schema": FORMS_DOCUMENT_SCHEMA,
        "id": "forms",
        "version": "1",
        "steps": [{ "id": "s", "title": "Inputs", "questions": [] }]
    })
}

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct FormsDocumentVcs {
        store: RefCell<JsonDocumentStore>,
    }

    #[wasm_bindgen]
    impl FormsDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<FormsDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => JsonDocumentStore::from_envelope_json(&json).map_err(|e| JsValue::from_str(&e.to_string()))?,
                None => JsonDocumentStore::new(FORMS_DOCUMENT_SCHEMA, "forms", empty_forms_projection()),
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

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forms_document_vcs_materializes() {
        let store = JsonDocumentStore::new(FORMS_DOCUMENT_SCHEMA, "forms", empty_forms_projection());
        let projection: Value = serde_json::from_str(&store.projection_json().expect("projection")).expect("json");
        assert_eq!(projection["schema"], FORMS_DOCUMENT_SCHEMA);
    }
}
//#endregion 🧪Tests
