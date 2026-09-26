//! Inverse for `change-assumed-silo-wall-friction`.
use super::ChangeAssumedSiloWallFriction;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAssumedSiloWallFriction, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAssumedSiloWallFriction(ChangeAssumedSiloWallFriction { new_assumed_silo_wall_friction: base.assumed_silo_wall_friction })]
}
