//! 🔺️ `change-fire-duration-s` diff: patches `duration_s` of the addressed fire scenario row.

use crate::change_fire_duration_s::ChangeFireDurationS;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FireScenariosPatch, En1999FireScenariosRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeFireDurationS, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_duration_s.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.fire_scenarios.iter().find(|row| row.id == payload.fire_scenario_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fire-scenario", Vec::<String>::new());
    };
    if r0.duration_s == payload.new_duration_s {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fire scenario already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { fire_scenarios: Some(En1999FireScenariosRows::modification(&payload.fire_scenario_id, En1999FireScenariosPatch { duration_s: Some(payload.new_duration_s), ..Default::default() })), ..Default::default() })
}
