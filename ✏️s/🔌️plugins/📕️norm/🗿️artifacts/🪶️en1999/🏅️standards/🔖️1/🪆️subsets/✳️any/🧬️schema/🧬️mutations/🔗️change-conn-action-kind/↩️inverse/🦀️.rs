//! ↩️ `change-conn-action-kind` inverse: restores the previous `kind`.

use crate::change_conn_action_kind::ChangeConnActionKind;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnActionKind, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_action_kind::ChangeConnActionKind> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id).and_then(|row| row.actions.iter().find(|row| row.id == payload.action_id)) {
        Some(row) => vec![Op::from(ChangeConnActionKind { connection_id: payload.connection_id.clone(), action_id: payload.action_id.clone(), new_kind: row.kind })],
        None => Vec::new(),
    })
}
