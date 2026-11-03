//! ↩️ `change-restraint-end-support` inverse: restores the previous `end_support`.

use crate::change_restraint_end_support::ChangeRestraintEndSupport;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRestraintEndSupport, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_restraint_end_support::ChangeRestraintEndSupport> {
    Ok(match base.restraints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRestraintEndSupport { id: payload.id.clone(), new_end_support: row.end_support })],
        None => Vec::new(),
    })
}
