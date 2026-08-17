//! 🔺️ `change-bb2-details-conform` — sparse diff construction.

use super::mutation::ChangeBb2DetailsConform;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeBb2DetailsConform, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.bb2_details_conform == payload.new_bb2_details_conform {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Bb2 details conform already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { bb2_details_conform: Some(payload.new_bb2_details_conform.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
