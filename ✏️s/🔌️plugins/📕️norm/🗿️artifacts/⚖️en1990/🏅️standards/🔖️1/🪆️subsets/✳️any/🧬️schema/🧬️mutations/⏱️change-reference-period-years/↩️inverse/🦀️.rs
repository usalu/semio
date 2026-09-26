//! ↩️ `change-reference-period-years` inverse.

use super::ChangeReferencePeriodYears;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeReferencePeriodYears, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    let _ = mutation;
    vec![En1990Mutation::ChangeReferencePeriodYears(ChangeReferencePeriodYears { new_reference_period_years: base.reference_period_years })]
}
