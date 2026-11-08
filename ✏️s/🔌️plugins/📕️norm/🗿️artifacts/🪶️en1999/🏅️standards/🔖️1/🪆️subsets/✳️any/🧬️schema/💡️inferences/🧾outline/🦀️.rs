//! 🧾 Outline inference for EN 1999 aluminium-structure subject.

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

const SECTION_FIELDS: &[&str] = &[
    "annex",
    "reliabilityClass",
    "materials",
    "sections",
    "members",
    "connections",
    "fireScenarios",
    "fatigueDetails",
    "sheetProfiles",
    "fireInsulations",
];

/// 🧾️ `En1999` document outline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1999Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
}

impl En1999Outline {
    pub fn compute(snapshot: &En1999Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = (
            snapshot.materials.len()
            + snapshot.sections.len()
            + snapshot.members.len()
            + snapshot.connections.len()
            + snapshot.fire_scenarios.len()
            + snapshot.fatigue_details.len()
            + snapshot.sheet_profiles.len()
            + snapshot.fire_insulations.len()
        ) as u32;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1999Outline {
    fn default() -> Self {
        Self::compute(&En1999Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De))
    }
}
