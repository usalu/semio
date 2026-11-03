//! ↩️ `change-tension-cable-horiz-force` inverse: restores the previous `cable_horizontal_force`.

use crate::change_tension_cable_horiz_force::ChangeTensionCableHorizForce;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTensionCableHorizForce, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tension_cable_horiz_force::ChangeTensionCableHorizForce> {
    Ok(match base.tension_components.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTensionCableHorizForce { id: payload.id.clone(), new_cable_horizontal_force: row.cable_horizontal_force })],
        None => Vec::new(),
    })
}
