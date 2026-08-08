//! 📤️ En1996 mutation — SetDocument payload + builder + apply.
use crate::artifacts::en1996::Document;
use crate::artifacts::en1996::mutations::En1996Mutation;

pub fn set_document(document: Document) -> En1996Mutation {
    En1996Mutation::SetDocument { document }
}

pub fn apply(projection: &mut Document, document: &Document) {
    *projection = document.clone();
}
