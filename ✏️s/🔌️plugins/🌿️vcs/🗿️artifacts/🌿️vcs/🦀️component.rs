//! 🌿️ VCS artifact — the document entity the `vcs-play` app edits.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub const VCS_DOCUMENT_SCHEMA: &str = "vcs.vcs";
pub use crate::artifacts::vcs::snapshot::schema::VcsSnapshot;

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::vcs::create_vcs_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "vcs.document".into(),
        name: "VCS Document".into(),
        source_format: VCS_DOCUMENT_SCHEMA.into(),
        component_kind: "vcs".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: VCS_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
