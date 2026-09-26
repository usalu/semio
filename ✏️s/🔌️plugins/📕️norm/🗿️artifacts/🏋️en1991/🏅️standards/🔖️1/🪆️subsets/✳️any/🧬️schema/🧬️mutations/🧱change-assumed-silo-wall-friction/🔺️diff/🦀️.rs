//! Diff for `change-assumed-silo-wall-friction`.
use super::ChangeAssumedSiloWallFriction;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAssumedSiloWallFriction, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_silo_wall_friction == payload.new_assumed_silo_wall_friction {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_silo_wall_friction: Some(payload.new_assumed_silo_wall_friction), ..Default::default() })
}
