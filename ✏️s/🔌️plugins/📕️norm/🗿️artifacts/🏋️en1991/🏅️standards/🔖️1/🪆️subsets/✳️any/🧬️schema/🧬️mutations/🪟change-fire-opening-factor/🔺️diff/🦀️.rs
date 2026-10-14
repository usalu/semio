//! 🔺️ Diff for `change-fire-opening-factor`.
use crate::change_fire_opening_factor::ChangeFireOpeningFactor;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeFireOpeningFactor, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.fire_opening_factor == payload.new_fire_opening_factor {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { fire_opening_factor: Some(payload.new_fire_opening_factor), ..Default::default() })
}
