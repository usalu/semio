//! 🧾 `outline` — composite structure subject field outline.

use semio_s_artifact_norm_en1994_model::En1994Snapshot;

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
    "annex",
    "reliabilityClass",
    "structure_kind",
    "steel_f_y_pa",
    "beams",
    "columns",
    "slabs",
    "fire_rating",
    "insulation_thickness_m",
    "fatigue_detail",
    "placements",
    "studs",
    "cases",
];

/// 🧾️ `En1994` document outline.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
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
        let entry_count = (snapshot.beams.len() + snapshot.columns.len() + snapshot.slabs.len() + snapshot.placements.len() + snapshot.studs.len() + snapshot.cases.len()) as u32;
        Self { section_outline, field_count, entry_count }
    }
}

impl Default for En1994Outline {
    fn default() -> Self {
        Self::compute(&En1994Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De))
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
