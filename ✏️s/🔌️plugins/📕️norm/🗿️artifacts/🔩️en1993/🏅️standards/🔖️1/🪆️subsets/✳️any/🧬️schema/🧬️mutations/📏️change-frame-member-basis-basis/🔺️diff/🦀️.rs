//! 🔺️ `change-frame-member-basis-basis` diff: patches `basis` of the addressed frame_member_basis row.

use crate::change_frame_member_basis_basis::ChangeFrameMemberBasisBasis;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FrameMemberBasisDelta, En1993FrameMemberBasisPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeFrameMemberBasisBasis, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.frame_member_bases.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "frame_member_basis", Vec::<String>::new());
    };
    if row.basis == payload.new_basis {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The frame_member_basis already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { frame_member_bases: En1993FrameMemberBasisDelta::modification(payload.id.clone(), En1993FrameMemberBasisPatch { basis: Some(payload.new_basis), ..Default::default() }), ..Default::default() })
}
