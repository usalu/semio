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
    ::semio_framework_schema_registry::register_schema_leaf_documents(&[
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::JsonSchema, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::JsonSchema, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::JsonSchema, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔣️.json" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐️spatial-structure/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏯️building-elements/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💎️types-and-materials/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️element-operations/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜️annotation-and-sheets/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🤝️components-and-coordination/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::JsonSchema, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️.json" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.bim.model", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛰️.proto" },
    ]);
    #[cfg(target_arch = "wasm32")]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, "");
    #[cfg(not(target_arch = "wasm32"))]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, env!("CARGO_MANIFEST_DIR"));
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.bim.model",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: "", graphql: "", json_schema: "", proto: "" },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: "",
            graphql: "",
            json_schema: "",
            proto: "",
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: "",
            graphql: "",
            json_schema: "",
            proto: "",
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: "",
            graphql: "",
            json_schema: "",
            proto: "",
        },
    }
}
//#endregion 🔖️Descriptor

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

