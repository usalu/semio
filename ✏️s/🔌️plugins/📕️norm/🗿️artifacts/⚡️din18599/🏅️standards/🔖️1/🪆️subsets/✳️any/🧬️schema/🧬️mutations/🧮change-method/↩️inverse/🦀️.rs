//! ↩️ `change-method` inverse.

use crate::change_method::ChangeMethod;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(_payload: &ChangeMethod, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_method::ChangeMethod> {
    Ok((|| {
    vec![Op::from(ChangeMethod { new_method: base.method })]

    })())
}
