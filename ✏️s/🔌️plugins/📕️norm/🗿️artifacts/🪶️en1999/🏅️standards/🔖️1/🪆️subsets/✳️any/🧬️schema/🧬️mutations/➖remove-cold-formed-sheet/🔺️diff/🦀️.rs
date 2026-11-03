//! 🔺️ `remove-cold-formed-sheet` diff.

use crate::remove_cold_formed_sheet::RemoveColdFormedSheet;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999ColdFormedRows};

pub fn diff(payload: &RemoveColdFormedSheet, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(index) = base.cold_formed.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown cold-formed sheet id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { cold_formed: Some(En1999ColdFormedRows::removal(&base.cold_formed, index)), ..Default::default() })
}
