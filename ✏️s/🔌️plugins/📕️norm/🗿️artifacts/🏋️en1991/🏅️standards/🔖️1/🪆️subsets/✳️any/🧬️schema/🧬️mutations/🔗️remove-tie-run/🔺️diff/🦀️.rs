//! 🔗 `remove-tie-run` diff: removes the addressed tie run row.

use crate::remove_tie_run::RemoveTieRun;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991TieRunDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveTieRun, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.tie_runs.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tie run {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { tie_runs: En1991TieRunDelta::removal(&base.tie_runs, index), ..Default::default() })
}
