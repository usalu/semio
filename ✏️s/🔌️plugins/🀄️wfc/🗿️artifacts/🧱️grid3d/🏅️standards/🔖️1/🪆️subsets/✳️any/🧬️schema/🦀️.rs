//! 🧬️ `s.wfc.grid3d` artifact schema — the persisted WFC problem is the artifact, plus the four
//! facet descriptors (artifact/snapshot/diff/mutations) the declaration tree binds.

use crate::schema::snapshot::Grid3dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Grid3dArtifact
/// 🧬️ `Grid3dArtifact` facet — the persisted problem spec IS the artifact; nothing is derived into
/// it, because the solve is an inference and never lands on the document.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.wfc.grid3d")]
pub struct Grid3dArtifact {
    #[state(artifact)]
    pub snapshot: Grid3dSnapshot,
}

impl Grid3dArtifact {
    pub fn to_snapshot(&self) -> Grid3dSnapshot {
        self.snapshot.clone()
    }

    pub fn from_snapshot(snapshot: Grid3dSnapshot) -> Self {
        Self { snapshot }
    }
}
//#endregion 🔖️Grid3dArtifact

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.wfc.grid3d` — twenty handcrafted schema leaves, JSON Schema normative.
pub fn grid3d_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.wfc.grid3d",
        artifact: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
