//! ✍️ Writer plugin — declarative writer app bundled as a hot-swappable WASM plugin.

pub use framework_editor::*;

pub type WriterHost = EditorHost;

#[cfg(target_arch = "wasm32")]
pub type WriterSession = EditorSession;

mod document_vcs {
    // #region document_vcs
    // #region 🔖DocumentVcs
    #[cfg(target_arch = "wasm32")]
    use std::cell::RefCell;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen::prelude::*;

    #[cfg(target_arch = "wasm32")]
    use store::{DocumentEnvelope, DocumentStore};

    #[cfg(target_arch = "wasm32")]
    type WriterEnvelope = DocumentEnvelope<writer::WriterProjection, writer_op::WriterOperation>;
    #[cfg(target_arch = "wasm32")]
    type WriterStore = DocumentStore<writer::WriterProjection, writer_op::WriterOperation>;

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub struct WriterDocumentVcs {
        store: RefCell<WriterStore>,
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    impl WriterDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: &str) -> Result<WriterDocumentVcs, JsValue> {
            let envelope: WriterEnvelope = serde_json::from_str(envelope_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(Self { store: RefCell::new(WriterStore::new(envelope)) })
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
    // #endregion 🔖DocumentVcs
    // #endregion document_vcs
}

#[cfg(target_arch = "wasm32")]
pub use document_vcs::*;

//#region 🔖Manifest
/// 🗂️ Registers `WriterProjection`'s pack↔dsl codec so `framework/sync`'s `FolderEndpoint::Pack`
/// (and any other schema-string-keyed caller) can print/parse it without depending on this crate's
/// concrete `Projection`/`Operation` types.
fn register_writer_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<writer_ui::WriterPlayApp>(writer::WRITER_DOCUMENT_SCHEMA);
}

semio_framework_plugin::semio_plugin! {
    id: "writer", label: "Writer", version: "0.1.0",
    setup: register_writer_exports,
    apps: [ writer_ui::create_writer_app => writer_ui::WriterPlayApp ],
}
//#endregion 🔖Manifest

//#region 🧪Tests
#[cfg(test)]
mod tests {
    /// 📚 `manifest.examples`/`manifest.apps` are only reachable through the `__semio_plugin_bundle()`
    /// item the `semio_plugin!` macro generates in this crate — so unlike every other app-behavior
    /// test (which lives in `writer_ui`), this one must stay at the bundle layer.
    #[test]
    fn manifest_includes_dag_jack_example() {
        let bundle = super::__semio_plugin_bundle();
        let manifest = &bundle.manifest;
        assert!(manifest.apps.iter().any(|a| a.id == writer_ui::WRITER_PLAY_APP_ID));
        assert!(manifest.examples.iter().any(|e| e.id == "dag.jack" && e.app_id == writer_ui::WRITER_PLAY_APP_ID));
    }
}
//#endregion 🧪Tests
