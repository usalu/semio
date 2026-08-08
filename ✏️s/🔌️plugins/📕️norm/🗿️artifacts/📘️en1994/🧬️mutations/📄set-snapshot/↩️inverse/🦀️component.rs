//! ↩️ Inverse for SetDocument on En1994.
use crate::artifacts::en1994::En1994Snapshot;
use crate::artifacts::en1994::mutations::En1994Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<En1994Mutation> {
    vec![En1994Mutation::SetSnapshot { snapshot: base.clone() }]
}
