//! Inverse for `change-assumed-crane-horizontal`.
use crate::change_assumed_crane_horizontal::ChangeAssumedCraneHorizontal;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedCraneHorizontal, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_crane_horizontal::ChangeAssumedCraneHorizontal> {
    Ok((|| {
    vec![Op::from(ChangeAssumedCraneHorizontal { new_assumed_crane_horizontal: base.assumed_crane_horizontal })]

    })())
}
