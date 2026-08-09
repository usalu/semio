//! ↩️ Inverse for SetSnapshot on Iso16757.
use crate::artifacts::iso16757::mutations::Iso16757Mutation;
use crate::artifacts::iso16757::Iso16757Snapshot;

pub fn inverse(base: &Iso16757Snapshot) -> Vec<Iso16757Mutation> {
    vec![Iso16757Mutation::SetSnapshot { snapshot: base.clone() }]
}
