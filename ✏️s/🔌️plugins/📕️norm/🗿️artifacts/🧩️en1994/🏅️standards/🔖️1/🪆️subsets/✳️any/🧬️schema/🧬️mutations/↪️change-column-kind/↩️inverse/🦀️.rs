//! ↩️ Inverse for `change-column-kind`.
use crate::change_column_kind::ChangeColumnKind;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
pub fn inverse<Op>(payload: &ChangeColumnKind, base: &En1994Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_column_kind::ChangeColumnKind> {
    Ok((|| {
    let Some(col) = base.columns.get(payload.index) else { return Vec::new(); };
    vec![Op::from(ChangeColumnKind { index: payload.index, new_kind: col.kind.clone() })]

    })())
}
