//! 📤️ En1991 mutation — SetDocument payload + builder + apply.
use crate::artifacts::en1991::Document;
use crate::artifacts::en1991::mutations::En1991Mutation;

pub fn set_document(document: Document) -> En1991Mutation {
    En1991Mutation::SetDocument { document }
}

pub fn apply(projection: &mut Document, document: &Document) {
    *projection = document.clone();
}
