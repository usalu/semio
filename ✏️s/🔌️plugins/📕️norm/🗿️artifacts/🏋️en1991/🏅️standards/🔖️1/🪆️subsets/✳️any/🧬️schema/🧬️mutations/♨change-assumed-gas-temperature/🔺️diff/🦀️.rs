//! Diff for `change-assumed-gas-temperature`.
use super::ChangeAssumedGasTemperature;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAssumedGasTemperature, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_gas_temperature == payload.new_assumed_gas_temperature {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_gas_temperature: Some(payload.new_assumed_gas_temperature), ..Default::default() })
}
