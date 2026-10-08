//! ➕️ `insert-fire-exposure` diff — inserts the row at its position, clamped to the end of the collection.

use super::InsertFireExposure;
use crate::diff::En1993RowEdit as _;
use crate::diff::{En1993Diff, En1993FireExposureEdit};
use crate::En1993Snapshot;

pub fn diff(payload: &InsertFireExposure, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.fire_exposures.iter().any(|existing| existing.id == payload.fire_exposure.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Fire exposure id {} already exists.", payload.fire_exposure.id), [payload.fire_exposure.id.clone()]);
    }
    let index = payload.index.min(base.fire_exposures.len());
    protocol::MutationOutcome::new(En1993Diff { fire_exposures: vec![En1993FireExposureEdit::insert(index, payload.fire_exposure.clone())], ..Default::default() })
}
