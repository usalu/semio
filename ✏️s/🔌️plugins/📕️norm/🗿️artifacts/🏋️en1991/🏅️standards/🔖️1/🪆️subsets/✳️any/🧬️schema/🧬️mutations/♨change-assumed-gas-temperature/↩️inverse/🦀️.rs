//! Inverse for `change-assumed-gas-temperature`.
use super::ChangeAssumedGasTemperature;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(_payload: &ChangeAssumedGasTemperature, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeAssumedGasTemperature(ChangeAssumedGasTemperature { new_assumed_gas_temperature: base.assumed_gas_temperature })]
}
