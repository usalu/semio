//! ↩️ `change-dust-explosion-static-opening-pressure` inverse: restores the previous `static_opening_pressure`.

use crate::change_dust_explosion_static_opening_pressure::ChangeDustExplosionStaticOpeningPressure;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDustExplosionStaticOpeningPressure, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_dust_explosion_static_opening_pressure::ChangeDustExplosionStaticOpeningPressure> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDustExplosionStaticOpeningPressure, base: &En1991Snapshot) -> Option<ChangeDustExplosionStaticOpeningPressure> {
    let r0 = base.dust_explosions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDustExplosionStaticOpeningPressure { id: payload.id.clone(), new_static_opening_pressure: r0.static_opening_pressure })
}
