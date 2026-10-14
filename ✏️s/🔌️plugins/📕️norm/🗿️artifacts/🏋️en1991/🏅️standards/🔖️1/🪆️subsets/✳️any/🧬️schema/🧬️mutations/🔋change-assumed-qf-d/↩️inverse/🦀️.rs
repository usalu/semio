//! Inverse for `change-assumed-qf-d`.
use crate::change_assumed_qf_d::ChangeAssumedQfD;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedQfD, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_qf_d::ChangeAssumedQfD> {
    Ok((|| {
    vec![Op::from(ChangeAssumedQfD { new_assumed_qf_d: base.assumed_qf_d })]

    })())
}
