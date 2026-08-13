//! 🧬️ Forms diff schema — sparse field delta over the artifact.

use crate::artifacts::forms::{FormQuestion, FormStep, FormsResultsChild, FormsStructureChild};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the forms artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
///
/// Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`forms→C:value,table`): the old
/// `steps: Option<FormsStepsDelta>` field (an id-keyed sparse collection delta) and the dead
/// whole-snapshot-replace `artifact: Option<Box<FormsArtifact>>` slot (the banned `SetSnapshot`
/// vocabulary — grepped: never constructed by any app command, only by this file's own now-removed
/// `diff_set_snapshot`/`sparse_diff_between` dead code) are both removed. `structure`/`results`
/// (`Option<ArtifactChild<S>>`, single-Option "always-present slot" shape) replace them: every
/// mutation triad still builds its change as a `FormsStepsDelta` internally (that type is UNCHANGED,
/// see `🔖️DeltaHelpers` below) and applies it against the WORKING-SCENE steps
/// (`crate::artifacts::forms::forms_steps`, not a snapshot field) to get the resulting `Vec<FormStep>`,
/// then regenerates both composed children from that result — the granular, cascade-aware mutation
/// semantics are unchanged, only the diff's own wire representation of "what changed" becomes a
/// pair of regenerated content-addressed handles, exactly like every other composed plugin in this
/// ticket (see `crate::artifacts::forms::🔖️Composition`'s own doc comment).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.forms.forms")]
pub struct FormsDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub version: Option<String>,
    #[state(artifact)]
    pub title: Option<Option<String>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.value")]
    pub structure: Option<FormsStructureChild>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.table")]
    pub results: Option<FormsResultsChild>,
    #[state(presence)]
    pub selected_ids: Option<FormsStringList>,
    #[state(config)]
    pub current_step_index: Option<u32>,
    #[state(config)]
    pub try_values_json: Option<String>,
    #[state(config)]
    pub locale: Option<String>,
    #[state(config)]
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

/// 🩹 Partial step replacement. `blocks`, when set, is the step's FULL new `blocks` list — a
/// bounded, single-step-scoped whole-value swap (mirrors how a sibling facet's `MathematicalDiff`
/// replaces a whole bounded sub-collection rather than diffing every element field-by-field), never
/// a whole-DOCUMENT replacement: every `🧬️mutations/*create-block/*delete-block/*move-block-to-step`
/// triad leaf builds this by cloning only the touched step(s)' own `blocks` Vec, not `FormsSnapshot`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct FormsStepPatch {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub blocks: Option<Vec<FormQuestion>>,
}
//#endregion 🔖️DeltaHelpers
