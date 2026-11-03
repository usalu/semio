//! 🎲 `change-reliability-resistance-sd` diff: patches `resistance_sd` of the addressed reliability analysis row.

use crate::change_reliability_resistance_sd::ChangeReliabilityResistanceSd;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990ReliabilityAnalysisDelta, En1990ReliabilityAnalysisPatch};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeReliabilityResistanceSd, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_resistance_sd.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.reliability_analyses.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Reliability analysis {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.resistance_sd == payload.new_resistance_sd {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The reliability analysis already has this value.");
    }
    protocol::MutationOutcome::new(En1990Diff { reliability_analyses: En1990ReliabilityAnalysisDelta::modification(&payload.id, En1990ReliabilityAnalysisPatch { resistance_sd: Some(payload.new_resistance_sd), ..Default::default() }), ..Default::default() })
}
