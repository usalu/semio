//! ↩️ `change-built-up-kind` inverse: restores the previous `kind`.

use crate::change_built_up_kind::ChangeBuiltUpKind;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBuiltUpKind, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_built_up_kind::ChangeBuiltUpKind> {
    Ok(match base.built_up_members.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBuiltUpKind { id: payload.id.clone(), new_kind: row.kind })],
        None => Vec::new(),
    })
}
