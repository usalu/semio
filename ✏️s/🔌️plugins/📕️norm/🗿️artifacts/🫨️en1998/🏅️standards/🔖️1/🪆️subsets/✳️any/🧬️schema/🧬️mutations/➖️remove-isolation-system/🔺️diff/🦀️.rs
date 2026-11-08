//! ➖️ `remove-isolation-system` diff: removes the addressed isolation_system row.

use crate::remove_isolation_system::RemoveIsolationSystem;
use semio_s_artifact_norm_en1998_diff::{En1998IsolationSystemDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveIsolationSystem, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.isolation_systems.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("isolation_system {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { isolation_systems: En1998IsolationSystemDelta::removal(&base.isolation_systems, index), ..Default::default() })
}
