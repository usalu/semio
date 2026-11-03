//! ↩️ `change-tension-cable-stress` inverse: restores the previous `cable_stress`.

use crate::change_tension_cable_stress::ChangeTensionCableStress;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionCableStress, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_cable_stress::ChangeTensionCableStress> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionCableStress { id: payload.id.clone(), new_cable_stress: row.cable_stress })],
        None => Vec::new(),
    })
}
