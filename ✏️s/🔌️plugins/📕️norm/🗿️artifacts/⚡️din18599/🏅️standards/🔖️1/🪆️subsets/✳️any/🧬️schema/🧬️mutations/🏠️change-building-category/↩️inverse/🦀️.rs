//! ↩️ `change-building-category` inverse.

use crate::change_building_category::ChangeBuildingCategory;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(_payload: &ChangeBuildingCategory, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_building_category::ChangeBuildingCategory> {
    Ok((|| {
    vec![Op::from(ChangeBuildingCategory { new_building_category: base.building_category })]

    })())
}
