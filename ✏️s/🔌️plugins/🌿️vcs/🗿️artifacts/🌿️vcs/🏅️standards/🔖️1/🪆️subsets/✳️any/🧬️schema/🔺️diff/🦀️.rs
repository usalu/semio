//! 🧬️ VCS diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the VCS artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact_schema(id = "s.vcs.vcs")]
pub struct VcsDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::vcs::schema::VcsArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub title: Option<String>,
    #[state(artifact)]
    pub counter: Option<i64>,
    #[state(artifact)]
    pub notes: Option<String>,
    #[state(artifact)]
    pub status: Option<String>,
    #[state(artifact)]
    pub tags: Option<VcsTagsDelta>,
    #[state(presence)]
    pub selected_checkpoint_ids: Option<VcsStringList>,
    #[state(config)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct VcsStringList {
    pub values: Vec<String>,
}

/// 🏷️ Tag-list sparse delta (added/removed tag strings).
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct VcsTagsDelta {
    pub added: Vec<String>,
    pub removed: Vec<String>,
}
//#endregion 🔖️DeltaHelpers
