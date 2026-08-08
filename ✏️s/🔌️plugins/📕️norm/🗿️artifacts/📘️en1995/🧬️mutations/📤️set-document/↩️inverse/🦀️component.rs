//! ↩️ Inverse for SetDocument on En1995.
use crate::artifacts::en1995::Document;
use crate::artifacts::en1995::mutations::En1995Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<En1995Mutation> {
    vec![En1995Mutation::SetDocument { document: base.clone() }]
}
