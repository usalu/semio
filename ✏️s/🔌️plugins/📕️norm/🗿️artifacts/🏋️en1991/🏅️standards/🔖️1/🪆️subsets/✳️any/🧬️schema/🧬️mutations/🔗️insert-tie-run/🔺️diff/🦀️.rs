//! 🔗 `insert-tie-run` diff: inserts the tie run row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_tie_run::InsertTieRun;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991TieRunDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertTieRun, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.tie_runs.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Tie run {key} already exists."), [key]);
    }
    if payload.index > base.tie_runs.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.tie_runs.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { tie_runs: En1991TieRunDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
