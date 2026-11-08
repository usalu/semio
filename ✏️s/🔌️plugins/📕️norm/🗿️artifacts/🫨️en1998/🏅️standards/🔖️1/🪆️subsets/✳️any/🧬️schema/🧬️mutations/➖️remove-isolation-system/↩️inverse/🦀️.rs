//! ↩️ `remove-isolation-system` inverse: inserts the removed row back at its position.

use crate::remove_isolation_system::RemoveIsolationSystem;
use crate::insert_isolation_system::InsertIsolationSystem;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveIsolationSystem, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_isolation_system::InsertIsolationSystem> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveIsolationSystem, base: &En1998Snapshot) -> Option<InsertIsolationSystem> {
    let (index, row) = base.isolation_systems.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertIsolationSystem { index: Some(index), isolation_system: row.clone() })
}
