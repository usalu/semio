//! ↩️ `change-slope-circle-centre-z` inverse: restores the previous `circle_centre_z`.

use crate::change_slope_circle_centre_z::ChangeSlopeCircleCentreZ;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeSlopeCircleCentreZ, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slope_circle_centre_z::ChangeSlopeCircleCentreZ> {
    Ok(match base.slopes.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSlopeCircleCentreZ { id: payload.id.clone(), new_circle_centre_z: row.circle_centre_z })],
        None => Vec::new(),
    })
}
