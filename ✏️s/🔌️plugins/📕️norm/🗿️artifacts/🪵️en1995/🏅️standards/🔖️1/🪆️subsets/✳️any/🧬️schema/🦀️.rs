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
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1995")]
pub struct En1995Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub members: Vec<semio_s_artifact_norm_en1995_model::TimberMember>,
    #[state(artifact)]
    pub connections: Vec<semio_s_artifact_norm_en1995_model::TimberConnection>,
    #[state(artifact)]
    pub uls_equation: semio_s_artifact_norm_en1995_model::UlsEquation,
    #[state(artifact)]
    pub cases: Vec<semio_s_artifact_norm_en1995_model::CaseRow>,
    #[state(artifact)]
    pub placements: Vec<semio_s_artifact_norm_en1995_model::Placement>,
    #[state(artifact)]
    pub tapers: Vec<semio_s_artifact_norm_en1995_model::Taper>,
    #[state(artifact)]
    pub cutouts: Vec<semio_s_artifact_norm_en1995_model::Cutout>,
    #[state(artifact)]
    pub fasteners: Vec<semio_s_artifact_norm_en1995_model::Fastener>,
    #[state(artifact)]
    pub panels: Vec<semio_s_artifact_norm_en1995_model::Panel>,
    #[state(artifact)]
    pub floors: Vec<semio_s_artifact_norm_en1995_model::Floor>,
    #[state(artifact)]
    pub diaphragms: Vec<semio_s_artifact_norm_en1995_model::Diaphragm>,
    #[state(artifact)]
    pub trusses: Vec<semio_s_artifact_norm_en1995_model::Truss>,
    #[state(artifact)]
    pub bracings: Vec<semio_s_artifact_norm_en1995_model::Bracing>,
    #[state(artifact)]
    pub decks: Vec<semio_s_artifact_norm_en1995_model::Deck>,
    #[state(artifact)]
    pub reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1995Artifact {
    fn default() -> Self { Self::from_snapshot(En1995Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)) }
}
impl From<En1995Snapshot> for En1995Artifact {
    fn from(snapshot: En1995Snapshot) -> Self { Self::from_snapshot(snapshot) }
}
impl En1995Artifact {
    pub fn to_snapshot(&self) -> En1995Snapshot {
        En1995Snapshot { annex: self.annex, reliability_class: self.reliability_class, members: self.members.clone(), connections: self.connections.clone(), uls_equation: self.uls_equation.clone(), cases: self.cases.clone(), placements: self.placements.clone(), tapers: self.tapers.clone(), cutouts: self.cutouts.clone(), fasteners: self.fasteners.clone(), panels: self.panels.clone(), floors: self.floors.clone(), diaphragms: self.diaphragms.clone(), trusses: self.trusses.clone(), bracings: self.bracings.clone(), decks: self.decks.clone() }
    }
    pub fn from_snapshot(snapshot: En1995Snapshot) -> Self {
        Self { annex: snapshot.annex, reliability_class: snapshot.reliability_class, members: snapshot.members, connections: snapshot.connections, uls_equation: snapshot.uls_equation, cases: snapshot.cases, placements: snapshot.placements, tapers: snapshot.tapers, cutouts: snapshot.cutouts, fasteners: snapshot.fasteners, panels: snapshot.panels, floors: snapshot.floors, diaphragms: snapshot.diaphragms, trusses: snapshot.trusses, bracings: snapshot.bracings, decks: snapshot.decks }
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
pub mod checks {
    pub use semio_s_artifact_norm_en1995_schema_timber as timber;
}

pub use semio_s_artifact_norm_en1995_spec_trace as spec_trace;
//#endregion 🪢️Mounts
