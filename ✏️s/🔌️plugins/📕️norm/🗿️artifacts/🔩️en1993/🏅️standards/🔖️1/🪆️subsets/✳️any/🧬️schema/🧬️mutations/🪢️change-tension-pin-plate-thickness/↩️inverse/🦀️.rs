//! ↩️ `change-tension-pin-plate-thickness` inverse: restores the previous `pin_plate_thickness`.

use crate::change_tension_pin_plate_thickness::ChangeTensionPinPlateThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionPinPlateThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_pin_plate_thickness::ChangeTensionPinPlateThickness> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionPinPlateThickness { id: payload.id.clone(), new_pin_plate_thickness: row.pin_plate_thickness })],
        None => Vec::new(),
    })
}
