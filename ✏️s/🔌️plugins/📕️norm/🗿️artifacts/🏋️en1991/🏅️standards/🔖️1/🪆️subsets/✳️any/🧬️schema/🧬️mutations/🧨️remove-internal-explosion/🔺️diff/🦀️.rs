//! 🧨 `remove-internal-explosion` diff: removes the addressed internal explosion row.

use crate::remove_internal_explosion::RemoveInternalExplosion;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991InternalExplosionDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveInternalExplosion, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.internal_explosions.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Internal explosion {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { internal_explosions: En1991InternalExplosionDelta::removal(&base.internal_explosions, index), ..Default::default() })
}
