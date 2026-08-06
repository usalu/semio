//! 🌉️ Fem2d play app — the `wasm-bindgen` VCS bridge: a JS-facing surface distinct from the WASM
//! Component Model plugin ABI the rest of this crate speaks. Only compiled for `target_arch = "wasm32"`.

#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use crate::artifacts::fem2d::op::{Fem2dEnvelope, Fem2dStore};
    use std::cell::RefCell;
    use store::create_document_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Fem2dDocumentVcs {
        store: RefCell<Fem2dStore>,
    }

    #[wasm_bindgen]
    impl Fem2dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Fem2dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Fem2dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Fem2dStore::new(envelope)
                }
                None => Fem2dStore::new(create_document_envelope(crate::artifacts::fem2d::FEM_2D_SCHEMA, "fem2d", crate::artifacts::fem2d::engine::empty_fem2d_projection(), None)),
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
}

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    /// 🧪️ The store type aliases live in `crate::artifacts::fem2d::op` (`Fem2dEnvelope`/`Fem2dStore`) and
    /// are exercised by that node's own tests plus `crate::artifacts::fem2d::spr`'s
    /// `fem2d_document_text_round_trips_through_the_store` — the `wasm_bridge` module above only builds
    /// under `target_arch = "wasm32"`, so this file's non-wasm32 half has nothing native-testable of its
    /// own beyond that shared coverage.
    #[test]
    fn fem2d_store_type_alias_constructs_from_an_empty_envelope() {
        let store = crate::artifacts::fem2d::op::Fem2dStore::new(store::create_document_envelope(crate::artifacts::fem2d::FEM_2D_SCHEMA, "fem2d", crate::artifacts::fem2d::engine::empty_fem2d_projection(), None));
        assert!(store.projection().expect("projection").nodes.is_empty());
    }
}
// #endregion 🧪️Tests
