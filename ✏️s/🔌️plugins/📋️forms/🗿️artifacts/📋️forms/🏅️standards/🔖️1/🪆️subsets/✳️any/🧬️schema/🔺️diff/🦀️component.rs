//! 🧬️ Forms diff schema — sparse field delta over the artifact.

use crate::artifacts::forms::{FormStep, schema::FormsArtifact};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the forms artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.forms.forms")]
pub struct FormsDiff {
    #[state(persistent)]
    pub artifact: Option<Box<FormsArtifact>>,
    #[state(persistent)]
    pub schema: Option<String>,
    #[state(persistent)]
    pub id: Option<String>,
    #[state(persistent)]
    pub version: Option<String>,
    #[state(persistent)]
    pub title: Option<Option<String>>,
    #[state(persistent)]
    pub steps: Option<FormsStepsDelta>,
    #[state(shared_ui)]
    pub selected_ids: Option<FormsStringList>,
    #[state(local_ui)]
    pub current_step_index: Option<u32>,
    #[state(local_ui)]
    pub try_values_json: Option<String>,
    #[state(local_ui)]
    pub locale: Option<String>,
    #[state(local_ui)]
    pub contributions_json: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FormsStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `steps`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FormsStepsDelta {
    pub added: Vec<FormStep>,
    pub removed: Vec<String>,
    pub patched: Vec<FormsStepPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched step entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormsStepPatchEntry {
    pub id: String,
    pub patch: FormsStepPatch,
}

/// 🩹 Partial step replacement.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FormsStepPatch {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
}
//#endregion 🔖️DeltaHelpers
