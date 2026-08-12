//! 🗂️ GIS 2D play app — the raw wasm-bindgen JS binding surface for `GisMapSnapshot`'s VCS store.
//!
//! Independent of the `App`/`ArtifactApp` plugin-registry path (`create_gis2d_app`/`Gis2dPlayApp`),
//! this exposes the document store directly for callers that talk to the compiled wasm module without
//! going through the host's app registry.

//#region 🔖️WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use crate::artifacts::gismap::schema::empty_gis_map_snapshot;
    use crate::artifacts::gismap::op::{GisMapEnvelope, GisMapStore};
    use crate::artifacts::gismap::GIS_MAP_SCHEMA;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct GisMapSnapshotVcs {
        store: RefCell<GisMapStore>,
    }

    #[wasm_bindgen]
    impl GisMapSnapshotVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<GisMapSnapshotVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: GisMapEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    GisMapStore::new(envelope)
                }
                None => GisMapStore::new(store::create_document_envelope(GIS_MAP_SCHEMA, "gis", empty_gis_map_snapshot(), None)),
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
            self.store.borrow().snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))
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
//#endregion 🔖️WasmBridge
