//! ⚙ `remove-local-fire` diff: removes the addressed localised fire row.

use crate::remove_local_fire::RemoveLocalFire;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991LocalFireDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveLocalFire, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.local_fires.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Localised fire {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { local_fires: En1991LocalFireDelta::removal(&base.local_fires, index), ..Default::default() })
}
