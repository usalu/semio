//! 🧬️ Wires diff schema — sparse field delta over the artifact.

use dsl::DslValue;
use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the wires artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
/// `content` is a single always-present-slot `Option` (never absent, only ever replaced — see
/// `📓️migration-recipe.md` §8), matching `dag`'s/`flow`'s/writer's `document`/`content` diff shape.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.reasoning.wires")]
pub struct WiresDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::schema::WiresArtifact>>,
    #[state(artifact)]
    pub wires_fixture: Option<DslValue>,
    #[state(artifact)]
    pub content: Option<crate::WiresContentChild>,
    #[state(artifact)]
    pub meta: Option<DslValue>,
}
//#endregion 🔖️Diff
