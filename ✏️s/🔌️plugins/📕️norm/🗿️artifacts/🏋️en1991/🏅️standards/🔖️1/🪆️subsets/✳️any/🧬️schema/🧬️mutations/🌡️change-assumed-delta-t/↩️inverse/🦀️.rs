//! ↩️ Inverse for `change-assumed-delta-t`.
use crate::change_assumed_delta_t::ChangeAssumedDeltaT;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedDeltaT, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_delta_t::ChangeAssumedDeltaT> {
    Ok((|| {
    vec![Op::from(ChangeAssumedDeltaT { new_assumed_delta_t: base.assumed_delta_t })]

    })())
}
