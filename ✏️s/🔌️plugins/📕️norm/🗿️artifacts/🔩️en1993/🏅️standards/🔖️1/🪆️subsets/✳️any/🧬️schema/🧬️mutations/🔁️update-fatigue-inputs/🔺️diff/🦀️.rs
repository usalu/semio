//! 🔁️ `update-fatigue-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateFatigueInputs;
use crate::diff::{En1993Diff, En1993FatigueDetailDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateFatigueInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let delta = match base.fatigue_details.iter().position(|row| row.id == payload.fatigue_detail.id) {
        Some(index) if base.fatigue_details[index] == payload.fatigue_detail => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => {
            let mut replacement = En1993FatigueDetailDelta::removal(&base.fatigue_details, index);
            replacement.absorb(En1993FatigueDetailDelta::insertion(index, payload.fatigue_detail.clone()));
            replacement
        }
        None => En1993FatigueDetailDelta::insertion(base.fatigue_details.len(), payload.fatigue_detail.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { fatigue_details: delta, ..Default::default() })
}
