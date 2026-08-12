//! 🧾 `outline` — one named inference: this document's own field/section structure. A norm
//! compliance record IS the document it describes, so its "outline" is its top-level field list
//! (`sectionOutline`/`fieldCount`, fixed by the snapshot's own schema shape) plus a real
//! `entryCount` over whatever repeated sub-entries it actually carries (0 when the snapshot has
//! no collection-typed top-level field).

use crate::artifacts::en1996::En1996Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
        "m_ed_knm",
        "n_ed_kn",
        "v_ed_kn",
        "h_ed_kn",
        "z_mm3",
        "area_mm2",
        "shear_area_mm2",
        "f_k_mpa",
        "f_vk_mpa",
        "annex",
        "masonry_class",
        "design_situation",
        "mu",
        "wall_thickness_mm",
        "fire_resistance_min",
        "unit",
        "exposure",
        "mortar",
        "bed_joint_thickness_mm",
        "storeys",
        "h_ef_mm",
        "t_ef_mm",
];

/// 🧾️ `En1996` document outline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct En1996Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl En1996Outline {
    pub fn compute(_snapshot: &En1996Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = 0;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1996Outline {
    fn default() -> Self {
        Self::compute(&En1996Snapshot::default())
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[test]
    fn outline_field_count_matches_section_outline_length() {
        let outline = En1996Outline::compute(&En1996Snapshot::default());
        assert_eq!(outline.field_count as usize, outline.section_outline.len());
    }

    #[test]
    fn outline_is_deterministic() {
        let snapshot = En1996Snapshot::default();
        assert_eq!(En1996Outline::compute(&snapshot), En1996Outline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
