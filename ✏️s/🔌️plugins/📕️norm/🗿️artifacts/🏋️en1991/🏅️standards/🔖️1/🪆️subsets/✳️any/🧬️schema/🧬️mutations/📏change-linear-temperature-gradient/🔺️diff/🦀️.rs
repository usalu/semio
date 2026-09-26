//! Diff for `change-linear-temperature-gradient`.
use super::ChangeLinearTemperatureGradient;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeLinearTemperatureGradient, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.delta_t_m == payload.new_delta_t_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { delta_t_m: Some(payload.new_delta_t_m), ..Default::default() })
}
