//! ↩️ `change-conn-bolts-diameter` inverse: restores the previous `bolts.diameter`.

use crate::change_conn_bolts_diameter::ChangeConnBoltsDiameter;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnBoltsDiameter, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_bolts_diameter::ChangeConnBoltsDiameter> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id) {
        Some(row) => vec![Op::from(ChangeConnBoltsDiameter { connection_id: payload.connection_id.clone(), new_bolts_diameter: row.bolts.diameter })],
        None => Vec::new(),
    })
}
