//! ↩️ `insert-building` inverse: removes the inserted row again.

use crate::insert_building::InsertBuilding;
use crate::remove_building::RemoveBuilding;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertBuilding, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_building::RemoveBuilding> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertBuilding, base: &En1998Snapshot) -> Option<RemoveBuilding> {
    Some(RemoveBuilding { id: payload.building.id.clone() })
}
