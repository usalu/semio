//! ↩️ `change-ground-reading-depth` inverse: restores the previous `depth`.

use crate::change_ground_reading_depth::ChangeGroundReadingDepth;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeGroundReadingDepth, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_ground_reading_depth::ChangeGroundReadingDepth> {
    Ok(match base.investigations.iter().find(|row| row.id == payload.investigation_id).and_then(|row| row.readings.iter().find(|row| row.id == payload.id)) {
        Some(row) => vec![Op::from(ChangeGroundReadingDepth { investigation_id: payload.investigation_id.clone(), id: payload.id.clone(), new_depth: row.depth })],
        None => Vec::new(),
    })
}
