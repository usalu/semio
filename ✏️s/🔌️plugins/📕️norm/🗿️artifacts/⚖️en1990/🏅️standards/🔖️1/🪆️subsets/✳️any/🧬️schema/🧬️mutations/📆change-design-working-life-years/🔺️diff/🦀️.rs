//! 🔺️ `change-design-working-life-years` sparse diff.

use crate::change_design_working_life_years::ChangeDesignWorkingLifeYears;
use semio_s_artifact_norm_en1990_diff::En1990Diff;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeDesignWorkingLifeYears, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.design_working_life_years == mutation.new_design_working_life_years {
        return MutationOutcome::empty().warning("mutation.no-op", "design_working_life_years already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        design_working_life_years: Some(mutation.new_design_working_life_years),
        ..En1990Diff::default()
    })
}
