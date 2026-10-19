//! ↩️ Inverse for `change-assumed-gas-temperature`.
use crate::change_assumed_gas_temperature::ChangeAssumedGasTemperature;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn inverse<Op>(_payload: &ChangeAssumedGasTemperature, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assumed_gas_temperature::ChangeAssumedGasTemperature> {
    Ok((|| {
    vec![Op::from(ChangeAssumedGasTemperature { new_assumed_gas_temperature: base.assumed_gas_temperature })]

    })())
}
