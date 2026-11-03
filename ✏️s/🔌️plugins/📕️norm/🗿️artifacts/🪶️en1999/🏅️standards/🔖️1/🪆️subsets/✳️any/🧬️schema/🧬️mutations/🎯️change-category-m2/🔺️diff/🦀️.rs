//! 🔺️ `change-category-m2` diff: patches `m2` of the addressed fatigue detail site row.

use crate::change_category_m2::ChangeCategoryM2;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FatigueCategoriesPatch, En1999FatigueCategoriesRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeCategoryM2, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.fatigue_categories.iter().find(|row| row.id == payload.fatigue_category_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fatigue-category", Vec::<String>::new());
    };
    if r0.m2 == payload.new_m2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue detail site already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { fatigue_categories: Some(En1999FatigueCategoriesRows::modification(&payload.fatigue_category_id, En1999FatigueCategoriesPatch { m2: Some(payload.new_m2), ..Default::default() })), ..Default::default() })
}
