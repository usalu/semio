//! 🧬️ Din18599 diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the Din18599 artifact. `climate` is a single-`Option` composed-child
/// slot (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM round 2) — always-present slot shape
/// per `📓️migration-recipe.md` §8, matching `➗️mathematical`'s/en1990's own composed-child diff
/// fields. The former whole-document-replace `artifact: Option<Box<Din18599Artifact>>` slot is
/// removed — dead code (never constructed by any app command; `set-snapshot` already decomposes
/// into the closed semantic mutation vocabulary via `Din18599Mutation::from_snapshot`) and shaped
/// exactly like the banned `SetSnapshot` vocabulary.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.din18599")]
pub struct Din18599Diff {
    #[state(artifact)] pub use_class: Option<crate::artifacts::din18599::UseClass>,
    #[state(artifact)] pub heated_area_m2: Option<f64>,
    #[state(artifact)] pub occupants: Option<u32>,
    #[state(artifact)] pub h_t: Option<f64>,
    #[state(artifact)] pub h_v: Option<f64>,
    #[state(artifact)] pub climate: Option<crate::artifacts::din18599::Din18599ClimateChild>,
    #[state(artifact)] pub internal_gains_w_m2: Option<f64>,
    #[state(artifact)] pub solar_gains_kwh: Option<f64>,
    #[state(artifact)] pub system_losses_kwh: Option<f64>,
    #[state(artifact)] pub renewable_kwh: Option<f64>,
    #[state(artifact)] pub annual_limit_kwh: Option<f64>,
    #[state(artifact)] pub energy_carrier: Option<String>,
    #[state(artifact)] pub reference_q_p_kwh: Option<f64>,
    #[state(presence)] pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Din18599StringList { pub values: Vec<String> }
//#endregion 🔖️DeltaHelpers
