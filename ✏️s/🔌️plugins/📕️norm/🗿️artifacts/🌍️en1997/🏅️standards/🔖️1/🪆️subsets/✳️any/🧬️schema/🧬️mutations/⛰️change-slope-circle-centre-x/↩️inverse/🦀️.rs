//! ↩️ `change-slope-circle-centre-x` inverse: restores the previous `circle_centre_x`.

use crate::change_slope_circle_centre_x::ChangeSlopeCircleCentreX;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeSlopeCircleCentreX, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slope_circle_centre_x::ChangeSlopeCircleCentreX> {
    Ok(match base.slopes.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSlopeCircleCentreX { id: payload.id.clone(), new_circle_centre_x: row.circle_centre_x })],
        None => Vec::new(),
    })
}
