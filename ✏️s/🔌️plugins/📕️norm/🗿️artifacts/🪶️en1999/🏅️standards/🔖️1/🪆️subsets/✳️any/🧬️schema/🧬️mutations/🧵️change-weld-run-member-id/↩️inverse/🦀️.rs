//! ↩️ `change-weld-run-member-id` inverse: restores the previous `member_id`.

use crate::change_weld_run_member_id::ChangeWeldRunMemberId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldRunMemberId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_run_member_id::ChangeWeldRunMemberId> {
    Ok(match base.weld_runs.iter().find(|row| row.id == payload.weld_run_id) {
        Some(row) => vec![Op::from(ChangeWeldRunMemberId { weld_run_id: payload.weld_run_id.clone(), new_member_id: row.member_id.clone() })],
        None => Vec::new(),
    })
}
