//! ↩️ Inverse for SetDocument on En1994.
use crate::artifacts::en1994::Document;
use crate::artifacts::en1994::mutations::En1994Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<En1994Mutation> {
    vec![En1994Mutation::SetDocument { document: base.clone() }]
}
