//! ✨️ EN 1999 artifact schema — aluminium structure subject + DIN EN 1999 clause checks.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
#[cfg(test)] use semio_s_plugin_norm_compliance::CheckStatus;
use semio_s_artifact_norm_en1999_model::{ColdFormedSheet, AluminiumShell, Placement, WeldRun, SectionVertex, FatigueCategory, PatchLoad, RingStiffener, TransverseStiffener, AluminiumConnection, AluminiumMaterial, AluminiumMember, AluminiumSection, FatigueDetail, FireScenario, SupportCondition};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ EN 1999 document artifact state (mirrors snapshot subject).
#[derive(Clone, Debug, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1999")]
pub struct En1999Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub materials: Vec<AluminiumMaterial>,
    #[state(artifact)]
    pub sections: Vec<AluminiumSection>,
    #[state(artifact)]
    pub members: Vec<AluminiumMember>,
    #[state(artifact)]
    pub connections: Vec<AluminiumConnection>,
    #[state(artifact)]
    pub fire_scenarios: Vec<FireScenario>,
    #[state(artifact)]
    pub fatigue_details: Vec<FatigueDetail>,
    #[state(artifact)]
    pub cold_formed: Vec<ColdFormedSheet>,
    #[state(artifact)]
    pub shells: Vec<AluminiumShell>,
    #[state(artifact)]
    pub placements: Vec<Placement>,
    #[state(artifact)]
    pub weld_runs: Vec<WeldRun>,
    #[state(artifact)]
    pub outline_vertices: Vec<SectionVertex>,
    #[state(artifact)]
    pub fatigue_categories: Vec<FatigueCategory>,
    #[state(artifact)]
    pub patch_loads: Vec<PatchLoad>,
    #[state(artifact)]
    pub ring_stiffeners: Vec<RingStiffener>,
    #[state(artifact)]
    pub transverse_stiffeners: Vec<TransverseStiffener>,
    #[state(artifact)]
    pub reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1999Artifact {
    fn default() -> Self {
        Self::from_snapshot(En1999Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De))
    }
}

impl From<En1999Snapshot> for En1999Artifact {
    fn from(snapshot: En1999Snapshot) -> Self {
        Self::from_snapshot(snapshot)
    }
}

impl En1999Artifact {
    pub fn to_snapshot(&self) -> En1999Snapshot {
        En1999Snapshot {
            annex: self.annex,
            reliability_class: self.reliability_class,
            materials: self.materials.clone(),
            sections: self.sections.clone(),
            members: self.members.clone(),
            connections: self.connections.clone(),
            fire_scenarios: self.fire_scenarios.clone(),
            fatigue_details: self.fatigue_details.clone(),
            cold_formed: self.cold_formed.clone(),
            shells: self.shells.clone(),
            placements: self.placements.clone(),
            weld_runs: self.weld_runs.clone(),
            outline_vertices: self.outline_vertices.clone(),
            fatigue_categories: self.fatigue_categories.clone(),
            patch_loads: self.patch_loads.clone(),
            ring_stiffeners: self.ring_stiffeners.clone(),
            transverse_stiffeners: self.transverse_stiffeners.clone(),
        }
    }

    pub fn from_snapshot(snapshot: En1999Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            reliability_class: snapshot.reliability_class,
            materials: snapshot.materials,
            sections: snapshot.sections,
            members: snapshot.members,
            connections: snapshot.connections,
            fire_scenarios: snapshot.fire_scenarios,
            fatigue_details: snapshot.fatigue_details,
            cold_formed: snapshot.cold_formed,
            shells: snapshot.shells,
            placements: snapshot.placements,
            weld_runs: snapshot.weld_runs,
            outline_vertices: snapshot.outline_vertices,
            fatigue_categories: snapshot.fatigue_categories,
            patch_loads: snapshot.patch_loads,
            ring_stiffeners: snapshot.ring_stiffeners,
            transverse_stiffeners: snapshot.transverse_stiffeners,
        }
    }

}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn en1999_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.norm.en1999",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_s_artifact_norm_en1999_model::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_norm_en1999_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_norm_en1999_mutations::MUTATION_LEAVES,
    }
}
//#endregion 🔖️Descriptor

