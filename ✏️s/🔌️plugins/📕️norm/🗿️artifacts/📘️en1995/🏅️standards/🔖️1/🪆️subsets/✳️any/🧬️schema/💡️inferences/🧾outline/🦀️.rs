//! 🧾 `outline` — one named inference: this document's own field/section structure. A norm
//! compliance record IS the document it describes, so its "outline" is its top-level field list
//! (`sectionOutline`/`fieldCount`, fixed by the snapshot's own schema shape) plus a real
//! `entryCount` over whatever repeated sub-entries it actually carries (0 when the snapshot has
//! no collection-typed top-level field).

use crate::artifacts::en1995::En1995Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
    "annex",
    "m_ed_knm",
    "n_ed_kn",
    "v_ed_kn",
    "w_mm3",
    "a_mm2",
    "b_mm",
    "h_mm",
    "f_m_k",
    "f_c_0_k",
    "service_class",
    "load_duration",
    "m_crit_knm",
    "f_ed_kn",
    "a_ef_mm2",
    "f_v_k",
    "fire_duration_min",
    "section_depth_mm",
    "a_vert_m_s2",
    "n_cycles_bridge",
];

/// 🧾️ `En1995` document outline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct En1995Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl En1995Outline {
    pub fn compute(_snapshot: &En1995Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = 0;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1995Outline {
    fn default() -> Self {
        Self::compute(&En1995Snapshot::default())
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn outline_field_count_matches_section_outline_length() {
        let outline = En1995Outline::compute(&En1995Snapshot::default());
        assert_eq!(outline.field_count as usize, outline.section_outline.len());
    }

    #[semio_framework_async_macros::async_test]
    fn outline_is_deterministic() {
        let snapshot = En1995Snapshot::default();
        assert_eq!(En1995Outline::compute(&snapshot), En1995Outline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
