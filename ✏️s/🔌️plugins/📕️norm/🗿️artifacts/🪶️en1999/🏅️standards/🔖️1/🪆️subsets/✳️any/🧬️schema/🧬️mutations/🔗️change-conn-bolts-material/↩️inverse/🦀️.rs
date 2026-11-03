//! ↩️ `change-conn-bolts-material` inverse: restores the previous `bolts.material`.

use crate::change_conn_bolts_material::ChangeConnBoltsMaterial;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnBoltsMaterial, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_bolts_material::ChangeConnBoltsMaterial> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id) {
        Some(row) => vec![Op::from(ChangeConnBoltsMaterial { connection_id: payload.connection_id.clone(), new_bolts_material: row.bolts.material.clone() })],
        None => Vec::new(),
    })
}
