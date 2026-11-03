//! ↩️ `change-conn-action-vyk` inverse: restores the previous `v_y_k`.

use crate::change_conn_action_v_y_k::ChangeConnActionVYK;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnActionVYK, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_action_v_y_k::ChangeConnActionVYK> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeConnActionVYK { connection_id: payload.connection_id.clone(), action_id: payload.action_id.clone(), new_v_y_k: row.v_y_k })],
        None => Vec::new(),
    })
}
