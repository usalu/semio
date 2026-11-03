//! ↩️ `change-tension-pin-fyp` inverse: restores the previous `pin_fyp`.

use crate::change_tension_pin_fyp::ChangeTensionPinFyp;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionPinFyp, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_pin_fyp::ChangeTensionPinFyp> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionPinFyp { id: payload.id.clone(), new_pin_fyp: row.pin_fyp })],
        None => Vec::new(),
    })
}
