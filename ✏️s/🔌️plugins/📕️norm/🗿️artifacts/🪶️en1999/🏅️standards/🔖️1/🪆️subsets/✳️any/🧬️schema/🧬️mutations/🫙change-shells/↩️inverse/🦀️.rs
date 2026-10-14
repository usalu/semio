//! ↩️ `change-shells` inverse.

use crate::change_shells::ChangeShells;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(_payload: &ChangeShells, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_shells::ChangeShells> {
    Ok((|| {
    vec![Op::from(ChangeShells { shells: base.shells.clone() })]

    })())
}
