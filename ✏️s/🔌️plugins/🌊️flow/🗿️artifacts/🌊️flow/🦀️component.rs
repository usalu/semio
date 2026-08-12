//! 🌊️ Flow artifact — the document entity this plugin's apps edit.
//!
//! The persisted snapshot type is [`FlowSnapshot`] (this plugin). The framework crate
//! `semio-framework-os-flow` still owns a separate `flow::FlowFixture` used by `FlowHost` and by
//! other plugins (e.g. procedural) that embed a flow graph; conversions live on `FlowSnapshot`.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️Types
pub use crate::artifacts::flow::snapshot::schema::FlowSnapshot;
pub use flow::FLOW_DOCUMENT_SCHEMA;
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::flow::create_flow_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "computation.flow".into(),
        name: "Flow".into(),
        source_format: "flow.artifact".into(),
        component_kind: "flow".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType {
            class: MediaClass::Computation,
            form: MediaForm::Flow,
        },
        schema: "flow.artifact".into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("flow.artifact") is deliberately NOT
    /// `FLOW_DOCUMENT_SCHEMA` ("flow.fixture") — the former names the artifact kind in the OS media
    /// catalogue, the latter keys the store envelope. Pinned so a future edit can't silently merge them.
    #[test]
    fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
        assert_eq!(artifact_kind().schema, "flow.artifact");
        assert_eq!(FLOW_DOCUMENT_SCHEMA, "flow.fixture");
    }

    #[test]
    fn default_snapshot_has_widgets() {
        assert!(!FlowSnapshot::default().widgets.is_empty());
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::flow::standards::v1::engine::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("FlowComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
