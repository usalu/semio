//! 🧾 `outline` — one named inference: this document's own field/section structure. A norm
//! compliance record IS the document it describes, so its "outline" is its top-level field list
//! (`sectionOutline`/`fieldCount`, fixed by the snapshot's own schema shape) plus a real
//! `entryCount` over whatever repeated sub-entries it actually carries (0 when the snapshot has
//! no collection-typed top-level field).

use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
    "seismic_zone",
    "ground_type",
    "importance_class",
    "structural_system",
    "t1_s",
    "mass_t",
    "v_rd_kn",
    "drift_mm",
    "height_m",
    "multiple_resisting_systems",
    "annex",
    "en_a_gr",
    "en_ground_type",
    "en_spectrum_type",
    "period_ratio",
    "bridge_v_rd_kn",
    "bearing_d_ed_mm",
    "bearing_d_rd_mm",
    "retrofit_knowledge_level",
    "retrofit_limit_state",
    "retrofit_e_d_kn",
    "retrofit_r_k_kn",
    "retrofit_gamma_el",
    "silo_height_m",
    "silo_radius_m",
    "silo_n_rd_kn",
    "silo_v_ed_kn",
    "silo_v_rd_kn",
    "silo_q_nominal",
    "tank_height_m",
    "tank_radius_m",
    "tank_mass_t",
    "tank_v_rd_kn",
    "tower_m_ed_knm",
    "tower_m_rd_knm",
    "tower_is_chimney",
    "tower_q_nominal",
    "tower_mass_t",
    "foundation_area_m2",
    "foundation_p_rd_kpa",
    "foundation_h_ed_kn",
    "foundation_h_rd_kn",
    "k_foundation",
    "k_soil",
    "wall_height_m",
    "wall_phi_deg",
    "wall_soil_gamma_kn_m3",
    "wall_r",
    "wall_h_rd_kn",
];

/// 🧾️ `En1998` document outline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct En1998Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl En1998Outline {
    pub async fn compute(_snapshot: &En1998Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = 0;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1998Outline {
    async fn default() -> Self {
        Self::compute(&En1998Snapshot::default())
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[test]
    async fn outline_field_count_matches_section_outline_length() {
        let outline = En1998Outline::compute(&En1998Snapshot::default());
        assert_eq!(outline.field_count as usize, outline.section_outline.len());
    }

    #[test]
    async fn outline_is_deterministic() {
        let snapshot = En1998Snapshot::default();
        assert_eq!(En1998Outline::compute(&snapshot), En1998Outline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
