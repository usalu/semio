//! ➖️ `remove-silo` diff: removes the addressed silo row.

use crate::remove_silo::RemoveSilo;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998SiloDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveSilo, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.silos.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("silo {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { silos: En1998SiloDelta::removal(&base.silos, index), ..Default::default() })
}
