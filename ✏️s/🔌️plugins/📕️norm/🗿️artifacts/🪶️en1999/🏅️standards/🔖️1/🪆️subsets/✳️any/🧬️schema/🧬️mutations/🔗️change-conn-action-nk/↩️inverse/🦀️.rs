//! ↩️ `change-conn-action-nk` inverse: restores the previous `n_k`.

use crate::change_conn_action_n_k::ChangeConnActionNK;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnActionNK, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_action_n_k::ChangeConnActionNK> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeConnActionNK { connection_id: payload.connection_id.clone(), action_id: payload.action_id.clone(), new_n_k: row.n_k })],
        None => Vec::new(),
    })
}
