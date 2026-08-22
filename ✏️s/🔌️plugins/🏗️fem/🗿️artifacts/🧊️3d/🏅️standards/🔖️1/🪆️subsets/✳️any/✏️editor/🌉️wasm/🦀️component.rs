//! 🌐️ FEM 3D app — the wasm bridge (constitutional: ui's `🔖️WasmBridge` region), `#[cfg(target_arch =
//! "wasm32")]`-gated.

#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use crate::artifacts::fem3d::mutations::{Fem3dEnvelope, Fem3dStore};
    use std::cell::RefCell;
    use store::create_document_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Fem3dSnapshotVcs {
        store: RefCell<Fem3dStore>,
    }

    #[wasm_bindgen]
    impl Fem3dSnapshotVcs {
        #[wasm_bindgen(js_name = create)]
        pub async fn create(envelope_json: Option<String>) -> Result<Fem3dSnapshotVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Fem3dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Fem3dStore::new(envelope).await.map_err(|e| JsValue::from_str(&e.to_string()))?
                }
                None => Fem3dStore::new(create_document_envelope(crate::artifacts::fem3d::FEM_3D_SCHEMA, "fem3d", crate::artifacts::fem3d::schema::empty_fem3d_snapshot(), None)).await.map_err(|e| JsValue::from_str(&e.to_string()))?,
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

        #[wasm_bindgen(js_name = projectionJson)]
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
}
