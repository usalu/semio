//! 🔺️ `change-design-working-life-years` sparse diff.

use super::ChangeDesignWorkingLifeYears;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeDesignWorkingLifeYears, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.design_working_life_years == mutation.new_design_working_life_years {
        return MutationOutcome::empty().warn("mutation.no-op", "design_working_life_years already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        design_working_life_years: Some(mutation.new_design_working_life_years),
        ..En1990Diff::default()
    })
}
