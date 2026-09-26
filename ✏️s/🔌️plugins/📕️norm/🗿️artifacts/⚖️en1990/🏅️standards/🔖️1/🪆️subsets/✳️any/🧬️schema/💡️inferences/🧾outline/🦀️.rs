//! 🧾 `outline` — document field structure plus clause summary from `evaluate`.

use crate::document::CheckStatus;
use crate::En1990Snapshot;

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
    "annex",
    "projectId",
    "consequenceClass",
    "reliabilityClass",
    "designWorkingLifeCategory",
    "designWorkingLifeYears",
    "referencePeriodYears",
    "supervisionLevel",
    "inspectionLevel",
    "betaComputed",
    "permanents",
    "variables",
    "accidentals",
    "seismics",
    "members",
    "effects",
];

/// ️ En1990 document outline and governing-clause summary.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1990Outline {
    pub section_outline: Vec<String>,
    pub field_count: u32,
    pub entry_count: u32,
    pub check_count: u32,
    pub pass_count: u32,
    pub all_pass: bool,
    pub governing_clause: String,
    pub governing_utilization: f64,
}

impl En1990Outline {
    pub fn compute(snapshot: &En1990Snapshot) -> Self {
        let section_outline: Vec<String> = SECTION_FIELDS.iter().map(|s| s.to_string()).collect();
        let field_count = section_outline.len() as u32;
        let entry_count = (snapshot.permanents.len() + snapshot.variables.len() + snapshot.members.len()) as u32;
        let report = crate::standards::v1::subsets::any::schema::inferences::evaluate(snapshot);
        let check_count = report.checks.len() as u32;
        let pass_count = report.checks.iter().filter(|check| check.status == CheckStatus::Pass).count() as u32;
        let all_pass = report.complies();
        let governing = report
            .checks
            .iter()
            .filter(|c| c.status != CheckStatus::NotApplicable)
            .max_by(|left, right| left.utilization.partial_cmp(&right.utilization).unwrap_or(std::cmp::Ordering::Equal))
            .map_or_else(|| ("EN 1990 §6.4 6.10".into(), 0.0), |check| (check.clause.to_string(), check.utilization));
        Self {
            section_outline,
            field_count,
            entry_count,
            check_count,
            pass_count,
            all_pass,
            governing_clause: governing.0,
            governing_utilization: governing.1,
        }
    }
}

impl Default for En1990Outline {
    fn default() -> Self {
        Self::compute(&En1990Snapshot::default())
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️tests
