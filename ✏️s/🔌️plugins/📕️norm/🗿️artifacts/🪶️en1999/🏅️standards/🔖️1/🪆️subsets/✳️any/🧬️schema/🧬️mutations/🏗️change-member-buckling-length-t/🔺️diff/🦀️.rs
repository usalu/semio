//! 🔺️ `change-member-buckling-length-t` diff: patches `buckling_length_t` of the addressed member row.

use crate::change_member_buckling_length_t::ChangeMemberBucklingLengthT;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999MembersPatch, En1999MembersRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeMemberBucklingLengthT, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_buckling_length_t.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "member", Vec::<String>::new());
    };
    if r0.buckling_length_t == payload.new_buckling_length_t {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { members: Some(En1999MembersRows::modification(&payload.member_id, En1999MembersPatch { buckling_length_t: Some(payload.new_buckling_length_t), ..Default::default() })), ..Default::default() })
}
