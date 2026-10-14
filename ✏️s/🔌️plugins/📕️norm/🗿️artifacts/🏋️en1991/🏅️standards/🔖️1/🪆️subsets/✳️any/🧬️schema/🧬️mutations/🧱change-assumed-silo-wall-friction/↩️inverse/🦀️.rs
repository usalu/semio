//! Inverse for `change-assumed-silo-wall-friction`.
use crate::change_assumed_silo_wall_friction::ChangeAssumedSiloWallFriction;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedSiloWallFriction, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_silo_wall_friction::ChangeAssumedSiloWallFriction> {
    Ok((|| {
    vec![Op::from(ChangeAssumedSiloWallFriction { new_assumed_silo_wall_friction: base.assumed_silo_wall_friction })]

    })())
}
