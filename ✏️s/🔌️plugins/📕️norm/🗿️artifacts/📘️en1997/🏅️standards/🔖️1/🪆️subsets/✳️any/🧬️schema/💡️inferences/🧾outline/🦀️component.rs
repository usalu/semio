//! 🧾 `outline` — one named inference: this document's own field/section structure. A norm
//! compliance record IS the document it describes, so its "outline" is its top-level field list
//! (`sectionOutline`/`fieldCount`, fixed by the snapshot's own schema shape) plus a real
//! `entryCount` over whatever repeated sub-entries it actually carries (0 when the snapshot has
//! no collection-typed top-level field).

use crate::artifacts::en1997::En1997Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
    "v_ed_kn",
    "h_ed_kn",
    "footing_area_m2",
    "phi_deg",
    "c_kpa",
    "gamma_kn_m3",
    "b_m",
    "d_f_m",
    "e_s_mpa",
    "nu",
    "design_approach",
    "annex",
    "settlement_limit_mm",
    "n_pile_ed_kn",
    "alpha_s",
    "pile_d_m",
    "q_s_kpa",
    "pile_l_m",
    "q_b_kpa",
    "pile_base_area_m2",
    "pile_n_profiles",
    "z_investigated_m",
];

/// 🧾️ `En1997` document outline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct En1997Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl En1997Outline {
    pub async fn compute(_snapshot: &En1997Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = 0;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1997Outline {
    async fn default() -> Self {
        Self::compute(&En1997Snapshot::default())
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[test]
    async fn outline_field_count_matches_section_outline_length() {
        let outline = En1997Outline::compute(&En1997Snapshot::default());
        assert_eq!(outline.field_count as usize, outline.section_outline.len());
    }

    #[test]
    async fn outline_is_deterministic() {
        let snapshot = En1997Snapshot::default();
        assert_eq!(En1997Outline::compute(&snapshot), En1997Outline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
