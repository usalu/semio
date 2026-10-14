//! Inverse for `change-assumed-silo-pressure`.
use crate::change_assumed_silo_pressure::ChangeAssumedSiloPressure;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedSiloPressure, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_silo_pressure::ChangeAssumedSiloPressure> {
    Ok((|| {
    vec![Op::from(ChangeAssumedSiloPressure { new_assumed_silo_pressure: base.assumed_silo_pressure })]

    })())
}
