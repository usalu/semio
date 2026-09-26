//! ↩️ `change-beta-computed` inverse.

use super::ChangeBetaComputed;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeBetaComputed, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    let _ = mutation;
    vec![En1990Mutation::ChangeBetaComputed(ChangeBetaComputed { new_beta_computed: base.beta_computed })]
}
