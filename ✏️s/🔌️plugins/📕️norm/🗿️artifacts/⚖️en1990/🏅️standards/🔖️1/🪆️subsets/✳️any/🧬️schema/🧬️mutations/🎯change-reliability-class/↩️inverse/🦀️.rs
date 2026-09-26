//! ↩️ `change-reliability-class` inverse.

use super::ChangeReliabilityClass;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeReliabilityClass, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    let _ = mutation;
    vec![En1990Mutation::ChangeReliabilityClass(ChangeReliabilityClass { new_reliability_class: base.reliability_class })]
}
