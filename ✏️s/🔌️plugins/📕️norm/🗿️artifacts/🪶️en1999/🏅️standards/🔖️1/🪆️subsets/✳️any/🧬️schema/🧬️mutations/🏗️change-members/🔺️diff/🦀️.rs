//! 🔺️ `change-members` diff.

use crate::change_members::ChangeMembers;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999MembersRows};

pub fn diff(payload: &ChangeMembers, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if &base.members == &payload.members {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "List unchanged.");
    }
    if let Some((_, row)) = payload.members.iter().enumerate().find(|(at, row)| payload.members[..*at].iter().any(|earlier| earlier.id == row.id)) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Member id {} appears twice.", row.id), [row.id.clone()]);
    }
    protocol::MutationOutcome::new(En1999Diff { members: Some(En1999MembersRows::setting(&base.members, &payload.members)), ..Default::default() })
}
