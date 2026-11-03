//! ➖️ `remove-tower-action` diff: removes the element at the index from the addressed row's list.

use crate::remove_tower_action::RemoveTowerAction;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993TowerLegDelta, En1993TowerLegPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveTowerAction, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.tower_legs.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "tower-action", Vec::<String>::new());
    };
    if payload.index >= row.actions.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("tower-action index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut value = row.actions.clone();
    value.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { tower_legs: En1993TowerLegDelta::modification(payload.id.clone(), En1993TowerLegPatch { actions: Some(value), ..Default::default() }), ..Default::default() })
}
