//! 🧬️ Iso16757 diff schema — sparse field delta over the artifact.

use crate::artifacts::iso16757::CatalogueValue;
use schema::ArtifactSchema;
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the Iso16757 artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.iso16757")]
pub struct Iso16757Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::iso16757::schema::Iso16757Artifact>>,
    #[state(artifact)]
    pub catalogue: Option<crate::artifacts::iso16757::part_1::Catalogue>,
    #[state(artifact)]
    pub dictionary: Option<crate::artifacts::iso16757::part_4::Dictionary>,
    #[state(artifact)]
    pub geometry: Option<crate::artifacts::iso16757::part_2::GeometryCatalogue>,
    #[state(artifact)]
    pub selection: Option<crate::artifacts::iso16757::part_1::SelectionRequest>,
    #[state(artifact)]
    pub part_number_rule: Option<crate::artifacts::iso16757::part_5::PartNumberRule>,
    #[state(artifact)]
    pub part_number_inputs: Option<BTreeMap<String, CatalogueValue>>,
    #[state(artifact)]
    pub script_limits: Option<crate::artifacts::iso16757::part_5::ScriptLimits>,
    #[state(artifact)]
    pub exchange_process: Option<crate::artifacts::iso16757::part_5::ExchangeProcess>,
    #[state(presence)]
    pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757StringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers
