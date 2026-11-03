//! 🔺️ `change-restraint-general-method` diff: patches `general_method` of the addressed restraint row.

use crate::change_restraint_general_method::ChangeRestraintGeneralMethod;
use semio_s_artifact_norm_en1993_diff::{En1993Diff, En1993RestraintDelta, En1993RestraintPatch};
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn diff(payload: &ChangeRestraintGeneralMethod, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    let Some(row) = base.restraints.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "restraint", Vec::<String>::new());
    };
    if row.general_method == payload.new_general_method {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The restraint already has this value.");
    }
    protocol::MutationOutcome::new(En1993Diff { restraints: En1993RestraintDelta::modification(payload.id.clone(), En1993RestraintPatch { general_method: Some(payload.new_general_method), ..Default::default() }), ..Default::default() })
}
