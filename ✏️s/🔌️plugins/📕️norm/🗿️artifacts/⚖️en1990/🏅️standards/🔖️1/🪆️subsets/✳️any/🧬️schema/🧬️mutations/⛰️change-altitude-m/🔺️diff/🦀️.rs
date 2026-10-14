//! 🔺️ `change-altitude-m` sparse diff.

use crate::change_altitude_m::ChangeAltitudeM;
use semio_s_artifact_norm_en1990_diff::En1990Diff;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeAltitudeM, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.altitude_m == mutation.new_altitude_m {
        return MutationOutcome::empty().warning("mutation.no-op", "altitude_m already has this value.");
    }
    MutationOutcome::new(En1990Diff {
        altitude_m: Some(mutation.new_altitude_m),
        ..En1990Diff::default()
    })
}
