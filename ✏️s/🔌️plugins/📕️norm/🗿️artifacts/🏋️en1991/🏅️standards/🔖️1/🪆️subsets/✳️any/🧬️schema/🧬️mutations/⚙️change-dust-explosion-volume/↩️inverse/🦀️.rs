//! ↩️ `change-dust-explosion-volume` inverse: restores the previous `volume`.

use crate::change_dust_explosion_volume::ChangeDustExplosionVolume;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDustExplosionVolume, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_dust_explosion_volume::ChangeDustExplosionVolume> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDustExplosionVolume, base: &En1991Snapshot) -> Option<ChangeDustExplosionVolume> {
    let r0 = base.dust_explosions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDustExplosionVolume { id: payload.id.clone(), new_volume: r0.volume })
}
