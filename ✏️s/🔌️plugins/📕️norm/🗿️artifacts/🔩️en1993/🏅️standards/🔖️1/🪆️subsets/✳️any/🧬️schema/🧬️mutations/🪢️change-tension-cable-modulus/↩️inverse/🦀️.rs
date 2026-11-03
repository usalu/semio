//! ↩️ `change-tension-cable-modulus` inverse: restores the previous `cable_modulus`.

use crate::change_tension_cable_modulus::ChangeTensionCableModulus;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionCableModulus, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_cable_modulus::ChangeTensionCableModulus> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionCableModulus { id: payload.id.clone(), new_cable_modulus: row.cable_modulus })],
        None => Vec::new(),
    })
}
