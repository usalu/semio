//! ↩️ `change-dust-explosion-vent-area` inverse: restores the previous `vent_area`.

use crate::change_dust_explosion_vent_area::ChangeDustExplosionVentArea;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDustExplosionVentArea, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_dust_explosion_vent_area::ChangeDustExplosionVentArea> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDustExplosionVentArea, base: &En1991Snapshot) -> Option<ChangeDustExplosionVentArea> {
    let r0 = base.dust_explosions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDustExplosionVentArea { id: payload.id.clone(), new_vent_area: r0.vent_area })
}
