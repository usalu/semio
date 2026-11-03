//! 🕳️ `change-opening-width` diff: patches `width` of the addressed opening row.

use crate::change_opening_width::ChangeOpeningWidth;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersOpeningsPatch, En1992MembersOpeningsRows, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeOpeningWidth, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_width.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let Some(r1) = r0.openings.iter().find(|row| row.id == payload.opening_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Opening {} does not exist.", payload.opening_id), [payload.opening_id.clone()]);
    };
    if r1.width == payload.new_width {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The opening already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { openings: Some(En1992MembersOpeningsRows::modification(&payload.opening_id, En1992MembersOpeningsPatch { width: Some(payload.new_width), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
