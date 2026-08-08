//! ↩️ Inverse for SetDocument on En1991.
use crate::artifacts::en1991::En1991Snapshot;
use crate::artifacts::en1991::mutations::En1991Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<En1991Mutation> {
    vec![En1991Mutation::SetSnapshot { snapshot: base.clone() }]
}
