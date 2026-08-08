//! 📤️ En1999 mutation — SetDocument payload + builder + apply.
use crate::artifacts::en1999::Document;
use crate::artifacts::en1999::mutations::En1999Mutation;

pub fn set_document(document: Document) -> En1999Mutation {
    En1999Mutation::SetDocument { document }
}

pub fn apply(projection: &mut Document, document: &Document) {
    *projection = document.clone();
}
