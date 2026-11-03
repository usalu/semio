//! 🔺️ `remove-weld-run` diff.

use crate::remove_weld_run::RemoveWeldRun;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999WeldRunsRows};

pub fn diff(payload: &RemoveWeldRun, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(index) = base.weld_runs.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown weld run id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { weld_runs: Some(En1999WeldRunsRows::removal(&base.weld_runs, index)), ..Default::default() })
}
