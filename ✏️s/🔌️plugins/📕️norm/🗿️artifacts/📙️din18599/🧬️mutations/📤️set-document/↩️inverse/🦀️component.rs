//! ↩️ Inverse for SetDocument on Din18599.
use crate::artifacts::din18599::Document;
use crate::artifacts::din18599::mutations::Din18599Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::SetDocument { document: base.clone() }]
}
