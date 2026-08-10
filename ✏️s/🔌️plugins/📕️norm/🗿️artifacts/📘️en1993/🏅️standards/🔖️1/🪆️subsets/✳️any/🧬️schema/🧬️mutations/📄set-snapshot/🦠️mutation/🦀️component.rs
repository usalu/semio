//! 📸️ En1993 mutation — SetSnapshot payload + builder + apply.
use crate::artifacts::en1993::En1993Snapshot;
use crate::artifacts::en1993::mutations::En1993Mutation;

pub fn set_snapshot(snapshot: En1993Snapshot) -> En1993Mutation {
    En1993Mutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut En1993Snapshot, replacement: &En1993Snapshot) {
    *base = replacement.clone();
}
