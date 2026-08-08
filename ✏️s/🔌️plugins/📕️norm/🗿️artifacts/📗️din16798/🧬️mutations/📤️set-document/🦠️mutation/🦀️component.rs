//! 📤️ Din16798 mutation — SetDocument payload + builder + apply.
use crate::artifacts::din16798::Document;
use crate::artifacts::din16798::mutations::Din16798Mutation;

pub fn set_document(document: Document) -> Din16798Mutation {
    Din16798Mutation::SetDocument { document }
}

pub fn apply(projection: &mut Document, document: &Document) {
    *projection = document.clone();
}
