//! ➕️ `insert-tower-action` diff: inserts the element into the addressed row's list, clamped to its end.

use crate::insert_tower_action::InsertTowerAction;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993TowerLegDelta, En1993TowerLegPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertTowerAction, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.tower_legs.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "tower-action", Vec::<String>::new());
    };
    if row.actions.iter().any(|existing| existing.id == payload.element.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Tower leg action id {} already exists.", payload.element.id), [payload.element.id.clone()]);
    }
    let mut value = row.actions.clone();
    let at = payload.index.unwrap_or(usize::MAX).min(value.len());
    value.insert(at, payload.element.clone());
    protocol::MutationOutcome::new(En1993Diff { tower_legs: En1993TowerLegDelta::modification(payload.id.clone(), En1993TowerLegPatch { actions: Some(value), ..Default::default() }), ..Default::default() })
}
