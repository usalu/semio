//! ↩️ `insert-storey` inverse: removes the inserted row again.

use crate::insert_storey::InsertStorey;
use crate::remove_storey::RemoveStorey;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertStorey, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_storey::RemoveStorey> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertStorey, base: &En1998Snapshot) -> Option<RemoveStorey> {
    let _r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    Some(RemoveStorey { building_id: payload.building_id.clone(), id: payload.storey.id.clone() })
}
