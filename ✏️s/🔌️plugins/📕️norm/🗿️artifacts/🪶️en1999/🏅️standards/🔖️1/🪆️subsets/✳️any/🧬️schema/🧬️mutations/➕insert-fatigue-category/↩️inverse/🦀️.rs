//! ↩️ `insert-fatigue-category` inverse.

use crate::insert_fatigue_category::InsertFatigueCategory;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_fatigue_category;

pub fn inverse<Op>(payload: &InsertFatigueCategory, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_fatigue_category::RemoveFatigueCategory> {
    Ok((|| {
    vec![Op::from(remove_fatigue_category::RemoveFatigueCategory { id: payload.fatigue_category.id.clone() })]

    })())
}
