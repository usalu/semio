//! 🌿️ VCS artifact — the document entity the `vcs-play` app edits.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

//#region 🔖️Types
pub const VCS_DEMO_SCHEMA: &str = "vcs.demo";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "vcsdemo")]
pub struct VcsDemoProjection {
    pub schema: String,
    pub title: String,
    pub counter: i64,
    pub notes: String,
    pub status: String,
    pub tags: Vec<String>,
}
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::vcs::create_vcs_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "vcs.document".into(),
        name: "VCS Document".into(),
        source_format: VCS_DEMO_SCHEMA.into(),
        component_kind: "vcs".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: VCS_DEMO_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
    }
}
//#endregion 🔖️ArtifactKind
