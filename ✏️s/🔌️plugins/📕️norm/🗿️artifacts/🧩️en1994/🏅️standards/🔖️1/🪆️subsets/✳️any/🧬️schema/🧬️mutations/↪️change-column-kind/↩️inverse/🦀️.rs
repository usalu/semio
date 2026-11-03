//! ↩️ Inverse for `change-column-kind`.
use crate::change_column_kind::ChangeColumnKind;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnKind, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_kind::ChangeColumnKind> {
    Ok((|| {
    let Some(index) = base.columns.iter().position(|row| row.id == payload.id) else {
        return Vec::new();
    };
    let Some(col) = base.columns.get(index) else { return Vec::new(); };
    vec![Op::from(ChangeColumnKind { id: payload.id.clone(), new_kind: col.kind })]

    })())
}
