//! 🚁 `remove-helicopter` diff: removes the addressed helicopter landing row.

use crate::remove_helicopter::RemoveHelicopter;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991HelicopterLandingDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveHelicopter, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.helicopter_landings.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Helicopter landing {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { helicopter_landings: En1991HelicopterLandingDelta::removal(&base.helicopter_landings, index), ..Default::default() })
}
