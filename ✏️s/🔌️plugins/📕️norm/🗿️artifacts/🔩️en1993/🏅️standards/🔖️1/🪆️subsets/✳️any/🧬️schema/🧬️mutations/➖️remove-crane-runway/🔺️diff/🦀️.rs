//! ➖️ `remove-crane-runway` diff — removes the row at the index.

use super::RemoveCraneRunway;
use crate::diff::{En1993Diff, En1993CraneRunwayDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveCraneRunway, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.crane_runways.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("crane-runway index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { crane_runways: En1993CraneRunwayDelta::removal(&base.crane_runways, payload.index), ..Default::default() })
}
