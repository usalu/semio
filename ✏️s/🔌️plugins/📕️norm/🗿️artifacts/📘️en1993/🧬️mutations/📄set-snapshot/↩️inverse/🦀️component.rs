//! ↩️ Inverse for SetSnapshot on En1993.
use crate::artifacts::en1993::En1993Snapshot;
use crate::artifacts::en1993::mutations::En1993Mutation;

pub fn inverse(base: &En1993Snapshot, _replacement: &En1993Snapshot) -> Vec<En1993Mutation> {
    vec![En1993Mutation::SetSnapshot { snapshot: base.clone() }]
}
