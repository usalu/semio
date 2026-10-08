//! 🪵️ EN 1995 artifact schema — every field with its state class.

use crate::document::AnnexChoice;
use crate::En1995Snapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ EN 1995 document artifact state — mirrors the timber-structure snapshot.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1995")]
pub struct En1995Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub members: Vec<crate::TimberMember>,
    #[state(artifact)]
    pub connections: Vec<crate::TimberConnection>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1995Artifact {
    fn default() -> Self { Self::from_snapshot(En1995Snapshot::default()) }
}
impl From<En1995Snapshot> for En1995Artifact {
    fn from(snapshot: En1995Snapshot) -> Self { Self::from_snapshot(snapshot) }
}
impl En1995Artifact {
    pub fn to_snapshot(&self) -> En1995Snapshot {
        En1995Snapshot { annex: self.annex, members: self.members.clone(), connections: self.connections.clone() }
    }
    pub fn from_snapshot(snapshot: En1995Snapshot) -> Self {
        Self { annex: snapshot.annex, members: snapshot.members, connections: snapshot.connections }
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn en1995_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.norm.en1995",
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
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️ComplianceHelpers
#[path = "⚖️timber/🦀️.rs"]
mod timber;
pub use timber::*;
//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
//#endregion 🧪️ComplianceTests
