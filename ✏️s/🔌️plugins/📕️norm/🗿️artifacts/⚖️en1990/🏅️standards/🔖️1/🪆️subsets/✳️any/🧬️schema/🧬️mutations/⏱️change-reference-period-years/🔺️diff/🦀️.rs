//! 🔺️ `change-reference-period-years` sparse diff.

use super::ChangeReferencePeriodYears;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeReferencePeriodYears, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.reference_period_years == mutation.new_reference_period_years {
        return MutationOutcome::empty().warn("mutation.no-op", "reference_period_years already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        reference_period_years: Some(mutation.new_reference_period_years),
        ..En1990Diff::default()
    })
}
