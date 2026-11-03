//! ↩️ `change-conn-member-id` inverse: restores the previous `member_id`.

use crate::change_conn_member_id::ChangeConnMemberId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnMemberId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_member_id::ChangeConnMemberId> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id) {
        Some(row) => vec![Op::from(ChangeConnMemberId { connection_id: payload.connection_id.clone(), new_member_id: row.member_id.clone() })],
        None => Vec::new(),
    })
}
