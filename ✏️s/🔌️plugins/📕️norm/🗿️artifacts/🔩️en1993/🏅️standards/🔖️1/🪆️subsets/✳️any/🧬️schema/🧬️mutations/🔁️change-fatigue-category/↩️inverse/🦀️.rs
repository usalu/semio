//! ↩️ `change-fatigue-category` inverse: restores the previous `category`.

use crate::change_fatigue_category::ChangeFatigueCategory;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeFatigueCategory, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fatigue_category::ChangeFatigueCategory> {
    Ok(match base.fatigue_details.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeFatigueCategory { id: payload.id.clone(), new_category: row.category })],
        None => Vec::new(),
    })
}
