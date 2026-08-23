//! 🌉️ Shooting play app — the `wasm-bindgen` VCS bridge: a JS-facing surface distinct from the WASM
//! Component Model plugin ABI the rest of this crate speaks. Only compiled for `target_arch = "wasm32"`.

use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::ShootingSnapshot;

//#region 🔖️Store
pub type ShootingEnvelope = store::ArtifactEnvelope<ShootingSnapshot, ShootingMutation>;
pub type ShootingStore = store::ArtifactStore<ShootingSnapshot, ShootingMutation>;
//#endregion 🔖️Store

//#region 🔖️WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use crate::artifacts::shooting::SHOOTING_DOCUMENT_SCHEMA;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct ShootingArtifactVcs {
        store: RefCell<ShootingStore>,
    }

    #[wasm_bindgen]
    impl ShootingArtifactVcs {
        #[wasm_bindgen(constructor)]
        pub async fn new(envelope_json: Option<String>) -> Result<ShootingArtifactVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: ShootingEnvelope = store::reject_whole_buffer_artifact_envelope_ingress(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    ShootingStore::new(envelope).map_err(|e| JsValue::from_str(&e.to_string()))?
                }
                None => ShootingStore::new(store::create_document_envelope(SHOOTING_DOCUMENT_SCHEMA, "shooting", crate::artifacts::shooting::empty_shooting_snapshot(), None)).map_err(|e| JsValue::from_str(&e.to_string()))?,
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub async fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub async fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map(|_| ()).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub async fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }
    }
}
//#endregion 🔖️WasmBridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ The non-wasm32 half of this file (the type aliases) must still compile and be usable from a
    /// native test — the `wasm_bridge` module itself only builds under `target_arch = "wasm32"`.
    #[semio_framework_async_macros::async_test]
    async fn shooting_store_type_alias_constructs_from_an_empty_envelope() {
        let store = ShootingStore::new(store::create_document_envelope(crate::artifacts::shooting::SHOOTING_DOCUMENT_SCHEMA, "shooting", crate::artifacts::shooting::empty_shooting_snapshot(), None));
        assert!(store.snapshot().expect("snapshot").assets.is_empty());
    }
}
//#endregion 🧪️Tests
