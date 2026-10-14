//! 🔺️ Diff for `change-coast-or-island`.
use crate::change_coast_or_island::ChangeCoastOrIsland;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeCoastOrIsland, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.coast_or_island == payload.new_coast_or_island {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { coast_or_island: Some(payload.new_coast_or_island), ..Default::default() })
}
