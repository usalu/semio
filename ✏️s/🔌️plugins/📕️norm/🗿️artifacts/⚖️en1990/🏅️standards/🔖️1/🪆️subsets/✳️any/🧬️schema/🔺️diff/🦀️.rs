//! 🧬️ En1990 diff schema — sparse field delta over the artifact.

use crate::document::AnnexChoice;
use crate::{AccidentalAction, BridgeSls, Member, MemberEffect, PermanentAction, SeismicAction, VariableAction};
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1990 artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
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
