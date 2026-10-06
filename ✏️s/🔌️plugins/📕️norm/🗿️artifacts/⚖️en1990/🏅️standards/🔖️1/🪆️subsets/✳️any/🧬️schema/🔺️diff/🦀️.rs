//! 🧬️ En1990 diff schema — sparse field delta over the artifact.

use crate::document::AnnexChoice;
use crate::{AccidentalAction, BridgeSls, Member, MemberEffect, PermanentAction, SeismicAction, VariableAction};
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1990 artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1990")]
pub struct En1990Diff {
    #[state(artifact)]
    pub annex: Option<AnnexChoice>,
    #[state(artifact)]
    pub project_id: Option<String>,
    #[state(artifact)]
    pub structure_kind: Option<String>,
    #[state(artifact)]
    pub altitude_m: Option<f64>,
    #[state(artifact)]
    pub consequence_class: Option<u8>,
    #[state(artifact)]
    pub reliability_class: Option<u8>,
    #[state(artifact)]
    pub design_working_life_category: Option<u8>,
    #[state(artifact)]
    pub design_working_life_years: Option<f64>,
    #[state(artifact)]
    pub reference_period_years: Option<f64>,
    #[state(artifact)]
    pub supervision_level: Option<String>,
    #[state(artifact)]
    pub inspection_level: Option<String>,
    #[state(artifact)]
    pub k_fi_declared: Option<f64>,
    #[state(artifact)]
    pub beta_computed: Option<f64>,
    #[state(artifact)]
    pub permanents: Option<Vec<PermanentAction>>,
    #[state(artifact)]
    pub variables: Option<Vec<VariableAction>>,
    #[state(artifact)]
    pub accidentals: Option<Vec<AccidentalAction>>,
    #[state(artifact)]
    pub seismics: Option<Vec<SeismicAction>>,
    #[state(artifact)]
    pub members: Option<Vec<Member>>,
    #[state(artifact)]
    pub bridge_sls: Option<Vec<BridgeSls>>,
    #[state(artifact)]
    pub effects: Option<Vec<MemberEffect>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct En1990StringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers

use crate::artifact_schema::diff::*;
use crate::artifact_schema::En1990Artifact;
use crate::En1990Snapshot;
use protocol::MutationDiff;

impl En1990Diff {
    pub fn apply_to_artifact(&self, artifact: &En1990Artifact) -> protocol::MutationApplyResult<En1990Artifact> {
        Ok({
            let mut next = artifact.clone();
            if let Some(value) = &self.annex {
                next.annex = *value;
            }
            if let Some(value) = &self.project_id {
                next.project_id = value.clone();
            }
            if let Some(value) = &self.structure_kind {
                next.structure_kind = value.clone();
            }
            if let Some(value) = self.altitude_m {
                next.altitude_m = value;
            }
            if let Some(value) = &self.consequence_class {
                next.consequence_class = *value;
            }
            if let Some(value) = &self.reliability_class {
                next.reliability_class = *value;
            }
            if let Some(value) = &self.design_working_life_category {
                next.design_working_life_category = *value;
            }
            if let Some(value) = &self.design_working_life_years {
                next.design_working_life_years = *value;
            }
            if let Some(value) = &self.reference_period_years {
                next.reference_period_years = *value;
            }
            if let Some(value) = &self.supervision_level {
                next.supervision_level = value.clone();
            }
            if let Some(value) = &self.inspection_level {
                next.inspection_level = value.clone();
            }
            if let Some(value) = &self.k_fi_declared {
                next.k_fi_declared = *value;
            }
            if let Some(value) = &self.beta_computed {
                next.beta_computed = *value;
            }
            if let Some(value) = &self.permanents {
                next.permanents = value.clone();
            }
            if let Some(value) = &self.variables {
                next.variables = value.clone();
            }
            if let Some(value) = &self.accidentals {
                next.accidentals = value.clone();
            }
            if let Some(value) = &self.seismics {
                next.seismics = value.clone();
            }
            if let Some(value) = &self.members {
                next.members = value.clone();
            }
            if let Some(value) = &self.bridge_sls {
                next.bridge_sls = value.clone();
            }
            if let Some(value) = &self.effects {
                next.effects = value.clone();
            }
            next
        })
    }
}

impl MutationDiff<En1990Snapshot> for En1990Diff {
    fn apply(&self, snapshot: &En1990Snapshot) -> protocol::MutationApplyResult<En1990Snapshot> {
        Ok({
            let mut next = snapshot.clone();
            if let Some(value) = &self.annex {
                next.annex = *value;
            }
            if let Some(value) = &self.project_id {
                next.project_id = value.clone();
            }
            if let Some(value) = &self.structure_kind {
                next.structure_kind = value.clone();
            }
            if let Some(value) = self.altitude_m {
                next.altitude_m = value;
            }
            if let Some(value) = &self.consequence_class {
                next.consequence_class = *value;
            }
            if let Some(value) = &self.reliability_class {
                next.reliability_class = *value;
            }
            if let Some(value) = &self.design_working_life_category {
                next.design_working_life_category = *value;
            }
            if let Some(value) = &self.design_working_life_years {
                next.design_working_life_years = *value;
            }
            if let Some(value) = &self.reference_period_years {
                next.reference_period_years = *value;
            }
            if let Some(value) = &self.supervision_level {
                next.supervision_level = value.clone();
            }
            if let Some(value) = &self.inspection_level {
                next.inspection_level = value.clone();
            }
            if let Some(value) = &self.k_fi_declared {
                next.k_fi_declared = *value;
            }
            if let Some(value) = &self.beta_computed {
                next.beta_computed = *value;
            }
            if let Some(value) = &self.permanents {
                next.permanents = value.clone();
            }
            if let Some(value) = &self.variables {
                next.variables = value.clone();
            }
            if let Some(value) = &self.accidentals {
                next.accidentals = value.clone();
            }
            if let Some(value) = &self.seismics {
                next.seismics = value.clone();
            }
            if let Some(value) = &self.members {
                next.members = value.clone();
            }
            if let Some(value) = &self.bridge_sls {
                next.bridge_sls = value.clone();
            }
            if let Some(value) = &self.effects {
                next.effects = value.clone();
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(annex);
        take!(project_id);
        take!(structure_kind);
        take!(altitude_m);
        take!(consequence_class);
        take!(reliability_class);
        take!(design_working_life_category);
        take!(design_working_life_years);
        take!(reference_period_years);
        take!(supervision_level);
        take!(inspection_level);
        take!(k_fi_declared);
        take!(beta_computed);
        take!(permanents);
        take!(variables);
        take!(accidentals);
        take!(seismics);
        take!(members);
        take!(bridge_sls);
        take!(effects);
    }

}
