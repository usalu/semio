//! ↩️ `change-shells` inverse.

use crate::mutations::change_shells::ChangeShells;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

pub fn inverse(_payload: &ChangeShells, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeShells(ChangeShells { shells: base.shells.clone() })]
}
