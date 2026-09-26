//! 🧾 `outline` — document field list plus zone/element entry counts.

use crate::Din18599Snapshot;

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
    "building_category",
    "attachment",
    "use_class",
    "method",
    "net_floor_area_m2",
    "heated_volume_m3",
    "geg_qp_factor",
    "delta_u_wb_w_m2k",
    "automation_class",
    "zones",
    "elements",
    "heating",
    "dhw",
    "ventilation",
    "cooling",
    "lighting",
    "renewables",
    "climate",
];

/// 🧾️ `Din18599` document outline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Din18599Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl Din18599Outline {
    pub fn compute(snapshot: &Din18599Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = (snapshot.zones.len() + snapshot.elements.len()) as u32;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for Din18599Outline {
    fn default() -> Self {
        Self { section_outline: Vec::new(), field_count: 0, entry_count: 0 }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
