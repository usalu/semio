//! ↩️ `change-conn-force-method` inverse: restores the previous `force_method`.

use crate::change_conn_force_method::ChangeConnForceMethod;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnForceMethod, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_force_method::ChangeConnForceMethod> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id) {
        Some(row) => vec![Op::from(ChangeConnForceMethod { connection_id: payload.connection_id.clone(), new_force_method: row.force_method })],
        None => Vec::new(),
    })
}
