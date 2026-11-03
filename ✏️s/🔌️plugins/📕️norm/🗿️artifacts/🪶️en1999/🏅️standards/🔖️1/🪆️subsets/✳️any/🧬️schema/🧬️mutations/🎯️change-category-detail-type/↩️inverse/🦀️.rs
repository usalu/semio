//! ↩️ `change-category-detail-type` inverse: restores the previous `detail_type`.

use crate::change_category_detail_type::ChangeCategoryDetailType;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeCategoryDetailType, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_category_detail_type::ChangeCategoryDetailType> {
    Ok(match base.fatigue_categories.iter().find(|row| row.id == payload.fatigue_category_id) {
        Some(row) => vec![Op::from(ChangeCategoryDetailType { fatigue_category_id: payload.fatigue_category_id.clone(), new_detail_type: row.detail_type })],
        None => Vec::new(),
    })
}
