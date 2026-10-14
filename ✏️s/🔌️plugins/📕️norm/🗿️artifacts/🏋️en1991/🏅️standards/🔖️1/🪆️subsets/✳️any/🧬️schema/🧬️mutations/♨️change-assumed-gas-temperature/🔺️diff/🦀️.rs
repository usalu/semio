//! 🔺️ Diff for `change-assumed-gas-temperature`.
use crate::change_assumed_gas_temperature::ChangeAssumedGasTemperature;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeAssumedGasTemperature, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_gas_temperature == payload.new_assumed_gas_temperature {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_gas_temperature: Some(payload.new_assumed_gas_temperature), ..Default::default() })
}
