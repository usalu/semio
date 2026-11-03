//! ↩️ `change-uplift-var-destabilizing` inverse: restores the previous `variable_destabilizing`.

use crate::change_uplift_var_destabilizing::ChangeUpliftVarDestabilizing;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangeUpliftVarDestabilizing, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_uplift_var_destabilizing::ChangeUpliftVarDestabilizing> {
    Ok(match base.uplift_cases.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeUpliftVarDestabilizing { id: payload.id.clone(), new_variable_destabilizing: row.variable_destabilizing })],
        None => Vec::new(),
    })
}
