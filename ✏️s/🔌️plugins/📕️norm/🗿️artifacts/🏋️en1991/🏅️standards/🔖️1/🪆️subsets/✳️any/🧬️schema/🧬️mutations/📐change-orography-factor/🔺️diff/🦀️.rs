//! 🔺️ Diff for `change-orography-factor`.
use crate::change_orography_factor::ChangeOrographyFactor;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeOrographyFactor, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.orography_factor == payload.new_orography_factor {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { orography_factor: Some(payload.new_orography_factor), ..Default::default() })
}
