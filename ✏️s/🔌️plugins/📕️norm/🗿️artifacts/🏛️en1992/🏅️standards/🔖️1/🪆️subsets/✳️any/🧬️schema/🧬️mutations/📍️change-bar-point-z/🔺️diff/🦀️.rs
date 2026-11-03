//! 📍 `change-bar-point-z` diff: patches `z` of the addressed bar point row.

use crate::change_bar_point_z::ChangeBarPointZ;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersBarsPatch, En1992MembersBarsRows, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeBarPointZ, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_z.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let Some(r1) = r0.bars.iter().find(|row| row.id == payload.bar_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Bar point {} does not exist.", payload.bar_id), [payload.bar_id.clone()]);
    };
    if r1.z == payload.new_z {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bar point already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { bars: Some(En1992MembersBarsRows::modification(&payload.bar_id, En1992MembersBarsPatch { z: Some(payload.new_z), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
