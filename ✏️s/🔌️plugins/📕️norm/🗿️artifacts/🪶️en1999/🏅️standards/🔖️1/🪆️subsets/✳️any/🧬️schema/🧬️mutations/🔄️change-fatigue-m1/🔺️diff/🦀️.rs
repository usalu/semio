//! 🔺️ `change-fatigue-m1` diff: patches `m1` of the addressed fatigue detail row.

use crate::change_fatigue_m1::ChangeFatigueM1;
use semio_s_artifact_norm_en1999_diff::{En1999Diff, En1999FatigueDetailsPatch, En1999FatigueDetailsRows};
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn diff(payload: &ChangeFatigueM1, base: &En1999Snapshot) -> protocol::MutationOutcome<En1999Diff> {
    if !payload.new_m1.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.fatigue_details.iter().find(|row| row.id == payload.fatigue_detail_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "fatigue-detail", Vec::<String>::new());
    };
    if r0.m1 == payload.new_m1 {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The fatigue detail already has this value.");
    }
    protocol::MutationOutcome::new(En1999Diff { fatigue_details: Some(En1999FatigueDetailsRows::modification(&payload.fatigue_detail_id, En1999FatigueDetailsPatch { m1: Some(payload.new_m1), ..Default::default() })), ..Default::default() })
}
