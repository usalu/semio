//! 📤️ En1994 mutation — SetDocument payload + builder + apply.
use crate::artifacts::en1994::Document;
use crate::artifacts::en1994::mutations::En1994Mutation;

pub fn set_document(document: Document) -> En1994Mutation {
    En1994Mutation::SetDocument { document }
}

pub fn apply(projection: &mut Document, document: &Document) {
    *projection = document.clone();
}
