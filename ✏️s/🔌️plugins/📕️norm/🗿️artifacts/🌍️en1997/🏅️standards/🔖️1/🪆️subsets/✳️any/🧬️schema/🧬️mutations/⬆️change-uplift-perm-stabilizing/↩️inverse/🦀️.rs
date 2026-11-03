//! ↩️ `change-uplift-perm-stabilizing` inverse: restores the previous `permanent_stabilizing`.

use crate::change_uplift_perm_stabilizing::ChangeUpliftPermStabilizing;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeUpliftPermStabilizing, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_uplift_perm_stabilizing::ChangeUpliftPermStabilizing> {
    Ok(match base.uplift_cases.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeUpliftPermStabilizing { id: payload.id.clone(), new_permanent_stabilizing: row.permanent_stabilizing })],
        None => Vec::new(),
    })
}
