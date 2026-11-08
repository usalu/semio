//! ➕️ `insert-isolation-system` diff: inserts the isolation_system row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_isolation_system::InsertIsolationSystem;
use semio_s_artifact_norm_en1998_diff::{En1998IsolationSystemDelta, En1998Diff};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &InsertIsolationSystem, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if base.isolation_systems.iter().any(|existing| existing.id == payload.isolation_system.id) {
        let key = payload.isolation_system.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("isolation_system {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.isolation_systems.len());
    let outcome = protocol::MutationOutcome::new(En1998Diff { isolation_systems: En1998IsolationSystemDelta::insertion(index, payload.isolation_system.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the isolation_system list; inserted at {index}.", payload.index.unwrap_or(index)))
}
