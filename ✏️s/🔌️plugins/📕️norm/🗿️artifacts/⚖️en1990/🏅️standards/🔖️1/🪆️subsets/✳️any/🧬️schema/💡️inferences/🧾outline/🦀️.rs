//! 🧾 `outline` — document field structure plus clause summary from `evaluate`.

use semio_s_plugin_norm_compliance::CheckStatus;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
    "annex",
    "projectId",
    "structureKind",
    "altitudeM",
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
    "bridgeSls",
    "effects",
    "fatigueSpectra",
    "reliabilityAnalyses",
    "testSeries",
    "placements",
    "cases",
    "factorOverrides",
    "trafficGroups",
];

/// 📏️ En1990 document outline and governing-clause summary.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(any(test, feature = "test-support"), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(any(test, feature = "test-support"), serde(rename_all = "camelCase"))]
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
        let entry_count = (snapshot.permanents.len() + snapshot.variables.len() + snapshot.accidentals.len() + snapshot.seismics.len() + snapshot.members.len() + snapshot.bridge_sls.len() + snapshot.effects.len() + snapshot.fatigue_spectra.len() + snapshot.reliability_analyses.len() + snapshot.test_series.len() + snapshot.placements.len() + snapshot.cases.len() + snapshot.factor_overrides.len() + snapshot.traffic_groups.len()) as u32;
        let report = crate::evaluate(snapshot);
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
        Self::compute(&En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De))
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️tests
