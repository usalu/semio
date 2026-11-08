//! ↩️ `insert-isolation-system` inverse: removes the inserted row again.

use crate::insert_isolation_system::InsertIsolationSystem;
use crate::remove_isolation_system::RemoveIsolationSystem;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertIsolationSystem, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_isolation_system::RemoveIsolationSystem> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertIsolationSystem, base: &En1998Snapshot) -> Option<RemoveIsolationSystem> {
    let _ = base;
    Some(RemoveIsolationSystem { id: payload.isolation_system.id.clone() })
}
