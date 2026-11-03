//! 📐 `insert-imperfection` diff: inserts the execution imperfection row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_imperfection::InsertImperfection;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991ExecutionImperfectionDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertImperfection, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.execution_imperfections.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Execution imperfection {key} already exists."), [key]);
    }
    if payload.index > base.execution_imperfections.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.execution_imperfections.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { execution_imperfections: En1991ExecutionImperfectionDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
