//! 🏗️ `update-crane-inputs` diff — upserts the row by id: a known id is replaced in place, an unknown id is appended.

use super::UpdateCraneInputs;
use crate::diff::{En1993Diff, En1993CraneRunwayDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &UpdateCraneInputs, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let delta = match base.crane_runways.iter().position(|row| row.id == payload.crane_runway.id) {
        Some(index) if base.crane_runways[index] == payload.crane_runway => return protocol::MutationOutcome::empty().warning("mutation.no-op", "Entity already has this value."),
        Some(index) => {
            let mut replacement = En1993CraneRunwayDelta::removal(&payload.crane_runway.id);
            replacement.absorb(En1993CraneRunwayDelta::insertion(&base.crane_runways, index, payload.crane_runway.clone()));
            replacement
        }
        None => En1993CraneRunwayDelta::insertion(&base.crane_runways, base.crane_runways.len(), payload.crane_runway.clone()),
    };
    protocol::MutationOutcome::new(En1993Diff { crane_runways: delta, ..Default::default() })
}
