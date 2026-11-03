//! ↩️ `change-conn-bolts-plate-thickness` inverse: restores the previous `bolts.plate_thickness`.

use crate::change_conn_bolts_plate_thickness::ChangeConnBoltsPlateThickness;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeConnBoltsPlateThickness, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_conn_bolts_plate_thickness::ChangeConnBoltsPlateThickness> {
    Ok(match base.connections.iter().find(|row| row.id == payload.connection_id) {
        Some(row) => vec![Op::from(ChangeConnBoltsPlateThickness { connection_id: payload.connection_id.clone(), new_bolts_plate_thickness: row.bolts.plate_thickness })],
        None => Vec::new(),
    })
}
