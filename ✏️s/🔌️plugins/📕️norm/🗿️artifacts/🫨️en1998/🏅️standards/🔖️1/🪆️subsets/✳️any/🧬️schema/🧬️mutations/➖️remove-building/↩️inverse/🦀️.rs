//! ↩️ `remove-building` inverse: inserts the removed row back at its position.

use crate::remove_building::RemoveBuilding;
use crate::insert_building::InsertBuilding;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveBuilding, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_building::InsertBuilding> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveBuilding, base: &En1998Snapshot) -> Option<InsertBuilding> {
    let (index, row) = base.buildings.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertBuilding { index: Some(index), building: row.clone() })
}
