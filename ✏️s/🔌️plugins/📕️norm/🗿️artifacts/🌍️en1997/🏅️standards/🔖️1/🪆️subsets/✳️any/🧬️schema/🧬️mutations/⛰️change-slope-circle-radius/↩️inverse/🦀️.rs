//! ↩️ `change-slope-circle-radius` inverse: restores the previous `circle_radius`.

use crate::change_slope_circle_radius::ChangeSlopeCircleRadius;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeSlopeCircleRadius, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_slope_circle_radius::ChangeSlopeCircleRadius> {
    Ok(match base.slopes.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSlopeCircleRadius { id: payload.id.clone(), new_circle_radius: row.circle_radius })],
        None => Vec::new(),
    })
}
