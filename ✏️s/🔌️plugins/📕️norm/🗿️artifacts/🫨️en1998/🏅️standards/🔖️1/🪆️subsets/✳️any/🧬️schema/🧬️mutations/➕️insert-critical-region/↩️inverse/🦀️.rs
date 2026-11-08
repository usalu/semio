//! ↩️ `insert-critical-region` inverse: removes the inserted row again.

use crate::insert_critical_region::InsertCriticalRegion;
use crate::remove_critical_region::RemoveCriticalRegion;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertCriticalRegion, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_critical_region::RemoveCriticalRegion> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertCriticalRegion, base: &En1998Snapshot) -> Option<RemoveCriticalRegion> {
    let _ = base;
    Some(RemoveCriticalRegion { id: payload.critical_region.id.clone() })
}
