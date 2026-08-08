//! SetSnapshot mutation payload + builder + apply.
use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::mutations::En1998Mutation;

pub fn set_snapshot(snapshot: En1998Snapshot) -> En1998Mutation {
    En1998Mutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut En1998Snapshot, replacement: &En1998Snapshot) {
    *base = replacement.clone();
}
