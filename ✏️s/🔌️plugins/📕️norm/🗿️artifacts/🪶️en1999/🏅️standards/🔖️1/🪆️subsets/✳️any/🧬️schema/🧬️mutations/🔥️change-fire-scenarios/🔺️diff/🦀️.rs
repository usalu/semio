//! 🔺️ `change-fire-scenarios` diff.

use crate::diff::En1999Diff;
use crate::mutations::change_fire_scenarios::ChangeFireScenarios;
use crate::En1999Snapshot;

pub fn diff(payload: &ChangeFireScenarios, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if &base.fire_scenarios == &payload.fire_scenarios {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "List unchanged.");
    }
    protocol::MutationOutcome::new(En1999Diff { fire_scenarios: Some(payload.fire_scenarios.clone()), ..Default::default() })
}
