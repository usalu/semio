//! 🧬️ Din18599 diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the Din18599 artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.din18599")]
pub struct Din18599Diff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::din18599::schema::Din18599Artifact>>,
    #[state(persistent)] pub use_class: Option<crate::artifacts::din18599::UseClass>,
    #[state(persistent)] pub heated_area_m2: Option<f64>,
    #[state(persistent)] pub occupants: Option<u32>,
    #[state(persistent)] pub h_t: Option<f64>,
    #[state(persistent)] pub h_v: Option<f64>,
    #[state(persistent)] pub climate: Option<crate::artifacts::din18599::MonthlyClimate>,
    #[state(persistent)] pub internal_gains_w_m2: Option<f64>,
    #[state(persistent)] pub solar_gains_kwh: Option<f64>,
    #[state(persistent)] pub system_losses_kwh: Option<f64>,
    #[state(persistent)] pub renewable_kwh: Option<f64>,
    #[state(persistent)] pub annual_limit_kwh: Option<f64>,
    #[state(persistent)] pub energy_carrier: Option<String>,
    #[state(persistent)] pub reference_q_p_kwh: Option<f64>,
    #[state(shared_ui)] pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Din18599StringList { pub values: Vec<String> }
//#endregion 🔖️DeltaHelpers
