//! Inverse for `change-terrain-category`.
use crate::change_terrain_category::ChangeTerrainCategory;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeTerrainCategory, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_terrain_category::ChangeTerrainCategory> {
    Ok((|| {
    vec![Op::from(ChangeTerrainCategory { new_terrain_category: base.terrain_category })]

    })())
}
