//! 🔺️ `change-night-setback` diff.
use crate::change_night_setback::ChangeNightSetback;
use semio_s_artifact_norm_din16798_diff::Din16798Diff;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn diff(payload: &ChangeNightSetback, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.night_setback_k == payload.new_night_setback_k {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(Din16798Diff { night_setback_k: Some(payload.new_night_setback_k), ..Default::default() })
}
