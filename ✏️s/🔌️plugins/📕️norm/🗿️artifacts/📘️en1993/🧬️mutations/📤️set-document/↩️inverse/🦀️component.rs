//! ↩️ Inverse for SetDocument on En1993.
use crate::artifacts::en1993::Document;
use crate::artifacts::en1993::mutations::En1993Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<En1993Mutation> {
    vec![En1993Mutation::SetDocument { document: base.clone() }]
}
