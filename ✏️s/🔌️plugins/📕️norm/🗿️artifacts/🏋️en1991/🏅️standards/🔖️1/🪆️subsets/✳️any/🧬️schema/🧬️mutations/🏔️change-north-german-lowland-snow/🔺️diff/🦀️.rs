//! 🔺️ Diff for `change-north-german-lowland-snow`.
use crate::change_north_german_lowland_snow::ChangeNorthGermanLowlandSnow;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeNorthGermanLowlandSnow, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.north_german_lowland_snow == payload.new_north_german_lowland_snow {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { north_german_lowland_snow: Some(payload.new_north_german_lowland_snow), ..Default::default() })
}
