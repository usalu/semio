//! ↩️ `change-ground-reading-measurement` inverse: restores the previous `measurement`.

use crate::change_ground_reading_measurement::ChangeGroundReadingMeasurement;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeGroundReadingMeasurement, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_ground_reading_measurement::ChangeGroundReadingMeasurement> {
    Ok(match base.investigations.iter().find(|row| row.id == payload.investigation_id).and_then(|row| row.readings.iter().find(|row| row.id == payload.id)) {
        Some(row) => vec![Op::from(ChangeGroundReadingMeasurement { investigation_id: payload.investigation_id.clone(), id: payload.id.clone(), new_measurement: row.measurement })],
        None => Vec::new(),
    })
}
