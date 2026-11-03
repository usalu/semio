//! 🔺️ `insert-fire-scenario` diff.

use crate::insert_fire_scenario::InsertFireScenario;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FireScenariosRows};

pub fn diff(payload: &InsertFireScenario, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.fire_scenarios.iter().any(|existing| existing.id == payload.fire_scenario.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Fire scenario id {} already exists.", payload.fire_scenario.id), [payload.fire_scenario.id.clone()]);
    }
    if (payload.index as usize) > base.fire_scenarios.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.fire_scenarios.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { fire_scenarios: Some(En1999FireScenariosRows::insertion(payload.index as usize, payload.fire_scenario.clone())), ..Default::default() })
}
