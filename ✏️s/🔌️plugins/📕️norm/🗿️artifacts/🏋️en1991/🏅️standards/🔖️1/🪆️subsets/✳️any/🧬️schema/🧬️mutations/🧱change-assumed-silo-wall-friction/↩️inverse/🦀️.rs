//! Inverse for `change-assumed-silo-wall-friction`.
use super::ChangeAssumedSiloWallFriction;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAssumedSiloWallFriction, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeAssumedSiloWallFriction(ChangeAssumedSiloWallFriction { new_assumed_silo_wall_friction: base.assumed_silo_wall_friction })]

    })())
}
