//! ↩️ `change-category-m2` inverse: restores the previous `m2`.

use crate::change_category_m2::ChangeCategoryM2;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeCategoryM2, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_category_m2::ChangeCategoryM2> {
    Ok(match base.fatigue_categories.iter().find(|row| row.id == payload.fatigue_category_id) {
        Some(row) => vec![Op::from(ChangeCategoryM2 { fatigue_category_id: payload.fatigue_category_id.clone(), new_m2: row.m2 })],
        None => Vec::new(),
    })
}
