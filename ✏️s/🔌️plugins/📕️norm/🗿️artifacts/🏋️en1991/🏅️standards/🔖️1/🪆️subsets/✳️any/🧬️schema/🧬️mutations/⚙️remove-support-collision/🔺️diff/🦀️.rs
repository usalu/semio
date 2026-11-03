//! ⚙ `remove-support-collision` diff: removes the addressed support collision row.

use crate::remove_support_collision::RemoveSupportCollision;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991SupportCollisionDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &RemoveSupportCollision, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(index) = base.support_collisions.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Support collision {} does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(En1991Diff { support_collisions: En1991SupportCollisionDelta::removal(&base.support_collisions, index), ..Default::default() })
}
