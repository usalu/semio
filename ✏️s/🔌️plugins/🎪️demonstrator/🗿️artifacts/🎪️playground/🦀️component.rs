//! 🎪️ Playground artifact — demonstrator's owned document entity (minimal schema stub).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub const PLAYGROUND_DOCUMENT_SCHEMA: &str = "playground.playground";
pub use crate::artifacts::playground::snapshot::schema::PlaygroundSnapshot;

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "playground.document".into(),
        name: "Playground Document".into(),
        source_format: PLAYGROUND_DOCUMENT_SCHEMA.into(),
        component_kind: "playground".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: PLAYGROUND_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
    }
}
//#endregion 🔖️ArtifactKind
