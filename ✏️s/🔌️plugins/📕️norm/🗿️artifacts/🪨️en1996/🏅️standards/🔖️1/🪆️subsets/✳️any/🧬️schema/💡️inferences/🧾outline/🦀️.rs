//! 🧾 Document outline for EN 1996 masonry building.

use crate::En1996Snapshot;

const SECTION_FIELDS: &[&str] = &["annex", "masonryClass", "designSituation", "storeys", "walls"];

/// 🧾️ `En1996` document outline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1996Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl En1996Outline {
    pub fn compute(snapshot: &En1996Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = snapshot.walls.len() as u32;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1996Outline {
    fn default() -> Self { Self::compute(&En1996Snapshot::default()) }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
