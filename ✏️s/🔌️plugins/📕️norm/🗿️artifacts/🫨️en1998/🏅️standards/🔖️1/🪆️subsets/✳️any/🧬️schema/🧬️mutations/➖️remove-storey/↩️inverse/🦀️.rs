//! ↩️ `remove-storey` inverse: inserts the removed row back at its position.

use crate::remove_storey::RemoveStorey;
use crate::insert_storey::InsertStorey;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveStorey, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_storey::InsertStorey> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveStorey, base: &En1998Snapshot) -> Option<InsertStorey> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let (index, row) = r0.storeys.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertStorey { building_id: payload.building_id.clone(), index: Some(index), storey: row.clone() })
}
