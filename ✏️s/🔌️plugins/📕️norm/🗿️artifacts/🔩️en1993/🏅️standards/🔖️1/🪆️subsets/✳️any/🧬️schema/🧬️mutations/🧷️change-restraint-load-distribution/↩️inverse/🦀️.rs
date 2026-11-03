//! ↩️ `change-restraint-load-distribution` inverse: restores the previous `load_distribution`.

use crate::change_restraint_load_distribution::ChangeRestraintLoadDistribution;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRestraintLoadDistribution, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_restraint_load_distribution::ChangeRestraintLoadDistribution> {
    Ok(match base.restraints.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRestraintLoadDistribution { id: payload.id.clone(), new_load_distribution: row.load_distribution })],
        None => Vec::new(),
    })
}
