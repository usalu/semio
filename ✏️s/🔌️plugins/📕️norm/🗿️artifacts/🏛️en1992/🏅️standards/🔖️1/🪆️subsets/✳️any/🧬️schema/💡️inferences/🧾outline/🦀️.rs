//! 🧾 EN 1992 inference outline — hierarchical subject fields read by evaluate().

use crate::En1992Snapshot;

/// 📋 Editable / evaluated field paths (camelCase).
pub const OUTLINE_FIELDS: &[&str] = &[
    "annex",
    "title",
    "designWorkingLifeYears",
    "deltaCDev",
    "cementType",
    "concreteGrades",
    "reinforcementGrades",
    "prestressSteels",
    "members",
    "anchors",
];

const SECTION_FIELDS: &[&str] = OUTLINE_FIELDS;

/// 🧾️ `En1992` document outline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1992Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl Default for En1992Outline {
    fn default() -> Self {
        Self::compute(&En1992Snapshot::default())
    }
}

impl En1992Outline {
    /// 🧮 Compute outline from snapshot.
    pub fn compute(snapshot: &En1992Snapshot) -> Self {
        Self {
            section_outline: SECTION_FIELDS.iter().map(|s| (*s).to_string()).collect(),
            field_count: SECTION_FIELDS.len() as u32,
            entry_count: (snapshot.members.len() + snapshot.anchors.len()) as u32,
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
