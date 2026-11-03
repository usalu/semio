//! ↩️ `change-tension-cable-horizontal-force` inverse: restores the previous `cable_horizontal_force`.

use crate::change_tension_cable_horizontal_force::ChangeTensionCableHorizontalForce;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionCableHorizontalForce, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_cable_horizontal_force::ChangeTensionCableHorizontalForce> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionCableHorizontalForce { id: payload.id.clone(), new_cable_horizontal_force: row.cable_horizontal_force })],
        None => Vec::new(),
    })
}
