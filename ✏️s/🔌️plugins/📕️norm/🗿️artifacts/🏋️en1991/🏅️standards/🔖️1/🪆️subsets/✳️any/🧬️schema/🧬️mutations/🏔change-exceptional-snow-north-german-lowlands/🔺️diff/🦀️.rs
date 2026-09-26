//! Diff for `change-exceptional-snow-north-german-lowlands`.
use super::ChangeExceptionalSnowNorthGermanLowlands;
use crate::{En1991Diff, En1991Snapshot};
pub fn diff(payload: &ChangeExceptionalSnowNorthGermanLowlands, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.exceptional_snow_north_german_lowlands == payload.new_exceptional_snow_north_german_lowlands {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Value unchanged.");
    }
    protocol::MutationOutcome::new(En1991Diff { exceptional_snow_north_german_lowlands: Some(payload.new_exceptional_snow_north_german_lowlands), ..Default::default() })
}
