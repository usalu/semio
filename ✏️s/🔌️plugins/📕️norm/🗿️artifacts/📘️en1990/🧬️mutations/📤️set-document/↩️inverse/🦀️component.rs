//! ↩️ Inverse for SetDocument on En1990.
use crate::artifacts::en1990::Document;
use crate::artifacts::en1990::mutations::En1990Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<En1990Mutation> {
    vec![En1990Mutation::SetDocument { document: base.clone() }]
}
