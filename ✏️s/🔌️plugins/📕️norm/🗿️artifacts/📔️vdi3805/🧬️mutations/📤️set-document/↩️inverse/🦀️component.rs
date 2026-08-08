//! ↩️ Inverse for SetDocument on Vdi3805.
use crate::artifacts::vdi3805::Document;
use crate::artifacts::vdi3805::mutations::Vdi3805Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<Vdi3805Mutation> {
    vec![Vdi3805Mutation::SetDocument { document: base.clone() }]
}
