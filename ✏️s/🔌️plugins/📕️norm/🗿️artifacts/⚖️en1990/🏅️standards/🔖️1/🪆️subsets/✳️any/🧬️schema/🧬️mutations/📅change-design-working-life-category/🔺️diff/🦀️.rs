//! 🔺️ `change-design-working-life-category` sparse diff.

use super::ChangeDesignWorkingLifeCategory;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeDesignWorkingLifeCategory, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.design_working_life_category == mutation.new_design_working_life_category {
        return MutationOutcome::empty().warn("mutation.no-op", "design_working_life_category already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        design_working_life_category: Some(mutation.new_design_working_life_category),
        ..En1990Diff::default()
    })
}
