//! 🔺️ `change-fatigue-m2` diff: patches `m2` of the addressed fatigue detail row.

use crate::change_fatigue_m2::ChangeFatigueM2;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FatigueDetailsPatch, En1999FatigueDetailsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeFatigueM2, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_m2.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.fatigue_details.iter().find(|row| row.id == payload.fatigue_detail_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fatigue-detail", Vec::<String>::new());
    };
    if r0.m2 == payload.new_m2 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue detail already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { fatigue_details: Some(En1999FatigueDetailsRows::modification(&payload.fatigue_detail_id, En1999FatigueDetailsPatch { m2: Some(payload.new_m2), ..Default::default() })), ..Default::default() })
}
