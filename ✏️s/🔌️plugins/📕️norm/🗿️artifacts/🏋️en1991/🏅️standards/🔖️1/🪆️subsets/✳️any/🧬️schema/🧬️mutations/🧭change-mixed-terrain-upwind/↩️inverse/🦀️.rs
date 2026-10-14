//! Inverse for `change-mixed-terrain-upwind`.
use crate::change_mixed_terrain_upwind::ChangeMixedTerrainUpwind;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeMixedTerrainUpwind, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_mixed_terrain_upwind::ChangeMixedTerrainUpwind> {
    Ok((|| {
    vec![Op::from(ChangeMixedTerrainUpwind { new_mixed_terrain_upwind: base.mixed_terrain_upwind })]

    })())
}
