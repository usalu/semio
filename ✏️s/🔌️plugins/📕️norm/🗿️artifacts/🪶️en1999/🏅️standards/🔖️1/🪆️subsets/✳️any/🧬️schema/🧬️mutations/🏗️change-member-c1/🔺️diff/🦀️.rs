//! 🔺️ `change-member-c1` diff: patches `c1` of the addressed member row.

use crate::change_member_c1::ChangeMemberC1;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999MembersPatch, En1999MembersRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeMemberC1, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_c1.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "member", Vec::<String>::new());
    };
    if r0.c1 == payload.new_c1 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { members: Some(En1999MembersRows::modification(&payload.member_id, En1999MembersPatch { c1: Some(payload.new_c1), ..Default::default() })), ..Default::default() })
}
