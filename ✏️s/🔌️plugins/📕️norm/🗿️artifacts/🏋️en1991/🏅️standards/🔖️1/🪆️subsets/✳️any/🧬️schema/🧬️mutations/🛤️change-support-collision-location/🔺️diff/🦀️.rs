//! 🛤 `change-support-collision-location` diff: patches `location` of the addressed fatigue location row.

use crate::change_support_collision_location::ChangeSupportCollisionLocation;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991SupportCollisionDelta, En1991SupportCollisionPatch};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &ChangeSupportCollisionLocation, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    let Some(r0) = base.support_collisions.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fatigue location {} does not exist.", payload.id), [payload.id.clone()]);
    };
    if r0.location == payload.new_location {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue location already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { support_collisions: En1991SupportCollisionDelta::modification(&payload.id, En1991SupportCollisionPatch { location: Some(payload.new_location.clone()), ..Default::default() }), ..Default::default() })
}
