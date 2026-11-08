//! 📍️ `change-isolation-system-stiffness` diff: patches `stiffness_n_per_m` of the addressed isolation_system row.

use crate::change_isolation_system_stiffness::ChangeIsolationSystemStiffnessNPerM;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998IsolationSystemDelta, En1998IsolationSystemPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeIsolationSystemStiffnessNPerM, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_stiffness_n_per_m.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.isolation_systems.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("isolation_system {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.stiffness_n_per_m == payload.new_stiffness_n_per_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The isolation_system already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { isolation_systems: En1998IsolationSystemDelta::modification(&payload.id, En1998IsolationSystemPatch { stiffness_n_per_m: Some(payload.new_stiffness_n_per_m), ..Default::default() }), ..Default::default() })
}
