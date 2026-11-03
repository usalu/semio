//! ↩️ `change-conn-kind` inverse: restores the previous `kind`.

use crate::change_conn_kind::ChangeConnKind;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnKind, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_kind::ChangeConnKind> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id) {
        Some(row) => vec![Op::from(ChangeConnKind { connection_id: payload.connection_id.clone(), new_kind: row.kind })],
        None => Vec::new(),
    })
}
