//! 🔺️ `change-altitude-m` sparse diff.

use super::ChangeAltitudeM;
use crate::diff::En1990Diff;
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeAltitudeM, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.altitude_m == mutation.new_altitude_m {
        return MutationOutcome::empty().warn("mutation.no-op", "altitude_m already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        altitude_m: Some(mutation.new_altitude_m),
        ..En1990Diff::default()
    })
}
