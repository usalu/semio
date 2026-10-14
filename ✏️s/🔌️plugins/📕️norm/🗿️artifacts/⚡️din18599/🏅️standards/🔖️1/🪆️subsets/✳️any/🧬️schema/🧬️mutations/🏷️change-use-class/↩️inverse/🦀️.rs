//! ↩️ `change-use-class` inverse.

use crate::change_use_class::ChangeUseClass;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(_payload: &ChangeUseClass, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_use_class::ChangeUseClass> {
    Ok((|| {
    vec![Op::from(ChangeUseClass { new_use_class: base.use_class })]

    })())
}
