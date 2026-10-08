//! 🔺️ `change-members` diff.

use crate::mutations::change_members::ChangeMembers;
use crate::En1999Snapshot;
use crate::diff::{En1999Diff, En1999MembersRows};

pub fn diff(payload: &ChangeMembers, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if &base.members == &payload.members {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "List unchanged.");
    }
    if let Some((_, row)) = payload.members.iter().enumerate().find(|(at, row)| payload.members[..*at].iter().any(|earlier| earlier.id == row.id)) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Member id {} appears twice.", row.id), [row.id.clone()]);
    }
    protocol::MutationOutcome::new(En1999Diff { members: Some(En1999MembersRows::setting(&base.members, &payload.members)), ..Default::default() })
}
