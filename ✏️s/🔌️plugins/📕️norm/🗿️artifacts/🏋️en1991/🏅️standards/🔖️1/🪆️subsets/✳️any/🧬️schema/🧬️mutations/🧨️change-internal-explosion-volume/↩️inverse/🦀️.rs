//! ↩️ `change-internal-explosion-volume` inverse: restores the previous `volume`.

use crate::change_internal_explosion_volume::ChangeInternalExplosionVolume;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeInternalExplosionVolume, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_internal_explosion_volume::ChangeInternalExplosionVolume> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeInternalExplosionVolume, base: &En1991Snapshot) -> Option<ChangeInternalExplosionVolume> {
    let r0 = base.internal_explosions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeInternalExplosionVolume { id: payload.id.clone(), new_volume: r0.volume })
}
