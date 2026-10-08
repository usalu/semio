//! 🔥️ `update-fire-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateFireInputs;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993FireExposureEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateFireInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let edit = match base.fire_exposures.iter().position(|row| row.id == payload.fire_exposure.id) {
        Some(index) if base.fire_exposures[index] == payload.fire_exposure => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => En1993FireExposureEdit::replace(index, payload.fire_exposure.id.clone(), payload.fire_exposure.clone()),
        None => En1993FireExposureEdit::insert(base.fire_exposures.len(), payload.fire_exposure.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { fire_exposures: vec![edit], ..Default::default() })
}
