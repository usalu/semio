//! 📤️ En1998 mutation — SetDocument payload + builder + apply.
use crate::artifacts::en1998::Document;
use crate::artifacts::en1998::mutations::En1998Mutation;

pub fn set_document(document: Document) -> En1998Mutation {
    En1998Mutation::SetDocument { document }
}

pub fn apply(projection: &mut Document, document: &Document) {
    *projection = document.clone();
}
