//! ↩️ Inverse for SetDocument on En1991.
use crate::artifacts::en1991::Document;
use crate::artifacts::en1991::mutations::En1991Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<En1991Mutation> {
    vec![En1991Mutation::SetDocument { document: base.clone() }]
}
