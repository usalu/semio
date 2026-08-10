//! ↩️ Inverse for SetSnapshot on En1996.
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;

pub fn inverse(base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::SetSnapshot { snapshot: base.clone() }]
}
