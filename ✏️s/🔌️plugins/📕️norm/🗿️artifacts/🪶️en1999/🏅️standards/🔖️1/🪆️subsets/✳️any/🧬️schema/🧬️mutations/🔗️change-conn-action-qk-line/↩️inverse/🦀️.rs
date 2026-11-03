//! ↩️ `change-conn-action-qk-line` inverse: restores the previous `q_k_line`.

use crate::change_conn_action_q_k_line::ChangeConnActionQKLine;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnActionQKLine, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_action_q_k_line::ChangeConnActionQKLine> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeConnActionQKLine { connection_id: payload.connection_id.clone(), action_id: payload.action_id.clone(), new_q_k_line: row.q_k_line })],
        None => Vec::new(),
    })
}
