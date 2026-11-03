//! ↩️ `change-conn-bolts-pitch` inverse: restores the previous `bolts.pitch`.

use crate::change_conn_bolts_pitch::ChangeConnBoltsPitch;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnBoltsPitch, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_bolts_pitch::ChangeConnBoltsPitch> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id) {
        Some(row) => vec![Op::from(ChangeConnBoltsPitch { connection_id: payload.connection_id.clone(), new_bolts_pitch: row.bolts.pitch })],
        None => Vec::new(),
    })
}
