//! ↩️ `change-rh-int` inverse.

use crate::change_rh_int::ChangeRhInt;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(_payload: &ChangeRhInt, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rh_int::ChangeRhInt> {
    Ok((|| {
    vec![Op::from(ChangeRhInt { new_rh_int: base.rh_int })]

    })())
}
