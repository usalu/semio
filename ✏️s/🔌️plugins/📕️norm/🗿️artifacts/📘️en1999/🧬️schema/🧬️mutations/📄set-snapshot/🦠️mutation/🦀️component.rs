//! SetSnapshot mutation payload + builder + apply.
use crate::artifacts::en1999::En1999Snapshot;
use crate::artifacts::en1999::mutations::En1999Mutation;

pub fn set_snapshot(snapshot: En1999Snapshot) -> En1999Mutation {
    En1999Mutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut En1999Snapshot, replacement: &En1999Snapshot) {
    *base = replacement.clone();
}
