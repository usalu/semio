//! 📤️ En1993 mutation — SetDocument payload + builder + apply.
use crate::artifacts::en1993::Document;
use crate::artifacts::en1993::mutations::En1993Mutation;

pub fn set_document(document: Document) -> En1993Mutation {
    En1993Mutation::SetDocument { document }
}

pub fn apply(projection: &mut Document, document: &Document) {
    *projection = document.clone();
}
