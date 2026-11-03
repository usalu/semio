//! 📍 `insert-bar-point` diff: inserts the bar point row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_bar_point::InsertBarPoint;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersBarsRows, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &InsertBarPoint, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    if r0.bars.iter().any(|existing| existing.id == payload.bar_point.id) {
        let key = payload.bar_point.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Bar point {key} already exists."), [key]);
    }
    if payload.index > r0.bars.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, r0.bars.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { bars: Some(En1992MembersBarsRows::insertion(payload.index, payload.bar_point.clone())), ..Default::default() })), ..Default::default() })
}
