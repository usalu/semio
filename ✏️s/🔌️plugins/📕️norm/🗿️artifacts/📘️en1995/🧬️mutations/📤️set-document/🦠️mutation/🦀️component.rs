//! 📤️ En1995 mutation — SetDocument payload + builder + apply.
use crate::artifacts::en1995::Document;
use crate::artifacts::en1995::mutations::En1995Mutation;

pub fn set_document(document: Document) -> En1995Mutation {
    En1995Mutation::SetDocument { document }
}

pub fn apply(projection: &mut Document, document: &Document) {
    *projection = document.clone();
}
