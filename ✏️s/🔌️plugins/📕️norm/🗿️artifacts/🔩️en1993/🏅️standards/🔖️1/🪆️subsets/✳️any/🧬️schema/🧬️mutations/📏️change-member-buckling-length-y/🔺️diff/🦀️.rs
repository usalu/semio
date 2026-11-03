//! 🔺️ `change-member-buckling-length-y` diff: patches `buckling_length_y` of the addressed member row.

use crate::change_member_buckling_length_y::ChangeMemberBucklingLengthY;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MemberDelta, En1993MemberPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeMemberBucklingLengthY, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_buckling_length_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "member", Vec::<String>::new());
    };
    if row.buckling_length_y == payload.new_buckling_length_y {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { members: En1993MemberDelta::modification(payload.id.clone(), En1993MemberPatch { buckling_length_y: Some(payload.new_buckling_length_y), ..Default::default() }), ..Default::default() })
}
