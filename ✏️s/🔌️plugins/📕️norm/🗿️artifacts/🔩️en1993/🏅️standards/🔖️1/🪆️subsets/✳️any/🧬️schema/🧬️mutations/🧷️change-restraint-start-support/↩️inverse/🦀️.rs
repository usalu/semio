//! ↩️ `change-restraint-start-support` inverse: restores the previous `start_support`.

use crate::change_restraint_start_support::ChangeRestraintStartSupport;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRestraintStartSupport, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_restraint_start_support::ChangeRestraintStartSupport> {
    Ok(match base.restraints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRestraintStartSupport { id: payload.id.clone(), new_start_support: row.start_support })],
        None => Vec::new(),
    })
}
