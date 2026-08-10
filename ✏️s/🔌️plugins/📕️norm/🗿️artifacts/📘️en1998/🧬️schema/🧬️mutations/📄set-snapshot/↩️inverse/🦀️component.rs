//! ↩️ Inverse for SetSnapshot on En1998.
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

pub fn inverse(base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::SetSnapshot { snapshot: base.clone() }]
}
