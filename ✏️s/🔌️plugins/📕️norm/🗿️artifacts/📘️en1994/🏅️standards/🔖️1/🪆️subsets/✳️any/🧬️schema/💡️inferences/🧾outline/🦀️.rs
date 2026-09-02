//! 🧾 `outline` — one named inference: this document's own field/section structure. A norm
//! compliance record IS the document it describes, so its "outline" is its top-level field list
//! (`sectionOutline`/`fieldCount`, fixed by the snapshot's own schema shape) plus a real
//! `entryCount` over whatever repeated sub-entries it actually carries (0 when the snapshot has
//! no collection-typed top-level field).

use crate::artifacts::en1994::En1994Snapshot;

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
    "annex",
    "m_ed_knm",
    "v_ed_kn",
    "m_pla",
    "m_pl_rd",
    "eta",
    "v_l_rd",
    "insulation_thickness_mm",
    "fire_rating",
    "deck_type",
    "delta_sigma_mpa",
    "fatigue_detail",
    "d_mm",
    "h_sc_mm",
    "f_ck_mpa",
    "f_u_mpa",
    "e_cm_mpa",
    "v_ed_per_stud_kn",
    "span_m",
    "f_y_mpa",
    "n_cycles_stud",
    "delta_tau_stud_mpa",
];

/// 🧾️ `En1994` document outline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1994Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl En1994Outline {
    pub fn compute(_snapshot: &En1994Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = 0;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1994Outline {
    fn default() -> Self {
        Self::compute(&En1994Snapshot::default())
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn outline_field_count_matches_section_outline_length() {
        let outline = En1994Outline::compute(&En1994Snapshot::default());
        assert_eq!(outline.field_count as usize, outline.section_outline.len());
    }

    #[semio_framework_async_macros::async_test]
    fn outline_is_deterministic() {
        let snapshot = En1994Snapshot::default();
        assert_eq!(En1994Outline::compute(&snapshot), En1994Outline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
