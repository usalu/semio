//! 🔗 `change-tie-run-provided-force` diff: patches `provided_force` of the addressed tie run row.

use crate::change_tie_run_provided_force::ChangeTieRunProvidedForce;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991TieRunDelta, En1991TieRunPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeTieRunProvidedForce, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_provided_force.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.tie_runs.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tie run {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.provided_force == payload.new_provided_force {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tie run already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { tie_runs: En1991TieRunDelta::modification(&payload.id, En1991TieRunPatch { provided_force: Some(payload.new_provided_force), ..Default::default() }), ..Default::default() })
}
