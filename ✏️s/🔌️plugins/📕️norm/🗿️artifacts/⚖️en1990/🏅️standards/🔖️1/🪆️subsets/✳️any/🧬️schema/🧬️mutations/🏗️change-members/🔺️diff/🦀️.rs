//! 🏗️ `change-members` diff — replaces the whole collection: every base row leaves from its base index, every new row enters at its index.

use super::ChangeMembers;
use crate::diff::{En1990Diff, En1990MemberDelta, En1990MemberInsertion, En1990MemberRemoval};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeMembers, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.members == mutation.new_members {
        return MutationOutcome::empty().warning("mutation.no-op", "members already has this value.");
    }
    let removed = base.members.iter().enumerate().map(|(index, row)| En1990MemberRemoval { id: row.id.clone(), index }).collect();
    let inserted = mutation.new_members.iter().enumerate().map(|(index, row)| En1990MemberInsertion { index, row: row.clone() }).collect();
    MutationOutcome::new(En1990Diff { members: En1990MemberDelta { removed, inserted, ..En1990MemberDelta::default() }, ..En1990Diff::default() })
}
