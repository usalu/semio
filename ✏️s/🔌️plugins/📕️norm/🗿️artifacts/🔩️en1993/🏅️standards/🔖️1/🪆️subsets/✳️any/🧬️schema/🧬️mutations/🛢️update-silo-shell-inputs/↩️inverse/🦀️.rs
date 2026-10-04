//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use super::UpdateSiloShellInputs;
use crate::mutations::remove_silo_shell;
use crate::{En1993Mutation, En1993Snapshot};
pub fn inverse(payload: &UpdateSiloShellInputs, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if let Some(prior) = base.silo_shells.iter().find(|x| x.id == payload.silo_shell.id) {
        vec![En1993Mutation::UpdateSiloShellInputs(UpdateSiloShellInputs { silo_shell: prior.clone() })]
    } else {
        vec![En1993Mutation::RemoveSiloShell(remove_silo_shell::RemoveSiloShell { index: base.silo_shells.len() })]
    }

    })())
}
