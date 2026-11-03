//! ↩️ `change-conn-welds-length` inverse: restores the previous `welds.length`.

use crate::change_conn_welds_length::ChangeConnWeldsLength;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnWeldsLength, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_welds_length::ChangeConnWeldsLength> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id) {
        Some(row) => vec![Op::from(ChangeConnWeldsLength { connection_id: payload.connection_id.clone(), new_welds_length: row.welds.length })],
        None => Vec::new(),
    })
}
