//! ➖️ `remove-crane-runway` diff — removes the row at the index, guarded by the row's own id.

use super::RemoveCraneRunway;
use crate::diff::{En1993Diff, En1993CraneRunwayDelta};
use crate::En1993Snapshot;

pub fn diff(payload: &RemoveCraneRunway, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.crane_runways.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("crane-runway index {} out of range.", payload.index), Vec::<String>::new());
    };
    protocol::MutationOutcome::new(En1993Diff { crane_runways: En1993CraneRunwayDelta::removal(&row.id), ..Default::default() })
}
