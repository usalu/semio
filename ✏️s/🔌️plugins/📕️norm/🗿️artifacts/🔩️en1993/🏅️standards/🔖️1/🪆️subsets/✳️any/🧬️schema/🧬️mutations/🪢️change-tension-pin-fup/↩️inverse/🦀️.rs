//! ↩️ `change-tension-pin-fup` inverse: restores the previous `pin_fup`.

use crate::change_tension_pin_fup::ChangeTensionPinFup;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionPinFup, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_pin_fup::ChangeTensionPinFup> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionPinFup { id: payload.id.clone(), new_pin_fup: row.pin_fup })],
        None => Vec::new(),
    })
}
