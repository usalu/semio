//! ↩️ Inverse for `change-assumed-rail-sw2`.
use crate::change_assumed_rail_sw2::ChangeAssumedRailSw2;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedRailSw2, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_rail_sw2::ChangeAssumedRailSw2> {
    Ok((|| {
    vec![Op::from(ChangeAssumedRailSw2 { new_assumed_rail_sw2: base.assumed_rail_sw2 })]

    })())
}
