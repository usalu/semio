//! 🧬️ En1992 artifact schema — hierarchical RC structure + clause engines.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::{AnnexChoice, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};
use semio_s_artifact_norm_en1992_model::FireRating;
use semio_s_artifact_norm_en1992_model::TightnessClass;
use semio_s_artifact_norm_en1992_model::{Anchor, ExposureClass, LoadCaseActions, MemberKind, RcMember, SupportCondition};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1992")]
pub struct En1992Artifact {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub title: String,
    #[state(artifact)]
    pub design_working_life_years: f64,
    #[state(artifact)]
    pub delta_c_dev: f64,
    #[state(artifact)]
    pub cement_type: String,
    #[state(artifact)]
    pub concrete_grades: Vec<semio_s_artifact_norm_en1992_model::ConcreteGrade>,
    #[state(artifact)]
    pub reinforcement_grades: Vec<semio_s_artifact_norm_en1992_model::ReinforcementGrade>,
    #[state(artifact)]
    pub prestress_steels: Vec<semio_s_artifact_norm_en1992_model::PrestressSteel>,
    #[state(artifact)]
    pub members: Vec<RcMember>,
    #[state(artifact)]
    pub anchors: Vec<Anchor>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl En1992Artifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> En1992Snapshot {
        En1992Snapshot {
            annex: self.annex,
            title: self.title.clone(),
            design_working_life_years: self.design_working_life_years,
            delta_c_dev: self.delta_c_dev,
            cement_type: self.cement_type.clone(),
            concrete_grades: self.concrete_grades.clone(),
            reinforcement_grades: self.reinforcement_grades.clone(),
            prestress_steels: self.prestress_steels.clone(),
            members: self.members.clone(),
            anchors: self.anchors.clone(),
        }
    }

    /// 🧬️ Builds artifact from snapshot.
    pub fn from_snapshot(snapshot: &En1992Snapshot) -> Self {
        Self {
            annex: snapshot.annex,
            title: snapshot.title.clone(),
            design_working_life_years: snapshot.design_working_life_years,
            delta_c_dev: snapshot.delta_c_dev,
            cement_type: snapshot.cement_type.clone(),
            concrete_grades: snapshot.concrete_grades.clone(),
            reinforcement_grades: snapshot.reinforcement_grades.clone(),
            prestress_steels: snapshot.prestress_steels.clone(),
            members: snapshot.members.clone(),
            anchors: snapshot.anchors.clone(),
        }
    }

}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.norm.en1992` — twenty handcrafted schema leaves.
pub fn en1992_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.norm.en1992",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_s_artifact_norm_en1992_model::SNAPSHOT_LEAVES,
        diff: semio_s_artifact_norm_en1992_diff::DIFF_LEAVES,
        mutations: semio_s_artifact_norm_en1992_mutations::MUTATION_LEAVES,
    }
}
//#endregion 🔖️Descriptor

