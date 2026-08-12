//! 🧾 `outline` — one named inference: this document's own field/section structure. A norm
//! compliance record IS the document it describes, so its "outline" is its top-level field list
//! (`sectionOutline`/`fieldCount`, fixed by the snapshot's own schema shape) plus a real
//! `entryCount` over whatever repeated sub-entries it actually carries (0 when the snapshot has
//! no collection-typed top-level field).

use crate::artifacts::en1990::En1990Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
        "g_k",
        "q_k",
        "resistance_kn",
        "consequence_class",
        "annex",
        "seismic_a_ed_kn",
];

/// 🧾️ `En1990` document outline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct En1990Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl En1990Outline {
    pub fn compute(snapshot: &En1990Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = snapshot.q_k.len() as u32;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1990Outline {
    fn default() -> Self {
        Self::compute(&En1990Snapshot::default())
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[test]
    fn outline_field_count_matches_section_outline_length() {
        let outline = En1990Outline::compute(&En1990Snapshot::default());
        assert_eq!(outline.field_count as usize, outline.section_outline.len());
    }

    #[test]
    fn outline_is_deterministic() {
        let snapshot = En1990Snapshot::default();
        assert_eq!(En1990Outline::compute(&snapshot), En1990Outline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
