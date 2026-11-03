//! 🎲 `remove-reliability-analysis` diff: removes the addressed reliability analysis row.

use crate::remove_reliability_analysis::RemoveReliabilityAnalysis;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990ReliabilityAnalysisDelta};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &RemoveReliabilityAnalysis, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let index = payload.index;
    if index >= base.reliability_analyses.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "reliability_analyses index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1990Diff { reliability_analyses: En1990ReliabilityAnalysisDelta::removal(&base.reliability_analyses, index), ..Default::default() })
}
