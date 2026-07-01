//! ✏️ Draw document domain + Rust-backed VCS on `framework_vcs`.

use framework_vcs::JsonDocumentStore;
use serde_json::{json, Value};

pub const DRAW_DOCUMENT_SCHEMA: &str = "draw.document/v1";

pub fn empty_draw_projection() -> Value {
    json!({
        "schema": DRAW_DOCUMENT_SCHEMA,
        "id": "draw",
        "version": "1",
        "layers": []
    })
}

pub fn create_draw_envelope_json(id: &str) -> String {
    serde_json::to_string(&framework_vcs::create_document_vcs_envelope(
        DRAW_DOCUMENT_SCHEMA,
        id,
        empty_draw_projection(),
        None,
    ))
    .expect("draw envelope")
}

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct DrawDocumentVcs {
        store: RefCell<JsonDocumentStore>,
    }

    #[wasm_bindgen]
    impl DrawDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<DrawDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => JsonDocumentStore::from_envelope_json(&json).map_err(|e| JsValue::from_str(&e.to_string()))?,
                None => JsonDocumentStore::new(DRAW_DOCUMENT_SCHEMA, "draw", empty_draw_projection()),
            };
            Ok(Self {
                store: RefCell::new(store),
            })
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

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use framework_vcs::{DocumentVcsCommand, json_replace_op};

    #[test]
    fn draw_document_vcs_materializes() {
        let mut store = JsonDocumentStore::new(DRAW_DOCUMENT_SCHEMA, "draw", empty_draw_projection());
        store
            .dispatch_json(&serde_json::to_string(&DocumentVcsCommand::Apply {
                forwards: vec![json_replace_op(json!({ "schema": DRAW_DOCUMENT_SCHEMA, "id": "patched", "version": "1", "layers": [] }))],
                backwards: vec![json_replace_op(empty_draw_projection())],
                description: None,
            }).unwrap())
            .expect("apply");
        let projection: Value = serde_json::from_str(&store.projection_json().expect("projection")).expect("json");
        assert_eq!(projection["id"], "patched");
    }
}
//#endregion 🧪Tests
