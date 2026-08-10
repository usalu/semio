//! 🧬️ VCS diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the VCS artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.vcs.vcs")]
pub struct VcsDiff {
    #[state(persistent)]
    pub artifact: Option<Box<crate::artifacts::vcs::schema::VcsArtifact>>,
    #[state(persistent)]
    pub schema: Option<String>,
    #[state(persistent)]
    pub title: Option<String>,
    #[state(persistent)]
    pub counter: Option<i64>,
    #[state(persistent)]
    pub notes: Option<String>,
    #[state(persistent)]
    pub status: Option<String>,
    #[state(persistent)]
    pub tags: Option<VcsTagsDelta>,
    #[state(shared_ui)]
    pub selected_checkpoint_ids: Option<VcsStringList>,
    #[state(local_ui)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct VcsStringList {
    pub values: Vec<String>,
}

/// 🏷️ Tag-list sparse delta (added/removed tag strings).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct VcsTagsDelta {
    pub added: Vec<String>,
    pub removed: Vec<String>,
}
//#endregion 🔖️DeltaHelpers
