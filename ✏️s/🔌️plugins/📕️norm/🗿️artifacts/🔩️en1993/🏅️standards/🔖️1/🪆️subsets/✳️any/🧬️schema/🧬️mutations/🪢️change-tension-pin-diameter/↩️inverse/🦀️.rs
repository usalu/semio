//! ↩️ `change-tension-pin-diameter` inverse: restores the previous `pin_diameter`.

use crate::change_tension_pin_diameter::ChangeTensionPinDiameter;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionPinDiameter, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_pin_diameter::ChangeTensionPinDiameter> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionPinDiameter { id: payload.id.clone(), new_pin_diameter: row.pin_diameter })],
        None => Vec::new(),
    })
}
