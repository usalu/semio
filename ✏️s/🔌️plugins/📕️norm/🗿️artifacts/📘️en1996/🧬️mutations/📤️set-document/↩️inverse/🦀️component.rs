//! ↩️ Inverse for SetDocument on En1996.
use crate::artifacts::en1996::Document;
use crate::artifacts::en1996::mutations::En1996Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<En1996Mutation> {
    vec![En1996Mutation::SetDocument { document: base.clone() }]
}
