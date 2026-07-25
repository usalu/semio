//! ✍️ Writer WASM package: re-exports framework editor and writer document VCS.

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

    use vcs::{DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff};

    /// 📷 Editor viewport transform persisted in the document projection.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WriterCamera {
        #[serde(default)]
        pub x: f64,
        #[serde(default)]
        pub y: f64,
        #[serde(default = "default_zoom")]
        pub zoom: f64,
    }

    fn default_zoom() -> f64 {
        1.0
    }

    fn default_uri() -> String {
        "writer://empty".into()
    }

    fn default_camera() -> WriterCamera {
        WriterCamera { x: 0.0, y: 0.0, zoom: 1.0 }
    }

    /// 📝 The full writer document projection: identity, language, source text and camera.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WriterProjection {
        pub schema: String,
        pub id: String,
        pub language_id: String,
        #[serde(default = "default_uri")]
        pub uri: String,
        #[serde(default)]
        pub text: String,
        #[serde(default = "default_camera")]
        pub camera: WriterCamera,
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(tag = "operation", rename_all = "camelCase")]
    pub enum WriterOperation {
        SetText { text: String },
        SetCamera { camera: WriterCamera },
        SetDocument { document: WriterProjection },
    }

    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WriterDiff {
        pub text: Option<String>,
        pub camera: Option<WriterCamera>,
        pub document: Option<WriterProjection>,
    }

    impl OperationDiff<WriterProjection> for WriterDiff {
        fn apply(&self, projection: &WriterProjection) -> WriterProjection {
            if let Some(document) = &self.document {
                return document.clone();
            }
            WriterProjection { text: self.text.clone().unwrap_or_else(|| projection.text.clone()), camera: self.camera.clone().unwrap_or_else(|| projection.camera.clone()), ..projection.clone() }
        }

        fn absorb(&mut self, other: Self) {
            if other.document.is_some() {
                *self = other;
                return;
            }
            if other.text.is_some() {
                self.text = other.text;
            }
            if other.camera.is_some() {
                self.camera = other.camera;
            }
        }
    }

    impl Operation<WriterProjection> for WriterOperation {
        type Diff = WriterDiff;

        fn diff(&self, _projection: &WriterProjection) -> WriterDiff {
            match self {
                WriterOperation::SetText { text } => WriterDiff { text: Some(text.clone()), ..Default::default() },
                WriterOperation::SetCamera { camera } => WriterDiff { camera: Some(camera.clone()), ..Default::default() },
                WriterOperation::SetDocument { document } => WriterDiff { document: Some(document.clone()), ..Default::default() },
            }
        }

        fn backwards(&self, projection: &WriterProjection) -> Vec<Self> {
            match self {
                WriterOperation::SetText { .. } => vec![WriterOperation::SetText { text: projection.text.clone() }],
                WriterOperation::SetCamera { .. } => vec![WriterOperation::SetCamera { camera: projection.camera.clone() }],
                WriterOperation::SetDocument { .. } => vec![WriterOperation::SetDocument { document: projection.clone() }],
            }
        }
    }

    pub type WriterEnvelope = DocumentVcsEnvelope<WriterProjection, WriterOperation>;
    pub type WriterStore = DocumentVcsStore<WriterProjection, WriterOperation>;

    pub fn empty_writer_projection() -> WriterProjection {
        WriterProjection { schema: "writer.document".into(), id: "empty".into(), language_id: "plaintext".into(), uri: "writer://empty".into(), text: String::new(), camera: default_camera() }
    }

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

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_json(command_json).map_err(|e| JsValue::from_str(&e.to_string()))
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

    #[cfg(test)]
    mod writer_vcs_tests {
        use super::*;
        use vcs::{create_document_vcs_envelope, DocumentVcsCommand};

        fn seeded_store() -> WriterStore {
            WriterStore::new(create_document_vcs_envelope("writer.document", "writer", empty_writer_projection(), None))
        }

        #[test]
        fn writer_document_vcs_replays_text_operations() {
            let mut store = seeded_store();
            store.dispatch(DocumentVcsCommand::Apply { operations: vec![WriterOperation::SetText { text: "hello".into() }], description: None }).expect("apply");
            assert_eq!(store.projection().expect("projection").text, "hello");
        }

        #[test]
        fn writer_document_vcs_replays_camera_and_document_operations() {
            let mut store = seeded_store();
            store.dispatch(DocumentVcsCommand::Apply { operations: vec![WriterOperation::SetCamera { camera: WriterCamera { x: 4.0, y: 5.0, zoom: 2.0 } }], description: None }).expect("apply camera");
            let projection = store.projection().expect("projection");
            assert_eq!(projection.camera.x, 4.0);
            assert_eq!(projection.camera.zoom, 2.0);

            let replacement = WriterProjection { schema: "writer.document".into(), id: "jack".into(), language_id: "jack".into(), uri: "writer://jack".into(), text: "MATCH (a) RETURN a".into(), camera: default_camera() };
            store.dispatch(DocumentVcsCommand::Apply { operations: vec![WriterOperation::SetDocument { document: replacement }], description: None }).expect("apply document");
            let projection = store.projection().expect("projection");
            assert_eq!(projection.id, "jack");
            assert_eq!(projection.text, "MATCH (a) RETURN a");
        }

        #[test]
        fn writer_document_vcs_undoes_text_operation() {
            let mut store = seeded_store();
            store.dispatch(DocumentVcsCommand::Apply { operations: vec![WriterOperation::SetText { text: "hello".into() }], description: None }).expect("apply");
            store.dispatch(DocumentVcsCommand::Undo).expect("undo");
            assert_eq!(store.projection().expect("projection").text, "");
        }
    }
    // #endregion 🔖DocumentVcs
    // #endregion document_vcs
}

pub use document_vcs::*;
