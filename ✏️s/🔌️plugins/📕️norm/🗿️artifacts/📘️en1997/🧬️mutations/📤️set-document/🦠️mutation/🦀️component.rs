//! 📤️ En1997 mutation — SetDocument payload + builder + apply.
use crate::artifacts::en1997::Document;
use crate::artifacts::en1997::mutations::En1997Mutation;

pub fn set_document(document: Document) -> En1997Mutation {
    En1997Mutation::SetDocument { document }
}

pub fn apply(projection: &mut Document, document: &Document) {
    *projection = document.clone();
}
