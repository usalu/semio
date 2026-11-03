//! ➖️ `remove-forming-gain` diff — removes the row at the index.

use crate::remove_forming_gain::RemoveFormingGain;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993FormingGainDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveFormingGain, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.forming_gains.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("forming-gain index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { forming_gains: En1993FormingGainDelta::removal(&base.forming_gains, payload.index), ..Default::default() })
}
