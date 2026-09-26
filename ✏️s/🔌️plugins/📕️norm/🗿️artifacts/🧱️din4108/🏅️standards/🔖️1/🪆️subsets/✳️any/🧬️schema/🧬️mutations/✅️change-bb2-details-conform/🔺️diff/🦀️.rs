//! 🔺️ `change-bb2-details-conform` diff.

use super::ChangeBb2DetailsConform;
use crate::{Din4108Diff, Din4108Snapshot};

pub fn diff(payload: &ChangeBb2DetailsConform, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.bb2_details_conform == payload.new_bb2_details_conform {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "bb2_details_conform already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { bb2_details_conform: Some(payload.new_bb2_details_conform), ..Default::default() })
}
