//! ➖️ `remove-fatigue-detail` diff — removes the row at the index.

use crate::remove_fatigue_detail::RemoveFatigueDetail;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FatigueDetailDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveFatigueDetail, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.fatigue_details.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("fatigue-detail index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { fatigue_details: En1993FatigueDetailDelta::removal(&base.fatigue_details, payload.index), ..Default::default() })
}
