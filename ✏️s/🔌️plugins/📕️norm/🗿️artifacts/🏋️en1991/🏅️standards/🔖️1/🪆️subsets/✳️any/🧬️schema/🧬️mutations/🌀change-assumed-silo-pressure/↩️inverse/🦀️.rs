//! Inverse for `change-assumed-silo-pressure`.
use super::ChangeAssumedSiloPressure;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAssumedSiloPressure, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![En1991Mutation::ChangeAssumedSiloPressure(ChangeAssumedSiloPressure { new_assumed_silo_pressure: base.assumed_silo_pressure })]

    })())
}
