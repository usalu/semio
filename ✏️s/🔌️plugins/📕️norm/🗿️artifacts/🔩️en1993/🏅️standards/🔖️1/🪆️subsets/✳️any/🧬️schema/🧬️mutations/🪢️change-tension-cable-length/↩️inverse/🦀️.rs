//! ↩️ `change-tension-cable-length` inverse: restores the previous `cable_length`.

use crate::change_tension_cable_length::ChangeTensionCableLength;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionCableLength, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_cable_length::ChangeTensionCableLength> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionCableLength { id: payload.id.clone(), new_cable_length: row.cable_length })],
        None => Vec::new(),
    })
}
