//! ➖️ `remove-tower-leg` diff — removes the row at the index, guarded by the row's own id.

use super::RemoveTowerLeg;
use crate::diff::{En1993Diff, En1993TowerLegDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveTowerLeg, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.tower_legs.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("tower-leg index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { tower_legs: En1993TowerLegDelta::removal(&row.id), ..Default::default() })
}
