//! 🕸️ CAD play app — the `wasm32` VCS bridge: a `wasm-bindgen` handle over the cad `ArtifactStore`
//! so a browser host can dispatch `.cad` op text / op binary and read the materialized projection
//! back without going through the component ABI. Compiled out on every other target.

#[cfg(target_arch = "wasm32")]
mod bridge {
    use crate::artifacts::cad::spr::{CadEnvelope, CadStore};
    use crate::artifacts::cad::{empty_cad_snapshot, CAD_DOCUMENT_SCHEMA};
    use std::cell::RefCell;
    use store::create_document_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct CadArtifactVcs {
        store: RefCell<CadStore>,
    }

    #[wasm_bindgen]
    impl CadArtifactVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<CadArtifactVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: CadEnvelope = store::reject_whole_buffer_artifact_envelope_ingress(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    CadStore::new(envelope).map_err(|e| JsValue::from_str(&e.to_string()))?
                }
                None => CadStore::new(create_document_envelope(CAD_DOCUMENT_SCHEMA, "cad", empty_cad_snapshot(), None)).map_err(|e| JsValue::from_str(&e.to_string()))?,
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
        pub fn snapshot_json(&self) -> Result<String, JsValue> {
            self.store.borrow().snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
}
