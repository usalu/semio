//! ➖️ `remove-member-action` diff — removes the row at the index.

use super::RemoveMemberAction;
use crate::diff::{En1993Diff, En1993MemberActionDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveMemberAction, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.member_actions.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("member-action index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { member_actions: En1993MemberActionDelta::removal(&base.member_actions, payload.index), ..Default::default() })
}
