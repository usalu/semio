//! 🔺️ `change-bb2-details-conform` diff.

use crate::change_bb2_details_conform::ChangeBb2DetailsConform;
use semio_s_artifact_norm_din4108_diff::Din4108Diff;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn diff(payload: &ChangeBb2DetailsConform, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.bb2_details_conform == payload.new_bb2_details_conform {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "bb2_details_conform already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { bb2_details_conform: Some(payload.new_bb2_details_conform), ..Default::default() })
}
