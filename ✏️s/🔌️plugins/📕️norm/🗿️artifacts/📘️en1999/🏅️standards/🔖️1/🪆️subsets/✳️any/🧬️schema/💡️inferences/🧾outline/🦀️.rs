//! 🧾 `outline` — one named inference: this document's own field/section structure. A norm
//! compliance record IS the document it describes, so its "outline" is its top-level field list
//! (`sectionOutline`/`fieldCount`, fixed by the snapshot's own schema shape) plus a real
//! `entryCount` over whatever repeated sub-entries it actually carries (0 when the snapshot has
//! no collection-typed top-level field).

use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
    "n_ed_kn",
    "m_ed_knm",
    "a_mm2",
    "w_el_mm3",
    "alloy",
    "chi",
    "i_t_mm4",
    "l_cr_mm",
    "theta_c",
    "delta_sigma_ed",
    "delta_sigma_c",
    "fatigue_m",
    "n_cycles",
    "v_weld_ed_kn",
    "weld_throat_mm",
    "weld_length_mm",
    "beta_w",
    "sheet_b_mm",
    "sheet_t_mm",
    "sheet_k_sigma",
    "sheet_w_el_mm3",
    "sheet_m_ed_knm",
    "shell_t_mm",
    "shell_r_mm",
    "sigma_ed_shell_mpa",
    "annex",
];

/// 🧾️ `En1999` document outline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1999Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl En1999Outline {
    pub fn compute(_snapshot: &En1999Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = 0;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1999Outline {
    fn default() -> Self {
        Self::compute(&En1999Snapshot::default())
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn outline_field_count_matches_section_outline_length() {
        let outline = En1999Outline::compute(&En1999Snapshot::default());
        assert_eq!(outline.field_count as usize, outline.section_outline.len());
    }

    #[semio_framework_async_macros::async_test]
    fn outline_is_deterministic() {
        let snapshot = En1999Snapshot::default();
        assert_eq!(En1999Outline::compute(&snapshot), En1999Outline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
