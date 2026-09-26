//! 🧾 Document outline for the DIN 4108 envelope subject.

use crate::Din4108Snapshot;

const SECTION_FIELDS: &[&str] = &[
    "climateZone",
    "usage",
    "tIntC",
    "rhInt",
    "hasMechanicalVentilation",
    "airtightnessN50",
    "bb2DetailsConform",
    "zones",
    "elements",
    "thermalBridges",
];

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct Din4108Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl Din4108Outline {
    pub fn compute(snapshot: &Din4108Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = (snapshot.zones.len() + snapshot.elements.len() + snapshot.thermal_bridges.len()) as u32;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for Din4108Outline {
    fn default() -> Self {
        Self::compute(&Din4108Snapshot::default())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
