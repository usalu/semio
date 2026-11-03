//! 🔺️ `change-member-ltb-length` diff: patches `ltb_length` of the addressed member row.

use crate::change_member_ltb_length::ChangeMemberLtbLength;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993MemberDelta, En1993MemberPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeMemberLtbLength, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_ltb_length.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.members.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "member", Vec::<String>::new());
    };
    if row.ltb_length == payload.new_ltb_length {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { members: En1993MemberDelta::modification(payload.id.clone(), En1993MemberPatch { ltb_length: Some(payload.new_ltb_length), ..Default::default() }), ..Default::default() })
}
