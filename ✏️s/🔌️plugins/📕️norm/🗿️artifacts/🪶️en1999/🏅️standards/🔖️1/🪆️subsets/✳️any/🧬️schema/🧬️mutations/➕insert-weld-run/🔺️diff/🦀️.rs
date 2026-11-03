//! 🔺️ `insert-weld-run` diff.

use crate::insert_weld_run::InsertWeldRun;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999WeldRunsRows};

pub fn diff(payload: &InsertWeldRun, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.weld_runs.iter().any(|existing| existing.id == payload.weld_run.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Weld run id {} already exists.", payload.weld_run.id), [payload.weld_run.id.clone()]);
    }
    if (payload.index as usize) > base.weld_runs.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.weld_runs.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { weld_runs: Some(En1999WeldRunsRows::insertion(payload.index as usize, payload.weld_run.clone())), ..Default::default() })
}
