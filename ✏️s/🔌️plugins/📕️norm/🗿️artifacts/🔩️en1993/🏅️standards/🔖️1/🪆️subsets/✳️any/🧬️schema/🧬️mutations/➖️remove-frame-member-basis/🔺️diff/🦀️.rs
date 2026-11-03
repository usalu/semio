//! ➖️ `remove-frame-member-basis` diff — removes the row at the index.

use crate::remove_frame_member_basis::RemoveFrameMemberBasis;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FrameMemberBasisDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveFrameMemberBasis, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.frame_member_bases.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("frame-member-basis index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { frame_member_bases: En1993FrameMemberBasisDelta::removal(&base.frame_member_bases, payload.index), ..Default::default() })
}
