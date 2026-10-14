//! ↩️ `change-delta-u-wb` inverse.

use crate::change_delta_u_wb::ChangeDeltaUWb;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(_payload: &ChangeDeltaUWb, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_delta_u_wb::ChangeDeltaUWb> {
    Ok((|| {
    vec![Op::from(ChangeDeltaUWb { new_delta_u_wb_w_m2k: base.delta_u_wb_w_m2k })]

    })())
}
