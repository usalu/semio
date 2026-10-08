//! ➖️ `remove-tower-leg` diff — removes the row at the index.

use super::RemoveTowerLeg;
use crate::diff::{En1993Diff, En1993TowerLegDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveTowerLeg, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.tower_legs.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("tower-leg index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { tower_legs: En1993TowerLegDelta::removal(&base.tower_legs, payload.index), ..Default::default() })
}
