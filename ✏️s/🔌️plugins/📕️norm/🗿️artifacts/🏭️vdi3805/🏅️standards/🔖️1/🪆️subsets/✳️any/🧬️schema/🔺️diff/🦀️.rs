//! 🧬️ Vdi3805 diff schema — sparse field delta over the artifact.

use crate::{CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ManufacturerFile, ParametricGeometry, SecurityLimits};
use framework_schema::ArtifactSchema;
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the Vdi3805 artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.vdi3805")]
pub struct Vdi3805Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifact_schema::Vdi3805Artifact>>,
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
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Vdi3805StringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers

use crate::artifact_schema::diff::*;
use crate::artifact_schema::Vdi3805Artifact;
use crate::Vdi3805Snapshot;
use protocol::MutationDiff;

impl Vdi3805Diff {
    pub fn apply_to_artifact(&self, artifact: &Vdi3805Artifact) -> protocol::MutationApplyResult<Vdi3805Artifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(value) = &self.manufacturer_file {
                next.catalog.file = value.clone();
            }
            if let Some(value) = &self.catalog {
                next.catalog = value.clone();
            }
            if let Some(value) = &self.edition_profile {
                next.edition_profile = value.clone();
            }
            if let Some(value) = &self.correction_as_of {
                next.correction_as_of = *value;
            }
            if let Some(value) = &self.strict_mode {
                next.strict_mode = *value;
            }
            if let Some(value) = &self.index {
                next.index = value.clone();
            }
            if let Some(value) = &self.geometry {
                next.geometry = value.clone();
            }
            if let Some(value) = &self.curves {
                next.curves = value.clone();
            }
            if let Some(value) = &self.limits {
                next.limits = value.clone();
            }
            next
        })
    }
}

impl MutationDiff<Vdi3805Snapshot> for Vdi3805Diff {
    fn apply(&self, snapshot: &Vdi3805Snapshot) -> protocol::MutationApplyResult<Vdi3805Snapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(value) = &self.manufacturer_file {
                next.catalog.file = value.clone();
            }
            if let Some(value) = &self.catalog {
                next.catalog = value.clone();
            }
            if let Some(value) = &self.edition_profile {
                next.edition_profile = value.clone();
            }
            if let Some(value) = &self.correction_as_of {
                next.correction_as_of = *value;
            }
            if let Some(value) = &self.strict_mode {
                next.strict_mode = *value;
            }
            if let Some(value) = &self.index {
                next.index = value.clone();
            }
            if let Some(value) = &self.geometry {
                next.geometry = value.clone();
            }
            if let Some(value) = &self.curves {
                next.curves = value.clone();
            }
            if let Some(value) = &self.limits {
                next.limits = value.clone();
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(manufacturer_file);
        take!(catalog);
        take!(edition_profile);
        take!(correction_as_of);
        take!(strict_mode);
        take!(index);
        take!(geometry);
        take!(curves);
        take!(limits);
    }
}

pub fn diff_set_snapshot(snapshot: &Vdi3805Snapshot) -> Vdi3805Diff {
    Vdi3805Diff { artifact: Some(Box::new(Vdi3805Artifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
