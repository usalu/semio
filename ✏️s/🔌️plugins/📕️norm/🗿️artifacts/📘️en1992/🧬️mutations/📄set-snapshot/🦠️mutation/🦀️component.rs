//! 📸️ En1992 mutation — SetSnapshot payload + builder + apply.
use crate::artifacts::en1992::En1992Snapshot;
use crate::artifacts::en1992::mutations::En1992Mutation;

pub fn set_snapshot(snapshot: En1992Snapshot) -> En1992Mutation {
    En1992Mutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut En1992Snapshot, replacement: &En1992Snapshot) {
    *base = replacement.clone();
}
