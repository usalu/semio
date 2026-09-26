//! 🧾 Din16798 outline inference over the hierarchical subject.

use crate::Din16798Snapshot;

const SECTION_FIELDS: &[&str] = &[
    "annex", "theta_rm_c", "outdoor_co2_ppm", "zones", "vent_systems",
    "envelope_n50_h_inv", "envelope_volume_m3", "cellar_area_m2", "cellar_ventilation_m3_h", "night_setback_k",
];

/// 🧾️ Din16798 document outline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Din16798Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl Din16798Outline {
    pub fn compute(snapshot: &Din16798Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| (*s).to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = (snapshot.zones.len() + snapshot.vent_systems.len()) as u32;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for Din16798Outline {
    fn default() -> Self {
        Self::compute(&Din16798Snapshot::default())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
