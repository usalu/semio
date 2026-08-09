//! ✒️ Writer artifact — the document entity this plugin's app edits.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

//#region 🔖️Constants
pub const WRITER_DOCUMENT_SCHEMA: &str = "writer.document";
//#endregion 🔖️Constants

//#region 🔖️Types
/// 📷️ Editor viewport transform — session-only runtime state (flattened on the artifact for schema).
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

/// 📐️ Editor text selection range.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct WriterEditorSelection {
    pub start: usize,
    pub end: usize,
}

/// ⚙️ Editor chrome settings.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase", default)]
pub struct WriterEditorSettings {
    pub show_line_numbers: bool,
    pub font_px: u32,
    pub line_height: u32,
    pub tab_size: u32,
}

impl Default for WriterEditorSettings {
    fn default() -> Self {
        Self {
            show_line_numbers: true,
            font_px: 14,
            line_height: 22,
            tab_size: 2,
        }
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

pub use crate::artifacts::writer::snapshot::schema::WriterSnapshot;
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
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
