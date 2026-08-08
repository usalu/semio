//! 🧬️ EN 1998 diff schema — sparse field delta.

use crate::artifacts::en1998::schema::En1998Artifact as EnArtifact;
use crate::artifacts::en1998::En1998Snapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.norm.en1998")]
pub struct En1998Diff {
    #[state(persistent)] pub artifact: Option<Box<crate::artifacts::en1998::schema::En1998Artifact>>,
    #[state(persistent)] pub seismic_zone: Option<u8>,
    #[state(persistent)] pub ground_type: Option<String>,
    #[state(persistent)] pub importance_class: Option<String>,
    #[state(persistent)] pub structural_system: Option<String>,
    #[state(persistent)] pub t1_s: Option<f64>,
    #[state(persistent)] pub mass_t: Option<f64>,
    #[state(persistent)] pub v_rd_kn: Option<f64>,
    #[state(persistent)] pub drift_mm: Option<f64>,
    #[state(persistent)] pub height_m: Option<f64>,
    #[state(persistent)] pub multiple_resisting_systems: Option<bool>,
    #[state(persistent)] pub annex: Option<String>,
    #[state(persistent)] pub en_a_gr: Option<f64>,
    #[state(persistent)] pub en_ground_type: Option<String>,
    #[state(persistent)] pub en_spectrum_type: Option<String>,
    #[state(persistent)] pub period_ratio: Option<f64>,
    #[state(persistent)] pub bridge_v_rd_kn: Option<f64>,
    #[state(persistent)] pub bearing_d_ed_mm: Option<f64>,
    #[state(persistent)] pub bearing_d_rd_mm: Option<f64>,
    #[state(persistent)] pub retrofit_knowledge_level: Option<String>,
    #[state(persistent)] pub retrofit_limit_state: Option<String>,
    #[state(persistent)] pub retrofit_e_d_kn: Option<f64>,
    #[state(persistent)] pub retrofit_r_k_kn: Option<f64>,
    #[state(persistent)] pub retrofit_gamma_el: Option<f64>,
    #[state(persistent)] pub silo_height_m: Option<f64>,
    #[state(persistent)] pub silo_radius_m: Option<f64>,
    #[state(persistent)] pub silo_n_rd_kn: Option<f64>,
    #[state(persistent)] pub silo_v_ed_kn: Option<f64>,
    #[state(persistent)] pub silo_v_rd_kn: Option<f64>,
    #[state(persistent)] pub silo_q_nominal: Option<f64>,
    #[state(persistent)] pub tank_height_m: Option<f64>,
    #[state(persistent)] pub tank_radius_m: Option<f64>,
    #[state(persistent)] pub tank_mass_t: Option<f64>,
    #[state(persistent)] pub tank_v_rd_kn: Option<f64>,
    #[state(persistent)] pub tower_m_ed_knm: Option<f64>,
    #[state(persistent)] pub tower_m_rd_knm: Option<f64>,
    #[state(persistent)] pub tower_is_chimney: Option<bool>,
    #[state(persistent)] pub tower_q_nominal: Option<f64>,
    #[state(persistent)] pub tower_mass_t: Option<f64>,
    #[state(persistent)] pub foundation_area_m2: Option<f64>,
    #[state(persistent)] pub foundation_p_rd_kpa: Option<f64>,
    #[state(persistent)] pub foundation_h_ed_kn: Option<f64>,
    #[state(persistent)] pub foundation_h_rd_kn: Option<f64>,
    #[state(persistent)] pub k_foundation: Option<f64>,
    #[state(persistent)] pub k_soil: Option<f64>,
    #[state(persistent)] pub wall_height_m: Option<f64>,
    #[state(persistent)] pub wall_phi_deg: Option<f64>,
    #[state(persistent)] pub wall_soil_gamma_kn_m3: Option<f64>,
    #[state(persistent)] pub wall_r: Option<f64>,
    #[state(persistent)] pub wall_h_rd_kn: Option<f64>,
    #[state(shared_ui)] pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff
