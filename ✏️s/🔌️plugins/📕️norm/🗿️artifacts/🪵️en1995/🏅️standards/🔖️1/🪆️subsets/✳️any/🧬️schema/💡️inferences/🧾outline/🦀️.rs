//! 🧾 Document outline for EN 1995 timber structure.

use semio_s_artifact_norm_en1995_model::En1995Snapshot;

const SECTION_FIELDS: &[&str] = &["annex", "reliabilityClass", "members", "connections", "ulsEquation", "cases", "placements", "tapers", "cutouts", "fasteners", "panels", "floors", "diaphragms", "trusses", "bracings", "decks"];

/// 🧾️ `En1995` document outline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
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
        let entry_count = (snapshot.members.len() + snapshot.connections.len() + snapshot.cases.len() + snapshot.placements.len() + snapshot.tapers.len() + snapshot.cutouts.len() + snapshot.fasteners.len() + snapshot.panels.len() + snapshot.floors.len() + snapshot.diaphragms.len() + snapshot.trusses.len() + snapshot.bracings.len() + snapshot.decks.len()) as u32;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1995Outline {
    fn default() -> Self { Self::compute(&En1995Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De)) }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
