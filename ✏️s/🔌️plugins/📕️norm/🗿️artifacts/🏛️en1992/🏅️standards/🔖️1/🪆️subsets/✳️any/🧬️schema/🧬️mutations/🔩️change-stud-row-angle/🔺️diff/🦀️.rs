//! 🔩 `change-stud-row-angle` diff: patches `angle` of the addressed stud row row.

use crate::change_stud_row_angle::ChangeStudRowAngle;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersPatch, En1992MembersRows, En1992MembersStudRowsPatch, En1992MembersStudRowsRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeStudRowAngle, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_angle.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let Some(r1) = r0.stud_rows.iter().find(|row| row.id == payload.stud_row_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Stud row {} does not exist.", payload.stud_row_id), [payload.stud_row_id.clone()]);
    };
    if r1.angle == payload.new_angle {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The stud row already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { stud_rows: Some(En1992MembersStudRowsRows::modification(&payload.stud_row_id, En1992MembersStudRowsPatch { angle: Some(payload.new_angle), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
