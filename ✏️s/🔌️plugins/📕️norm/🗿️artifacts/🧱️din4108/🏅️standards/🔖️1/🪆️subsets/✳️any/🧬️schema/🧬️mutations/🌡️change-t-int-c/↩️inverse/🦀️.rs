//! ↩️ `change-t-int-c` inverse.

use crate::change_t_int_c::ChangeTIntC;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(_payload: &ChangeTIntC, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_t_int_c::ChangeTIntC> {
    Ok((|| {
    vec![Op::from(ChangeTIntC { new_t_int_c: base.t_int_c })]

    })())
}
