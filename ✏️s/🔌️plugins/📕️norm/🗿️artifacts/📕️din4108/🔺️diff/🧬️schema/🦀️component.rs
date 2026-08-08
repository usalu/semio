//! 🧬️ Din4108 diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the Din4108 artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.din4108")]
pub struct Din4108Diff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::din4108::schema::Din4108Artifact>>,
    #[state(persistent)] pub category: Option<String>,
    #[state(persistent)] pub layers: Option<Din4108StringList>,
    #[state(persistent)] pub climate: Option<crate::document::ClimateZoneDe>,
    #[state(persistent)] pub airtightness_n50: Option<f64>,
    #[state(persistent)] pub psi_times_l_sum: Option<f64>,
    #[state(persistent)] pub rh_int: Option<f64>,
    #[state(persistent)] pub catalog_id: Option<String>,
    #[state(persistent)] pub material_id: Option<String>,
    #[state(persistent)] pub airtightness_class: Option<String>,
    #[state(persistent)] pub t_int_c: Option<f64>,
    #[state(persistent)] pub solar_absorptance: Option<f64>,
    #[state(persistent)] pub irradiance_w_m2: Option<f64>,
    #[state(persistent)] pub moisture_mu_exterior: Option<f64>,
    #[state(persistent)] pub moisture_mu_interior: Option<f64>,
    #[state(persistent)] pub envelope_area_m2: Option<f64>,
    #[state(persistent)] pub bb2_details_conform: Option<bool>,
    #[state(persistent)] pub application_type: Option<String>,
    #[state(persistent)] pub declared_application_class: Option<String>,
    #[state(shared_ui)] pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Din4108StringList { pub values: Vec<String> }
//#endregion 🔖️DeltaHelpers
