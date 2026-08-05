//! 🌊️ Flow artifact — the document entity this plugin's apps edit.
//!
//! Unlike most artifacts, `FlowFixture`'s fields, `Widget`/`SynapseSpec` variants and the
//! `FLOW_DOCUMENT_SCHEMA` constant are NOT owned here — they live in the shared flow kernel crate
//! ([`flow_core`], `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core`) because multiple apps compile
//! against the same flow domain model. This component re-exports the app-facing surface so every sibling
//! taxonomy node (`🔺️diff`, `🔧️op`, `🗣️dsl`, `🎒️pack`, `📡️spr`, `⚙️engine`) names one artifact-owned
//! symbol instead of reaching into the kernel path directly.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️Types
pub use flow_core::{FlowFixture, FLOW_DOCUMENT_SCHEMA};
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::flow::create_flow_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "computation.flow".into(),
        name: "Flow".into(),
        source_format: "flow.document".into(),
        component_kind: "flow".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Flow },
        schema: "flow.document".into(),
        export_formats: vec![],
        import_formats: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("flow.document") is deliberately NOT
    /// `FLOW_DOCUMENT_SCHEMA` ("flow.fixture") — the former names the artifact kind in the OS media
    /// catalogue, the latter keys the store envelope. Pinned so a future edit can't silently merge them.
    #[test]
    fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
        assert_eq!(artifact_kind().schema, "flow.document");
        assert_eq!(FLOW_DOCUMENT_SCHEMA, "flow.fixture");
    }

    #[test]
    fn default_fixture_has_widgets() {
        assert!(!FlowFixture::default().widgets.is_empty());
    }
}
//#endregion 🧪️Tests
