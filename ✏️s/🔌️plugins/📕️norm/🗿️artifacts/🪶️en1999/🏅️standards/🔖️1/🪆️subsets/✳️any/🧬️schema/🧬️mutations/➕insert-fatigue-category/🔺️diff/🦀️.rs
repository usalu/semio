//! 🔺️ `insert-fatigue-category` diff.

use crate::insert_fatigue_category::InsertFatigueCategory;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FatigueCategoriesRows};

pub fn diff(payload: &InsertFatigueCategory, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if base.fatigue_categories.iter().any(|existing| existing.id == payload.fatigue_category.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Fatigue detail site id {} already exists.", payload.fatigue_category.id), [payload.fatigue_category.id.clone()]);
    }
    if (payload.index as usize) > base.fatigue_categories.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", (payload.index as usize), base.fatigue_categories.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1999Diff { fatigue_categories: Some(En1999FatigueCategoriesRows::insertion(payload.index as usize, payload.fatigue_category.clone())), ..Default::default() })
}
