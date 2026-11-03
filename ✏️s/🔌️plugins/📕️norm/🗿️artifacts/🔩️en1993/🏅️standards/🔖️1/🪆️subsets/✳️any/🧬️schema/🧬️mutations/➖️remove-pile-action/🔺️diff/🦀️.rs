//! ➖️ `remove-pile-action` diff: removes the element at the index from the addressed row's list.

use crate::remove_pile_action::RemovePileAction;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993PileDelta, En1993PilePatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemovePileAction, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.piles.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "pile-action", Vec::<String>::new());
    };
    if payload.index >= row.actions.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("pile-action index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut value = row.actions.clone();
    value.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { piles: En1993PileDelta::modification(payload.id.clone(), En1993PilePatch { actions: Some(value), ..Default::default() }), ..Default::default() })
}
