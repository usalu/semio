//! ↩️ Inverse for SetDocument on En1992.
use crate::artifacts::en1992::Document;
use crate::artifacts::en1992::mutations::En1992Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<En1992Mutation> {
    vec![En1992Mutation::SetDocument { document: base.clone() }]
}
