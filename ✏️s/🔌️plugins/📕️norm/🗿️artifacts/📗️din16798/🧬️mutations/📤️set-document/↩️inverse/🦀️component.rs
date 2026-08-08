//! ↩️ Inverse for SetDocument on Din16798.
use crate::artifacts::din16798::Document;
use crate::artifacts::din16798::mutations::Din16798Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<Din16798Mutation> {
    vec![Din16798Mutation::SetDocument { document: base.clone() }]
}
