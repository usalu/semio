//! Diff for `change-coast-or-island`.
use super::ChangeCoastOrIsland;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeCoastOrIsland, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.coast_or_island == payload.new_coast_or_island {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { coast_or_island: Some(payload.new_coast_or_island), ..Default::default() })
}
