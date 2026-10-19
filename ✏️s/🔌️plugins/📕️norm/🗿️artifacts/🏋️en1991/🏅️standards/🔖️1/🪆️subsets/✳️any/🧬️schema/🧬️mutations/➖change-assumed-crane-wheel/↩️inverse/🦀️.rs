//! ↩️ Inverse for `change-assumed-crane-wheel`.
use crate::change_assumed_crane_wheel::ChangeAssumedCraneWheel;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedCraneWheel, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_crane_wheel::ChangeAssumedCraneWheel> {
    Ok((|| {
    vec![Op::from(ChangeAssumedCraneWheel { new_assumed_crane_wheel: base.assumed_crane_wheel })]

    })())
}
