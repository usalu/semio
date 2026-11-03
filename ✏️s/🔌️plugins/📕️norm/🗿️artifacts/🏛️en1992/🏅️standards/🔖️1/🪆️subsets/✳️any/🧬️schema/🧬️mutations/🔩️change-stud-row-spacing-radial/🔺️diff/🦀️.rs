//! 🔩 `change-stud-row-spacing-radial` diff: patches `spacing_radial` of the addressed stud row row.

use crate::change_stud_row_spacing_radial::ChangeStudRowSpacingRadial;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersPatch, En1992MembersRows, En1992MembersStudRowsPatch, En1992MembersStudRowsRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeStudRowSpacingRadial, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    if !payload.new_spacing_radial.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let Some(r1) = r0.stud_rows.iter().find(|row| row.id == payload.stud_row_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Stud row {} does not exist.", payload.stud_row_id), [payload.stud_row_id.clone()]);
    };
    if r1.spacing_radial == payload.new_spacing_radial {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The stud row already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { stud_rows: Some(En1992MembersStudRowsRows::modification(&payload.stud_row_id, En1992MembersStudRowsPatch { spacing_radial: Some(payload.new_spacing_radial), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
