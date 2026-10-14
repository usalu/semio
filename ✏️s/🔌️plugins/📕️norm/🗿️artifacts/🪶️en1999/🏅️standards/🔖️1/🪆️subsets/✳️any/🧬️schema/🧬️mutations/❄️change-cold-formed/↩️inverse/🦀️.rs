//! ↩️ `change-cold-formed` inverse.

use crate::change_cold_formed::ChangeColdFormed;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(_payload: &ChangeColdFormed, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_cold_formed::ChangeColdFormed> {
    Ok((|| {
    vec![Op::from(ChangeColdFormed { cold_formed: base.cold_formed.clone() })]

    })())
}
