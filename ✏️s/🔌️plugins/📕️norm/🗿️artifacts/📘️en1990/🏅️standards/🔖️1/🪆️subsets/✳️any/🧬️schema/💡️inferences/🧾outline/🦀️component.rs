//! 🧾 `outline` — one named inference: this document's own field/section structure plus a clause
//! summary from headless EN 1990 norm computation (`engine::evaluate`).

use crate::artifacts::en1990::En1990Snapshot;
use crate::document::CheckStatus;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
const SECTION_FIELDS: &[&str] = &[
        "g_k",
        "q_k",
        "resistance_kn",
        "consequence_class",
        "annex",
        "seismic_a_ed_kn",
];

/// 🧾️ `En1990` document outline and governing-clause summary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
        let entry_count = snapshot.q_k.len() as u32;
        let report = crate::artifacts::en1990::engine::evaluate(snapshot);
        let check_count = report.checks.len() as u32;
        let pass_count = report.checks.iter().filter(|check| check.status == CheckStatus::Pass).count() as u32;
        let all_pass = report.all_pass();
        let governing = report
            .checks
            .iter()
            .max_by(|left, right| left.utilization.partial_cmp(&right.utilization).unwrap_or(std::cmp::Ordering::Equal))
            .map(|check| (check.clause.to_string(), check.utilization))
            .unwrap_or_else(|| ("EN 1990 §6.4 6.10".into(), 0.0));
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
//#region 🧪️Tests
mod tests {
    use super::*;

    #[test]
    fn outline_field_count_matches_section_outline_length() {
        let outline = En1990Outline::compute(&En1990Snapshot::default());
        assert_eq!(outline.field_count as usize, outline.section_outline.len());
    }

    #[test]
    fn outline_is_deterministic() {
        let snapshot = En1990Snapshot::default();
        assert_eq!(En1990Outline::compute(&snapshot), En1990Outline::compute(&snapshot));
    }

    #[test]
    fn outline_counts_checks_from_norm_computation() {
        let outline = En1990Outline::compute(&En1990Snapshot::default());
        assert!(outline.check_count > 0);
        assert!(outline.pass_count <= outline.check_count, "pass count cannot exceed total checks");
        assert!(!outline.governing_clause.is_empty());
        assert!(outline.governing_utilization > 0.0);
    }
}
//#endregion 🧪️Tests
