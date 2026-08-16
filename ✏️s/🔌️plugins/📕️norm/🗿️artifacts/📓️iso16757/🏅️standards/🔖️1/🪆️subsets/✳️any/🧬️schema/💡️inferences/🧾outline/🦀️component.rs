//! 🧾 `outline` — one named inference: this document's own field/section structure. A norm
//! compliance record IS the document it describes, so its "outline" is its top-level field list
//! (`sectionOutline`/`fieldCount`, fixed by the snapshot's own schema shape) plus a real
//! `entryCount` over whatever repeated sub-entries it actually carries (0 when the snapshot has
//! no collection-typed top-level field).

use crate::artifacts::iso16757::Iso16757Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &["catalogue", "dictionary", "geometry", "selection", "part_number_rule", "part_number_inputs", "script_limits", "exchange_process"];

/// 🧾️ `Iso16757` document outline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Iso16757Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl Iso16757Outline {
    pub fn compute(snapshot: &Iso16757Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = snapshot.part_number_inputs.len() as u32;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for Iso16757Outline {
    fn default() -> Self {
        Self::compute(&Iso16757Snapshot::default())
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[test]
    fn outline_field_count_matches_section_outline_length() {
        let outline = Iso16757Outline::compute(&Iso16757Snapshot::default());
        assert_eq!(outline.field_count as usize, outline.section_outline.len());
    }

    #[test]
    fn outline_is_deterministic() {
        let snapshot = Iso16757Snapshot::default();
        assert_eq!(Iso16757Outline::compute(&snapshot), Iso16757Outline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
