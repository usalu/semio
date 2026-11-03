//! ↩️ `change-category-location-x` inverse: restores the previous `location_x`.

use crate::change_category_location_x::ChangeCategoryLocationX;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeCategoryLocationX, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_category_location_x::ChangeCategoryLocationX> {
    Ok(match base.fatigue_categories.iter().find(|row| row.id == payload.fatigue_category_id) {
        Some(row) => vec![Op::from(ChangeCategoryLocationX { fatigue_category_id: payload.fatigue_category_id.clone(), new_location_x: row.location_x })],
        None => Vec::new(),
    })
}
