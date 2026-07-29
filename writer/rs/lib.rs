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

    use protocol::{Operation, OperationDiff};
    use store::{DocumentEnvelope, DocumentStore};

    /// 📷 Editor viewport transform persisted in the document projection. No `#[dsl(keyword = ...)]`:
    /// every field that embeds it (`WriterProjection::camera`, `WriterOperation::SetCamera::camera`)
    /// is itself `#[dsl(block)]`, which already supplies the bare leading keyword.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord)]
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
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslDocument)]
    #[serde(rename_all = "camelCase")]
    #[dsl(extension = "writer", layout = "lines")]
    pub struct WriterProjection {
        pub schema: String,
        pub id: String,
        pub language_id: String,
        #[serde(default = "default_uri")]
        pub uri: String,
        #[serde(default)]
        pub text: String,
        #[serde(default = "default_camera")]
        #[dsl(block)]
        pub camera: WriterCamera,
    }

    /// 📐 Typed content mutation for a `WriterProjection`. Each variant's op keyword is the
    /// auto-derived kebab-case of its own name (`SetText` -> `set-text`, ...) — see {@link protocol::OpText}.
    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslOps)]
    #[serde(tag = "operation", rename_all = "camelCase")]
    pub enum WriterOperation {
        SetText { text: String },
        SetCamera {
            #[dsl(block)]
            camera: WriterCamera,
        },
        SetDocument {
            #[dsl(block)]
            document: WriterProjection,
        },
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

    // #region 🔖Dsl
    // `store::DocumentDsl for WriterProjection` and `protocol::OpText for WriterOperation` are now generated
    // by `#[derive(dsl::DslDocument)]`/`#[derive(dsl::DslOps)]` on the type definitions above — the
    // engine's `dsl_schema` grammar replaces this crate's own hand-rolled kv printer (the old
    // `writer_dsl` lexer/parser/printer module was deleted after regenerating `writer-program`'s two
    // fixture files to the new canonical format).
    // #endregion 🔖Dsl

    pub type WriterEnvelope = DocumentEnvelope<WriterProjection, WriterOperation>;
    pub type WriterStore = DocumentStore<WriterProjection, WriterOperation>;

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

    #[cfg(test)]
    mod writer_vcs_tests {
        use super::*;
        use store::{create_document_envelope, DocumentDsl, DocumentCommand};

        fn seeded_store() -> WriterStore {
            WriterStore::new(create_document_envelope("writer.document", "writer", empty_writer_projection(), None))
        }

        #[test]
        fn writer_document_vcs_replays_text_operations() {
            let mut store = seeded_store();
            store.dispatch(DocumentCommand::Apply { operations: vec![WriterOperation::SetText { text: "hello".into() }], description: None }).expect("apply");
            assert_eq!(store.projection().expect("projection").text, "hello");
        }

        #[test]
        fn writer_document_vcs_replays_camera_and_document_operations() {
            let mut store = seeded_store();
            store.dispatch(DocumentCommand::Apply { operations: vec![WriterOperation::SetCamera { camera: WriterCamera { x: 4.0, y: 5.0, zoom: 2.0 } }], description: None }).expect("apply camera");
            let projection = store.projection().expect("projection");
            assert_eq!(projection.camera.x, 4.0);
            assert_eq!(projection.camera.zoom, 2.0);

            let replacement = WriterProjection { schema: "writer.document".into(), id: "jack".into(), language_id: "jack".into(), uri: "writer://jack".into(), text: "MATCH (a) RETURN a".into(), camera: default_camera() };
            store.dispatch(DocumentCommand::Apply { operations: vec![WriterOperation::SetDocument { document: replacement }], description: None }).expect("apply document");
            let projection = store.projection().expect("projection");
            assert_eq!(projection.id, "jack");
            assert_eq!(projection.text, "MATCH (a) RETURN a");
        }

        #[test]
        fn writer_document_vcs_undoes_text_operation() {
            let mut store = seeded_store();
            store.dispatch(DocumentCommand::Apply { operations: vec![WriterOperation::SetText { text: "hello".into() }], description: None }).expect("apply");
            store.dispatch(DocumentCommand::Undo).expect("undo");
            assert_eq!(store.projection().expect("projection").text, "");
        }

        //#region 🔖DslAndOpText
        fn jack_projection() -> WriterProjection {
            WriterProjection {
                schema: "writer.document".into(),
                id: "jack".into(),
                language_id: "jack".into(),
                uri: "writer://jack".into(),
                text: "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name".into(),
                camera: WriterCamera { x: 0.0, y: 0.0, zoom: 1.0 },
            }
        }

        #[test]
        fn writer_dsl_round_trips_empty_and_jack_projections() {
            store::test_support::assert_dsl_round_trip(&empty_writer_projection());
            store::test_support::assert_dsl_round_trip(&jack_projection());
            store::test_support::assert_dsl_pack_equivalence(&empty_writer_projection());
            store::test_support::assert_dsl_pack_equivalence(&jack_projection());
        }

        #[test]
        fn writer_dsl_prints_readable_multiline_text() {
            let printed = jack_projection().print_dsl();
            // Bare-ident-shaped scalars print unquoted (`is_bare_ident`); `writer://jack` contains `:`
            // and `/`, so it isn't bare and stays quoted.
            assert!(printed.contains("schema=writer.document"));
            assert!(printed.contains("id=jack"));
            assert!(printed.contains("language-id=jack"));
            assert!(printed.contains("uri=\"writer://jack\""));
            assert!(printed.contains("camera {"));
            // The multiline query text is a single escaped-quoted field (`\n`, not raw newlines); the
            // embedded Jack string literal uses the unified double-quoting too, so its own `"` is
            // backslash-escaped one level deeper inside the outer DSL string.
            assert!(printed.contains("MATCH (a:Piece)-[r:Connection]->(b:Piece)\\nWHERE a.name = \\\"core\\\"\\nRETURN a.name, b.name"));
        }

        #[test]
        fn writer_op_text_round_trips_every_variant() {
            store::test_support::assert_op_line_round_trip(&WriterOperation::SetText { text: "line one\nline two".into() });
            store::test_support::assert_op_line_round_trip(&WriterOperation::SetCamera { camera: WriterCamera { x: 4.0, y: 5.0, zoom: 2.0 } });
            store::test_support::assert_op_line_round_trip(&WriterOperation::SetDocument { document: jack_projection() });
        }

        #[test]
        fn writer_document_text_round_trips_through_the_store() {
            let mut store = seeded_store();
            store
                .dispatch(DocumentCommand::Apply { operations: vec![WriterOperation::SetDocument { document: jack_projection() }], description: None })
                .expect("apply");
            store::test_support::assert_document_text_round_trip(&store);
            store::test_support::assert_document_pack_round_trip(&store);
        }
        //#endregion 🔖DslAndOpText
    }
    // #endregion 🔖DocumentVcs
    // #endregion document_vcs
}

pub use document_vcs::*;
