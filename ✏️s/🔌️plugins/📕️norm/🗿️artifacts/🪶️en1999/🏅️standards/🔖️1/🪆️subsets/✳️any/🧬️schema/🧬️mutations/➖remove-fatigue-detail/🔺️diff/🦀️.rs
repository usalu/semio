//! 🔺️ `remove-fatigue-detail` diff.

use crate::remove_fatigue_detail::RemoveFatigueDetail;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FatigueDetailsRows};

pub fn diff(payload: &RemoveFatigueDetail, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    let Some(index) = base.fatigue_details.iter().position(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Unknown fatigue detail id {}", payload.id), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1999Diff { fatigue_details: Some(En1999FatigueDetailsRows::removal(&base.fatigue_details, index)), ..Default::default() })
}
