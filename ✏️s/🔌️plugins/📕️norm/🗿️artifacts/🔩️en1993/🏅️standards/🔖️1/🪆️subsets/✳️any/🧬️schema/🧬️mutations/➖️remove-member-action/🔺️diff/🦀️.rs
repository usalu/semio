//! ➖️ `remove-member-action` diff — removes the row at the index, guarded by the row's own id.

use super::RemoveMemberAction;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993MemberActionEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveMemberAction, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.member_actions.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("member-action index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { member_actions: vec![En1993MemberActionEdit::remove(payload.index, row.id.clone())], ..Default::default() })
}
