//! ↩️ `change-fatigue-detail-category` inverse: restores the previous `detail_category`.

use crate::change_fatigue_detail_category::ChangeFatigueDetailCategory;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueDetailCategory, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_detail_category::ChangeFatigueDetailCategory> {
    Ok(match base.fatigue_details.iter().find(|row| row.id == payload.fatigue_detail_id) {
        Some(row) => vec![Op::from(ChangeFatigueDetailCategory { fatigue_detail_id: payload.fatigue_detail_id.clone(), new_detail_category: row.detail_category.clone() })],
        None => Vec::new(),
    })
}
