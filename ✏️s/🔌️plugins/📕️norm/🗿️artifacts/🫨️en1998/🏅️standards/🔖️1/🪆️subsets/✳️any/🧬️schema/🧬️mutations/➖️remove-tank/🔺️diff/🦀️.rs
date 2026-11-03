//! ➖️ `remove-tank` diff: removes the addressed tank row.

use crate::remove_tank::RemoveTank;
use semio_s_artifact_norm_en1998_diff::{En1998Diff, En1998TankDelta};
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn diff(payload: &RemoveTank, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
    let Some(index) = base.tanks.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("tank {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1998Diff { tanks: En1998TankDelta::removal(&base.tanks, index), ..Default::default() })
}
