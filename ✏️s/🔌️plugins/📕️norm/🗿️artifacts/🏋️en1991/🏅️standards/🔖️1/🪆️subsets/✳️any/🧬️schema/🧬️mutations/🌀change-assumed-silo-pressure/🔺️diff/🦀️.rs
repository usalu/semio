//! Diff for `change-assumed-silo-pressure`.
use super::ChangeAssumedSiloPressure;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeAssumedSiloPressure, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.assumed_silo_pressure == payload.new_assumed_silo_pressure {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { assumed_silo_pressure: Some(payload.new_assumed_silo_pressure), ..Default::default() })
}
