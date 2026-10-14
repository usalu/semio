//! 🪵️ EN 1995 artifact schema — every field with its state class.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
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
    pub members: Vec<semio_s_artifact_norm_en1995_model::TimberMember>,
    #[state(artifact)]
    pub connections: Vec<semio_s_artifact_norm_en1995_model::TimberConnection>,
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
        snapshot: semio_s_artifact_norm_en1995_model::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_norm_en1995_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_norm_en1995_mutations::MUTATION_LEAVES,
    }
}
//#endregion 🔖️Descriptor

//#region 🪢️Mounts
#[path = "."]
pub mod checks {
    #[path = "⚖️timber/🦀️.rs"]
    mod timber;
}
//#endregion 🪢️Mounts
