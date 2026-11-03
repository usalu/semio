//! ↩️ Inverse for `change-assumed-rail-sw0`.
use crate::change_assumed_rail_sw0::ChangeAssumedRailSw0;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedRailSw0, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_rail_sw0::ChangeAssumedRailSw0> {
    Ok((|| {
    vec![Op::from(ChangeAssumedRailSw0 { new_assumed_rail_sw0: base.assumed_rail_sw0 })]

    })())
}
