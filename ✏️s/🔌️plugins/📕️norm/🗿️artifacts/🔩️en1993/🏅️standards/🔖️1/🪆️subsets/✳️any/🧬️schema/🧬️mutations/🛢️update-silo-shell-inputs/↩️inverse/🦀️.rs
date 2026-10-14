//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use crate::update_silo_shell_inputs::UpdateSiloShellInputs;
use crate::remove_silo_shell;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &UpdateSiloShellInputs, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::update_silo_shell_inputs::UpdateSiloShellInputs> + From<crate::remove_silo_shell::RemoveSiloShell> {
    Ok((|| {
    if let Some(prior) = base.silo_shells.iter().find(|x| x.id == payload.silo_shell.id) {
        vec![Op::from(UpdateSiloShellInputs { silo_shell: prior.clone() })]
    } else {
        vec![Op::from(remove_silo_shell::RemoveSiloShell { index: base.silo_shells.len() })]
    }

    })())
}
