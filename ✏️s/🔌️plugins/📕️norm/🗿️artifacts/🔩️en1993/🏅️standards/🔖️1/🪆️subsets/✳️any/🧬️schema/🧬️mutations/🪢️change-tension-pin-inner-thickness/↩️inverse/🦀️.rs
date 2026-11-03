//! ↩️ `change-tension-pin-inner-thickness` inverse: restores the previous `pin_inner_thickness`.

use crate::change_tension_pin_inner_thickness::ChangeTensionPinInnerThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionPinInnerThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_pin_inner_thickness::ChangeTensionPinInnerThickness> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionPinInnerThickness { id: payload.id.clone(), new_pin_inner_thickness: row.pin_inner_thickness })],
        None => Vec::new(),
    })
}
