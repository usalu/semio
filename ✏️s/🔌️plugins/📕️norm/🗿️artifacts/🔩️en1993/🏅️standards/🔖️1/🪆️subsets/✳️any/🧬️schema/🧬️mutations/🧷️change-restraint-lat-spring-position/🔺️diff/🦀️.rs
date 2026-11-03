//! 🔺️ `change-restraint-lat-spring-position` diff: patches `lateral_spring_position` of the addressed restraint row.

use crate::change_restraint_lat_spring_position::ChangeRestraintLatSpringPosition;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993RestraintDelta, En1993RestraintPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeRestraintLatSpringPosition, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_lateral_spring_position.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.restraints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "restraint", Vec::<String>::new());
    };
    if row.lateral_spring_position == payload.new_lateral_spring_position {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The restraint already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { restraints: En1993RestraintDelta::modification(payload.id.clone(), En1993RestraintPatch { lateral_spring_position: Some(payload.new_lateral_spring_position), ..Default::default() }), ..Default::default() })
}
