//! 📤️ Din18599 mutation — SetDocument payload + builder + apply.
use crate::artifacts::din18599::Document;
use crate::artifacts::din18599::mutations::Din18599Mutation;

pub fn set_document(document: Document) -> Din18599Mutation {
    Din18599Mutation::SetDocument { document }
}

pub fn apply(projection: &mut Document, document: &Document) {
    *projection = document.clone();
}
