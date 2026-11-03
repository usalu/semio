//! ➖️ `remove-roof` diff: removes the addressed roof row.

use crate::remove_roof::RemoveRoof;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991RoofDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveRoof, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.roofs.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("roof {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { roofs: En1991RoofDelta::removal(&base.roofs, index), ..Default::default() })
}
