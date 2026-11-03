//! ↩️ `remove-dust-explosion` inverse: inserts the removed row back at its position.

use crate::remove_dust_explosion::RemoveDustExplosion;
use crate::insert_dust_explosion::InsertDustExplosion;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveDustExplosion, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_dust_explosion::InsertDustExplosion> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveDustExplosion, base: &En1991Snapshot) -> Option<InsertDustExplosion> {
    let (index, row) = base.dust_explosions.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertDustExplosion { index, item: row.clone() })
}
