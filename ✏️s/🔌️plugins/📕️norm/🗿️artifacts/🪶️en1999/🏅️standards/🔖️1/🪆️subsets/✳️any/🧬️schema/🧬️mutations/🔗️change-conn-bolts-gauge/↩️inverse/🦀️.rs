//! ↩️ `change-conn-bolts-gauge` inverse: restores the previous `bolts.gauge`.

use crate::change_conn_bolts_gauge::ChangeConnBoltsGauge;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnBoltsGauge, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_bolts_gauge::ChangeConnBoltsGauge> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id) {
        Some(row) => vec![Op::from(ChangeConnBoltsGauge { connection_id: payload.connection_id.clone(), new_bolts_gauge: row.bolts.gauge })],
        None => Vec::new(),
    })
}
