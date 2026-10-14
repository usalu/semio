//! 🧱️ EN 1996 artifact schema — every field with its state class.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
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
    pub masonry_class: semio_s_artifact_norm_en1996_model::MasonryClass,
    #[state(artifact)]
    pub design_situation: semio_s_plugin_norm_compliance::DesignSituation,
    #[state(artifact)]
    pub storeys: u32,
    #[state(artifact)]
    pub walls: Vec<semio_s_artifact_norm_en1996_model::MasonryWall>,
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
        snapshot: semio_s_artifact_norm_en1996_model::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_norm_en1996_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_norm_en1996_mutations::MUTATION_LEAVES,
    }
}
//#endregion 🔖️Descriptor

//#region 🪢️Mounts
#[path = "."]
pub mod checks {
    #[path = "⚖️masonry/🦀️.rs"]
    mod masonry;
    #[cfg(test)]
    #[path = "🧪️tests/🔬️oracle/🦀️.rs"]
    mod oracle_tests;
}
//#endregion 🪢️Mounts
