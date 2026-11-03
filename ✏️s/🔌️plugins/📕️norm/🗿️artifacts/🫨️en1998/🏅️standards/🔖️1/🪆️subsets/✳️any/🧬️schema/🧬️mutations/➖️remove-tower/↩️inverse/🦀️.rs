//! ↩️ `remove-tower` inverse: inserts the removed row back at its position.

use crate::remove_tower::RemoveTower;
use crate::insert_tower::InsertTower;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveTower, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_tower::InsertTower> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveTower, base: &En1998Snapshot) -> Option<InsertTower> {
    let (index, row) = base.towers.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertTower { index: Some(index), tower: row.clone() })
}
