//! 🔩 `insert-stud-row` diff: inserts the stud row row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_stud_row::InsertStudRow;
use semio_s_artifact_norm_en1992_diff::{En1992Diff, En1992MembersPatch, En1992MembersRows, En1992MembersStudRowsRows};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn diff(payload: &InsertStudRow, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
    let Some(r0) = base.members.iter().find(|row| row.id == payload.member_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member {} does not exist.", payload.member_id), [payload.member_id.clone()]);
    };
    if r0.stud_rows.iter().any(|existing| existing.id == payload.stud_row.id) {
        let key = payload.stud_row.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Stud row {key} already exists."), [key]);
    }
    if payload.index > r0.stud_rows.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, r0.stud_rows.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1992Diff { members: Some(En1992MembersRows::modification(&payload.member_id, En1992MembersPatch { stud_rows: Some(En1992MembersStudRowsRows::insertion(payload.index, payload.stud_row.clone())), ..Default::default() })), ..Default::default() })
}
