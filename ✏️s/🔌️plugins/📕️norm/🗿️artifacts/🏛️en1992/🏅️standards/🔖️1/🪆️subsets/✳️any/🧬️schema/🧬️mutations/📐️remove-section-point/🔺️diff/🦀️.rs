//! 📐️ `remove-section-point` diff: removes the addressed section point row.

use crate::remove_section_point::RemoveSectionPoint;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersPatch, En1992MembersRows, En1992MembersSectionPointsRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &RemoveSectionPoint, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let index = payload.index;
    if index >= r0.section_points.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "section_points index out of range", [index.to_string()]);
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { section_points: Some(En1992MembersSectionPointsRows::removal(&r0.section_points, index)), ..Default::default() })), ..Default::default() })
}
