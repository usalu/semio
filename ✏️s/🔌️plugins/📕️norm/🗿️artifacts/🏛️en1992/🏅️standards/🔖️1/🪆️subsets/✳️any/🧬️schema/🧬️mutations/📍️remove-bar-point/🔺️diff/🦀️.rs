//! 📍 `remove-bar-point` diff: removes the addressed bar point row.

use crate::remove_bar_point::RemoveBarPoint;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersBarsRows, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &RemoveBarPoint, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let index = payload.index;
    if index >= r0.bars.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "bars index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { bars: Some(En1992MembersBarsRows::removal(&r0.bars, index)), ..Default::default() })), ..Default::default() })
}
