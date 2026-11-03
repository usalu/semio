//! 📐️ `change-section-point-z` diff: patches `z` of the addressed section point row.

use crate::change_section_point_z::ChangeSectionPointZ;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersPatch, En1992MembersRows, En1992MembersSectionPointsPatch, En1992MembersSectionPointsRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeSectionPointZ, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_z.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let Some(r1) = r0.section_points.iter().find(|row| row.id == payload.section_point_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Section point {} does not exist.", payload.section_point_id), [payload.section_point_id.clone()]);
    };
    if r1.z == payload.new_z {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The section point already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { section_points: Some(En1992MembersSectionPointsRows::modification(&payload.section_point_id, En1992MembersSectionPointsPatch { z: Some(payload.new_z), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
