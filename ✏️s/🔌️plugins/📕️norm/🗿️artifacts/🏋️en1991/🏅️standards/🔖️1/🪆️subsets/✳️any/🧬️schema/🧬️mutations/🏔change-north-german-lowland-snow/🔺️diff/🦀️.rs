//! Diff for `change-north-german-lowland-snow`.
use super::ChangeNorthGermanLowlandSnow;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeNorthGermanLowlandSnow, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.north_german_lowland_snow == payload.new_north_german_lowland_snow {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { north_german_lowland_snow: Some(payload.new_north_german_lowland_snow), ..Default::default() })
}
