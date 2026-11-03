//! ↩️ `change-tension-pin-plate-fy` inverse: restores the previous `pin_plate_fy`.

use crate::change_tension_pin_plate_fy::ChangeTensionPinPlateFy;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionPinPlateFy, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_pin_plate_fy::ChangeTensionPinPlateFy> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionPinPlateFy { id: payload.id.clone(), new_pin_plate_fy: row.pin_plate_fy })],
        None => Vec::new(),
    })
}
