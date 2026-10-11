//! 🧬️ `wfc3d` artifact schema — the persisted WFC problem spec and its descriptor leaves.

use crate::schema::snapshot::Wfc3dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Wfc3dArtifact
/// 🧬️ `Wfc3dArtifact` facet — the persisted problem spec IS the artifact; the solve is derived.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.wfc.wfc3d")]
pub struct Wfc3dArtifact {
    #[state(artifact)]
    pub snapshot: Wfc3dSnapshot,
}

impl Wfc3dArtifact {
    pub fn to_snapshot(&self) -> Wfc3dSnapshot {
        self.snapshot.clone()
    }

    pub fn from_snapshot(snapshot: Wfc3dSnapshot) -> Self {
        Self { snapshot }
    }
}
//#endregion 🔖️Wfc3dArtifact

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.wfc.wfc3d` — twenty handcrafted schema leaves (four facets × five
/// languages). The JSON Schema leaf is normative; the other four mirror it.
pub fn wfc3d_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.wfc.wfc3d",
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

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
