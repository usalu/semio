//! ↩️ `change-category-detail-category` inverse: restores the previous `detail_category`.

use crate::change_category_detail_category::ChangeCategoryDetailCategory;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeCategoryDetailCategory, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_category_detail_category::ChangeCategoryDetailCategory> {
    Ok(match base.fatigue_categories.iter().find(|row| row.id == payload.fatigue_category_id) {
        Some(row) => vec![Op::from(ChangeCategoryDetailCategory { fatigue_category_id: payload.fatigue_category_id.clone(), new_detail_category: row.detail_category.clone() })],
        None => Vec::new(),
    })
}
