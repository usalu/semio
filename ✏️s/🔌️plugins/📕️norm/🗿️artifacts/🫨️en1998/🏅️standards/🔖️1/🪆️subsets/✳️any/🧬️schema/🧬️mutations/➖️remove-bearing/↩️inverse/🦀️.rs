//! ↩️ `remove-bearing` inverse: inserts the removed row back at its position.

use crate::remove_bearing::RemoveBearing;
use crate::insert_bearing::InsertBearing;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveBearing, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_bearing::InsertBearing> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveBearing, base: &En1998Snapshot) -> Option<InsertBearing> {
    let (index, row) = base.bearings.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertBearing { index: Some(index), bearing: row.clone() })
}
