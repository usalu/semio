//! Inverse for `change-assumed-construction-qk`.
use crate::change_assumed_construction_qk::ChangeAssumedConstructionQk;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedConstructionQk, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_construction_qk::ChangeAssumedConstructionQk> {
    Ok((|| {
    vec![Op::from(ChangeAssumedConstructionQk { new_assumed_construction_qk: base.assumed_construction_qk })]

    })())
}
