//! 🧾 `outline` — composite structure subject field outline.

use crate::En1994Snapshot;

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
    "annex",
    "structure_kind",
    "steel_f_y_pa",
    "beams",
    "columns",
    "slabs",
    "fire_rating",
    "insulation_thickness_m",
    "fatigue_detail",
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
    pub fn compute(snapshot: &En1994Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = (snapshot.beams.len() + snapshot.columns.len() + snapshot.slabs.len()) as u32;
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
