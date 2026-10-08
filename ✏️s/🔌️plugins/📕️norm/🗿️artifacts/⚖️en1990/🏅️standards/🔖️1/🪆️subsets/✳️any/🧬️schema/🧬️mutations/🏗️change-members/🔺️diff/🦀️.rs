//! 🏗️ `change-members` diff — replaces the whole collection: every base row is removed back to front, then every new row is inserted in order.

use super::ChangeMembers;
use crate::diff::En1990RowEdit as _;
use crate::diff::{En1990Diff, En1990MemberEdit};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeMembers, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.members == mutation.new_members {
        return MutationOutcome::empty().warning("mutation.no-op", "members already has this value.");
    }
    let removed = (0..base.members.len()).rev().map(|index| En1990MemberEdit::remove(index, base.members[index].id.clone()));
    let inserted = mutation.new_members.iter().cloned().enumerate().map(|(index, row)| En1990MemberEdit::insert(index, row));
    MutationOutcome::new(En1990Diff { members: removed.chain(inserted).collect(), ..En1990Diff::default() })
}
