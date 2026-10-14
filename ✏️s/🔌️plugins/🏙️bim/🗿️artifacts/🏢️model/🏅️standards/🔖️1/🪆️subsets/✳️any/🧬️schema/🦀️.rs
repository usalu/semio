//! 🧬️ BIM model artifact schema: the descriptor that publishes every facet leaf (artifact, snapshot, diff, mutations) of
//! `s.bim.model` in every schema language. The Rust types live in the sibling family folders.

use {semio_framework_artifact_reference::Dialect, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId, semio_framework_plugin::ArtifactKindSpec, semio_framework_plugin::MediaClass, semio_framework_plugin::MediaForm, semio_framework_plugin::MediaType, semio_framework_plugin::OsMediaCapability};

//#region 🪪️Identity
/// 🪪️ The document schema identity of the artifact.
pub const BIM_MODEL_DOCUMENT_SCHEMA: &str = "s.bim.model@1";

/// 🪪️ This artifact's dialect: the canonical surface ids are `s.bim.model@1/*#editor` and `s.bim.model@1/*#viewer`.
pub const BIM_MODEL_DIALECT: Dialect = Dialect { artifact_kind: "s.bim.model", standard: StandardId("1"), subset: SubsetId::ANY };

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`, stitched into the app manifests.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.bim-model".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("BIM Model", "BIM-Modell"),
        source_format: "bim.model".into(),
        component_kind: "bim".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        schema: BIM_MODEL_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into()],
        import_stdio_kinds: vec!["stdio.json".into()],
    }
}
//#endregion 🔖️ArtifactKind
//#endregion 🪪️Identity

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.bim.model`: the artifact, snapshot, diff and mutation facets, five leaves each.
pub fn bim_model_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.bim.model",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
