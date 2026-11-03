//! ↩️ `change-dust-explosion-design-pressure` inverse: restores the previous `design_pressure`.

use crate::change_dust_explosion_design_pressure::ChangeDustExplosionDesignPressure;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDustExplosionDesignPressure, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_dust_explosion_design_pressure::ChangeDustExplosionDesignPressure> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDustExplosionDesignPressure, base: &En1991Snapshot) -> Option<ChangeDustExplosionDesignPressure> {
    let r0 = base.dust_explosions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDustExplosionDesignPressure { id: payload.id.clone(), new_design_pressure: r0.design_pressure })
}
