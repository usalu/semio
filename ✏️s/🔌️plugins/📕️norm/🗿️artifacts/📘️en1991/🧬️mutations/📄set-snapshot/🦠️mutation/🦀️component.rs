//! 📸️ En1991 mutation — SetSnapshot payload + builder + apply.
use crate::artifacts::en1991::En1991Snapshot;
use crate::artifacts::en1991::mutations::En1991Mutation;

pub fn set_snapshot(snapshot: En1991Snapshot) -> En1991Mutation {
    En1991Mutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut En1991Snapshot, replacement: &En1991Snapshot) {
    *base = replacement.clone();
}
