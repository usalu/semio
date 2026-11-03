//! 📍 `change-bar-point-layer-id` diff: patches `layer_id` of the addressed bar point row.

use crate::change_bar_point_layer_id::ChangeBarPointLayerId;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersBarsPatch, En1992MembersBarsRows, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeBarPointLayerId, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let Some(r1) = r0.bars.iter().find(|row| row.id == payload.bar_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Bar point {} does not exist.", payload.bar_id), [payload.bar_id.clone()]);
    };
    if r1.layer_id == payload.new_layer_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The bar point already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { bars: Some(En1992MembersBarsRows::modification(&payload.bar_id, En1992MembersBarsPatch { layer_id: Some(payload.new_layer_id.clone()), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
