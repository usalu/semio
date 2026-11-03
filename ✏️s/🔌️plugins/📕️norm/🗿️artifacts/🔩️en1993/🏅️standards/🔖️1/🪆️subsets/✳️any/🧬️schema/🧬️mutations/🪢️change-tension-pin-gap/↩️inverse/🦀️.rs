//! ↩️ `change-tension-pin-gap` inverse: restores the previous `pin_gap`.

use crate::change_tension_pin_gap::ChangeTensionPinGap;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionPinGap, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_pin_gap::ChangeTensionPinGap> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionPinGap { id: payload.id.clone(), new_pin_gap: row.pin_gap })],
        None => Vec::new(),
    })
}
