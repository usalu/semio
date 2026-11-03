//! ➕️ `insert-restraint` diff — inserts the row at its position, clamped to the end of the collection.

use crate::insert_restraint::InsertRestraint;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993RestraintDelta};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &InsertRestraint, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.restraints.iter().any(|existing| existing.id == payload.restraint.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Restraint id {} already exists.", payload.restraint.id), [payload.restraint.id.clone()]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.restraints.len());
    protocol::MutationOutcome::new(En1993Diff { restraints: En1993RestraintDelta::insertion(index, payload.restraint.clone()), ..Default::default() })
}
