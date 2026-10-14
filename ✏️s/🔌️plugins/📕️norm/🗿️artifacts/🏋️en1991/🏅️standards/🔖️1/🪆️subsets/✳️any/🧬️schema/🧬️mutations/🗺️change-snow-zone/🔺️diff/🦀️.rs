//! 🔺️ Diff for `change-snow-zone`.
use crate::change_snow_zone::ChangeSnowZone;
use semio_s_artifact_norm_en1991_diff::En1991Diff;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
pub fn diff(payload: &ChangeSnowZone, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.snow_zone == payload.new_snow_zone {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { snow_zone: Some(payload.new_snow_zone.clone()), ..Default::default() })
}
