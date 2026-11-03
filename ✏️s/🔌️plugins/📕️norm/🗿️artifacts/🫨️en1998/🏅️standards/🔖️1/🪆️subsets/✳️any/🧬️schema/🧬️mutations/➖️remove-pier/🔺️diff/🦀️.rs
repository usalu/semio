//! ➖️ `remove-pier` diff: removes the addressed pier row.

use crate::remove_pier::RemovePier;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998PierDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemovePier, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.piers.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pier {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { piers: En1998PierDelta::removal(&base.piers, index), ..Default::default() })
}
