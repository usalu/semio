//! 🔺️ `change-outdoor-co2` diff.
use super::ChangeOutdoorCo2;
use crate::{Din16798Diff, Din16798Snapshot};
pub fn diff(payload: &ChangeOutdoorCo2, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.outdoor_co2_ppm == payload.new_outdoor_co2_ppm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "unchanged");
    }
    protocol::MutationOutcome::new(Din16798Diff { outdoor_co2_ppm: Some(payload.new_outdoor_co2_ppm), ..Default::default() })
}
