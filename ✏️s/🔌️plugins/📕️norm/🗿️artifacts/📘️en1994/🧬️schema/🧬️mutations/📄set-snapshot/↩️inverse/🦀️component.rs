//! ↩️ Inverse for SetSnapshot on En1994.
use crate::artifacts::en1994::En1994Snapshot;
use crate::artifacts::en1994::mutations::En1994Mutation;

pub fn inverse(base: &En1994Snapshot, _replacement: &En1994Snapshot) -> Vec<En1994Mutation> {
    vec![En1994Mutation::SetSnapshot { snapshot: base.clone() }]
}
