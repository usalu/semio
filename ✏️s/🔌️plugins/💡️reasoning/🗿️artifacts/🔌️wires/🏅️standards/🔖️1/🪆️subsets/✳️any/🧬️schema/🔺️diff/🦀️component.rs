//! 🧬️ Wires diff schema — sparse field delta over the artifact.

use dsl::DslValue;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the wires artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
/// `content` is a single always-present-slot `Option` (never absent, only ever replaced — see
/// `📓️migration-recipe.md` §8), matching `dag`'s/`flow`'s/writer's `document`/`content` diff shape.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.reasoning.wires")]
pub struct WiresDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::wires::schema::WiresArtifact>>,
    #[state(artifact)]
    pub wires_fixture: Option<DslValue>,
    #[state(artifact)]
    pub content: Option<crate::artifacts::wires::WiresContentChild>,
    #[state(artifact)]
    pub camera: Option<DslValue>,
    #[state(artifact)]
    pub meta: Option<DslValue>,
    #[state(presence)]
    pub selected_ids: Option<WiresStringList>,
    #[state(artifact)]
    pub drag_node_id: Option<Option<String>>,
    #[state(artifact)]
    pub drag_last_x: Option<f64>,
    #[state(artifact)]
    pub drag_last_y: Option<f64>,
    #[state(config)]
    pub locale: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WiresStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers
