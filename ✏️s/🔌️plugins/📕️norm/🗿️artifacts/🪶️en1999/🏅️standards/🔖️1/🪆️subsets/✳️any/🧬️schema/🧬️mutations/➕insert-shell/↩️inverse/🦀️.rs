//! ↩️ `insert-shell` inverse.

use crate::insert_shell::InsertShell;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_shell;

pub fn inverse<Op>(payload: &InsertShell, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_shell::RemoveShell> {
    Ok((|| {
    vec![Op::from(remove_shell::RemoveShell { id: payload.shell.id.clone() })]

    })())
}
