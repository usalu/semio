//! ↩️ `change-explosion-assumed-pressure` inverse: restores the previous `assumed_pressure`.

use crate::change_explosion_assumed_pressure::ChangeExplosionAssumedPressure;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeExplosionAssumedPressure, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_explosion_assumed_pressure::ChangeExplosionAssumedPressure> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeExplosionAssumedPressure, base: &En1991Snapshot) -> Option<ChangeExplosionAssumedPressure> {
    let r0 = base.internal_explosions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeExplosionAssumedPressure { id: payload.id.clone(), new_assumed_pressure: r0.assumed_pressure })
}
