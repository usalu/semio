//! 🖼️ Raster plugin — declarative raster board bundled as a hot-swappable WASM plugin.

//#region 🔖️WasmDocumentVcs
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct RasterDocumentVcs {
    store: RefCell<raster_op::RasterStore>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl RasterDocumentVcs {
    #[wasm_bindgen(constructor)]
    pub fn new(envelope_json: &str) -> Result<RasterDocumentVcs, JsValue> {
        let envelope: raster_op::RasterEnvelope = serde_json::from_str(envelope_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(Self { store: RefCell::new(raster_op::RasterStore::new(envelope)) })
    }

    #[wasm_bindgen(js_name = dispatchText)]
    pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
        self.store.borrow_mut().dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = dispatchBinary)]
    pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
        self.store.borrow_mut().dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
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
//#endregion 🔖️WasmDocumentVcs

fn register_raster_exports() {
    semio_framework_os::register_2d_export_handlers("2d.raster", "raster", raster_engine::raster_document_json_to_svg);
    semio_framework_os::register_dwg_import_handler("2d.raster", raster_engine::raster_document_json_from_dwg);
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<raster_ui::RasterPlayApp>(raster::RASTER_DOCUMENT_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "raster", label: "Raster", version: "0.1.0",
    setup: register_raster_exports,
    apps: [ raster_ui::create_raster_app => raster_ui::RasterPlayApp ],
}
