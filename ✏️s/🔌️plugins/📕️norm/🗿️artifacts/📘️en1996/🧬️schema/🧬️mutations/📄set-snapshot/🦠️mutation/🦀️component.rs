//! SetSnapshot mutation payload + builder + apply.
use crate::artifacts::en1996::En1996Snapshot;
use crate::artifacts::en1996::mutations::En1996Mutation;

pub fn set_snapshot(snapshot: En1996Snapshot) -> En1996Mutation {
    En1996Mutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut En1996Snapshot, replacement: &En1996Snapshot) {
    *base = replacement.clone();
}
