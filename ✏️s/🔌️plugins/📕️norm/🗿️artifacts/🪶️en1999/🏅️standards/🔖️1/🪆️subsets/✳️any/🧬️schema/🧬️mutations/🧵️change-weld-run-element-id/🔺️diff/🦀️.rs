//! 🔺️ `change-weld-run-element-id` diff: patches `element_id` of the addressed weld run row.

use crate::change_weld_run_element_id::ChangeWeldRunElementId;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999WeldRunsPatch, En1999WeldRunsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeWeldRunElementId, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.weld_runs.iter().find(|row| row.id == payload.weld_run_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "weld-run", Vec::<String>::new());
    };
    if r0.element_id == payload.new_element_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The weld run already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { weld_runs: Some(En1999WeldRunsRows::modification(&payload.weld_run_id, En1999WeldRunsPatch { element_id: Some(payload.new_element_id.clone()), ..Default::default() })), ..Default::default() })
}
