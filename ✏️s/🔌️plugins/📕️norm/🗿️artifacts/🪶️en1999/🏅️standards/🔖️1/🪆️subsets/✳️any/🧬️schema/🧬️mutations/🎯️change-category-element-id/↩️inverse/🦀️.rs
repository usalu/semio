//! ↩️ `change-category-element-id` inverse: restores the previous `element_id`.

use crate::change_category_element_id::ChangeCategoryElementId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeCategoryElementId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_category_element_id::ChangeCategoryElementId> {
    Ok(match base.fatigue_categories.iter().find(|row| row.id == payload.fatigue_category_id) {
        Some(row) => vec![Op::from(ChangeCategoryElementId { fatigue_category_id: payload.fatigue_category_id.clone(), new_element_id: row.element_id.clone() })],
        None => Vec::new(),
    })
}
