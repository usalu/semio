//! ↩️ `change-dust-explosion-p-max` inverse: restores the previous `p_max`.

use crate::change_dust_explosion_p_max::ChangeDustExplosionPMax;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDustExplosionPMax, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_dust_explosion_p_max::ChangeDustExplosionPMax> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDustExplosionPMax, base: &En1991Snapshot) -> Option<ChangeDustExplosionPMax> {
    let r0 = base.dust_explosions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDustExplosionPMax { id: payload.id.clone(), new_p_max: r0.p_max })
}
