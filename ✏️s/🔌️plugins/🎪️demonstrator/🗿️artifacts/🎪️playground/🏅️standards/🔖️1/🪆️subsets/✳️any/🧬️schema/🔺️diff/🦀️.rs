//! 🧬️ Playground diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the playground artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.demonstrator.playground")]
pub struct PlaygroundDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::playground::standards::v1::subsets::any::schema::PlaygroundArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
}
//#endregion 🔖️Diff
