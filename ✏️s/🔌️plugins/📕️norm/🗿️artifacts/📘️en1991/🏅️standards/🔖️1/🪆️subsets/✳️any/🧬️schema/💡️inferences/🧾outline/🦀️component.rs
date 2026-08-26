//! 🧾 `outline` — one named inference: this document's own field/section structure. A norm
//! compliance record IS the document it describes, so its "outline" is its top-level field list
//! (`sectionOutline`/`fieldCount`, fixed by the snapshot's own schema shape) plus a real
//! `entryCount` over whatever repeated sub-entries it actually carries (0 when the snapshot has
//! no collection-typed top-level field).

use crate::artifacts::en1991::En1991Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
    "area_m2",
    "category",
    "annex",
    "self_weight_material",
    "self_weight_thickness_m",
    "assumed_g_k_kn_m2",
    "fire_curve",
    "fire_resistance_min",
    "fire_member_capacity_c",
    "snow_zone",
    "snow_altitude_m",
    "en_s_k_kn_m2",
    "wind_zone",
    "en_v_b_m_s",
    "delta_t_k",
    "construction_activity",
    "accidental_mass_t",
    "accidental_speed_km_h",
    "bridge_lane",
    "bridge_span_m",
    "bridge_lane_width_m",
    "bridge_moment_resistance_knm",
    "crane_class",
    "hoist_class",
    "hoisting_speed_m_s",
    "silo_bulk_density_kn_m3",
    "silo_height_m",
    "silo_hydraulic_radius_m",
    "silo_mu",
    "silo_k",
    "c_s",
    "c_d",
];

/// 🧾️ `En1991` document outline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct En1991Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl En1991Outline {
    pub fn compute(_snapshot: &En1991Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = 0;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1991Outline {
    fn default() -> Self {
        Self::compute(&En1991Snapshot::default())
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn outline_field_count_matches_section_outline_length() {
        let outline = En1991Outline::compute(&En1991Snapshot::default());
        assert_eq!(outline.field_count as usize, outline.section_outline.len());
    }

    #[semio_framework_async_macros::async_test]
    fn outline_is_deterministic() {
        let snapshot = En1991Snapshot::default();
        assert_eq!(En1991Outline::compute(&snapshot), En1991Outline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
