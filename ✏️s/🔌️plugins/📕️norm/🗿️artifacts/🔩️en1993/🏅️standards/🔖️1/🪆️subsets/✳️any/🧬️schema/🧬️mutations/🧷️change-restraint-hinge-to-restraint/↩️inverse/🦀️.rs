//! ↩️ `change-restraint-hinge-to-restraint` inverse: restores the previous `hinge_to_restraint`.

use crate::change_restraint_hinge_to_restraint::ChangeRestraintHingeToRestraint;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRestraintHingeToRestraint, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_restraint_hinge_to_restraint::ChangeRestraintHingeToRestraint> {
    Ok(match base.restraints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRestraintHingeToRestraint { id: payload.id.clone(), new_hinge_to_restraint: row.hinge_to_restraint })],
        None => Vec::new(),
    })
}
