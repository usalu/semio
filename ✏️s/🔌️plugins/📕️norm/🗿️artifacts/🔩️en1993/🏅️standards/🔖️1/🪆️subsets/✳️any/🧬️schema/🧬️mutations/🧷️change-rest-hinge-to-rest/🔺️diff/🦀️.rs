//! 🔺️ `change-rest-hinge-to-rest` diff: patches `hinge_to_restraint` of the addressed restraint row.

use crate::change_rest_hinge_to_rest::ChangeRestHingeToRest;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993RestraintDelta, En1993RestraintPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeRestHingeToRest, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_hinge_to_restraint.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.restraints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "restraint", Vec::<String>::new());
    };
    if row.hinge_to_restraint == payload.new_hinge_to_restraint {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The restraint already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { restraints: En1993RestraintDelta::modification(payload.id.clone(), En1993RestraintPatch { hinge_to_restraint: Some(payload.new_hinge_to_restraint), ..Default::default() }), ..Default::default() })
}
