//! ↩️ Inverse for SetDocument on En1993.
use crate::artifacts::en1993::En1993Snapshot;
use crate::artifacts::en1993::mutations::En1993Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<En1993Mutation> {
    vec![En1993Mutation::SetSnapshot { snapshot: base.clone() }]
}
