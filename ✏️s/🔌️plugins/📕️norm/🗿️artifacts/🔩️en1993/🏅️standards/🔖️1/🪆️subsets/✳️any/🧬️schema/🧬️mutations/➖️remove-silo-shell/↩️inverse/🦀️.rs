use super::RemoveSiloShell;
use crate::mutations::{insert_silo_shell, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &RemoveSiloShell, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if payload.index >= base.silo_shells.len() { return Vec::new(); }
    vec![En1993Mutation::InsertSiloShell(insert_silo_shell::InsertSiloShell { index: payload.index, silo_shell: base.silo_shells[payload.index].clone() })]
}
