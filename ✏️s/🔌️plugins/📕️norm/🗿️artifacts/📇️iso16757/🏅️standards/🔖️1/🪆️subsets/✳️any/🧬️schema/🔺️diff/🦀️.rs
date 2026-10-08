//! 🧬️ Iso16757 diff schema — sparse field delta over the artifact.

use crate::CatalogueValue;
use framework_schema::ArtifactSchema;
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the Iso16757 artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.iso16757")]
pub struct Iso16757Diff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifact_schema::Iso16757Artifact>>,
    #[state(artifact)]
    pub catalogue: Option<crate::part_1::Catalogue>,
    #[state(artifact)]
    pub dictionary: Option<crate::part_4::Dictionary>,
    #[state(artifact)]
    pub geometry: Option<crate::part_2::GeometryCatalogue>,
    #[state(artifact)]
    pub selection: Option<crate::part_1::SelectionRequest>,
    #[state(artifact)]
    pub part_number_rule: Option<crate::part_5::PartNumberRule>,
    #[state(artifact)]
    pub part_number_inputs: Option<BTreeMap<String, CatalogueValue>>,
    #[state(artifact)]
    pub script_limits: Option<crate::part_5::ScriptLimits>,
    #[state(artifact)]
    pub exchange_process: Option<crate::part_5::ExchangeProcess>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct Iso16757StringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers

use crate::artifact_schema::diff::*;
use crate::artifact_schema::Iso16757Artifact;
use crate::Iso16757Snapshot;
use protocol::MutationDiff;

impl Iso16757Diff {
    pub fn apply_to_artifact(&self, artifact: &Iso16757Artifact) -> protocol::MutationApplyResult<Iso16757Artifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(value) = &self.catalogue {
                next.catalogue = value.clone();
            }
            if let Some(value) = &self.dictionary {
                next.dictionary = value.clone();
            }
            if let Some(value) = &self.geometry {
                next.geometry = value.clone();
            }
            if let Some(value) = &self.selection {
                next.selection = value.clone();
            }
            if let Some(value) = &self.part_number_rule {
                next.part_number_rule = value.clone();
            }
            if let Some(value) = &self.part_number_inputs {
                next.part_number_inputs = value.clone();
            }
            if let Some(value) = &self.script_limits {
                next.script_limits = *value;
            }
            if let Some(value) = &self.exchange_process {
                next.exchange_process = *value;
            }
            next
        })
    }
}

impl MutationDiff<Iso16757Snapshot> for Iso16757Diff {
    fn apply(&self, snapshot: &Iso16757Snapshot) -> protocol::MutationApplyResult<Iso16757Snapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(value) = &self.catalogue {
                next.catalogue = value.clone();
            }
            if let Some(value) = &self.dictionary {
                next.dictionary = value.clone();
            }
            if let Some(value) = &self.geometry {
                next.geometry = value.clone();
            }
            if let Some(value) = &self.selection {
                next.selection = value.clone();
            }
            if let Some(value) = &self.part_number_rule {
                next.part_number_rule = value.clone();
            }
            if let Some(value) = &self.part_number_inputs {
                next.part_number_inputs = value.clone();
            }
            if let Some(value) = &self.script_limits {
                next.script_limits = *value;
            }
            if let Some(value) = &self.exchange_process {
                next.exchange_process = *value;
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
        take!(catalogue);
        take!(dictionary);
        take!(geometry);
        take!(selection);
        take!(part_number_rule);
        take!(part_number_inputs);
        take!(script_limits);
        take!(exchange_process);
    }
}

pub fn diff_set_snapshot(snapshot: &Iso16757Snapshot) -> Iso16757Diff {
    Iso16757Diff { artifact: Some(Box::new(Iso16757Artifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}
