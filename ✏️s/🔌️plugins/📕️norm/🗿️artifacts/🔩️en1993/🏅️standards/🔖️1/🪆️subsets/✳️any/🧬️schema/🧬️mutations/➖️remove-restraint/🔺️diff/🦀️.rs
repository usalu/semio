//! ➖️ `remove-restraint` diff — removes the row at the index.

use crate::remove_restraint::RemoveRestraint;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993RestraintDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &RemoveRestraint, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.restraints.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("restraint index {} out of range.", payload.index), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1993Diff { restraints: En1993RestraintDelta::removal(&base.restraints, payload.index), ..Default::default() })
}
