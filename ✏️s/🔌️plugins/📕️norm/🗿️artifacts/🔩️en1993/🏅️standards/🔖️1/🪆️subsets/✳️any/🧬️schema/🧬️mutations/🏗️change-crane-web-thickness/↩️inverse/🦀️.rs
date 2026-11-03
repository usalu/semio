//! ↩️ `change-crane-web-thickness` inverse: restores the previous `web_thickness`.

use crate::change_crane_web_thickness::ChangeCraneWebThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneWebThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_web_thickness::ChangeCraneWebThickness> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneWebThickness { id: payload.id.clone(), new_web_thickness: row.web_thickness })],
        None => Vec::new(),
    })
}
