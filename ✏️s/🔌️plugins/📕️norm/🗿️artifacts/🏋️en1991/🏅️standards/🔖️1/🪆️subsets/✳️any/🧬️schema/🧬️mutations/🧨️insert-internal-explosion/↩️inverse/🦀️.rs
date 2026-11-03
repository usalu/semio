//! ↩️ `insert-internal-explosion` inverse: removes the inserted row again.

use crate::insert_internal_explosion::InsertInternalExplosion;
use crate::remove_internal_explosion::RemoveInternalExplosion;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertInternalExplosion, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_internal_explosion::RemoveInternalExplosion> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertInternalExplosion, base: &En1991Snapshot) -> Option<RemoveInternalExplosion> {
    Some(RemoveInternalExplosion { id: payload.item.id.clone() })
}
