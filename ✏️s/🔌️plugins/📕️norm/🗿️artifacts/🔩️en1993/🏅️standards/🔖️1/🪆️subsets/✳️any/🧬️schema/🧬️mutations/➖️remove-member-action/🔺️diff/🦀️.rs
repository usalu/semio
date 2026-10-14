//! ➖️ `remove-member-action` diff — removes the row at the index.

use crate::remove_member_action::RemoveMemberAction;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MemberActionDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveMemberAction, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.member_actions.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("member-action index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { member_actions: En1993MemberActionDelta::removal(&base.member_actions, payload.index), ..Default::default() })
}
