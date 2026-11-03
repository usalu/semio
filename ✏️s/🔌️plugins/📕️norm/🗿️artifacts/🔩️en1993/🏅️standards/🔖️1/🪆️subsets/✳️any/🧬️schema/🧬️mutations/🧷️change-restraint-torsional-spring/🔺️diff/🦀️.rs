//! 🔺️ `change-restraint-torsional-spring` diff: patches `torsional_spring` of the addressed restraint row.

use crate::change_restraint_torsional_spring::ChangeRestraintTorsionalSpring;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993RestraintDelta, En1993RestraintPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeRestraintTorsionalSpring, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_torsional_spring.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.restraints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "restraint", Vec::<String>::new());
    };
    if row.torsional_spring == payload.new_torsional_spring {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The restraint already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { restraints: En1993RestraintDelta::modification(payload.id.clone(), En1993RestraintPatch { torsional_spring: Some(payload.new_torsional_spring), ..Default::default() }), ..Default::default() })
}
