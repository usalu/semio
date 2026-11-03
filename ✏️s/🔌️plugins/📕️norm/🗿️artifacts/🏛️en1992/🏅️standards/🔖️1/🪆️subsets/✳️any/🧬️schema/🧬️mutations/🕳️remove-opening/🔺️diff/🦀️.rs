//! 🕳️ `remove-opening` diff: removes the addressed opening row.

use crate::remove_opening::RemoveOpening;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersOpeningsRows, En1992MembersPatch, En1992MembersRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &RemoveOpening, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let index = payload.index;
    if index >= r0.openings.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "openings index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { openings: Some(En1992MembersOpeningsRows::removal(&r0.openings, index)), ..Default::default() })), ..Default::default() })
}
