//! ↩️ `change-explosion-vent-pressure` inverse: restores the previous `vent_pressure`.

use crate::change_explosion_vent_pressure::ChangeExplosionVentPressure;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeExplosionVentPressure, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_explosion_vent_pressure::ChangeExplosionVentPressure> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeExplosionVentPressure, base: &En1991Snapshot) -> Option<ChangeExplosionVentPressure> {
    let r0 = base.internal_explosions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeExplosionVentPressure { id: payload.id.clone(), new_vent_pressure: r0.vent_pressure })
}
