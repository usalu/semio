//! 🔺️ `change-outdoor-co2` diff.
use crate::change_outdoor_co2::ChangeOutdoorCo2;
use semio_s_artifact_norm_din16798_diff::Din16798Diff;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn diff(payload: &ChangeOutdoorCo2, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.outdoor_co2_ppm == payload.new_outdoor_co2_ppm {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(Din16798Diff { outdoor_co2_ppm: Some(payload.new_outdoor_co2_ppm), ..Default::default() })
}
