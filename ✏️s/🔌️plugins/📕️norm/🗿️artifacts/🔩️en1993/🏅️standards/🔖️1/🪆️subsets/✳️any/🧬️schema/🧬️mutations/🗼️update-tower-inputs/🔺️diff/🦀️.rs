//! 🗼️ `update-tower-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateTowerInputs;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993TowerLegEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateTowerInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let edit = match base.tower_legs.iter().position(|row| row.id == payload.tower_leg.id) {
        Some(index) if base.tower_legs[index] == payload.tower_leg => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => En1993TowerLegEdit::replace(index, payload.tower_leg.id.clone(), payload.tower_leg.clone()),
        None => En1993TowerLegEdit::insert(base.tower_legs.len(), payload.tower_leg.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { tower_legs: vec![edit], ..Default::default() })
}
