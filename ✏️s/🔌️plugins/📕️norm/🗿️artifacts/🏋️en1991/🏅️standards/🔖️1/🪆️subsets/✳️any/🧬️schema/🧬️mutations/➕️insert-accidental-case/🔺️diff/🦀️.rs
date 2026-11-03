//! ➕️ `insert-accidental-case` diff: inserts the accidental case row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_accidental_case::InsertAccidentalCase;
use semio_s_artifact_norm_en1991_diff::{En1991AccidentalCaseDelta, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertAccidentalCase, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.accidental_cases.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("accidental case {key} already exists."), [key]);
    }
    if payload.index > base.accidental_cases.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.accidental_cases.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { accidental_cases: En1991AccidentalCaseDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
