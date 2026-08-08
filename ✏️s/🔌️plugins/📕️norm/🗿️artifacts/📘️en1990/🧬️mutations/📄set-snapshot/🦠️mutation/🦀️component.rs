//! 📸️ En1990 mutation — SetSnapshot payload + builder + apply.
use crate::artifacts::en1990::En1990Snapshot;
use crate::artifacts::en1990::mutations::En1990Mutation;

pub fn set_snapshot(snapshot: En1990Snapshot) -> En1990Mutation {
    En1990Mutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut En1990Snapshot, replacement: &En1990Snapshot) {
    *base = replacement.clone();
}
