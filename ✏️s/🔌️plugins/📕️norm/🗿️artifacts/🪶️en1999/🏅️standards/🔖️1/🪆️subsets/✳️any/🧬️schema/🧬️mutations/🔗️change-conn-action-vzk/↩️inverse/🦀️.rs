//! ↩️ `change-conn-action-vzk` inverse: restores the previous `v_z_k`.

use crate::change_conn_action_v_z_k::ChangeConnActionVZK;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnActionVZK, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_action_v_z_k::ChangeConnActionVZK> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeConnActionVZK { connection_id: payload.connection_id.clone(), action_id: payload.action_id.clone(), new_v_z_k: row.v_z_k })],
        None => Vec::new(),
    })
}
