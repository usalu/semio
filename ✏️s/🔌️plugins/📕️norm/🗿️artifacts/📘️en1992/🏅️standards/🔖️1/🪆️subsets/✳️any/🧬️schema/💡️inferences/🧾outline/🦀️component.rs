//! 🧾 `outline` — one named inference: this document's own field/section structure. A norm
//! compliance record IS the document it describes, so its "outline" is its top-level field list
//! (`sectionOutline`/`fieldCount`, fixed by the snapshot's own schema shape) plus a real
//! `entryCount` over whatever repeated sub-entries it actually carries (0 when the snapshot has
//! no collection-typed top-level field).

use crate::artifacts::en1992::En1992Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
    "annex",
    "m_ed_knm",
    "v_ed_kn",
    "f_ck",
    "b_mm",
    "d_mm",
    "a_s_mm2",
    "f_yk",
    "rho_l",
    "n_ed_kn",
    "p_kn",
    "a_c_mm2",
    "use_fem",
    "span_m",
    "udl_kn_m",
    "fire_rating",
    "provided_axis_distance_mm",
    "bridge_sigma_c_mpa",
    "bridge_delta_sigma_s_mpa",
    "tightness_class",
    "hd_over_h",
    "liquid_sigma_s_mpa",
    "liquid_rho_p_eff",
    "liquid_f_ct_eff_mpa",
    "liquid_e_s_mpa",
    "liquid_s_r_max_mm",
    "anchor_h_ef_mm",
    "anchor_cracked",
    "anchor_f_uk_mpa",
    "anchor_f_yk_mpa",
    "anchor_a_s_mm2",
    "anchor_d_mm",
    "anchor_c1_mm",
    "anchor_n_ed_kn",
    "anchor_v_ed_kn",
];

/// 🧾️ `En1992` document outline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct En1992Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl En1992Outline {
    pub async fn compute(_snapshot: &En1992Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = 0;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1992Outline {
    fn default() -> Self {
        Self::compute(&En1992Snapshot::default())
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn outline_field_count_matches_section_outline_length() {
        let outline = En1992Outline::compute(&En1992Snapshot::default());
        assert_eq!(outline.field_count as usize, outline.section_outline.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn outline_is_deterministic() {
        let snapshot = En1992Snapshot::default();
        assert_eq!(En1992Outline::compute(&snapshot), En1992Outline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
