//! 🔺️ `change-restraint-design-axial` diff: patches `design_axial` of the addressed restraint row.

use crate::change_restraint_design_axial::ChangeRestraintDesignAxial;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993RestraintDelta, En1993RestraintPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeRestraintDesignAxial, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if !payload.new_design_axial.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.restraints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "restraint", Vec::<String>::new());
    };
    if row.design_axial == payload.new_design_axial {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The restraint already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { restraints: En1993RestraintDelta::modification(payload.id.clone(), En1993RestraintPatch { design_axial: Some(payload.new_design_axial), ..Default::default() }), ..Default::default() })
}
