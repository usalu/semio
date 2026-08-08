//! SetSnapshot mutation payload + builder + apply.
use crate::artifacts::en1995::En1995Snapshot;
use crate::artifacts::en1995::mutations::En1995Mutation;

pub fn set_snapshot(snapshot: En1995Snapshot) -> En1995Mutation {
    En1995Mutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut En1995Snapshot, replacement: &En1995Snapshot) {
    *base = replacement.clone();
}
