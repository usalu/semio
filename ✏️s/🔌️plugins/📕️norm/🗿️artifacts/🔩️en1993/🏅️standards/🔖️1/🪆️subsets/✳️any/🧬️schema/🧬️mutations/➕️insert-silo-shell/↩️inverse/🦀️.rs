use super::InsertSiloShell;
use crate::mutations::{remove_silo_shell, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertSiloShell, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    let at = payload.index.min(base.silo_shells.len());
    vec![En1993Mutation::RemoveSiloShell(remove_silo_shell::RemoveSiloShell { index: at })]
}
