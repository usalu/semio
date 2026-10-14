//! 🔺️ `remove-member` diff.

use crate::remove_member::RemoveMember;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999MembersRows};

pub fn diff(payload: &RemoveMember, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(index) = base.members.iter().position(|m| m.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown member id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { members: Some(En1999MembersRows::removal(&base.members, index)), ..Default::default() })
}
