//! ⚙ `change-support-collision-assumed-force-x` diff: patches `assumed_force_x` of the addressed support collision row.

use crate::change_support_collision_assumed_force_x::ChangeSupportCollisionAssumedForceX;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991SupportCollisionDelta, En1991SupportCollisionPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeSupportCollisionAssumedForceX, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if !payload.new_assumed_force_x.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.support_collisions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Support collision {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.assumed_force_x == payload.new_assumed_force_x {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The support collision already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { support_collisions: En1991SupportCollisionDelta::modification(&payload.id, En1991SupportCollisionPatch { assumed_force_x: Some(payload.new_assumed_force_x), ..Default::default() }), ..Default::default() })
}
