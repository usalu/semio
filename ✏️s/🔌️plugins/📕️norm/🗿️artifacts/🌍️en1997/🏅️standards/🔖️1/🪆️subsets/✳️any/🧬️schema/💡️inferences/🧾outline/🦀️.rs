//! 🧾 `outline` — geotechnical project field structure plus governing-situation summary.

use crate::document::CheckStatus;
use crate::En1997Snapshot;

const SECTION_FIELDS: &[&str] = &[
    "structure_id",
    "geotechnical_category",
    "design_situation",
    "design_approach",
    "annex",
    "groundwater_level",
    "investigation_depth",
    "layers",
    "footings",
    "piles",
    "retaining_walls",
    "slopes",
    "uplift_cases",
];

/// 🧾 EN 1997 document outline and governing design-situation summary.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1997Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
    pub check_count: u32,
    pub pass_count: u32,
    pub all_pass: bool,
    pub governing_clause: String,
    pub governing_utilization: f64,
    pub governing_situation: String,
    pub governing_approach: String,
}

impl En1997Outline {
    pub fn compute(snapshot: &En1997Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = (snapshot.layers.len()
            + snapshot.footings.len()
            + snapshot.piles.len()
            + snapshot.retaining_walls.len()
            + snapshot.slopes.len()
            + snapshot.uplift_cases.len()) as u32;
        let report = crate::standards::v1::subsets::any::schema::check_project(snapshot);
        let check_count = report.checks.len() as u32;
        let pass_count = report.checks.iter().filter(|c| c.status == CheckStatus::Pass).count() as u32;
        let all_pass = report.complies();
        let gov = report
            .checks
            .iter()
            .find(|c| c.id == "en1997.governing.situation")
            .or_else(|| {
                report
                    .checks
                    .iter()
                    .filter(|c| c.status != CheckStatus::NotApplicable)
                    .max_by(|a, b| a.utilization.partial_cmp(&b.utilization).unwrap_or(std::cmp::Ordering::Equal))
            });
        let (governing_clause, governing_utilization, governing_situation) = match gov {
            Some(c) if c.id == "en1997.governing.situation" => {
                let sit = if c.explanation.en.contains("BS-T") {
                    "BS-T".into()
                } else if c.explanation.en.contains("BS-A") {
                    "BS-A".into()
                } else {
                    "BS-P".into()
                };
                (c.explanation.en.clone(), c.utilization, sit)
            }
            Some(c) => (c.id.clone(), c.utilization, snapshot.design_situation.clone()),
            None => (String::new(), 0.0, snapshot.design_situation.clone()),
        };
        Self {
            section_outline,
            field_count,
            entry_count,
            check_count,
            pass_count,
            all_pass,
            governing_clause,
            governing_utilization,
            governing_situation,
            governing_approach: snapshot.design_approach.clone(),
        }
    }
}
