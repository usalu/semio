//! 🔺️ `change-category-detail-type` diff: patches `detail_type` of the addressed fatigue detail site row.

use crate::change_category_detail_type::ChangeCategoryDetailType;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FatigueCategoriesPatch, En1999FatigueCategoriesRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeCategoryDetailType, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(r0) = base.fatigue_categories.iter().find(|row| row.id == payload.fatigue_category_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fatigue-category", Vec::<String>::new());
    };
    if r0.detail_type == payload.new_detail_type {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue detail site already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { fatigue_categories: Some(En1999FatigueCategoriesRows::modification(&payload.fatigue_category_id, En1999FatigueCategoriesPatch { detail_type: Some(payload.new_detail_type), ..Default::default() })), ..Default::default() })
}
