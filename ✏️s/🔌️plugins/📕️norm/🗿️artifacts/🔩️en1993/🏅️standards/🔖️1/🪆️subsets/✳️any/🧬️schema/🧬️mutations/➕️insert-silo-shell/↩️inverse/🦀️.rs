use crate::insert_silo_shell::InsertSiloShell;
use crate::remove_silo_shell;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertSiloShell, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_silo_shell::RemoveSiloShell> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.silo_shells.len());
    vec![Op::from(remove_silo_shell::RemoveSiloShell { index: at })]

    })())
}
