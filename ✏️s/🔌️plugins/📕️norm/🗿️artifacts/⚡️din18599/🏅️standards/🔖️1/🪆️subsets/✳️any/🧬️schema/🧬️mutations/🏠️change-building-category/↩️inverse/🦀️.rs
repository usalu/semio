//! ↩️ `change-building-category` inverse.

use crate::mutations::change_building_category::ChangeBuildingCategory;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &ChangeBuildingCategory, base: &Din18599Snapshot) -> Result<Vec<Din18599Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![Din18599Mutation::ChangeBuildingCategory(ChangeBuildingCategory { new_building_category: base.building_category })]

    })())
}
