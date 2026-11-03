//! ↩️ `change-conn-action-myk` inverse: restores the previous `m_y_k`.

use crate::change_conn_action_m_y_k::ChangeConnActionMYK;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnActionMYK, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_action_m_y_k::ChangeConnActionMYK> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeConnActionMYK { connection_id: payload.connection_id.clone(), action_id: payload.action_id.clone(), new_m_y_k: row.m_y_k })],
        None => Vec::new(),
    })
}
