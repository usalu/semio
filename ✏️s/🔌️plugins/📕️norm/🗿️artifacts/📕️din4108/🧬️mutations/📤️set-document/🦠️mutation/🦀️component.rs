//! 📤️ Din4108 mutation — SetDocument payload + builder + apply.
use crate::artifacts::din4108::Document;
use crate::artifacts::din4108::mutations::Din4108Mutation;

pub fn set_document(document: Document) -> Din4108Mutation {
    Din4108Mutation::SetDocument { document }
}

pub fn apply(projection: &mut Document, document: &Document) {
    *projection = document.clone();
}
