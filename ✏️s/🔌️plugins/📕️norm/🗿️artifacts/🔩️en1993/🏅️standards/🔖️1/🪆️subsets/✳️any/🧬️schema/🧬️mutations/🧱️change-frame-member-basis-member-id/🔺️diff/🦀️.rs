//! 🔺️ `change-frame-member-basis-member-id` diff: patches `member_id` of the addressed frame_member_basis block row.

use crate::change_frame_member_basis_member_id::ChangeFrameMemberBasisMemberId;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FrameMemberBasisDelta, En1993FrameMemberBasisPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFrameMemberBasisMemberId, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.frame_member_bases.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "frame-member-basis", Vec::<String>::new());
    };
    if row.member_id == payload.new_member_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The frame_member_basis block already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { frame_member_bases: En1993FrameMemberBasisDelta::modification(payload.id.clone(), En1993FrameMemberBasisPatch { member_id: Some(payload.new_member_id.clone()), ..Default::default() }), ..Default::default() })
}
