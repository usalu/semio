//! ➕️ `insert-member-action` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_member_action::InsertMemberAction;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MemberActionDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertMemberAction, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.member_actions.iter().any(|existing| existing.id == payload.member_action.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Member action id {} already exists.", payload.member_action.id), [payload.member_action.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.member_actions.len());
    protocol::MutationOutcome::new(En1993Diff { member_actions: En1993MemberActionDelta::insertion(index, payload.member_action.clone()), ..Default::default() })
}
