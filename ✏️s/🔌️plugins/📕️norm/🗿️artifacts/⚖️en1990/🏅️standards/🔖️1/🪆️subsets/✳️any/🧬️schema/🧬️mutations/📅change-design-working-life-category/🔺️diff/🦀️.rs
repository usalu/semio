//! 🔺️ `change-design-working-life-category` sparse diff.

use crate::change_design_working_life_category::ChangeDesignWorkingLifeCategory;
use semio_s_artifact_norm_en1990_diff::En1990Diff;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeDesignWorkingLifeCategory, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.design_working_life_category == mutation.new_design_working_life_category {
        return MutationOutcome::empty().warning("mutation.no-op", "design_working_life_category already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        design_working_life_category: Some(mutation.new_design_working_life_category),
        ..En1990Diff::default()
    })
}
