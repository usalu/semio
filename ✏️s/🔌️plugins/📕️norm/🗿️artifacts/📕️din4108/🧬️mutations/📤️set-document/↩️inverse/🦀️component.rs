//! ↩️ Inverse for SetDocument on Din4108.
use crate::artifacts::din4108::Document;
use crate::artifacts::din4108::mutations::Din4108Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::SetDocument { document: base.clone() }]
}
