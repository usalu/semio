//! 🪚️ `remove-chase` diff: removes the addressed chase row.

use crate::remove_chase::RemoveChase;
use semio_s_artifact_norm_en1996_diff::{En1996ChaseDelta, En1996Diff};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn diff(payload: &RemoveChase, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    let Some(index) = base.chases.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Chase {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1996Diff { chases: En1996ChaseDelta::removal(&base.chases, index), ..Default::default() })
}
