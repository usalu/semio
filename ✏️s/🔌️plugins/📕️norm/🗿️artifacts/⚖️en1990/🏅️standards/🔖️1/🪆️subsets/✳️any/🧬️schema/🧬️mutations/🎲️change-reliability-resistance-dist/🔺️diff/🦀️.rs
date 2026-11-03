//! 🎲 `change-reliability-resistance-dist` diff: patches `resistance_distribution` of the addressed reliability analysis row.

use crate::change_reliability_resistance_dist::ChangeReliabilityResistanceDist;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990ReliabilityAnalysisDelta, En1990ReliabilityAnalysisPatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeReliabilityResistanceDist, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    let Some(r0) = base.reliability_analyses.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Reliability analysis {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.resistance_distribution == payload.new_resistance_distribution {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The reliability analysis already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { reliability_analyses: En1990ReliabilityAnalysisDelta::modification(&payload.id, En1990ReliabilityAnalysisPatch { resistance_distribution: Some(payload.new_resistance_distribution), ..Default::default() }), ..Default::default() })
}
