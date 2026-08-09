//! 🧬️ En1991 diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;
use crate::artifacts::en1991::part_1_2::FireCurve;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the En1991 artifact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1991")]
pub struct En1991Diff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::en1991::schema::En1991Artifact>>,
    #[state(persistent)] pub area_m2: Option<f64>,
    #[state(persistent)] pub category: Option<crate::document::ImposedCategory>,
    #[state(persistent)] pub annex: Option<crate::document::AnnexChoice>,
    #[state(persistent)] pub self_weight_material: Option<String>,
    #[state(persistent)] pub self_weight_thickness_m: Option<f64>,
    #[state(persistent)] pub assumed_g_k_kn_m2: Option<f64>,
    #[state(persistent)] pub fire_curve: Option<crate::artifacts::en1991::part_1_2::FireCurve>,
    #[state(persistent)] pub fire_resistance_min: Option<f64>,
    #[state(persistent)] pub fire_member_capacity_c: Option<f64>,
    #[state(persistent)] pub snow_zone: Option<u8>,
    #[state(persistent)] pub snow_altitude_m: Option<f64>,
    #[state(persistent)] pub en_s_k_kn_m2: Option<f64>,
    #[state(persistent)] pub wind_zone: Option<u8>,
    #[state(persistent)] pub en_v_b_m_s: Option<f64>,
    #[state(persistent)] pub delta_t_k: Option<f64>,
    #[state(persistent)] pub construction_activity: Option<String>,
    #[state(persistent)] pub accidental_mass_t: Option<f64>,
    #[state(persistent)] pub accidental_speed_km_h: Option<f64>,
    #[state(persistent)] pub bridge_lane: Option<u8>,
    #[state(persistent)] pub bridge_span_m: Option<f64>,
    #[state(persistent)] pub bridge_lane_width_m: Option<f64>,
    #[state(persistent)] pub bridge_moment_resistance_knm: Option<f64>,
    #[state(persistent)] pub crane_class: Option<String>,
    #[state(persistent)] pub hoist_class: Option<String>,
    #[state(persistent)] pub hoisting_speed_m_s: Option<f64>,
    #[state(persistent)] pub silo_bulk_density_kn_m3: Option<f64>,
    #[state(persistent)] pub silo_height_m: Option<f64>,
    #[state(persistent)] pub silo_hydraulic_radius_m: Option<f64>,
    #[state(persistent)] pub silo_mu: Option<f64>,
    #[state(persistent)] pub silo_k: Option<f64>,
    #[state(persistent)] pub c_s: Option<f64>,
    #[state(persistent)] pub c_d: Option<f64>,
    #[state(shared_ui)] pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 List wrapper for optional vector diffs.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct En1991StringList { pub values: Vec<String> }
//#endregion 🔖️DeltaHelpers
