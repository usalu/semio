//! ↩️ `change-conn-bolts-bolts-per-row` inverse: restores the previous `bolts.bolts_per_row`.

use crate::change_conn_bolts_bolts_per_row::ChangeConnBoltsBoltsPerRow;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnBoltsBoltsPerRow, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_bolts_bolts_per_row::ChangeConnBoltsBoltsPerRow> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id) {
        Some(row) => vec![Op::from(ChangeConnBoltsBoltsPerRow { connection_id: payload.connection_id.clone(), new_bolts_bolts_per_row: row.bolts.bolts_per_row })],
        None => Vec::new(),
    })
}
