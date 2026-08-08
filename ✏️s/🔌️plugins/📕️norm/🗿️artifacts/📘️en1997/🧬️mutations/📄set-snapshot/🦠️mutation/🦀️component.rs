//! SetSnapshot mutation payload + builder + apply.
use crate::artifacts::en1997::En1997Snapshot;
use crate::artifacts::en1997::mutations::En1997Mutation;

pub fn set_snapshot(snapshot: En1997Snapshot) -> En1997Mutation {
    En1997Mutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut En1997Snapshot, replacement: &En1997Snapshot) {
    *base = replacement.clone();
}
