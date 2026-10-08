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
    let removed: Vec<String> = base.members.iter().filter(|row| !payload.members.contains(row)).map(|row| row.id.clone()).collect();
    let added: Vec<_> = payload.members.iter().filter(|row| !base.members.contains(row)).cloned().collect();
    let mut natural: Vec<String> = base.members.iter().filter(|row| !removed.contains(&row.id)).map(|row| row.id.clone()).collect();
    natural.extend(added.iter().map(|row| row.id.clone()));
    let wanted: Vec<String> = payload.members.iter().map(|row| row.id.clone()).collect();
    let order = (natural != wanted).then_some(wanted);
    protocol::MutationOutcome::new(En1999Diff { members: Some(En1999MembersRows { added, removed, order, ..Default::default() }), ..Default::default() })
}
