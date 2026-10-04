//! ↩️ `change-beta-computed` inverse.

use super::ChangeBetaComputed;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangeBetaComputed, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let _ = mutation;
    vec![En1990Mutation::ChangeBetaComputed(ChangeBetaComputed { new_beta_computed: base.beta_computed })]

    })())
}
