//! 🌉️ Writer play app — editor-host aliases and the wasm-bindgen document VCS bridge (was: the
//! plugin-root `document_vcs` module + `WriterHost`/`WriterSession` aliases in the old bundle crate's
//! `📦️glue.rs`).

pub use framework_editor::*;

pub type WriterHost = EditorHost;

#[cfg(target_arch = "wasm32")]
pub type WriterSession = EditorSession;

#[cfg(target_arch = "wasm32")]
mod document_vcs {
    //#region 🔖️DocumentVcs
    use std::cell::RefCell;

    use wasm_bindgen::prelude::*;

    use store::{DocumentEnvelope, DocumentStore};

    use crate::artifacts::writer::op::WriterOperation;
    use crate::artifacts::writer::WriterProjection;

    type WriterEnvelope = DocumentEnvelope<WriterProjection, WriterOperation>;
    type WriterStore = DocumentStore<WriterProjection, WriterOperation>;

    #[wasm_bindgen]
    pub struct WriterDocumentVcs {
        store: RefCell<WriterStore>,
    }

    #[wasm_bindgen]
    impl WriterDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: &str) -> Result<WriterDocumentVcs, JsValue> {
            let envelope: WriterEnvelope = serde_json::from_str(envelope_json).map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(Self { store: RefCell::new(WriterStore::new(envelope)) })
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
    //#endregion 🔖️DocumentVcs
}

#[cfg(target_arch = "wasm32")]
pub use document_vcs::*;
