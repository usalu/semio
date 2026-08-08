//! ↩️ Inverse for SetDocument on Iso16757.
use crate::artifacts::iso16757::Document;
use crate::artifacts::iso16757::mutations::Iso16757Mutation;

pub fn inverse(base: &Document, _replacement: &Document) -> Vec<Iso16757Mutation> {
    vec![Iso16757Mutation::SetDocument { document: base.clone() }]
}
