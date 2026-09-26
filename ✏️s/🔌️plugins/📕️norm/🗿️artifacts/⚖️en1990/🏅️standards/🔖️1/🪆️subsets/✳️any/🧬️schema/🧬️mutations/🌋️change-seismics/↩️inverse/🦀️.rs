//! ↩️ `change-seismics` inverse.

use super::ChangeSeismics;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeSeismics, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    let _ = mutation;
    vec![En1990Mutation::ChangeSeismics(ChangeSeismics { new_seismics: base.seismics.clone() })]
}
