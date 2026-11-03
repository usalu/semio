//! ↩️ `change-bridge-surfacing-thickness` inverse: restores the previous `surfacing_thickness`.

use crate::change_bridge_surfacing_thickness::ChangeBridgeSurfacingThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeSurfacingThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_surfacing_thickness::ChangeBridgeSurfacingThickness> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeSurfacingThickness { id: payload.id.clone(), new_surfacing_thickness: row.surfacing_thickness })],
        None => Vec::new(),
    })
}
