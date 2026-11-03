//! ➖️ `remove-accidental-case` diff: removes the addressed accidental case row.

use crate::remove_accidental_case::RemoveAccidentalCase;
use semio_s_artifact_norm_en1991_diff::{En1991AccidentalCaseDelta, En1991Diff};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveAccidentalCase, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.accidental_cases.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("accidental case {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { accidental_cases: En1991AccidentalCaseDelta::removal(&base.accidental_cases, index), ..Default::default() })
}
