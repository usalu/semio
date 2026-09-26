//! ↩️ `change-effects` inverse.

use super::ChangeEffects;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeEffects, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    let _ = mutation;
    vec![En1990Mutation::ChangeEffects(ChangeEffects { new_effects: base.effects.clone() })]
}
