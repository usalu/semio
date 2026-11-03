//! ↩️ `change-tension-pin-outer-thickness` inverse: restores the previous `pin_outer_thickness`.

use crate::change_tension_pin_outer_thickness::ChangeTensionPinOuterThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionPinOuterThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_pin_outer_thickness::ChangeTensionPinOuterThickness> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionPinOuterThickness { id: payload.id.clone(), new_pin_outer_thickness: row.pin_outer_thickness })],
        None => Vec::new(),
    })
}
