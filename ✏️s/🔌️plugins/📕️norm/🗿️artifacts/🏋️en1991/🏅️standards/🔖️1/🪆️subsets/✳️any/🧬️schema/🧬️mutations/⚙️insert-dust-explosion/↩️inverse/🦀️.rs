//! ↩️ `insert-dust-explosion` inverse: removes the inserted row again.

use crate::insert_dust_explosion::InsertDustExplosion;
use crate::remove_dust_explosion::RemoveDustExplosion;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertDustExplosion, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_dust_explosion::RemoveDustExplosion> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertDustExplosion, base: &En1991Snapshot) -> Option<RemoveDustExplosion> {
    Some(RemoveDustExplosion { id: payload.item.id.clone() })
}
