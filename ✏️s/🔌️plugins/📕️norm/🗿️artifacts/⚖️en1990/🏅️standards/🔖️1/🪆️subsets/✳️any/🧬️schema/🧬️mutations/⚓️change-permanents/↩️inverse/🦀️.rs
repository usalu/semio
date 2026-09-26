//! ↩️ `change-permanents` inverse.

use super::ChangePermanents;
use crate::En1990Mutation;
use crate::En1990Snapshot;

pub fn inverse(mutation: &ChangePermanents, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    let _ = mutation;
    vec![En1990Mutation::ChangePermanents(ChangePermanents { new_permanents: base.permanents.clone() })]
}
