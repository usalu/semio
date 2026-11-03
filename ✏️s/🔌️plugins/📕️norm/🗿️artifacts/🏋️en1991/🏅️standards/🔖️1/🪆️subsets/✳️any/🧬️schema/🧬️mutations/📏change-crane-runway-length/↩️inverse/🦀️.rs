//! ↩️ Inverse for `change-crane-runway-length`.
use crate::change_crane_runway_length::ChangeCraneRunwayLength;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeCraneRunwayLength, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_crane_runway_length::ChangeCraneRunwayLength> {
    Ok((|| {
    vec![Op::from(ChangeCraneRunwayLength { new_crane_runway_length: base.crane_runway_length })]

    })())
}
