//! ↩️ Inverse for SetSnapshot on En1995.
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

pub fn inverse(base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::SetSnapshot { snapshot: base.clone() }]
}
