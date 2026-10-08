//! 🏗️ `change-members` diff — replaces the whole collection: every base row leaves, every new row enters after the new row before it.

use super::ChangeMembers;
use crate::diff::{En1990Diff, En1990MemberAddition, En1990MemberDelta};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeMembers, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.members == mutation.new_members {
        return MutationOutcome::empty().warning("mutation.no-op", "members already has this value.");
    }
    let removed = base.members.iter().map(|row| row.id.clone()).collect();
    let added = mutation.new_members.iter().enumerate().map(|(index, row)| En1990MemberAddition { after: index.checked_sub(1).map(|previous| mutation.new_members[previous].id.clone()), row: row.clone() }).collect();
    MutationOutcome::new(En1990Diff { members: En1990MemberDelta { removed, added, ..En1990MemberDelta::default() }, ..En1990Diff::default() })
}
