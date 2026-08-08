//! ↩️ Inverse for SetDocument on En1990.
use crate::artifacts::en1990::En1990Snapshot;
use crate::artifacts::en1990::mutations::En1990Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<En1990Mutation> {
    vec![En1990Mutation::SetSnapshot { snapshot: base.clone() }]
}
