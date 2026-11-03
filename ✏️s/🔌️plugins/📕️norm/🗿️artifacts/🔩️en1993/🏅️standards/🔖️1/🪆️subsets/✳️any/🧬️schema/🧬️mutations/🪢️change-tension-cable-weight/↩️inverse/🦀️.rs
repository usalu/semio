//! ↩️ `change-tension-cable-weight` inverse: restores the previous `cable_weight`.

use crate::change_tension_cable_weight::ChangeTensionCableWeight;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionCableWeight, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_cable_weight::ChangeTensionCableWeight> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionCableWeight { id: payload.id.clone(), new_cable_weight: row.cable_weight })],
        None => Vec::new(),
    })
}
