//! ↩️ `remove-critical-region` inverse: inserts the removed row back at its position.

use crate::remove_critical_region::RemoveCriticalRegion;
use crate::insert_critical_region::InsertCriticalRegion;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveCriticalRegion, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_critical_region::InsertCriticalRegion> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveCriticalRegion, base: &En1998Snapshot) -> Option<InsertCriticalRegion> {
    let (index, row) = base.critical_regions.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertCriticalRegion { index: Some(index), critical_region: row.clone() })
}
