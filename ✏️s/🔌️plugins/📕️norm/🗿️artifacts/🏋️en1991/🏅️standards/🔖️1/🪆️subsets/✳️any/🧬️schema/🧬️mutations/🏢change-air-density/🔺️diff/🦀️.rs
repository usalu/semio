//! Diff for `change-air-density`.
use super::ChangeAirDensity;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAirDensity, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.air_density == payload.new_air_density {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { air_density: Some(payload.new_air_density), ..Default::default() })
}
