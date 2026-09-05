//! 🧬️ Vdi3805 diff schema — sparse field delta over the artifact.

use crate::artifacts::vdi3805::{CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ManufacturerFile, ParametricGeometry, SecurityLimits};
use schema::ArtifactSchema;
use std::collections::BTreeMap;


//#region 🔖️Diff
/// 🔺️ Sparse field delta for the Vdi3805 artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.vdi3805")]
pub struct Vdi3805Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::vdi3805::schema::Vdi3805Artifact>>,
    #[state(artifact)]
    pub manufacturer_file: Option<ManufacturerFile>,
    #[state(artifact)]
    pub catalog: Option<ManufacturerCatalog>,
    #[state(artifact)]
    pub edition_profile: Option<BTreeMap<String, EditionProfileChoice>>,
    #[state(artifact)]
    pub correction_as_of: Option<EditionId>,
    #[state(artifact)]
    pub strict_mode: Option<bool>,
    #[state(artifact)]
    pub index: Option<CatalogIndex>,
    #[state(artifact)]
    pub geometry: Option<BTreeMap<String, ParametricGeometry>>,
    #[state(artifact)]
    pub curves: Option<BTreeMap<String, CharacteristicCurve>>,
    #[state(artifact)]
    pub limits: Option<SecurityLimits>,
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
pub struct Vdi3805StringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers
