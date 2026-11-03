//! ↩️ `change-conn-action-mzk` inverse: restores the previous `m_z_k`.

use crate::change_conn_action_m_z_k::ChangeConnActionMZK;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnActionMZK, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_action_m_z_k::ChangeConnActionMZK> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeConnActionMZK { connection_id: payload.connection_id.clone(), action_id: payload.action_id.clone(), new_m_z_k: row.m_z_k })],
        None => Vec::new(),
    })
}
