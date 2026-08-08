//! ↩️ Inverse for SetDocument on En1998.
use crate::artifacts::en1998::Document;
use crate::artifacts::en1998::mutations::En1998Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<En1998Mutation> {
    vec![En1998Mutation::SetDocument { document: base.clone() }]
}
