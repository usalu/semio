//! ➖️ `remove-foundation` diff: removes the addressed foundation row.

use crate::remove_foundation::RemoveFoundation;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998FoundationDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveFoundation, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.foundations.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("foundation {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { foundations: En1998FoundationDelta::removal(&base.foundations, index), ..Default::default() })
}
