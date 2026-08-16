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
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::en1998::schema::En1998Artifact>>,
    #[state(artifact)]
    pub seismic_zone: Option<u8>,
    #[state(artifact)]
    pub ground_type: Option<String>,
    #[state(artifact)]
    pub importance_class: Option<String>,
    #[state(artifact)]
    pub structural_system: Option<String>,
    #[state(artifact)]
    pub t1_s: Option<f64>,
    #[state(artifact)]
    pub mass_t: Option<f64>,
    #[state(artifact)]
    pub v_rd_kn: Option<f64>,
    #[state(artifact)]
    pub drift_mm: Option<f64>,
    #[state(artifact)]
    pub height_m: Option<f64>,
    #[state(artifact)]
    pub multiple_resisting_systems: Option<bool>,
    #[state(artifact)]
    pub annex: Option<String>,
    #[state(artifact)]
    pub en_a_gr: Option<f64>,
    #[state(artifact)]
    pub en_ground_type: Option<String>,
    #[state(artifact)]
    pub en_spectrum_type: Option<String>,
    #[state(artifact)]
    pub period_ratio: Option<f64>,
    #[state(artifact)]
    pub bridge_v_rd_kn: Option<f64>,
    #[state(artifact)]
    pub bearing_d_ed_mm: Option<f64>,
    #[state(artifact)]
    pub bearing_d_rd_mm: Option<f64>,
    #[state(artifact)]
    pub retrofit_knowledge_level: Option<String>,
    #[state(artifact)]
    pub retrofit_limit_state: Option<String>,
    #[state(artifact)]
    pub retrofit_e_d_kn: Option<f64>,
    #[state(artifact)]
    pub retrofit_r_k_kn: Option<f64>,
    #[state(artifact)]
    pub retrofit_gamma_el: Option<f64>,
    #[state(artifact)]
    pub silo_height_m: Option<f64>,
    #[state(artifact)]
    pub silo_radius_m: Option<f64>,
    #[state(artifact)]
    pub silo_n_rd_kn: Option<f64>,
    #[state(artifact)]
    pub silo_v_ed_kn: Option<f64>,
    #[state(artifact)]
    pub silo_v_rd_kn: Option<f64>,
    #[state(artifact)]
    pub silo_q_nominal: Option<f64>,
    #[state(artifact)]
    pub tank_height_m: Option<f64>,
    #[state(artifact)]
    pub tank_radius_m: Option<f64>,
    #[state(artifact)]
    pub tank_mass_t: Option<f64>,
    #[state(artifact)]
    pub tank_v_rd_kn: Option<f64>,
    #[state(artifact)]
    pub tower_m_ed_knm: Option<f64>,
    #[state(artifact)]
    pub tower_m_rd_knm: Option<f64>,
    #[state(artifact)]
    pub tower_is_chimney: Option<bool>,
    #[state(artifact)]
    pub tower_q_nominal: Option<f64>,
    #[state(artifact)]
    pub tower_mass_t: Option<f64>,
    #[state(artifact)]
    pub foundation_area_m2: Option<f64>,
    #[state(artifact)]
    pub foundation_p_rd_kpa: Option<f64>,
    #[state(artifact)]
    pub foundation_h_ed_kn: Option<f64>,
    #[state(artifact)]
    pub foundation_h_rd_kn: Option<f64>,
    #[state(artifact)]
    pub k_foundation: Option<f64>,
    #[state(artifact)]
    pub k_soil: Option<f64>,
    #[state(artifact)]
    pub wall_height_m: Option<f64>,
    #[state(artifact)]
    pub wall_phi_deg: Option<f64>,
    #[state(artifact)]
    pub wall_soil_gamma_kn_m3: Option<f64>,
    #[state(artifact)]
    pub wall_r: Option<f64>,
    #[state(artifact)]
    pub wall_h_rd_kn: Option<f64>,
    #[state(presence)]
    pub selected_check_index: Option<Option<u32>>,
}
//#endregion 🔖️Diff
