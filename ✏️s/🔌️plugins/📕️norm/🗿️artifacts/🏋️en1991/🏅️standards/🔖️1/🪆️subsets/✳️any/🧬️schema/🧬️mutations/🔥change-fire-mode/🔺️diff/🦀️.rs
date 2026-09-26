//! Diff for `change-fire-mode`.
use super::ChangeFireMode;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeFireMode, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fire_mode == payload.new_fire_mode {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_mode: Some(payload.new_fire_mode), ..Default::default() })
}
