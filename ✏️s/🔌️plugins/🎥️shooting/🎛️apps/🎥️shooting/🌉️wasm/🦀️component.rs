//! 🌉️ Shooting play app — the `wasm-bindgen` VCS bridge: a JS-facing surface distinct from the WASM
//! Component Model plugin ABI the rest of this crate speaks. Only compiled for `target_arch = "wasm32"`.

use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::ShootingFixture;

//#region 🔖️Store
pub type ShootingEnvelope = store::DocumentEnvelope<ShootingFixture, ShootingMutation>;
pub type ShootingStore = store::DocumentStore<ShootingFixture, ShootingMutation>;
//#endregion 🔖️Store

//#region 🔖️WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use crate::artifacts::shooting::SHOOTING_FIXTURE_SCHEMA;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct ShootingDocumentVcs {
        store: RefCell<ShootingStore>,
    }

    #[wasm_bindgen]
    impl ShootingDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<ShootingDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: ShootingEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    ShootingStore::new(envelope)
                }
                None => ShootingStore::new(store::create_document_envelope(SHOOTING_FIXTURE_SCHEMA, "shooting", crate::artifacts::shooting::empty_shooting_fixture(), None)),
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
    }
}
//#endregion 🔖️WasmBridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ The non-wasm32 half of this file (the type aliases) must still compile and be usable from a
    /// native test — the `wasm_bridge` module itself only builds under `target_arch = "wasm32"`.
    #[test]
    fn shooting_store_type_alias_constructs_from_an_empty_envelope() {
        let store = ShootingStore::new(store::create_document_envelope(crate::artifacts::shooting::SHOOTING_FIXTURE_SCHEMA, "shooting", crate::artifacts::shooting::empty_shooting_fixture(), None));
        assert!(store.projection().expect("projection").assets.is_empty());
    }
}
//#endregion 🧪️Tests
