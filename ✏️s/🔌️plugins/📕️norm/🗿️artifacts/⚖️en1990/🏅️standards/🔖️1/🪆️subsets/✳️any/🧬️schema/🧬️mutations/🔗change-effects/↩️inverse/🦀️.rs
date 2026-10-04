//! ↩️ `change-effects` inverse.

use super::ChangeEffects;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeEffects, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let _ = mutation;
    vec![En1990Mutation::ChangeEffects(ChangeEffects { new_effects: base.effects.clone() })]

    })())
}
