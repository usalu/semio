//! ↩️ `change-tension-cable-unit-weight` inverse: restores the previous `cable_unit_weight`.

use crate::change_tension_cable_unit_weight::ChangeTensionCableUnitWeight;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionCableUnitWeight, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_cable_unit_weight::ChangeTensionCableUnitWeight> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionCableUnitWeight { id: payload.id.clone(), new_cable_unit_weight: row.cable_unit_weight })],
        None => Vec::new(),
    })
}
