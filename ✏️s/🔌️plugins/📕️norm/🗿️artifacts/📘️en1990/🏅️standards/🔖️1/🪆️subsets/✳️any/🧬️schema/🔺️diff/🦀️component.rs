//! 🧬️ En1990 diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use crate::artifacts::en1990::En1990QkChild;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1990 artifact. `q_k` is a single-`Option` composed-child slot
/// (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2) — always-present slot shape per
/// `📓️migration-recipe.md` §8, matching `➗️mathematical`'s `notation`/`results`/`computed` diff
/// fields. The former whole-document-replace `artifact: Option<Box<En1990Artifact>>` slot is
/// removed — dead code (never constructed by any app command; `set-snapshot` already decomposes
/// into the closed semantic mutation vocabulary via `En1990Mutation::from_snapshot`) and shaped
/// exactly like the banned `SetSnapshot` vocabulary.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1990")]
pub struct En1990Diff {
    #[state(persistent)] pub g_k: Option<f64>,
    #[state(persistent)] pub q_k: Option<En1990QkChild>,
    #[state(persistent)] pub resistance_kn: Option<f64>,
    #[state(persistent)] pub consequence_class: Option<u8>,
    #[state(persistent)] pub annex: Option<crate::document::AnnexChoice>,
    #[state(persistent)] pub seismic_a_ed_kn: Option<f64>,
    #[state(shared_ui)] pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct En1990StringList { pub values: Vec<String> }
//#endregion 🔖️DeltaHelpers
