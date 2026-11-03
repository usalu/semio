//! 🔗 `change-tie-run-imposed-load` diff: patches `imposed_load` of the addressed tie run row.

use crate::change_tie_run_imposed_load::ChangeTieRunImposedLoad;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991TieRunDelta, En1991TieRunPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeTieRunImposedLoad, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_imposed_load.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.tie_runs.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tie run {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.imposed_load == payload.new_imposed_load {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tie run already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { tie_runs: En1991TieRunDelta::modification(&payload.id, En1991TieRunPatch { imposed_load: Some(payload.new_imposed_load), ..Default::default() }), ..Default::default() })
}
