//! 🪚 `remove-member` diff — removes the row at the index; an index past the collection's end is a `mutation.target-missing`.

use super::RemoveMember;
use crate::diff::En1990RowEdit as _;
use crate::diff::{En1990Diff, En1990MemberEdit};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(payload: &RemoveMember, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if payload.index >= base.members.len() {
        return MutationOutcome::error("mutation.target-missing", "members index out of range", [payload.index.to_string()]);
    }
    MutationOutcome::new(En1990Diff { members: vec![En1990MemberEdit::remove(payload.index, base.members[payload.index].id.clone())], ..En1990Diff::default() })
}
