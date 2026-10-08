//! 🔺️ `change-fire-scenarios` diff.

use crate::mutations::change_fire_scenarios::ChangeFireScenarios;
use crate::En1999Snapshot;
use crate::diff::{En1999Diff, En1999FireScenariosRows};

pub fn diff(payload: &ChangeFireScenarios, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if &base.fire_scenarios == &payload.fire_scenarios {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "List unchanged.");
    }
    if let Some((_, row)) = payload.fire_scenarios.iter().enumerate().find(|(at, row)| payload.fire_scenarios[..*at].iter().any(|earlier| earlier.id == row.id)) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Fire scenario id {} appears twice.", row.id), [row.id.clone()]);
    }
    protocol::MutationOutcome::new(En1999Diff { fire_scenarios: Some(En1999FireScenariosRows::setting(&base.fire_scenarios, &payload.fire_scenarios)), ..Default::default() })
}
