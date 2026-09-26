//! 🧾 Document outline for EN 1995 timber structure.

use crate::En1995Snapshot;

const SECTION_FIELDS: &[&str] = &["annex", "members", "connections"];

/// 🧾️ `En1995` document outline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1995Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl En1995Outline {
    pub fn compute(snapshot: &En1995Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = (snapshot.members.len() + snapshot.connections.len()) as u32;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1995Outline {
    fn default() -> Self { Self::compute(&En1995Snapshot::default()) }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
