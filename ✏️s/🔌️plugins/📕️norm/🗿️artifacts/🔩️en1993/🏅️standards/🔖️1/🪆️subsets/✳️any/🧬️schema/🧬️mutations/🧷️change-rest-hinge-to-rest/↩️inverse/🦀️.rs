//! ↩️ `change-rest-hinge-to-rest` inverse: restores the previous `hinge_to_restraint`.

use crate::change_rest_hinge_to_rest::ChangeRestHingeToRest;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRestHingeToRest, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_rest_hinge_to_rest::ChangeRestHingeToRest> {
    Ok(match base.restraints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRestHingeToRest { id: payload.id.clone(), new_hinge_to_restraint: row.hinge_to_restraint })],
        None => Vec::new(),
    })
}
