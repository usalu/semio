//! ↩️ `change-beta-computed` inverse.

use crate::change_beta_computed::ChangeBetaComputed;

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeBetaComputed, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_beta_computed::ChangeBetaComputed> {
    Ok((|| {
    let _ = mutation;
    vec![Op::from(ChangeBetaComputed { new_beta_computed: base.beta_computed })]

    })())
}
