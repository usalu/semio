//! 🔺️ `change-climate` — sparse diff construction.

use super::ChangeClimate;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeClimate, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.climate == payload.new_climate {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Climate already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { climate: Some(payload.new_climate.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
