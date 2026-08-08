//! 📸️ En1994 mutation — SetSnapshot payload + builder + apply.
use crate::artifacts::en1994::En1994Snapshot;
use crate::artifacts::en1994::mutations::En1994Mutation;

pub fn set_snapshot(snapshot: En1994Snapshot) -> En1994Mutation {
    En1994Mutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut En1994Snapshot, replacement: &En1994Snapshot) {
    *base = replacement.clone();
}
