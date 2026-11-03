//! ↩️ `remove-internal-explosion` inverse: inserts the removed row back at its position.

use crate::remove_internal_explosion::RemoveInternalExplosion;
use crate::insert_internal_explosion::InsertInternalExplosion;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveInternalExplosion, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_internal_explosion::InsertInternalExplosion> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveInternalExplosion, base: &En1991Snapshot) -> Option<InsertInternalExplosion> {
    let (index, row) = base.internal_explosions.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertInternalExplosion { index, item: row.clone() })
}
