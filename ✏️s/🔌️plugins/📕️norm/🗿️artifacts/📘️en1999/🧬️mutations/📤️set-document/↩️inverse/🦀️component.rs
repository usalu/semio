//! ↩️ Inverse for SetDocument on En1999.
use crate::artifacts::en1999::Document;
use crate::artifacts::en1999::mutations::En1999Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<En1999Mutation> {
    vec![En1999Mutation::SetDocument { document: base.clone() }]
}
