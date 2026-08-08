//! 📤️ Iso16757 mutation — SetDocument payload + builder + apply.
use crate::artifacts::iso16757::Document;
use crate::artifacts::iso16757::mutations::Iso16757Mutation;

pub fn set_document(document: Document) -> Iso16757Mutation {
    Iso16757Mutation::SetDocument { document }
}

pub fn apply(projection: &mut Document, document: &Document) {
    *projection = document.clone();
}
