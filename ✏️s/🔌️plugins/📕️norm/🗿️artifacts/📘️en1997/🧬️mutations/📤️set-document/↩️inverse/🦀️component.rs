//! ↩️ Inverse for SetDocument on En1997.
use crate::artifacts::en1997::Document;
use crate::artifacts::en1997::mutations::En1997Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<En1997Mutation> {
    vec![En1997Mutation::SetDocument { document: base.clone() }]
}
