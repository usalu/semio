//! ↩️ `change-conn-action-torsion-k` inverse: restores the previous `torsion_k`.

use crate::change_conn_action_torsion_k::ChangeConnActionTorsionK;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnActionTorsionK, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_action_torsion_k::ChangeConnActionTorsionK> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeConnActionTorsionK { connection_id: payload.connection_id.clone(), action_id: payload.action_id.clone(), new_torsion_k: row.torsion_k })],
        None => Vec::new(),
    })
}
