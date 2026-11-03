//! 🔩 `change-stud-row-count` diff: patches `count` of the addressed stud row row.

use crate::change_stud_row_count::ChangeStudRowCount;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersPatch, En1992MembersRows, En1992MembersStudRowsPatch, En1992MembersStudRowsRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &ChangeStudRowCount, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    let Some(r1) = r0.stud_rows.iter().find(|row| row.id == payload.stud_row_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Stud row {} does not exist.", payload.stud_row_id), [payload.stud_row_id.clone()]);
    };
    if r1.count == payload.new_count {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The stud row already has this value.");
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { stud_rows: Some(En1992MembersStudRowsRows::modification(&payload.stud_row_id, En1992MembersStudRowsPatch { count: Some(payload.new_count), ..Default::default() })), ..Default::default() })), ..Default::default() })
}
