//! ➖️ `remove-fire-exposure` diff — removes the row at the index.

use crate::remove_fire_exposure::RemoveFireExposure;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FireExposureDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveFireExposure, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.fire_exposures.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("fire-exposure index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { fire_exposures: En1993FireExposureDelta::removal(&base.fire_exposures, payload.index), ..Default::default() })
}
