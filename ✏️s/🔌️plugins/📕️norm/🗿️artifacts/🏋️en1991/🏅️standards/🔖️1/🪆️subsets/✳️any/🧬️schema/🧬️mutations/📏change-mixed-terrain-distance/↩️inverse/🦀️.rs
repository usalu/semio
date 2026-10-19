//! ↩️ Inverse for `change-mixed-terrain-distance`.
use crate::change_mixed_terrain_distance::ChangeMixedTerrainDistance;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeMixedTerrainDistance, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_mixed_terrain_distance::ChangeMixedTerrainDistance> {
    Ok((|| {
    vec![Op::from(ChangeMixedTerrainDistance { new_mixed_terrain_distance: base.mixed_terrain_distance })]

    })())
}
