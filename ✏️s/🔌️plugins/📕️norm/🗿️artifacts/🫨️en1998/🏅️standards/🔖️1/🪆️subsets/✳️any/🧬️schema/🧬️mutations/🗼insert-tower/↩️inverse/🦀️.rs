//! ↩️ `insert-tower` inverse: removes the inserted row again.

use crate::insert_tower::InsertTower;
use crate::remove_tower::RemoveTower;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertTower, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_tower::RemoveTower> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertTower, base: &En1998Snapshot) -> Option<RemoveTower> {
    Some(RemoveTower { id: payload.tower.id.clone() })
}
