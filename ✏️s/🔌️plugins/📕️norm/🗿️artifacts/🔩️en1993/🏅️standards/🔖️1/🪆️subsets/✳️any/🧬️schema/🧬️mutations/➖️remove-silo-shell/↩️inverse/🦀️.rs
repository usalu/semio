use crate::remove_silo_shell::RemoveSiloShell;
use crate::insert_silo_shell;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveSiloShell, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_silo_shell::InsertSiloShell> {
    Ok((|| {
    if payload.index >= base.silo_shells.len() { return Vec::new(); }
    vec![Op::from(insert_silo_shell::InsertSiloShell { index: Some(payload.index), silo_shell: base.silo_shells[payload.index].clone() })]

    })())
}
