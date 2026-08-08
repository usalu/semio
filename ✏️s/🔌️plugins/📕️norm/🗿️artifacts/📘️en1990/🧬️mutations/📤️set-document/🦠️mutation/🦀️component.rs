//! 📤️ En1990 mutation — SetDocument payload + builder + apply.
use crate::artifacts::en1990::Document;
use crate::artifacts::en1990::mutations::En1990Mutation;

pub fn set_document(document: Document) -> En1990Mutation {
    En1990Mutation::SetDocument { document }
}

pub fn apply(projection: &mut Document, document: &Document) {
    *projection = document.clone();
}
