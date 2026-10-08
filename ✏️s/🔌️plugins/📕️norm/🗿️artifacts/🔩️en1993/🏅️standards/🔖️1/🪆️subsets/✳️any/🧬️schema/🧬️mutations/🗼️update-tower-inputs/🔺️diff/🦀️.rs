//! 🗼️ `update-tower-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateTowerInputs;
use crate::diff::{En1993Diff, En1993TowerLegDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateTowerInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let delta = match base.tower_legs.iter().position(|row| row.id == payload.tower_leg.id) {
        Some(index) if base.tower_legs[index] == payload.tower_leg => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => {
            let mut replacement = En1993TowerLegDelta::removal(&payload.tower_leg.id);
            replacement.absorb(En1993TowerLegDelta::insertion(&base.tower_legs, index, payload.tower_leg.clone()));
            replacement
        }
        None => En1993TowerLegDelta::insertion(&base.tower_legs, base.tower_legs.len(), payload.tower_leg.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { tower_legs: delta, ..Default::default() })
}
