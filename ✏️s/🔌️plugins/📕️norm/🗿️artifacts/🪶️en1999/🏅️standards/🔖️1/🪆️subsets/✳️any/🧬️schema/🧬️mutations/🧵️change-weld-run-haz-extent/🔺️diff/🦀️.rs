//! 🔺️ `change-weld-run-haz-extent` diff: patches `haz_extent` of the addressed weld run row.

use crate::change_weld_run_haz_extent::ChangeWeldRunHazExtent;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999WeldRunsPatch, En1999WeldRunsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeWeldRunHazExtent, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_haz_extent.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.weld_runs.iter().find(|row| row.id == payload.weld_run_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "weld-run", Vec::<String>::new());
    };
    if r0.haz_extent == payload.new_haz_extent {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The weld run already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { weld_runs: Some(En1999WeldRunsRows::modification(&payload.weld_run_id, En1999WeldRunsPatch { haz_extent: Some(payload.new_haz_extent), ..Default::default() })), ..Default::default() })
}
