//! ↩️ `remove-fatigue-category` inverse.

use crate::remove_fatigue_category::RemoveFatigueCategory;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_fatigue_category;

pub fn inverse<Op>(payload: &RemoveFatigueCategory, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_fatigue_category::InsertFatigueCategory> {
    Ok((|| {
    if let Some((index, row)) = base.fatigue_categories.iter().enumerate().find(|(_, row)| row.id == payload.id) {
        vec![Op::from(insert_fatigue_category::InsertFatigueCategory { index: index as u32, fatigue_category: row.clone() })]
    } else { Vec::new() }

    })())
}
