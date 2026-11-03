//! ↩️ `change-crane-detail-category-stress` inverse: restores the previous `detail_category_stress`.

use crate::change_crane_detail_category_stress::ChangeCraneDetailCategoryStress;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeCraneDetailCategoryStress, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_detail_category_stress::ChangeCraneDetailCategoryStress> {
    Ok(match base.crane_runways.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeCraneDetailCategoryStress { id: payload.id.clone(), new_detail_category_stress: row.detail_category_stress })],
        None => Vec::new(),
    })
}
