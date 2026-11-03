//! 🔺️ `change-category-element-id` diff: patches `element_id` of the addressed fatigue detail site row.

use crate::change_category_element_id::ChangeCategoryElementId;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FatigueCategoriesPatch, En1999FatigueCategoriesRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeCategoryElementId, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.fatigue_categories.iter().find(|row| row.id == payload.fatigue_category_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fatigue-category", Vec::<String>::new());
    };
    if r0.element_id == payload.new_element_id {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue detail site already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { fatigue_categories: Some(En1999FatigueCategoriesRows::modification(&payload.fatigue_category_id, En1999FatigueCategoriesPatch { element_id: Some(payload.new_element_id.clone()), ..Default::default() })), ..Default::default() })
}
