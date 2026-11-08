//! 📍️ `change-isolation-system-strength-n` diff: patches `strength_n` of the addressed isolation_system row.

use crate::change_isolation_system_strength_n::ChangeIsolationSystemStrengthN;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998IsolationSystemDelta, En1998IsolationSystemPatch};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &ChangeIsolationSystemStrengthN, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    if !payload.new_strength_n.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.isolation_systems.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("isolation_system {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.strength_n == payload.new_strength_n {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The isolation_system already has this value.");
    }
    protocol::MutationOutcome::new(En1998Diff { isolation_systems: En1998IsolationSystemDelta::modification(&payload.id, En1998IsolationSystemPatch { strength_n: Some(payload.new_strength_n), ..Default::default() }), ..Default::default() })
}
