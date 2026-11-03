//! ↩️ `change-uplift-total-stress` inverse: restores the previous `total_stress`.

use crate::change_uplift_total_stress::ChangeUpliftTotalStress;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeUpliftTotalStress, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_uplift_total_stress::ChangeUpliftTotalStress> {
    Ok(match base.uplift_cases.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeUpliftTotalStress { id: payload.id.clone(), new_total_stress: row.total_stress })],
        None => Vec::new(),
    })
}
