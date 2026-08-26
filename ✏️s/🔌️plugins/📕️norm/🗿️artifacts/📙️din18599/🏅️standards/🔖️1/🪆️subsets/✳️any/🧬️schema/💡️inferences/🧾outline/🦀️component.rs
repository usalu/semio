//! 🧾 `outline` — one named inference: this document's own field/section structure. A norm
//! compliance record IS the document it describes, so its "outline" is its top-level field list
//! (`sectionOutline`/`fieldCount`, fixed by the snapshot's own schema shape) plus a real
//! `entryCount` over whatever repeated sub-entries it actually carries (0 when the snapshot has
//! no collection-typed top-level field).

use crate::artifacts::din18599::Din18599Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &["use_class", "heated_area_m2", "occupants", "h_t", "h_v", "climate", "internal_gains_w_m2", "solar_gains_kwh", "system_losses_kwh", "renewable_kwh", "annual_limit_kwh", "energy_carrier", "reference_q_p_kwh"];

/// 🧾️ `Din18599` document outline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Din18599Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl Din18599Outline {
    pub fn compute(_snapshot: &Din18599Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = 0;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for Din18599Outline {
    fn default() -> Self {
        Self::compute(&Din18599Snapshot::default())
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn outline_field_count_matches_section_outline_length() {
        let outline = Din18599Outline::compute(&Din18599Snapshot::default());
        assert_eq!(outline.field_count as usize, outline.section_outline.len());
    }

    #[semio_framework_async_macros::async_test]
    fn outline_is_deterministic() {
        let snapshot = Din18599Snapshot::default();
        assert_eq!(Din18599Outline::compute(&snapshot), Din18599Outline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
