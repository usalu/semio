//! ↩️ `remove-snow-guard` inverse: inserts the removed row back at its position.

use crate::remove_snow_guard::RemoveSnowGuard;
use crate::insert_snow_guard::InsertSnowGuard;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveSnowGuard, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_snow_guard::InsertSnowGuard> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveSnowGuard, base: &En1991Snapshot) -> Option<InsertSnowGuard> {
    let (index, row) = base.snow_guards.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertSnowGuard { index, item: row.clone() })
}
