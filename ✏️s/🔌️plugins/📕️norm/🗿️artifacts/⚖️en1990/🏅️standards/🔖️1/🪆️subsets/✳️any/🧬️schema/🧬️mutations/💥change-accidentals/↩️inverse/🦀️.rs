//! ↩️ `change-accidentals` inverse.

use super::ChangeAccidentals;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeAccidentals, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    let _ = mutation;
    vec![En1990Mutation::ChangeAccidentals(ChangeAccidentals { new_accidentals: base.accidentals.clone() })]
}
