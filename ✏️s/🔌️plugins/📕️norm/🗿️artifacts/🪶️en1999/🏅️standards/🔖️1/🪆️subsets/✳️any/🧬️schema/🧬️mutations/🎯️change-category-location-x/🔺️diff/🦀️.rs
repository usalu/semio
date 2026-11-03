//! 🔺️ `change-category-location-x` diff: patches `location_x` of the addressed fatigue detail site row.

use crate::change_category_location_x::ChangeCategoryLocationX;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FatigueCategoriesPatch, En1999FatigueCategoriesRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeCategoryLocationX, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_location_x.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.fatigue_categories.iter().find(|row| row.id == payload.fatigue_category_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fatigue-category", Vec::<String>::new());
    };
    if r0.location_x == payload.new_location_x {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue detail site already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { fatigue_categories: Some(En1999FatigueCategoriesRows::modification(&payload.fatigue_category_id, En1999FatigueCategoriesPatch { location_x: Some(payload.new_location_x), ..Default::default() })), ..Default::default() })
}
