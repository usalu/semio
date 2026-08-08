//! 📤️ Vdi3805 mutation — SetDocument payload + builder + apply.
use crate::artifacts::vdi3805::Document;
use crate::artifacts::vdi3805::mutations::Vdi3805Mutation;

pub fn set_document(document: Document) -> Vdi3805Mutation {
    Vdi3805Mutation::SetDocument { document }
}

pub fn apply(projection: &mut Document, document: &Document) {
    *projection = document.clone();
}
