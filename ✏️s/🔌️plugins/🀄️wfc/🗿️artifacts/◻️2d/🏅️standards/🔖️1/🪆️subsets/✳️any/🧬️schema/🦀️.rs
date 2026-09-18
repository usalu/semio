//! 🧬️ WFC 2D artifact schema — the persisted problem spec IS the artifact, plus the twenty
//! handcrafted descriptor leaves (four facets × five languages) the declaration tree binds.

use crate::schema::snapshot::Wfc2dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Wfc2dArtifact
/// 🧬️ The artifact facet — a WFC problem has no ambient state beyond its own snapshot, so this is a
/// one-field wrapper rather than a wider artifact-only projection.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.wfc.wfc2d")]
pub struct Wfc2dArtifact {
    #[state(artifact)]
    pub snapshot: Wfc2dSnapshot,
}

impl Wfc2dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Wfc2dSnapshot {
        self.snapshot.clone()
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: Wfc2dSnapshot) -> Self {
        Self { snapshot }
    }
}
//#endregion 🔖️Wfc2dArtifact

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.wfc.wfc2d` — twenty handcrafted schema leaves. The JSON Schema leaf of each
/// facet is normative; the Rust, TypeScript, GraphQL and Protobuf leaves mirror it.
pub fn wfc2d_artifact_schema_descriptor() -> ::semio_framework_schema::ArtifactSchemaDescriptor {
    ::semio_framework_schema::ArtifactSchemaDescriptor {
        id: "s.wfc.wfc2d",
        artifact: ::semio_framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
