//! ↩️ `change-uplift-perm-destabilizing` inverse: restores the previous `permanent_destabilizing`.

use crate::change_uplift_perm_destabilizing::ChangeUpliftPermDestabilizing;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeUpliftPermDestabilizing, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_uplift_perm_destabilizing::ChangeUpliftPermDestabilizing> {
    Ok(match base.uplift_cases.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeUpliftPermDestabilizing { id: payload.id.clone(), new_permanent_destabilizing: row.permanent_destabilizing })],
        None => Vec::new(),
    })
}
