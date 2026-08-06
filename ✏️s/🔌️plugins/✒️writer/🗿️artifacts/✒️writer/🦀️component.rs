//! ✒️ Writer artifact — the document entity this plugin's app edits.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
pub const WRITER_DOCUMENT_SCHEMA: &str = "writer.document";
//#endregion 🔖️Constants

//#region 🔖️Types
/// 📷️ Editor viewport transform — session-only runtime state (see `WriterConfig::camera` in the app's
/// `🦀️config.rs`), never a `WriterProjection` document field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WriterCamera {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "default_zoom")]
    pub zoom: f64,
}

impl Default for WriterCamera {
    fn default() -> Self {
        default_camera()
    }
}

pub fn default_zoom() -> f64 {
    1.0
}

pub fn default_uri() -> String {
    "writer://empty".into()
}

pub fn default_camera() -> WriterCamera {
    WriterCamera { x: 0.0, y: 0.0, zoom: 1.0 }
}

/// 📝️ The full writer document projection: identity, language and source text. The editor viewport
/// camera is session-only view state (never a document field) — see `WriterCamera`'s doc comment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "writer.writer", layout = "lines")]
pub struct WriterProjection {
    pub schema: String,
    pub id: String,
    pub language_id: String,
    #[serde(default = "default_uri")]
    pub uri: String,
    #[serde(default)]
    #[dsl(lang_from = "language_id")]
    pub text: String,
}
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::writer::create_writer_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "text.document".into(),
        name: "Text Document".into(),
        source_format: WRITER_DOCUMENT_SCHEMA.into(),
        component_kind: "writer".into(),
        dimension: "text".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: WRITER_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_kind_keeps_the_media_schema_matching_the_store_schema() {
        assert_eq!(artifact_kind().schema, WRITER_DOCUMENT_SCHEMA);
        assert_eq!(WRITER_DOCUMENT_SCHEMA, "writer.document");
    }

    #[test]
    fn default_camera_is_centered_and_unzoomed() {
        assert_eq!(WriterCamera::default(), WriterCamera { x: 0.0, y: 0.0, zoom: 1.0 });
    }
}
//#endregion 🧪️Tests
