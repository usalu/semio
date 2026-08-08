//! 📤️ En1992 mutation — SetDocument payload + builder + apply.
use crate::artifacts::en1992::Document;
use crate::artifacts::en1992::mutations::En1992Mutation;

pub fn set_document(document: Document) -> En1992Mutation {
    En1992Mutation::SetDocument { document }
}

pub fn apply(projection: &mut Document, document: &Document) {
    *projection = document.clone();
}
