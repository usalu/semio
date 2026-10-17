//! 🧬️ Playground artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full playground artifact state (artifact-lane fields only today).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.demonstrator.playground")]
pub struct PlaygroundArtifact {
    #[state(artifact)]
    pub schema: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for PlaygroundArtifact {
    fn default() -> Self {
        Self { schema: crate::PLAYGROUND_DOCUMENT_SCHEMA.into() }
    }
}

impl PlaygroundArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot {
        crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot { schema: self.schema.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot) -> Self {
        Self { schema: snapshot.schema }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot) {
        self.schema = snapshot.schema;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️DocumentHelpers
/// 🏗️ Empty default playground snapshot (relocated from the deleted `⚙️engine`, ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES: pure document helper, no `&mut self`,
/// no app type — belongs beside the snapshot it builds).
pub fn empty_playground_snapshot() -> crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot {
    crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.demonstrator.playground` — twenty handcrafted schema leaves.
pub fn playground_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::register_schema_leaf_documents(&[
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.demonstrator.playground", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.demonstrator.playground", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.demonstrator.playground", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.demonstrator.playground", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.demonstrator.playground", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.demonstrator.playground", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.demonstrator.playground", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.demonstrator.playground", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.demonstrator.playground", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.demonstrator.playground", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.demonstrator.playground", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.demonstrator.playground", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛰️.proto" },
    ]);
    #[cfg(target_arch = "wasm32")]
    semio_framework_schema_registry::install_default_schema_leaf_reader(no_package_assets, "");
    #[cfg(not(target_arch = "wasm32"))]
    semio_framework_schema_registry::install_default_schema_leaf_reader(no_package_assets, env!("CARGO_MANIFEST_DIR"));
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.demonstrator.playground",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: "", graphql: "", json_schema: include_str!("🔣️.json"), proto: "" },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: "",
            graphql: "",
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: "",
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"), typescript: "", graphql: "", json_schema: include_str!("🔺️diff/🔣️.json"), proto: ""
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: "",
            graphql: "",
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: "",
        },
    }
}

/// 📭️ The playground guest receives no schema documents as package assets: the TypeScript, GraphQL and proto leaves are read from the checkout, and no guest consumes them.
fn no_package_assets(_name: &str) -> Option<&'static [u8]> {
    None
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🧩️ArtifactItems
pub mod artifact_items {
pub const PLAYGROUND_DOCUMENT_SCHEMA: &str = "playground.playground";
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
pub type PlaygroundCapabilityRow<'a> = (&'a str, &'a str, &'a str, &'a [(&'a str, &'a str)], Option<(&'a str, &'a str)>);
}
//#endregion 🧩️ArtifactItems4
