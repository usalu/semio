//! 🔥️ `update-fire-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateFireInputs;
use crate::diff::{En1993Diff, En1993FireExposureDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateFireInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let delta = match base.fire_exposures.iter().position(|row| row.id == payload.fire_exposure.id) {
        Some(index) if base.fire_exposures[index] == payload.fire_exposure => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => {
            let mut replacement = En1993FireExposureDelta::removal(&payload.fire_exposure.id);
            replacement.absorb(En1993FireExposureDelta::insertion(&base.fire_exposures, index, payload.fire_exposure.clone()));
            replacement
        }
        None => En1993FireExposureDelta::insertion(&base.fire_exposures, base.fire_exposures.len(), payload.fire_exposure.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { fire_exposures: delta, ..Default::default() })
}
