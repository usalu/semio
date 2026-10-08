//! 🧱️ EN 1996 artifact schema — every field with its state class.

use crate::document::AnnexChoice;
use crate::En1996Snapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ EN 1996 document artifact state.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1996")]
pub struct En1996Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub masonry_class: crate::MasonryClass,
    #[state(artifact)]
    pub design_situation: crate::document::DesignSituation,
    #[state(artifact)]
    pub storeys: u32,
    #[state(artifact)]
    pub walls: Vec<crate::MasonryWall>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1996Artifact {
    fn default() -> Self {
        Self::from_snapshot(En1996Snapshot::default())
    }
}

impl From<En1996Snapshot> for En1996Artifact {
    fn from(snapshot: En1996Snapshot) -> Self {
        Self::from_snapshot(snapshot)
    }
}

impl En1996Artifact {
    pub fn to_snapshot(&self) -> En1996Snapshot {
        En1996Snapshot {
            annex: self.annex,
            masonry_class: self.masonry_class,
            design_situation: self.design_situation,
            storeys: self.storeys,
            walls: self.walls.clone(),
        }
    }

    pub fn from_snapshot(snapshot: En1996Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            masonry_class: snapshot.masonry_class,
            design_situation: snapshot.design_situation,
            storeys: snapshot.storeys,
            walls: snapshot.walls,
        }
    }

}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn en1996_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.norm.en1996",
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
#[path = "⚖️masonry/🦀️.rs"]
mod masonry;
pub use masonry::*;
//#endregion 🔖️ComplianceHelpers

//#region 🧪️ComplianceTests
#[cfg(test)]
#[path = "🧪️tests/⚖️compliance/🦀️.rs"]
mod compliance_tests;
//#endregion 🧪️ComplianceTests

#[cfg(test)]
#[path = "🧪️tests/🔬️oracle/🦀️.rs"]
mod oracle_tests;

