//! 🔺️ `remove-fatigue-category` diff.

use crate::remove_fatigue_category::RemoveFatigueCategory;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FatigueCategoriesRows};

pub fn diff(payload: &RemoveFatigueCategory, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(index) = base.fatigue_categories.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown fatigue detail site id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { fatigue_categories: Some(En1999FatigueCategoriesRows::removal(&base.fatigue_categories, index)), ..Default::default() })
}
