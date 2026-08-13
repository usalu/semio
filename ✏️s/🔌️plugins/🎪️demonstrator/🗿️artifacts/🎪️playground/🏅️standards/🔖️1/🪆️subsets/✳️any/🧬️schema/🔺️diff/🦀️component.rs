//! 🧬️ Playground diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the playground artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.demonstrator.playground")]
pub struct PlaygroundDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::playground::schema::PlaygroundArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
}
//#endregion 🔖️Diff
