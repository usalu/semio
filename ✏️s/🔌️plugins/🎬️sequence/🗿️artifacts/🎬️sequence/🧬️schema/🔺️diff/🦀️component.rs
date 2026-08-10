//! 🧬️ Sequence diff schema — sparse field delta over the artifact.

use crate::artifacts::sequence::{SequenceCamera, SequenceEdge, SequenceEdgePatch, SequenceStep, SequenceStepPatch};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the sequence artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.sequence.sequence")]
pub struct SequenceDiff {
    #[state(persistent)]
    pub artifact: Option<Box<crate::artifacts::sequence::schema::SequenceArtifact>>,
    #[state(persistent)]
    pub schema: Option<String>,
    #[state(persistent)]
    pub steps: Option<SequenceStepsDelta>,
    #[state(persistent)]
    pub edges: Option<SequenceEdgesDelta>,
    #[state(shared_ui)]
    pub selected_step_ids: Option<SequenceStringList>,
    #[state(local_ui)]
    pub last_run_json: Option<String>,
    #[state(local_ui)]
    pub orientation: Option<String>,
    #[state(local_ui)]
    pub camera: Option<SequenceCamera>,
    #[state(local_ui)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SequenceStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `steps`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SequenceStepsDelta {
    pub added: Vec<SequenceStep>,
    pub removed: Vec<String>,
    pub patched: Vec<SequenceStepPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🧩 Identified-collection delta for `edges`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SequenceEdgesDelta {
    pub added: Vec<SequenceEdge>,
    pub removed: Vec<String>,
    pub patched: Vec<SequenceEdgePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched step entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceStepPatchEntry {
    pub id: String,
    pub patch: SequenceStepPatch,
}

/// 🩹 One patched edge entry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SequenceEdgePatchEntry {
    pub id: String,
    pub patch: SequenceEdgePatch,
}
//#endregion 🔖️DeltaHelpers
