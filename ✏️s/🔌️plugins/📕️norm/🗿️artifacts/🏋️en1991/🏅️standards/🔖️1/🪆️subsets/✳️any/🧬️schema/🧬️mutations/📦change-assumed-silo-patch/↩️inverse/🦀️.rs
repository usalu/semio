//! Inverse for `change-assumed-silo-patch`.
use crate::change_assumed_silo_patch::ChangeAssumedSiloPatch;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedSiloPatch, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_silo_patch::ChangeAssumedSiloPatch> {
    Ok((|| {
    vec![Op::from(ChangeAssumedSiloPatch { new_assumed_silo_patch: base.assumed_silo_patch })]

    })())
}
