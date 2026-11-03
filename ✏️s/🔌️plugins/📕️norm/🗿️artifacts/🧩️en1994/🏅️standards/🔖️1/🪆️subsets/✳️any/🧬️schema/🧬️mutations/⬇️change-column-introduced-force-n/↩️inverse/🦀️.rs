//! ↩️ Inverse for `change-column-introduced-force-n`.
use crate::change_column_introduced_force_n::ChangeColumnIntroducedForceN;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnIntroducedForceN, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_introduced_force_n::ChangeColumnIntroducedForceN> {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return Ok(Vec::new());
    };
    let Some(column) = base.columns.get(index) else { return Ok(Vec::new()); };
    Ok(vec![Op::from(ChangeColumnIntroducedForceN { id: payload.id.clone(), new_introduced_force_n: column.introduced_force_n })])
}
