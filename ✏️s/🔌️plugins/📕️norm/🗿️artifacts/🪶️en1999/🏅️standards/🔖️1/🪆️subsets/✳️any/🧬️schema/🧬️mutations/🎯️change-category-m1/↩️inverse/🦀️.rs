//! ↩️ `change-category-m1` inverse: restores the previous `m1`.

use crate::change_category_m1::ChangeCategoryM1;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeCategoryM1, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_category_m1::ChangeCategoryM1> {
    Ok(match base.fatigue_categories.iter().find(|row| row.id == payload.fatigue_category_id) {
        Some(row) => vec![Op::from(ChangeCategoryM1 { fatigue_category_id: payload.fatigue_category_id.clone(), new_m1: row.m1 })],
        None => Vec::new(),
    })
}
