//! ↩️ `remove-shell` inverse.

use crate::remove_shell::RemoveShell;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_shell;

pub fn inverse<Op>(payload: &RemoveShell, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_shell::InsertShell> {
    Ok((|| {
    if let Some((index, row)) = base.shells.iter().enumerate().find(|(_, row)| row.id == payload.id) {
        vec![Op::from(insert_shell::InsertShell { index: index as u32, shell: row.clone() })]
    } else { Vec::new() }

    })())
}
