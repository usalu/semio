//! ↩️ `change-dust-explosion-kst` inverse: restores the previous `kst`.

use crate::change_dust_explosion_kst::ChangeDustExplosionKst;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &ChangeDustExplosionKst, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_dust_explosion_kst::ChangeDustExplosionKst> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeDustExplosionKst, base: &En1991Snapshot) -> Option<ChangeDustExplosionKst> {
    let r0 = base.dust_explosions.iter().find(|row| row.id == payload.id)?;
    Some(ChangeDustExplosionKst { id: payload.id.clone(), new_kst: r0.kst })
}
