//! Diff for `change-initial-temperature`.
use super::ChangeInitialTemperature;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeInitialTemperature, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.t_0 == payload.new_t_0 {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { t_0: Some(payload.new_t_0), ..Default::default() })
}
