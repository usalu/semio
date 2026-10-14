//! 🧬️ En1990 artifact schema — every field of the artifact with its state class.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full En1990 artifact state across the artifact and presence lanes.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1990")]
pub struct En1990Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub project_id: String,
    #[state(artifact)]
    pub structure_kind: String,
    #[state(artifact)]
    pub altitude_m: f64,
    #[state(artifact)]
    pub consequence_class: u8,
    #[state(artifact)]
    pub reliability_class: u8,
    #[state(artifact)]
    pub design_working_life_category: u8,
    #[state(artifact)]
    pub design_working_life_years: f64,
    #[state(artifact)]
    pub reference_period_years: f64,
    #[state(artifact)]
    pub supervision_level: String,
    #[state(artifact)]
    pub inspection_level: String,
    #[state(artifact)]
    pub k_fi_declared: f64,
    #[state(artifact)]
    pub beta_computed: f64,
    #[state(artifact)]
    pub permanents: Vec<semio_s_artifact_norm_en1990_model::PermanentAction>,
    #[state(artifact)]
    pub variables: Vec<semio_s_artifact_norm_en1990_model::VariableAction>,
    #[state(artifact)]
    pub accidentals: Vec<semio_s_artifact_norm_en1990_model::AccidentalAction>,
    #[state(artifact)]
    pub seismics: Vec<semio_s_artifact_norm_en1990_model::SeismicAction>,
    #[state(artifact)]
    pub members: Vec<semio_s_artifact_norm_en1990_model::Member>,
    #[state(artifact)]
    pub bridge_sls: Vec<semio_s_artifact_norm_en1990_model::BridgeSls>,
    #[state(artifact)]
    pub effects: Vec<semio_s_artifact_norm_en1990_model::MemberEffect>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl En1990Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> semio_s_artifact_norm_en1990_model::En1990Snapshot {
        semio_s_artifact_norm_en1990_model::En1990Snapshot {
            annex: self.annex,
            project_id: self.project_id.clone(),
            structure_kind: self.structure_kind.clone(),
            altitude_m: self.altitude_m,
            consequence_class: self.consequence_class,
            reliability_class: self.reliability_class,
            design_working_life_category: self.design_working_life_category,
            design_working_life_years: self.design_working_life_years,
            reference_period_years: self.reference_period_years,
            supervision_level: self.supervision_level.clone(),
            inspection_level: self.inspection_level.clone(),
            k_fi_declared: self.k_fi_declared,
            beta_computed: self.beta_computed,
            permanents: self.permanents.clone(),
            variables: self.variables.clone(),
            accidentals: self.accidentals.clone(),
            seismics: self.seismics.clone(),
            members: self.members.clone(),
            bridge_sls: self.bridge_sls.clone(),
            effects: self.effects.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: semio_s_artifact_norm_en1990_model::En1990Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            project_id: snapshot.project_id,
            structure_kind: snapshot.structure_kind,
            altitude_m: snapshot.altitude_m,
            consequence_class: snapshot.consequence_class,
            reliability_class: snapshot.reliability_class,
            design_working_life_category: snapshot.design_working_life_category,
            design_working_life_years: snapshot.design_working_life_years,
            reference_period_years: snapshot.reference_period_years,
            supervision_level: snapshot.supervision_level,
            inspection_level: snapshot.inspection_level,
            k_fi_declared: snapshot.k_fi_declared,
            beta_computed: snapshot.beta_computed,
            permanents: snapshot.permanents,
            variables: snapshot.variables,
            accidentals: snapshot.accidentals,
            seismics: snapshot.seismics,
            members: snapshot.members,
            bridge_sls: snapshot.bridge_sls,
            effects: snapshot.effects,
        }
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1990` — twenty handcrafted schema leaves.
pub fn en1990_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.norm.en1990",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_s_artifact_norm_en1990_model::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_norm_en1990_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_norm_en1990_mutations::MUTATION_LEAVES,
    }
}
//#endregion 🔖️Descriptor

