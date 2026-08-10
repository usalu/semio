//! 🌋️ EN 1998 artifact schema — every field with its state class.

use crate::artifacts::en1998::En1998Snapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full EN 1998 artifact state (persisted document + shared UI).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.norm.en1998")]
pub struct En1998Artifact {
    #[state(persistent)] pub seismic_zone: u8,
    #[state(persistent)] pub ground_type: String,
    #[state(persistent)] pub importance_class: String,
    #[state(persistent)] pub structural_system: String,
    #[state(persistent)] pub t1_s: f64,
    #[state(persistent)] pub mass_t: f64,
    #[state(persistent)] pub v_rd_kn: f64,
    #[state(persistent)] pub drift_mm: f64,
    #[state(persistent)] pub height_m: f64,
    #[state(persistent)] pub multiple_resisting_systems: bool,
    #[state(persistent)] pub annex: String,
    #[state(persistent)] pub en_a_gr: f64,
    #[state(persistent)] pub en_ground_type: String,
    #[state(persistent)] pub en_spectrum_type: String,
    #[state(persistent)] pub period_ratio: f64,
    #[state(persistent)] pub bridge_v_rd_kn: f64,
    #[state(persistent)] pub bearing_d_ed_mm: f64,
    #[state(persistent)] pub bearing_d_rd_mm: f64,
    #[state(persistent)] pub retrofit_knowledge_level: String,
    #[state(persistent)] pub retrofit_limit_state: String,
    #[state(persistent)] pub retrofit_e_d_kn: f64,
    #[state(persistent)] pub retrofit_r_k_kn: f64,
    #[state(persistent)] pub retrofit_gamma_el: f64,
    #[state(persistent)] pub silo_height_m: f64,
    #[state(persistent)] pub silo_radius_m: f64,
    #[state(persistent)] pub silo_n_rd_kn: f64,
    #[state(persistent)] pub silo_v_ed_kn: f64,
    #[state(persistent)] pub silo_v_rd_kn: f64,
    #[state(persistent)] pub silo_q_nominal: f64,
    #[state(persistent)] pub tank_height_m: f64,
    #[state(persistent)] pub tank_radius_m: f64,
    #[state(persistent)] pub tank_mass_t: f64,
    #[state(persistent)] pub tank_v_rd_kn: f64,
    #[state(persistent)] pub tower_m_ed_knm: f64,
    #[state(persistent)] pub tower_m_rd_knm: f64,
    #[state(persistent)] pub tower_is_chimney: bool,
    #[state(persistent)] pub tower_q_nominal: f64,
    #[state(persistent)] pub tower_mass_t: f64,
    #[state(persistent)] pub foundation_area_m2: f64,
    #[state(persistent)] pub foundation_p_rd_kpa: f64,
    #[state(persistent)] pub foundation_h_ed_kn: f64,
    #[state(persistent)] pub foundation_h_rd_kn: f64,
    #[state(persistent)] pub k_foundation: f64,
    #[state(persistent)] pub k_soil: f64,
    #[state(persistent)] pub wall_height_m: f64,
    #[state(persistent)] pub wall_phi_deg: f64,
    #[state(persistent)] pub wall_soil_gamma_kn_m3: f64,
    #[state(persistent)] pub wall_r: f64,
    #[state(persistent)] pub wall_h_rd_kn: f64,
    #[state(shared_ui)] pub selected_check_index: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for En1998Artifact {
    fn default() -> Self {
        Self::from_snapshot(En1998Snapshot::default())
    }
}

impl From<En1998Snapshot> for En1998Artifact {
    fn from(snapshot: crate::artifacts::en1998::En1998Snapshot) -> Self {
        Self::from_snapshot(snapshot)
    }
}

impl En1998Artifact {
    pub fn to_snapshot(&self) -> crate::artifacts::en1998::En1998Snapshot {
        crate::artifacts::en1998::En1998Snapshot {
            seismic_zone: self.seismic_zone.clone(),
            ground_type: self.ground_type.clone(),
            importance_class: self.importance_class.clone(),
            structural_system: self.structural_system.clone(),
            t1_s: self.t1_s.clone(),
            mass_t: self.mass_t.clone(),
            v_rd_kn: self.v_rd_kn.clone(),
            drift_mm: self.drift_mm.clone(),
            height_m: self.height_m.clone(),
            multiple_resisting_systems: self.multiple_resisting_systems.clone(),
            annex: self.annex.clone(),
            en_a_gr: self.en_a_gr.clone(),
            en_ground_type: self.en_ground_type.clone(),
            en_spectrum_type: self.en_spectrum_type.clone(),
            period_ratio: self.period_ratio.clone(),
            bridge_v_rd_kn: self.bridge_v_rd_kn.clone(),
            bearing_d_ed_mm: self.bearing_d_ed_mm.clone(),
            bearing_d_rd_mm: self.bearing_d_rd_mm.clone(),
            retrofit_knowledge_level: self.retrofit_knowledge_level.clone(),
            retrofit_limit_state: self.retrofit_limit_state.clone(),
            retrofit_e_d_kn: self.retrofit_e_d_kn.clone(),
            retrofit_r_k_kn: self.retrofit_r_k_kn.clone(),
            retrofit_gamma_el: self.retrofit_gamma_el.clone(),
            silo_height_m: self.silo_height_m.clone(),
            silo_radius_m: self.silo_radius_m.clone(),
            silo_n_rd_kn: self.silo_n_rd_kn.clone(),
            silo_v_ed_kn: self.silo_v_ed_kn.clone(),
            silo_v_rd_kn: self.silo_v_rd_kn.clone(),
            silo_q_nominal: self.silo_q_nominal.clone(),
            tank_height_m: self.tank_height_m.clone(),
            tank_radius_m: self.tank_radius_m.clone(),
            tank_mass_t: self.tank_mass_t.clone(),
            tank_v_rd_kn: self.tank_v_rd_kn.clone(),
            tower_m_ed_knm: self.tower_m_ed_knm.clone(),
            tower_m_rd_knm: self.tower_m_rd_knm.clone(),
            tower_is_chimney: self.tower_is_chimney.clone(),
            tower_q_nominal: self.tower_q_nominal.clone(),
            tower_mass_t: self.tower_mass_t.clone(),
            foundation_area_m2: self.foundation_area_m2.clone(),
            foundation_p_rd_kpa: self.foundation_p_rd_kpa.clone(),
            foundation_h_ed_kn: self.foundation_h_ed_kn.clone(),
            foundation_h_rd_kn: self.foundation_h_rd_kn.clone(),
            k_foundation: self.k_foundation.clone(),
            k_soil: self.k_soil.clone(),
            wall_height_m: self.wall_height_m.clone(),
            wall_phi_deg: self.wall_phi_deg.clone(),
            wall_soil_gamma_kn_m3: self.wall_soil_gamma_kn_m3.clone(),
            wall_r: self.wall_r.clone(),
            wall_h_rd_kn: self.wall_h_rd_kn.clone(),
        }
    }

    pub fn from_snapshot(snapshot: crate::artifacts::en1998::En1998Snapshot) -> Self {
        Self {
            seismic_zone: snapshot.seismic_zone,
            ground_type: snapshot.ground_type,
            importance_class: snapshot.importance_class,
            structural_system: snapshot.structural_system,
            t1_s: snapshot.t1_s,
            mass_t: snapshot.mass_t,
            v_rd_kn: snapshot.v_rd_kn,
            drift_mm: snapshot.drift_mm,
            height_m: snapshot.height_m,
            multiple_resisting_systems: snapshot.multiple_resisting_systems,
            annex: snapshot.annex,
            en_a_gr: snapshot.en_a_gr,
            en_ground_type: snapshot.en_ground_type,
            en_spectrum_type: snapshot.en_spectrum_type,
            period_ratio: snapshot.period_ratio,
            bridge_v_rd_kn: snapshot.bridge_v_rd_kn,
            bearing_d_ed_mm: snapshot.bearing_d_ed_mm,
            bearing_d_rd_mm: snapshot.bearing_d_rd_mm,
            retrofit_knowledge_level: snapshot.retrofit_knowledge_level,
            retrofit_limit_state: snapshot.retrofit_limit_state,
            retrofit_e_d_kn: snapshot.retrofit_e_d_kn,
            retrofit_r_k_kn: snapshot.retrofit_r_k_kn,
            retrofit_gamma_el: snapshot.retrofit_gamma_el,
            silo_height_m: snapshot.silo_height_m,
            silo_radius_m: snapshot.silo_radius_m,
            silo_n_rd_kn: snapshot.silo_n_rd_kn,
            silo_v_ed_kn: snapshot.silo_v_ed_kn,
            silo_v_rd_kn: snapshot.silo_v_rd_kn,
            silo_q_nominal: snapshot.silo_q_nominal,
            tank_height_m: snapshot.tank_height_m,
            tank_radius_m: snapshot.tank_radius_m,
            tank_mass_t: snapshot.tank_mass_t,
            tank_v_rd_kn: snapshot.tank_v_rd_kn,
            tower_m_ed_knm: snapshot.tower_m_ed_knm,
            tower_m_rd_knm: snapshot.tower_m_rd_knm,
            tower_is_chimney: snapshot.tower_is_chimney,
            tower_q_nominal: snapshot.tower_q_nominal,
            tower_mass_t: snapshot.tower_mass_t,
            foundation_area_m2: snapshot.foundation_area_m2,
            foundation_p_rd_kpa: snapshot.foundation_p_rd_kpa,
            foundation_h_ed_kn: snapshot.foundation_h_ed_kn,
            foundation_h_rd_kn: snapshot.foundation_h_rd_kn,
            k_foundation: snapshot.k_foundation,
            k_soil: snapshot.k_soil,
            wall_height_m: snapshot.wall_height_m,
            wall_phi_deg: snapshot.wall_phi_deg,
            wall_soil_gamma_kn_m3: snapshot.wall_soil_gamma_kn_m3,
            wall_r: snapshot.wall_r,
            wall_h_rd_kn: snapshot.wall_h_rd_kn,
            selected_check_index: None,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::en1998::En1998Snapshot) {
        self.seismic_zone = snapshot.seismic_zone;
        self.ground_type = snapshot.ground_type;
        self.importance_class = snapshot.importance_class;
        self.structural_system = snapshot.structural_system;
        self.t1_s = snapshot.t1_s;
        self.mass_t = snapshot.mass_t;
        self.v_rd_kn = snapshot.v_rd_kn;
        self.drift_mm = snapshot.drift_mm;
        self.height_m = snapshot.height_m;
        self.multiple_resisting_systems = snapshot.multiple_resisting_systems;
        self.annex = snapshot.annex;
        self.en_a_gr = snapshot.en_a_gr;
        self.en_ground_type = snapshot.en_ground_type;
        self.en_spectrum_type = snapshot.en_spectrum_type;
        self.period_ratio = snapshot.period_ratio;
        self.bridge_v_rd_kn = snapshot.bridge_v_rd_kn;
        self.bearing_d_ed_mm = snapshot.bearing_d_ed_mm;
        self.bearing_d_rd_mm = snapshot.bearing_d_rd_mm;
        self.retrofit_knowledge_level = snapshot.retrofit_knowledge_level;
        self.retrofit_limit_state = snapshot.retrofit_limit_state;
        self.retrofit_e_d_kn = snapshot.retrofit_e_d_kn;
        self.retrofit_r_k_kn = snapshot.retrofit_r_k_kn;
        self.retrofit_gamma_el = snapshot.retrofit_gamma_el;
        self.silo_height_m = snapshot.silo_height_m;
        self.silo_radius_m = snapshot.silo_radius_m;
        self.silo_n_rd_kn = snapshot.silo_n_rd_kn;
        self.silo_v_ed_kn = snapshot.silo_v_ed_kn;
        self.silo_v_rd_kn = snapshot.silo_v_rd_kn;
        self.silo_q_nominal = snapshot.silo_q_nominal;
        self.tank_height_m = snapshot.tank_height_m;
        self.tank_radius_m = snapshot.tank_radius_m;
        self.tank_mass_t = snapshot.tank_mass_t;
        self.tank_v_rd_kn = snapshot.tank_v_rd_kn;
        self.tower_m_ed_knm = snapshot.tower_m_ed_knm;
        self.tower_m_rd_knm = snapshot.tower_m_rd_knm;
        self.tower_is_chimney = snapshot.tower_is_chimney;
        self.tower_q_nominal = snapshot.tower_q_nominal;
        self.tower_mass_t = snapshot.tower_mass_t;
        self.foundation_area_m2 = snapshot.foundation_area_m2;
        self.foundation_p_rd_kpa = snapshot.foundation_p_rd_kpa;
        self.foundation_h_ed_kn = snapshot.foundation_h_ed_kn;
        self.foundation_h_rd_kn = snapshot.foundation_h_rd_kn;
        self.k_foundation = snapshot.k_foundation;
        self.k_soil = snapshot.k_soil;
        self.wall_height_m = snapshot.wall_height_m;
        self.wall_phi_deg = snapshot.wall_phi_deg;
        self.wall_soil_gamma_kn_m3 = snapshot.wall_soil_gamma_kn_m3;
        self.wall_r = snapshot.wall_r;
        self.wall_h_rd_kn = snapshot.wall_h_rd_kn;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn en1998_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.norm.en1998",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
