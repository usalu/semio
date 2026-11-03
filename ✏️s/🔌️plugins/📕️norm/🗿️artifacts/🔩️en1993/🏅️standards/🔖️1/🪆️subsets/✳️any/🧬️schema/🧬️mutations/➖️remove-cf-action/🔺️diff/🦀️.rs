//! ➖️ `remove-cf-action` diff: removes the element at the index from the addressed row's list.

use crate::remove_cf_action::RemoveCfAction;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993ColdFormedMemberDelta, En1993ColdFormedMemberPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveCfAction, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.cold_formed_members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "cf-action", Vec::<String>::new());
    };
    if payload.index >= row.actions.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("cf-action index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut value = row.actions.clone();
    value.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { cold_formed_members: En1993ColdFormedMemberDelta::modification(payload.id.clone(), En1993ColdFormedMemberPatch { actions: Some(value), ..Default::default() }), ..Default::default() })
}
