//! ➕️ `insert-pile-action` diff: inserts the element into the addressed row's list, clamped to its end.

use crate::insert_pile_action::InsertPileAction;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993PileDelta, En1993PilePatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertPileAction, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.piles.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "pile-action", Vec::<String>::new());
    };
    if row.actions.iter().any(|existing| existing.id == payload.element.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Pile action id {} already exists.", payload.element.id), [payload.element.id.clone()]);
    }
    let mut value = row.actions.clone();
    let at = payload.index.unwrap_or(usize::MAX).min(value.len());
    value.insert(at, payload.element.clone());
    protocol::MutationOutcome::new(En1993Diff { piles: En1993PileDelta::modification(payload.id.clone(), En1993PilePatch { actions: Some(value), ..Default::default() }), ..Default::default() })
}
