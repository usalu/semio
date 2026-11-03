//! ➕️ `insert-frame-member-basis` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_frame_member_basis::InsertFrameMemberBasis;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FrameMemberBasisDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertFrameMemberBasis, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.frame_member_bases.iter().any(|existing| existing.id == payload.frame_member_basis.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Member force basis id {} already exists.", payload.frame_member_basis.id), [payload.frame_member_basis.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.frame_member_bases.len());
    protocol::MutationOutcome::new(En1993Diff { frame_member_bases: En1993FrameMemberBasisDelta::insertion(index, payload.frame_member_basis.clone()), ..Default::default() })
}
