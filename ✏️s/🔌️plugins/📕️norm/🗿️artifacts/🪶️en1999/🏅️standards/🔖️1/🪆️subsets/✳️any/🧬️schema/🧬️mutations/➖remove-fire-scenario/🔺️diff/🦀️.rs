//! 🔺️ `remove-fire-scenario` diff.

use crate::remove_fire_scenario::RemoveFireScenario;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FireScenariosRows};

pub fn diff(payload: &RemoveFireScenario, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(index) = base.fire_scenarios.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown fire scenario id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { fire_scenarios: Some(En1999FireScenariosRows::removal(&base.fire_scenarios, index)), ..Default::default() })
}
