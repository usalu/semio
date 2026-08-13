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
    #[state(persistent)]
    pub artifact: Option<Box<crate::artifacts::wires::schema::WiresArtifact>>,
    #[state(persistent)]
    pub wires_fixture: Option<DslValue>,
    #[state(persistent)]
    pub content: Option<crate::artifacts::wires::WiresContentChild>,
    #[state(persistent)]
    pub camera: Option<DslValue>,
    #[state(persistent)]
    pub meta: Option<DslValue>,
    #[state(shared_ui)]
    pub selected_ids: Option<WiresStringList>,
    #[state(preview)]
    pub drag_node_id: Option<Option<String>>,
    #[state(preview)]
    pub drag_last_x: Option<f64>,
    #[state(preview)]
    pub drag_last_y: Option<f64>,
    #[state(local_ui)]
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
