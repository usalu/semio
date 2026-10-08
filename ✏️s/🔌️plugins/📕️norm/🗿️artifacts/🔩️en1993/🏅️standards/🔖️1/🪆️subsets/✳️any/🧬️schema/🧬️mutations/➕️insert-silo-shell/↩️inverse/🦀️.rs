use super::InsertSiloShell;
use crate::mutations::{remove_silo_shell, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertSiloShell, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.silo_shells.len());
    vec![En1993Mutation::RemoveSiloShell(remove_silo_shell::RemoveSiloShell { index: at })]

    })())
}
