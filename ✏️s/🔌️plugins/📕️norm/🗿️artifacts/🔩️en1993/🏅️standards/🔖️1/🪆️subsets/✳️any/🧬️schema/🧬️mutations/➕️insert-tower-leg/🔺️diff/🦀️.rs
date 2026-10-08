//! ➕️ `insert-tower-leg` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertTowerLeg;
use crate::diff::{En1993Diff, En1993TowerLegDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertTowerLeg, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.tower_legs.iter().any(|existing| existing.id == payload.tower_leg.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Tower leg id {} already exists.", payload.tower_leg.id), [payload.tower_leg.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.tower_legs.len());
    protocol::MutationOutcome::new(En1993Diff { tower_legs: En1993TowerLegDelta::insertion(index, payload.tower_leg.clone()), ..Default::default() })
}
